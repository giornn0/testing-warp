use warp::{reply::Json,Rejection};

pub fn response<T: serde::Serialize>(data:T)-> Result<Json,Rejection>{
  let response = warp::reply::json(&data);
  Ok(response)
}