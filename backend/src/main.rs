use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
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
            .service(list_subjects)
            .service(list_books)
            .service(list_sections)
            .service(list_topics)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[derive(Deserialize)]
struct QueryStruct {
    subject_id: Option<i64>,
    book_id: Option<i64>,
    section_id: Option<i64>
}

#[derive(Serialize, Deserialize)]
struct Subject {
    id: i64,
    name: String,
}

#[derive(Serialize, Deserialize)]
struct Topic {
    id: i64,
    name: String,
    markdown: String
}

#[derive(Deserialize)]
struct UpdateTopic {
    markdown: String,
    topic_id: i64,
}


#[get("/")]
async fn list_subjects(pool: web::Data<SqlitePool>) -> impl Responder {
    // List all Subjects
    match sqlx::query_as!(Subject, "SELECT id, name FROM subjects ORDER BY id")
        .fetch_all(pool.get_ref())
        .await
    {
        Ok(subjects) => HttpResponse::Ok().json(subjects), // Return subjects as JSON
        Err(e) => {
            eprintln!("Error fetching subjects: {}", e);
            HttpResponse::InternalServerError().body("Error fetching subjects")
        }
    }
}

#[get("/books")]
async fn list_books(
    pool: web::Data<SqlitePool>,
    query: web::Query<QueryStruct>,  // Extract query params from the URL
) -> impl Responder {
    let subject_id = &query.subject_id;
    // List all Books
    match sqlx::query_as!(Subject, "SELECT id, name FROM books WHERE subject_id=?", subject_id)
        .fetch_all(pool.get_ref())
        .await
    {
        Ok(books) => HttpResponse::Ok().json(books), // Return books as JSON
        Err(e) => {
            eprintln!("Error fetching books: {}", e);
            HttpResponse::InternalServerError().body("Error fetching books")
        }
    }
}

#[get("/sections")]
async fn list_sections(
    pool: web::Data<SqlitePool>,
    query: web::Query<QueryStruct>,  // Extract query params from the URL
) -> impl Responder {
    let book_id = &query.book_id;
    // List all Sections
    match sqlx::query_as!(Subject, "SELECT id, name FROM sections WHERE book_id=?", book_id)
        .fetch_all(pool.get_ref())
        .await
    {
        Ok(sections) => HttpResponse::Ok().json(sections), // Return sections as JSON
        Err(e) => {
            eprintln!("Error fetching sections: {}", e);
            HttpResponse::InternalServerError().body("Error fetching sections")
        }
    }
}

#[get("/topics")]
async fn list_topics(
    pool: web::Data<SqlitePool>,
    query: web::Query<QueryStruct>,  // Extract query params from the URL
) -> impl Responder {
    let subject_id = &query.subject_id;
    let book_id = &query.book_id;
    let section_id = &query.section_id;
    // List all Topics
    if let Some(section_id) = section_id {
        match sqlx::query_as!(Topic, "SELECT id, name, markdown FROM topics WHERE subject_id=? AND book_id=? AND section_id=?", subject_id, book_id, section_id)
            .fetch_all(pool.get_ref())
            .await
        {
            Ok(topics) => HttpResponse::Ok().json(topics), // Return topics as JSON
            Err(e) => {
                eprintln!("Error fetching sections: {}", e);
                HttpResponse::InternalServerError().body("Error fetching sections")
            }
        }
    } else {
        match sqlx::query_as!(Topic, "SELECT id, name, markdown FROM topics WHERE subject_id=?", subject_id)
        .fetch_all(pool.get_ref())
        .await
    {
        Ok(topics) => HttpResponse::Ok().json(topics), // Return topics as JSON
        Err(e) => {
            eprintln!("Error fetching sections: {}", e);
            HttpResponse::InternalServerError().body("Error fetching sections")
        }
    }
    }
}

#[post("/update")]
async fn update(
    pool: web::Data<SqlitePool>,
    body: web::Json<UpdateTopic>,  // Use JSON for the POST request body
) -> impl Responder {
    let UpdateTopic { markdown, topic_id } = body.into_inner();

    // Update the topic in the database
    match sqlx::query!("UPDATE topics SET markdown = ? WHERE id = ?", markdown, topic_id)
        .execute(pool.get_ref())
        .await
    {
        Ok(result) => {
            if result.rows_affected() > 0 {
                HttpResponse::Ok().body("Topic successfully updated!")
            } else {
                HttpResponse::NotFound().body("No topic found with the given ID")
            }
        }
        Err(e) => {
            eprintln!("Error updating topic: {}", e);
            HttpResponse::InternalServerError().body("Error updating topic")
        }
    }
}