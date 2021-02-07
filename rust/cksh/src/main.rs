use std::env;
use vedaweb;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

const DN: &str = "0.0.0.0:8000";

struct Data {
    r: Vec<Vec<Vec<vedaweb::Rk>>>,
    c: Vec<Vec<i32>>,
    p: Vec<String>,
}

#[get("/{path}")]
async fn hello(data: web::Data<Data>, path: web::Path<String>) -> impl Responder {
    fn gurucet(b: bool, s: &str, h: &str) -> String {
        if b {
            format!("<span style='color:blue'><b>{}</b></span>", s)
        } else {
            format!("<a href=\"{}\">{}</a>", h, s)
        }
    }

    fn lekh(r: &vedaweb::Rk, mulm: &str) -> String {
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
                                gurucet(p.mulm == mulm, &p.rupm, &p.mulm),
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
    }
    let mulm = path.into_inner();
    let mut pos: usize = 0;
    for (i, m) in data.p.iter().enumerate() {
        if *m == mulm {
            pos = i;
            break;
        }
    }

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
            "{}
            <p><b>{}</b> {}</p>
            {}",
            head,
            &mulm,
            {
                let mut v: Vec<usize> = (0..data.p.len())
                    .filter(|i| *i != pos && data.c[pos][*i] > 0)
                    .collect();
                v.sort_by_key(|i| -data.c[pos][*i]);
                v
            }
            .iter()
            .take(10)
            .map(|i| String::from(&data.p[*i]))
            .collect::<Vec<String>>()
            .join(" "),
            data.r
                .iter()
                .enumerate()
                .map(|(mi, m)| {
                    let mulm = &mulm;
                    m.iter().enumerate().map(move |(si, s)| {
                        s.iter()
                            .filter(|r| {
                                r.strata == "A".to_string()
                                    && r.crnani
                                        .iter()
                                        .any(|c| c.iter().any(|p| p.mulm == String::from(mulm)))
                            })
                            .enumerate()
                            .map(|(ri, r)| format!("{}.{}.{}", mi+1, si+1, ri+1) + &lekh(&r, mulm))
                            .collect::<Vec<String>>()
                    })
                })
                .flatten()
                .flatten()
                .collect::<Vec<String>>()
                .join("")
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

    let covariance = {
        let mut sngh: Vec<Vec<i32>> = (0..pdmulani.len())
            .map(|_| vec![0; pdmulani.len()])
            .collect();
        let mut num = 0;
        let count = mndlani.iter().flatten().flatten().count();
        for r in mndlani.iter().flatten().flatten() {
            num += 1;
            println!("{}/{}", num, count);
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
        for i in 0..pdmulani.len() {
            for j in 0..pdmulani.len() {
                sngh[i][j] = rksnkya * sngh[i][j] - pdrgyogh[i] * pdrgyogh[j];
            }
        }
        sngh
    };

    println!("...");

    let data = web::Data::new(Data {
        r: mndlani,
        c: covariance,
        p: pdmulani,
    });

    HttpServer::new(move || App::new().app_data(data.clone()).service(hello))
        .bind(DN)?
        .run()
        .await
}
