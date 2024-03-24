use anyhow::Context;
use futures::prelude::*;
use std::net::{IpAddr, Ipv4Addr};
use tokio::net::TcpListener;
use tokio_serde::formats::Json;
use tokio_util::codec::{Framed, LengthDelimitedCodec};

use crate::{Request, Response};

pub async fn start(ip_addr: Option<IpAddr>, port: Option<u16>) -> anyhow::Result<()> {
    let ip_addr = ip_addr.unwrap_or(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));
    let port = port.unwrap_or(9696);

    // Create a TCP listener which will listen for incoming connections.
    let listener = TcpListener::bind((ip_addr, port))
        .await
        .context("Failed to bind to the TCP socket.")?;
    println!("Listening on: {}", listener.local_addr()?);

    loop {
        // Asynchronously wait for an inbound socket.
        let (stream, addr) = listener
            .accept()
            .await
            .context("Failed to accept new incoming connection.")?;

        println!("Incoming request from client {addr}.");

        // Delimit frames using a length header.
        let transport = Framed::new(stream, LengthDelimitedCodec::new());

        // Serialize frames with JSON.
        let mut framed = tokio_serde::Framed::new(transport, Json::<Request, Response>::default());

        // Spawning a task enables the task to execute concurrently to other tasks.
        tokio::spawn(async move {
            if let Some(request) = framed
                .try_next()
                .await
                .expect("Failed to get request from client")
            {
                println!("Receiving job ({request:?})...");
            }

            let response = Response { max_jobs: 42 };

            // Send data about the busiest server activity in the future.
            framed
                .send(response)
                .await
                .expect("Failed to send response to the client");
        });
    }
}
