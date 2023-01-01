use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result};
use serde::{Deserialize, Serialize};

// route
#[get("/hello/{name}")]
async fn greeting(name: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().json(format!("Hello {name}!"))
}

#[derive(Deserialize, Serialize)]
struct PageQuery {
    page_index: usize,
    page_count: usize,
}

// query
#[get("/books")]
async fn get_books(query: web::Query<PageQuery>) -> Result<impl Responder> {
     let page_query = PageQuery {
        page_index: query.page_index,
        page_count: query.page_count,
    };
    Ok(web::Json(page_query))
}

#[derive(Deserialize, Serialize)]
struct PageBody {
    page_index: usize,
    page_count: usize,
}

// body
#[post("/books")]
async fn create_book(query: web::Json<PageBody>) -> Result<impl Responder> {
    let page_body = PageBody {
        page_index: query.page_index,
        page_count: query.page_count,
    };
    Ok(web::Json(page_body))
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/api")
                    .service(greeting)
                    .service(get_books)
                    .service(create_book)
            )
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
