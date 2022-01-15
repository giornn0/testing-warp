use diesel::{r2d2::{self, ConnectionManager}, PgConnection, Queryable, Insertable};
use serde::{Serialize, Deserialize};

use crate::schema::{contactos,jardines};
pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Queryable,Serialize,Deserialize, Debug)]
pub struct Jardin{
  id: i32,
  nombre: String,
  domicilio: String,
  telefono: String,
  email: String,
  estado: bool,
  created_at: chrono::NaiveDateTime,
  updated_at: chrono::NaiveDateTime
}
#[derive(Serialize,Deserialize,Debug, Insertable, AsChangeset)]
#[table_name = "jardines"]
pub struct NewJardin{
  nombre: String,
  domicilio: String,
  telefono: String,
  email: String,
  estado: Option<bool>,
}

#[derive(Queryable,Serialize, Insertable)]
#[table_name = "contactos"]
pub struct Contacto{
  id: i32,
  telefono: String,
  email: String,
  detalles: String,
  jardin_id:i32,
  created_at: chrono::NaiveDateTime,
  updated_at: chrono::NaiveDateTime
}