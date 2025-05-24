use dxr::{chrono::Date, TryFromParams, TryToValue, Value};
use dxr_server::{
    async_trait, axum::http::HeaderMap, Handler, HandlerFn, HandlerResult, RouteBuilder, Server,
};
use iso8601;
use std::collections::{BTreeMap, HashMap};

fn hello(params: &[Value], _headers: HeaderMap) -> HandlerResult {
    let name = String::try_from_params(params)?;
    Ok(format!("Handler function says: Hello, {name}!").try_to_value()?)
    //Ok(true.try_to_value()?)
}

fn map_h(params: &[Value], _headers: HeaderMap) -> HandlerResult {
    let mut map = HashMap::new();
    map.insert("rust", true);
    map.insert("test", false);
    Ok(map.try_to_value()?)
}

pub async fn serve() -> () {
    let route = RouteBuilder::new()
        .set_path("/")
        .add_method("hello", Box::new(hello as HandlerFn))
        .add_method("map_h", Box::new(map_h as HandlerFn))
        .build();
    let mut server = Server::from_route(route);

    let barrier = server.shutdown_trigger();
    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.unwrap();
        barrier.notify_one();
    });

    println!("Server is running on 3000...");
    server
        .serve("0.0.0.0:3000".parse().unwrap())
        .await
        .expect("Failed to run server.")
}
