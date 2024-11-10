use actix_web::{web, App, HttpServer, Responder};
use mysql::*;
use mysql::prelude::*;
use serde::{Deserialize, Serialize};
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
    let users: Vec<User> = conn.query_map(
        "SELECT id, name FROM users",
        |(id, name)| User { id, name },
    ).unwrap();
    web::Json(users)
}

async fn get_products(pool: web::Data<Pool>) -> impl Responder {
    let mut conn = pool.get_conn().unwrap();
    let products: Vec<Product> = conn.query_map(
        "SELECT id, name, user_id FROM products",
        |(id, name, user_id)| Product { id, name, user_id },
    ).unwrap();
    web::Json(products)
}


#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("Connecting to {}", database_url);
    match Pool::new(database_url) {
        Ok(pool) => {
            HttpServer::new(move || {
                App::new()
                    .app_data(pool.clone())
                    .route("/users", web::get().to(get_users))
                    .route("/products", web::get().to(get_products))
            })
            .bind("0.0.0.0:8080")?
            .run()
            .await
        }
        Err(e) => {
            eprintln!("Failed to connect to the database: {}", e);
            std::process::exit(1);
        }
    }
}