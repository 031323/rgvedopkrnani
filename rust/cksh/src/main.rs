use std::env;
use std::io::{stdout, Write};
use vedaweb;
use std::sync::Arc;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/{path}")]
async fn hello(data: web::Data<Vec<Vec<Vec<vedaweb::Rc>>>>, path: web::Path<String>) -> impl Responder {
    format!("{}\n{}", path.into_inner(), data[0][0][0])
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        println!("c-salt_vedaweb_tei ítyasyá sthā́nam ápekṣyate.");
        std::process::exit(0);
    }
    
    let mndlani = web::Data::new(vedaweb::aropnm(&args[1]));
    
    HttpServer::new(move || {
        App::new()
        		.app_data(mndlani.clone())
            .service(hello)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
