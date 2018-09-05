extern crate tokio_signal;
extern crate tokio;

use tokio::prelude::*;

fn main() {
    let mut rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(future::lazy::<_, Result<(), ()>>(move || {
        // Spawn more futures on the pool here
        Ok(())
    }).map_err(|_| {
        println!("Startup failure");
    })).unwrap();

    let stream = tokio_signal::ctrl_c().flatten_stream().map_err(move |e| {
        println!("Failed to handle ctrl-c event {:?}", e);
    });
    stream.into_future().wait().ok().unwrap();

    println!("Received ctrl-c signal, shutting down...");
    rt.shutdown_now().wait().ok().unwrap();
}
