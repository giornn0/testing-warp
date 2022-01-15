use std::sync::Arc;

use warp::{reply::Json, Rejection, Filter, Reply};
use serde::Serialize;

use crate::{models::Pool, with_pool};

#[derive(Serialize)]
struct Test{
  hello:String,
  id: Option<i64>
}


pub fn jardines_filter(db_pool: Arc<Pool>)->impl Filter<Extract=impl Reply,Error = Rejection> + Clone{
  let scope = warp::path("jardines");
  let list = scope
    .and(warp::get())
    .and(warp::path::end())
    .and(with_pool(db_pool.clone()))
    .and_then(all_jardines);
  let get_one = scope
    .and(warp::get())
    .and(warp::path::param())
    .and(with_pool(db_pool.clone()))
    .and_then(one_jardin);
  list.or(get_one)
}

async fn all_jardines(_db_pool: Arc<Pool>)-> Result<Json,Rejection>{
  let mut list:Vec<Test> = Vec::new();
  let json1 = Test{
      hello:"Pasamos un listado de jardines111".to_string(),
      id:None,
  };
  let json2 = Test{
      hello:"Pasamos un listado de jardines".to_string(),
      id:None
  };
  list.push(json1);
  list.push(json2);

  let response = warp::reply::json(&list);
  Ok(response)
}
async fn one_jardin(id:i64,_db_pool: Arc<Pool>)->Result<Json,Rejection>{
  let json = Test{
    hello:"Prueba con params!".to_string(),
    id: Some(id)
  };
  let response = warp::reply::json(&json);
  Ok(response)
}