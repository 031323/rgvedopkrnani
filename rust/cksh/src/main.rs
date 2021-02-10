use std::env;
use vedaweb;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Anuyogh {
   istm: Option<String>,
   krmh: Option<String>,
}

const DN: &str = "0.0.0.0:8000";
const VB: &str = "https://vedaweb.uni-koeln.de/rigveda/view/id/";

struct Data {
    r: Vec<Vec<Vec<vedaweb::Rk>>>,
    c: Vec<Vec<f32>>,
    //Vec<Vec<i64>>,
    p: Vec<String>,
    y: Vec<i32>,
    n: Vec<f32>,
}

async fn hello(data: web::Data<Data>, web::Query(info): web::Query<Anuyogh>) -> impl Responder {
    

    let lekh = |r: &vedaweb::Rk, istm: &Option<String>| -> String {
        format!(
            "<p>{}</p><p>{}</p>",
            r.smhita.replace("\n", "<br>"),
            r.crnani
                .iter()
                .map(|c| {
                    c.iter()
                        .map(|p| {
                            format!(
                                "{}<sub> {} <span style='font-family: serif'>{}</span></sub>",
                                
                                if match istm {Some(pdm) => p.mulm == String::from(pdm), None => false,} {
            format!("<span style='color:blue'><b>{}</b></span>", &p.rupm)
        } else {
            format!("<a href=\"?{}istm={}\">{}</a>", match &info.krmh {
                Some(krmh) => format!("krmh={}&", krmh),
                None => String::from(""),
            }, &p.mulm, &p.rupm)
        }
                      ,
                                p.mulm,
                                vedaweb::drmnamani(&p)
                                    .iter()
                                    .map(|d| d.to_string())
                                    .filter(|d| !["nominal stem", "root", "invariable", "pronoun"]
                                        .iter()
                                        .any(|t| t.to_string() == d.to_string()))
                                    .collect::<Vec<String>>()
                                    .join(" ")
                            )
                        })
                        .collect::<Vec<String>>()
                        .join(" ")
                })
                .collect::<Vec<String>>()
                .join("<br>")
        )
    };
    
    

    let head = "

        <title>वे॒द॒च॒क्षः</title>

        <style>
            body { font-family: sans-serif; }
            a    { color: blue; text-decoration: none; }
        </style>

    ";

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(format!(
            "{}{}{}",
            head,
            match info.istm {
                Some(ref istm) => format!("<p><b>{}</b> {}</p>", istm, {
                let mut pos: usize = 0;
                for (i, m) in data.p.iter().enumerate() {
                   if *m == String::from(istm) {
                       pos = i;
                       break;
                   }
                }
                let it:Vec<usize> = (0..data.c[0].len()).collect();
                    //(0..data.p.len()).filter(|&i| data.y[i] > 1).collect();
                let posj = &data.c[pos];
                let cos:Vec<f32> = (0..data.p.len()).map(|i| {
                    //println!("{}/{}", i, data.p.len());
                    let ij = &data.c[i];
                    it.iter().fold(0_f32, |mn1, j| mn1 as f32 + posj[*j]*ij[*j]) as f32 / data.n[i]
                }).collect();
                let mut v:Vec<usize> = (0..data.p.len()).filter(|&i| i != pos /*&& data.c[pos][i] < 0*/).collect();
                //v.sort_by_key(|i| -data.c[pos][*i]);
                v.sort_by(|i, j| cos[*j].partial_cmp(&cos[*i]).unwrap());

                v
            }
            .iter()
            .take(10)
            .map(|i| String::from(&data.p[*i]))
            .collect::<Vec<String>>()
            .join(" ")),
            
                None => String::from(""),
            },
            { 
            
            let sadarnkrmh: Vec<(usize, usize, usize)> = {
                let mut v: Vec<(usize, usize, usize)> = Vec::new();
                for mi in (0..data.r.len()) {
                    for si in (0..data.r[mi].len()) {
                        for ri in (0..data.r[mi][si].len()) {
                            v.push((mi, si, ri));
                        }
                    }
                }
                v
            };
            
            let iter: Vec<(usize, usize, usize)> = match info.krmh {
                Some(ref krmh) => std::fs::read_to_string(String::from("../") + krmh).expect("krmh?!").split("\n").map(|l| {
                        let s:Vec<&str> = l.split(".").collect();
                        (s[0].parse::<usize>().unwrap()-1, s[1].parse::<usize>().unwrap()-1, s[2].parse::<usize>().unwrap()-1)
                    }).collect::<Vec<(usize, usize, usize)>>(),
                None => sadarnkrmh,
            };
            
            iter.iter().map(|(mi, si, ri)| (mi, si, ri, &data.r[*mi][*si][*ri]))
                       .filter(|(_, _, _, r)| match info.istm {
                           Some(ref istm) => r.crnani
                                        .iter()
                                        .any(|c| c.iter().any(|p| p.mulm == String::from(istm))),
                           None => true,
                        })
                       .map(|(mi, si, ri, r)| format!("<a href='{}{1}.{2}.{3}'>{}.{}.{}</a>", VB, mi+1, si+1, ri+1) + &lekh(&r, &info.istm))
                       .collect::<Vec<String>>()
                       .join("")
            }
        ))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        println!("c-salt_vedaweb_tei ítyasyá sthā́nam ápekṣyate.");
        std::process::exit(0);
    }

    let mndlani = vedaweb::aropnm(&args[1]).unwrap().0;

    let rksnkya = mndlani.iter().flatten().flatten().count() as i32;

    println!("abc");

    let (pdmulani, pdrgyogh) = {
        let mut pdmulani: Vec<String> = Vec::new();
        let mut pdrgyogh: Vec<i32> = Vec::new();

        for r in mndlani.iter().flatten().flatten() {
            let v = {
                let mut v: Vec<String> = r
                    .crnani
                    .iter()
                    .flatten()
                    .map(|pdm| String::from(&pdm.mulm))
                    .collect();
                v.sort();
                v.dedup();
                v
            };

            for m in v {
                match pdmulani.binary_search(&m) {
                    Ok(pos) => {
                        pdrgyogh[pos] += 1;
                    }
                    Err(pos) => {
                        pdmulani.insert(pos, m);
                        pdrgyogh.insert(pos, 1);
                    }
                }
            }
        }
        (pdmulani, pdrgyogh)
    };

    println!("def");
		println!("pdmulsnkya {}", pdmulani.len());
		
		let vectors = {
		    let mut vectortxt:Vec<Vec<String>> = std::fs::read_to_string("../../glove/vectors.txt").expect("?!").split("\n").map(|line| line.split(" ").map(|s| String::from(s)).collect()).collect();
		    println!("\n{:?}\n", vectortxt.iter().find(|t| t[0]==String::from("abalá-")));
		    vectortxt.sort_by_key(|l| String::from(&l[0]));
		    println!("v len {}", vectortxt.len());
        for v in vectortxt.iter().take(10) {
            println!("{:?}", v);
        }
		    let vectors:Vec<Vec<f32>> = (0..pdmulani.len()).map(|i| vectortxt[i+2].iter().enumerate().filter(|(j, _)| *j>0_usize).map(|(_, f)| f.parse::<f32>().unwrap()).collect()).collect();
		    vectors
		};

		println!("\n{:?}\n", vectors[0]);

    let (covariance, norm) = {
        let mut sngh: Vec<Vec<i64>> = (0..pdmulani.len())
            .map(|_| vec![0; pdmulani.len()])
            .collect();
        let mut num = 0;
        let count = mndlani.iter().flatten().flatten().count();
        /*for r in mndlani.iter().flatten().flatten() {
            num += 1;
            //println!("{}/{}", num, count);
            let rnmulani = {
                let mut rnmulani: Vec<String> = r
                    .crnani
                    .iter()
                    .flatten()
                    .map(|p| String::from(&p.mulm))
                    .collect();
                rnmulani.sort();
                rnmulani.dedup();
                rnmulani
            };
            for (i, pi) in rnmulani
                .iter()
                .map(|m| pdmulani.binary_search(&m).unwrap())
                .enumerate()
            {
                sngh[pi][pi] += 1;
                for qi in rnmulani
                    .iter()
                    .enumerate()
                    .filter(|(j, _)| j < &i)
                    .map(|(_, m)| pdmulani.binary_search(&m).unwrap())
                {
                    sngh[pi][qi] += 1;
                    sngh[qi][pi] += 1;
                }
            }
        }
        */
        let mut norm:Vec<f32> = Vec::new();
        
        for i in 0..pdmulani.len() {
            /*for j in 0..pdmulani.len() {
                sngh[i][j] = rksnkya as i64 * sngh[i][j] - (pdrgyogh[i] as i64) * (pdrgyogh[j] as i64);
            }*/
            //////////////////// norm replace sngh>vector
            let nn = ((0..vectors[0].len()).fold(0_f32, |n1, j| n1 + vectors[i][j]*vectors[i][j])).sqrt();
            if nn==0.0 {
                println!("\n0: {}\n", pdmulani[i]);
            }
            norm.push(nn);
        }
        (vectors, norm)
    };

    println!("...");

    let data = web::Data::new(Data {
        r: mndlani,
        c: covariance,
        p: pdmulani,
        y: pdrgyogh,
        n: norm,
    });

    HttpServer::new(move || App::new().app_data(data.clone()).service(web::resource("/").route(web::get().to(hello))))
        .bind(DN)?
        .run()
        .await
}
