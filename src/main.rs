#[macro_use]
extern crate rocket;

use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{Build, Rocket};
use rocket_sync_db_pools::database;
use rocket_sync_db_pools::diesel;

use diesel::prelude::*;
use diesel::PgConnection;
use dotenv::dotenv;
use std::env;

mod models;
mod schema;

use crate::models::{NovoUsuario, Usuario};
use crate::schema::usuario;

#[database("postgres_db")]
struct DbConn(diesel::PgConnection);

#[get("/usuarios")]
async fn get_usuarios(conn: DbConn) -> Result<Json<Vec<Usuario>>, Status> {
    match conn.run(|c| usuario::table.load::<Usuario>(c)).await {
        Ok(usuarios) => Ok(Json(usuarios)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[post("/usuarios", format = "json", data = "<novo_usuario>")]
async fn create_usuario(
    conn: DbConn,
    novo_usuario: Json<NovoUsuario>,
) -> Result<Json<Usuario>, Status> {
    let usuario = Usuario {
        id: 0, // O ID será atribuído automaticamente pelo banco de dados
        name: novo_usuario.name.clone(),
        password: novo_usuario.password.clone(),
        email: novo_usuario.email.clone(),
        phone: novo_usuario.phone.clone(),
    };

    match conn
        .run(|c| {
            diesel::insert_into(usuario::table)
                .values(&usuario)
                .get_result(c)
        })
        .await
    {
        Ok(usuario_inserido) => Ok(Json(usuario_inserido)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[launch]
fn rocket() -> Rocket<Build> {
    dotenv().ok();
    rocket::build()
        .attach(DbConn::fairing())
        .mount("/", routes![get_usuarios, create_usuario])
}
