extern crate diesel;

use std::sync::Arc;
use diesel::prelude::*;
use warp::{reply::Json, Rejection, Filter, Reply};

use crate::{models::{Pool,NewJardin, Jardin, SearchQuery}, with_pool, schema::jardines, response::response};



pub fn jardines_filter(db_pool: &Arc<Pool>)->impl Filter<Extract=impl Reply,Error = Rejection> + Clone{
  let scope = warp::path("jardines");
  let list = scope
    .and(warp::get())
    .and(warp::query())
    .and(warp::path::end())
    .and(with_pool(db_pool.clone()))
    .and_then(all_jardines);
  let get_one = scope
    .and(warp::get())
    .and(warp::path::param())
    .and(with_pool(db_pool.clone()))
    .and_then(one_jardin);
  let create = scope
    .and(warp::post())
    .and(warp::body::json())
    .and(with_pool(db_pool.clone()))
    .and_then(create_jardin);
  let update = scope
    .and(warp::put())
    .and(warp::path::param())
    .and(warp::body::json())
    .and(with_pool(db_pool.clone()))
    .and_then(update_jardin);
  let delete = scope
    .and(warp::delete())
    .and(warp::path::param())
    .and(with_pool(db_pool.clone()))
    .and_then(delete_jardin);
  list.or(get_one).or(create).or(update).or(delete)
}

async fn all_jardines(query:SearchQuery,db_pool: Arc<Pool>)-> Result<Json,Rejection>{
  use crate::schema::jardines::dsl::jardines;
  let conn = db_pool.get().unwrap();
  let result = if let Some(take) = query._take {
    jardines.limit(take).offset(if let Some(page)= query._page{(page-1)*take}else{0}).load::<Jardin>(&conn).expect("Error while retrieving jardines")
  } else{
    jardines.load::<Jardin>(&conn).expect("Error while retrieving jardines")
  };
  response(result)
  
}
async fn one_jardin(id:i32,db_pool: Arc<Pool>)->Result<Json,Rejection>{
  use crate::schema::jardines::dsl::jardines;
  let conn = db_pool.get().unwrap();
  let result:Jardin = jardines.find(id).get_result(&conn).expect("Error while getting jardin");
  response(result)

}
async fn create_jardin(value:NewJardin, db_pool: Arc<Pool>)-> Result<Json,Rejection>{
  let conn = db_pool.get().unwrap();
  let result:Jardin = diesel::insert_into(jardines::table).values(&value).get_result(&conn).expect("Error while creating jardin");
  response(result)

}
async fn update_jardin(id:i32,value:NewJardin, db_pool: Arc<Pool>) -> Result<Json,Rejection>{
  use crate::schema::jardines::dsl::jardines;
  let conn = db_pool.get().unwrap();
  let result: Jardin = diesel::update(jardines.find(id)).set(value).get_result(&conn).expect("Error updating the jardin");
  response(result)
}
async fn delete_jardin(delete_id:i32, db_pool: Arc<Pool>)-> Result<Json,Rejection>{
  use crate::schema::jardines::dsl::{jardines, id};
  let conn = db_pool.get().unwrap();
  let result = diesel::delete(jardines.filter(id.eq(delete_id))).execute(&conn).expect("Error while deleting jardin");
  response(result)
}