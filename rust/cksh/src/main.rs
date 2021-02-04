use std::env;
use vedaweb;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

const DN: &str = "127.0.0.1:8080";

#[get("/{path}")]
async fn hello(data: web::Data<Vec<Vec<Vec<vedaweb::Rk>>>>, path: web::Path<String>) -> impl Responder {
    fn gurucet(b: bool, s: &str, h:&str) -> String {
        if b { format!("<span style='color:blue'><b>{}</b></span>", s) }
        else { format!("<a href=\"{}\">{}</a>", h, s) }
    }
  
    fn lekh(r: &vedaweb::Rk, mulm: &str) -> String {
        format!("<p>{}</p><p>{}</p>", r.smhita.replace("\n", "<br>"), r.crnani.iter().map(|c| { c
            .iter()
            .map(|p| format!("{}<sub> {} <span style='font-family: serif'>{}</span></sub>", gurucet(p.mulm == mulm, &p.rupm, &p.mulm), p.mulm, vedaweb::drmnamani(&p).iter().map(|d| d.to_string()).filter(|d| ! ["nominal stem", "root", "invariable", "pronoun"].iter().any(|t| t.to_string() == d.to_string())).collect::<Vec<String>>().join(" ")))
            .collect::<Vec<String>>()
            .join(" ")
        }).collect::<Vec<String>>().join("<br>")
        )
    }
    let s = path.into_inner();
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(format!("<title>वे॒द॒च॒क्षः</title><style>a {{ color: blue; text-decoration: none; }}</style>{}", data.iter().flatten().flatten().filter(|r| r.strata == "A".to_string() && r.crnani.iter().any(|c| {
            c.iter().any(|p| p.mulm == s)
        })).map(|r| lekh(&r, &s)).collect::<Vec<String>>().join("\n\n")))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        println!("c-salt_vedaweb_tei ítyasyá sthā́nam ápekṣyate.");
        std::process::exit(0);
    }
    
    let mndlani = web::Data::new(vedaweb::aropnm(&args[1]).unwrap().0);
 /*
 		for m in mndlani.iter() {
		for s in m {
    	for r in s {
    		for c in &r.crnani {
    			for p in c {
    				let ds = vedaweb::drmnamani(&p);
    				if ["nominal stem", "root", "invariable", "pronoun"].iter().fold(0, |n, t| n + if ds.iter().any(|d| d.to_string() == t.to_string()) {1} else { 0 }) > 1 {
    				//if ds.iter().any(|d| d.to_string()=="1") && ds.iter().any(|d| d.to_string()=="2") {
    					println!("{}", &p);
    				}    				
    			}
    		}
    	}
    }
    }*/
        
    HttpServer::new(move || {
        App::new()
        		.app_data(mndlani.clone())
            .service(hello)
    })
    .bind(DN)?
    .run()
    .await
}
