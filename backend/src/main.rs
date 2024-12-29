use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
// use actix_web::http::header::LOCATION;
use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqlitePool;
use actix_cors::Cors;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Create database with connection and share it using with_state below
    dotenv::dotenv().ok();
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = match SqlitePool::connect(&url).await {
        Ok(pool) => pool,
        Err(e) => {
            eprintln!("Error connecting to database: {}", e);
            return Err(std::io::Error::new(std::io::ErrorKind::Other, e));
        }
    };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone())) // Share the pool with all handlers
            .wrap(Cors::permissive()) // Allow all origins, similar to `CorsLayer::very_permissive()`
            .service(list)
            .service(create)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[derive(Deserialize)]
struct NewTodo {
    description: String,
}

#[derive(Serialize, Deserialize)]
struct Todo {
    id: i64,
    description: String,
    done: bool,
}

#[get("/")]
async fn list(pool: web::Data<SqlitePool>) -> impl Responder {
    // List all todos
    match sqlx::query_as!(Todo, "SELECT id, description, done FROM todos ORDER BY id")
        .fetch_all(pool.get_ref())
        .await
    {
        Ok(todos) => HttpResponse::Ok().json(todos), // Return todos as JSON
        Err(e) => {
            eprintln!("Error fetching todos: {}", e);
            HttpResponse::InternalServerError().body("Error fetching todos")
        }
    }
}

#[get("/create")]
// async fn create(pool: web::Data<SqlitePool>, form: web::Form<NewTodo>) -> impl Responder {
//     // List all todos
//     match sqlx::query!(
//         "INSERT INTO todos (description) VALUES (?)",
//         form.description,
//     )
//         .execute(pool.get_ref())
//         .await
//     {
//         Ok(_) => HttpResponse::Found()
//             .append_header((LOCATION, "http://localhost:5173")) // Use append_header for the Location header
//             .finish(),
//         Err(e) => {
//             eprintln!("Error creating todo: {}", e);
//             HttpResponse::InternalServerError().body("Error creating todo")
//         }
//     }
// }
async fn create(
    pool: web::Data<SqlitePool>, // Shared database pool
    query: web::Query<NewTodo>, // Extract query parameters from the URL
) -> impl Responder {
    // Insert the new todo into the database
    match sqlx::query!(
        "INSERT INTO todos (description, done) VALUES (?, ?)",
        query.description,
        false, // Default `done` to `false`
    )
    .execute(pool.get_ref())
    .await
    {
        Ok(_) => HttpResponse::Ok().body("Successfully inserted todo!"),
        Err(e) => {
            eprintln!("Error inserting todo: {}", e);
            HttpResponse::InternalServerError().body("Error inserting todo")
        }
    }
}


#[get("/delete/{id}")]
async fn delete(
    pool: web::Data<SqlitePool>, // Shared database pool
    path: web::Path<i64>,        // Extract `id` from the URL path
) -> impl Responder {
    let id = path.into_inner(); // Get the `id` from the URL

    // Delete the todo with the given `id` from the database
    match sqlx::query!("DELETE FROM todos WHERE id = ?", id)
        .execute(pool.get_ref())
        .await
    {
        Ok(_) => HttpResponse::Ok().body(format!("Successfully deleted todo with id {}", id)),
        Err(e) => {
            eprintln!("Error deleting todo: {}", e);
            HttpResponse::InternalServerError().body("Error deleting todo")
        }
    }
}