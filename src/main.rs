use actix_web::{web, App, HttpServer, Responder};
use actix_web::middleware::Logger;
use actix_files as fs;

// msg for curl response
async fn hello() -> impl Responder {
    format!("Hello CLIENT, your request was SUCCESSFUL!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

  // server configuration -------------------------------------------------------
  let ip_add_v4 = "localhost";
  let port_no: u16 = 8080;
  // ----------------------------------------------------------------------------
  
  // start event logger
  env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    
  // msg server started
    println!("Server started on {ip_add_v4} port {port_no}");

    HttpServer::new(|| {
      App::new()
          // for curl response ---------------
          // .route("/", web::get().to(hello))

          // server services -----------------
          // .service(Files::new("/images", "static/images/").show_files_listing())
          // .service(Files::new("/", "./static/root/").index_file("index.html"))
          .service(fs::Files::new("/", ".").show_files_listing())

          // Enable the logger.
          .wrap(Logger::default())
  })
  .bind((ip_add_v4, port_no))?
  .run()
  .await
}