#[macro_use]
extern crate diesel;

use std::{convert::Infallible, sync::Arc};
use serde::Serialize;
use warp::{Filter, Rejection, reply::Json};
use diesel::{r2d2::{ConnectionManager, self}, PgConnection};
use crate::{handlers::{jardines::jardines_filter, contactos::contactos_filter, image::images_filter}, models::Pool};

mod models;
mod handlers;
mod schema;
mod response;

#[derive(Serialize)]
struct Serving{
    message:String,
    author:String
}

#[tokio::main]
async fn main()-> Result<(), Box<dyn std::error::Error>> {
    let _ = dotenv::dotenv();
    let port = std::env::var("PORT")
        .ok()
        .map(|val| val.parse::<u16>())
        .unwrap_or(Ok(8080))?;

    let started = warp::path("api").and(warp::get()).and_then(serve_start);
    
    let db_url = std::env::var("DATABASE_URL").expect("Missing database credentials!");
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    let pool:Arc<Pool> = Arc::new(r2d2::Pool::builder().build(manager).expect("Failed connection to database!"));


    let fallback = warp::any().map(|| "Ninguna pagina!");
    let download_route = warp::path("files").and(warp::fs::dir("./files/"));
    let apis = started.or(jardines_filter(&pool)).or(contactos_filter(&pool)).or(images_filter()); 
    let routes = apis.or(download_route).or(fallback);

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
async fn serve_start()->Result<Json,Rejection>{
    let api_service = warp::reply::json(&Serving{
        message:"Bienvenido, API con warp/postgres".to_string(),
        author:"giornn0".to_string()
    });
    Ok(api_service)
}
pub fn with_pool (db_pool: Arc<Pool>)->impl Filter<Extract=(Arc<Pool>,),Error= Infallible> + Clone{
    warp::any().map(move||db_pool.clone())
}