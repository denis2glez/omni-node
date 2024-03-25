use anyhow::Context;
use chrono::{DateTime, Local};
use futures::prelude::*;
use rand::Rng;
use std::net::{IpAddr, Ipv4Addr};
use std::thread::sleep;
use std::time::Duration;
use tokio::net::TcpStream;
use tokio_serde::formats::MessagePack;
use tokio_util::codec::{Framed, LengthDelimitedCodec};
use uuid::Uuid;

use crate::{Request, Response};

// The client terminates after sending 30 jobs.
const REQUESTS_NUMBER: usize = 30;

pub async fn start(ip_addr: Option<IpAddr>, port: Option<u16>) -> anyhow::Result<()> {
    let ip_addr = ip_addr.unwrap_or(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));
    let port = port.unwrap_or(9696);

    let mut rng = rand::thread_rng();

    for _ in 0..REQUESTS_NUMBER {
        let stream = TcpStream::connect((ip_addr, port))
            .await
            .context("Failed to open a TCP connection to the server.")?;

        // Delimit frames using a length header.
        let transport = Framed::new(stream, LengthDelimitedCodec::new());

        // Serialize frames with MessagePack.
        let mut framed =
            tokio_serde::Framed::new(transport, MessagePack::<Response, Request>::default());

        let now = Local::now().timestamp_millis();

        let job = Request {
            start_time: DateTime::from_timestamp_millis(rng.gen_range(now..now + 600_000)).unwrap(),
            duration: Duration::from_millis(rng.gen_range(10_000..1_000_000)),
            id: Uuid::new_v4(),
        };

        // Send a job description to the server.
        framed
            .send(job)
            .await
            .context("Failed to send request to server")?;

        if let Some(response) = framed
            .try_next()
            .await
            .context("Failed to get response from server")?
        {
            tracing::info!(
                "The busiest server activity in the future contains {} jobs.",
                response.max_jobs
            );
        }

        // Send job descriptions at 5-second intervals.
        sleep(Duration::from_secs(5));
    }

    Ok(())
}
