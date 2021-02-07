use std::env;
use vedaweb;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

const DN: &str = "127.0.0.1:8080";

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
    let s = path.into_inner();
    let mut pos:usize = 0;
    for (i, m) in data.p.iter().enumerate() {
        if *m == s {
            pos = i;
            break;
        }
    }
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(format!("<title>वे॒द॒च॒क्षः</title><style>body {{font-family: sans-serif}} a {{ color: blue; text-decoration: none; }}</style>{}<br><br>{}", {
                let mut v = vec![(0); 10];
                for data.c[pos].iter().enumerate()
                v
            }.map(|i| data.p[i]).collect::<Vec<String>>().join(" "),  data.r.iter().flatten().flatten().filter(|r| r.strata == "A".to_string() && r.crnani.iter().any(|c| {
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

    let mndlani = vedaweb::aropnm(&args[1]).unwrap().0;

    let rksnkya = mndlani
        .iter()
        .flatten()
        .flatten().count() as i32;

    println!("abc");

    let (pdmulani, pdrgyogh) = {
        let mut pdmulani: Vec<String> = Vec::new();
        let mut pdrgyogh: Vec<i32> = Vec::new();

        for r in mndlani.iter().flatten().flatten() {
            let v = {
                let mut v: Vec<String> = r.crnani.iter().flatten().map(|pdm| String::from(&pdm.mulm)).collect();
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
        let mut sngh: Vec<Vec<i32>> = (0..pdmulani.len()).map(|_| vec![0; pdmulani.len()]).collect();
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
            for (i, p) in rnmulani.iter().enumerate() {
                for (j, q) in rnmulani.iter().enumerate() {
                    if i > j {
                        break;
                    }
                    let pi = pdmulani.binary_search(p).unwrap();
                    let qi = pdmulani.binary_search(q).unwrap();

                    sngh[pi][qi] += 1;
                    sngh[qi][pi] += 1;
                }
            }
        }
        (0..pdmulani.len()).map(|i| (0..pdmulani.len()).map(|j| rksnkya * sngh[i][j] - pdrgyogh[i]*pdrgyogh[j]).collect()).collect()
    };
    /*
        let covariance: Vec<Vec<f64>> = pdmulani
            .iter()
            .enumerate()
            .map(|(p, pm)| {
                println!("{} {}/{}", pm, p, pdmulani.len());
                pdmulani
                    .iter()
                    .enumerate()
                    .filter(|(q, _)| q > &p)
                    .map(|(q, qm)| {
                        println!("{} {}/{}", qm, q, pdmulani.len());
                        mndlani.iter().flatten().flatten().fold(0_f64, |c, r| {
                            let (ps, qs, ss) =
                                r.crnani
                                    .iter()
                                    .flatten()
                                    .fold((0, 0, 0), |(ps1, qs1, ss1), pdm| {

                                        if *pm == pdm.mulm {
                                            (ps1+1, qs1, ss1)
                                        }
                                        else if *qm == pdm.mulm {
                                            (ps1, qs1+1, ss1)
                                        }
                                        else {
                                            (ps1, qs1, ss1+1)
                                        }
                                        /*
                                        (
                                            ps1 + if *pm == *m { 1 } else { 0 },
                                            qs1 + if *qm == *m { 1 } else { 0 },
                                            ss1 + 1,
                                        )*/
                                    });
                            let ss2 = ss + ps + qs;
                            c + ({ ps as f64 } / { ss2 as f64 } - mulavrttyh[p])
                                * ({ qs as f64 } / { ss2 as f64 } - mulavrttyh[q])
                        })
                    })
                    .collect()
            })
            .collect();
    */
    println!("...");

    let data = web::Data::new(Data {
        r: mndlani,
        c: covariance,
        p: pdmulani,
    });

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

    HttpServer::new(move || App::new().app_data(data.clone()).service(hello))
        .bind(DN)?
        .run()
        .await
}
