use actix_web::{web, App, HttpServer, Responder};
use mysql::*;
use mysql::prelude::*;
use serde::{Deserialize, Serialize};
use dotenv::dotenv;
use std::env;

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: u32,
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Product {
    id: u32,
    name: String,
    user_id: u32,
}

async fn get_users(pool: web::Data<Pool>) -> impl Responder {
    let mut conn = pool.get_conn().unwrap();
    let users: Vec<User> = conn.query("SELECT id, name FROM users")
        .unwrap()
        .map(|row| User {
            id: row.get(0),
            name: row.get(1),
        })
        .collect();
    web::Json(users)
}

async fn get_products(pool: web::Data<Pool>) -> impl Responder {
    let mut conn = pool.get_conn().unwrap();
    let products: Vec<Product> = conn.query("SELECT id, name, user_id FROM products")
        .unwrap()
        .map(|row| Product {
            id: row.get(0),
            name: row.get(1),
            user_id: row.get(2),
        })
        .collect();
    web::Json(products)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = Pool::new(database_url).unwrap();

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .route("/users", web::get().to(get_users))
            .route("/products", web::get().to(get_products))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
