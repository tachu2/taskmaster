//! # Simple example client
//!
//! Run this example to test interaction with the `server` example from the
//! `dxr_server_axum` crate.

use std::collections::HashMap;

use dxr_client::{ClientBuilder, Url};

#[tokio::main]
async fn main() -> Result<(), String> {
    let url = Url::parse("http://0.0.0.0:3000/").expect("Failed to parse hardcoded URL.");

    let client = ClientBuilder::new(url)
        .user_agent("dxr-client-example")
        .build();

    let result: String = client
        .call("hello", "DXR")
        .await
        .map_err(|error| error.to_string())?;
    println!("Server message: {result}");

    let result: HashMap<String, bool> = client
        .call("map_h", ())
        .await
        .map_err(|error| error.to_string())?;
    println!("Server counter: {:?}", result);

    Ok(())
}
