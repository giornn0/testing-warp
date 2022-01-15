use warp::{Filter, reply::Json};
use serde_json::json;

#[tokio::main]
async fn main()-> Result<(), Box<dyn std::error::Error>> {
    let port = std::env::var("PORT")
        .ok()
        .map(|val| val.parse::<u16>())
        .unwrap_or(Ok(8080))?;

    let hello = warp::path("hello").and(warp::get()).and(warp::path::end()).and_then( json_response);

    let fallback = warp::any().map(|| "Ninguna pagina!");

    let routes = hello.or(fallback);

    println!("Starting server on port: {}", port);

    let (_addr, server)  = warp::serve(routes).bind_with_graceful_shutdown(([0,0,0,0],port), async {
        tokio::signal::ctrl_c()
            .await
            .expect("Http server: Failed to listen to CTRL+C");
        println!("Shutting down HTTP Server!");
    });
    server.await;

    Ok(())
}
async fn json_response()-> Result<Json,warp::Rejection>{
    let json = json!({"hello":"Prueba de Warp!"});
    let response = warp::reply::json(&json);
    Ok(response)
}