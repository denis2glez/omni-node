use anyhow::Context;
use chrono::Local;
use futures::prelude::*;
use std::{
    collections::BTreeSet,
    net::{IpAddr, Ipv4Addr},
    sync::{Arc, Mutex},
};
use tokio::net::{TcpListener, TcpStream};
use tokio_serde::formats::MessagePack;
use tokio_util::codec::{Framed, LengthDelimitedCodec};

use crate::{Request, Response};

type JobDb = Arc<Mutex<BTreeSet<Request>>>;

pub async fn start(ip_addr: Option<IpAddr>, port: Option<u16>) -> anyhow::Result<()> {
    let ip_addr = ip_addr.unwrap_or(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));
    let port = port.unwrap_or(9696);

    // Create a TCP listener which will listen for incoming connections.
    let listener = TcpListener::bind((ip_addr, port))
        .await
        .context("Failed to bind to the TCP socket.")?;
    tracing::debug!("Listening on: {}", listener.local_addr()?);

    // Shared state guarded by a mutex.
    let db = Arc::new(Mutex::new(BTreeSet::new()));

    loop {
        // Asynchronously wait for an inbound socket.
        let (stream, addr) = listener
            .accept()
            .await
            .context("Failed to accept new incoming connection.")?;

        tracing::debug!("Incoming request from client {addr}.");

        // Clone the handle to the set.
        let db = db.clone();
        // Spawning a task enables the task to execute concurrently to other tasks.
        tokio::spawn(async move {
            process_request(stream, db).await;
        });
    }
}

async fn process_request(stream: TcpStream, db: JobDb) {
    // Delimit frames using a length header.
    let transport = Framed::new(stream, LengthDelimitedCodec::new());

    // Serialize frames with MessagePack.
    let mut framed =
        tokio_serde::Framed::new(transport, MessagePack::<Request, Response>::default());
    if let Some(request) = framed
        .try_next()
        .await
        .expect("Failed to get request from client")
    {
        tracing::debug!("Receiving job ({request:?})...");

        // Given that job data is relatively small, contention should play a larger role.
        // Attention: Critical section.
        let jobs_shard: Vec<_> = {
            let mut db = db.lock().unwrap();
            // Insert the new job.
            db.insert(request);

            // Create a contiguous copy.
            db.iter().cloned().collect()
        };

        let max_jobs = job_metrics(jobs_shard);

        // Send data about the busiest server activity in the future.
        framed
            .send(Response { max_jobs })
            .await
            .expect("Failed to send response to the client");
    }
}

/// Compute the metrics about about scheduled jobs, logging the list the currently running jobs and
/// returning the maximum number of jobs scheduled at the busiest interval.
///
/// # Remark
/// In a different scenario where the job data is larger, a more specific data structure, such as a
/// treap (random binary search tree), could be implemented to avoid copying.
fn job_metrics(jobs_shard: Vec<Request>) -> usize {
    let mut jobs: Vec<_> = jobs_shard
        .iter()
        .flat_map(|req| {
            [
                (req.start_time, Bound::Start),
                (req.start_time + req.duration, Bound::End),
            ]
        })
        .collect();

    jobs.sort_unstable();

    // Determine the maximum number of jobs scheduled at the busiest interval.
    let max_jobs = jobs
        .iter()
        .scan(0, |count, (_, bound)| {
            if *bound == Bound::Start {
                *count += 1;
            } else {
                *count -= 1;
            }
            Some(*count)
        })
        .max()
        .unwrap();

    // List the currently running jobs.
    let now = Local::now();
    let currents: Vec<_> = jobs_shard
        .into_iter()
        .filter(|req| req.start_time <= now && now <= req.start_time + req.duration)
        .collect();

    let span = tracing::info_span!("Currently running jobs");

    span.in_scope(|| {
        tracing::info!("{currents:?}");
    });

    max_jobs
}

#[derive(PartialOrd, Ord, PartialEq, Eq, Debug)]
enum Bound {
    Start,
    End,
}
