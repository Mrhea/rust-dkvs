use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn index() -> impl Responder {
    HttpResponse::Ok().body("This is the index.")
}

async fn get_hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

async fn get_test() -> impl Responder {
    HttpResponse::Ok().body("GET request received.")
} 

async fn post_test(bytes: web::Bytes) -> Result<String, HttpResponse> {
    match String::from_utf8(bytes.to_vec()) {
        Ok(text) => Ok(format!("Hello, {}!\n", text)),
        Err(_) => Err(HttpResponse::BadRequest().into())
    }
}

async fn post_hello() -> impl Responder {
    HttpResponse::BadRequest()
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/"   ,  web::get().to(index))
            .route("/getHello", web::get().to(get_hello))
            .route("/getTest" , web::get().to(get_test))
            .route("/postHello", web::post().to(post_hello))
            .route("/postTest" , web::post().to(post_test))
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
