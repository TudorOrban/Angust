use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use mysql::*;
use mysql::prelude::*;
use serde::{Deserialize, Serialize};

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
    let mut conn = match pool.get_conn() {
        Ok(conn) => conn,
        Err(e) => {
            eprintln!("Failed to get DB connection: {}", e);
            return HttpResponse::InternalServerError().body("Failed to connect to database");
        }
    };

    let users: Result<Vec<User>, mysql::Error> = conn.query_map(
        "SELECT id, name FROM users",
        |(id, name)| User { id, name },
    );

    match users {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(e) => {
            eprintln!("Query failed: {}", e);
            HttpResponse::InternalServerError().body("Failed to execute query")
        }
    }
}

async fn get_products(pool: web::Data<Pool>) -> impl Responder {
    let mut conn = match pool.get_conn() {
        Ok(conn) => conn,
        Err(e) => {
            eprintln!("Failed to get DB connection: {}", e);
            return HttpResponse::InternalServerError().body("Failed to connect to database");
        }
    };

    let products: Result<Vec<Product>, mysql::Error> = conn.query_map(
        "SELECT id, name, user_id FROM products",
        |(id, name, user_id)| Product { id, name, user_id },
    );

    match products {
        Ok(products) => HttpResponse::Ok().json(products),
        Err(e) => {
            eprintln!("Query failed: {}", e);
            HttpResponse::InternalServerError().body("Failed to execute query")
        }
    }
}


#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let database_url = "mysql://root@mysql:3306/chain_optim_database";
    println!("Connecting to {}", database_url);

    match Pool::new(database_url) {
        Ok(pool) => {
            println!("Connected to {}", database_url);
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