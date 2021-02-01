use std::env;
use vedaweb;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

#[get("/{path}")]
async fn hello(data: web::Data<Vec<Vec<Vec<vedaweb::Rc>>>>, path: web::Path<String>) -> impl Responder {
    fn lekh(r: &vedaweb::Rc) -> String {
        format!("<p>{}</p><p>{}</p>", r.smhita.replace("\n", "<br>"), r.crnani.iter().map(|c| { c
            .iter()
            .map(|p| format!("<b>{}</b>", p.rupm))
            .collect::<Vec<String>>()
            .join(" ")
        }).collect::<Vec<String>>().join("<br>")
        )
    }
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(format!("<title>वे॒द॒च॒क्षः</title>{}{}", path.into_inner(), lekh(&data[0][0][0])))
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
