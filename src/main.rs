// use actix_web::{web, App, HttpResponse, HttpServer, Responder};

// fn getHello() -> impl Responder {
//     HttpResponse::Ok().body("Hello, world!")
// }

// fn getTest() -> impl Responder {
//     HttpResponse::Ok().body("GET request received.")
// } 

// // async fn postHello() -> impl Responder {

// // }

// // async fn postTest() -> impl Responder {

// // }

// #[actix_rt::main]
// fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         App::new()
//             .route("/getHello", web::get().to(getHello))
//             .route("/getTest" ,  web::get().to(getTest))
//     })
//     .bind("127.0.0.1:8088")?
//     .run()
//     .await
// }
fn main() {
    println!("Hello again");
}
