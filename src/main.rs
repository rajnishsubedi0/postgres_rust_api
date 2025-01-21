use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use tokio_postgres::{NoTls, Client};
use dotenv::dotenv;
use std::env;

// Define a struct for incoming requests
#[derive(Serialize, Deserialize)]
struct User {
    name: String,
    email: String,
}

// Function to establish PostgreSQL connection
async fn connect_db() -> Client {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let (client, connection) = tokio_postgres::connect(&database_url, NoTls)
        .await
        .expect("Failed to connect to the database");

    // Spawn a new async task to handle the database connection
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Database connection error: {}", e);
        }
    });

    client
}

// Handler to fetch all users
async fn get_users() -> impl Responder {
    let client = connect_db().await;
    let rows = client.query("SELECT name, email FROM users", &[]).await.unwrap();

    let users: Vec<User> = rows.iter()
        .map(|row| User {
            name: row.get("name"),
            email: row.get("email"),
        })
        .collect();

    HttpResponse::Ok().json(users)
}

// Handler to add a user
async fn add_user(user: web::Json<User>) -> impl Responder {
    let client = connect_db().await;
    let result = client
        .execute("INSERT INTO users (name, email) VALUES ($1, $2)", &[&user.name, &user.email])
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().body("User added successfully"),
        Err(_) => HttpResponse::InternalServerError().body("Failed to add user"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🚀 Server running at http://localhost:8080");

    HttpServer::new(|| {
        App::new()
            .route("/users", web::get().to(get_users))
            .route("/users", web::post().to(add_user))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
