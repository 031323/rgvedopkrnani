use std::env;
use vedaweb;
use std::process::Command;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Anuyogh {
    krmh: Option<String>,
    istm: Option<String>,
}

const DN: &str = "0.0.0.0:8000";

struct Data {
    r: Vec<Vec<Vec<vedaweb::Rk>>>,
    c: Vec<Vec<f32>>,
    p: Vec<String>,
    n: Vec<f32>,
}

async fn hello(data: web::Data<Data>, web::Query(info): web::Query<Anuyogh>) -> impl Responder {
    let nirdesh = |krmh: &Option<String>, istm: &Option<String>| -> String {
        let f = |nam: &str, os: &Option<String>| match os {
            Some(s) => nam.to_string() + "=" + &s,
            None => String::new(),
        };
        format!(
            "?{}{}{}",
            f("krmh", krmh),
            if krmh.is_some() && istm.is_some() {
                "&"
            } else {
                ""
            },
            f("istm", istm)
        )
    };

    let krmnirdesh = |krmh: &Option<String>| nirdesh(krmh, &info.istm);
    let istnirdesh = |istm: &Option<String>| nirdesh(&info.krmh, istm);

    let lekh = |r: &vedaweb::Rk| -> String {
        format!(
            "<p onclick='suvacnarmbh(\"{}\", function(){{}});'>{}</p><p>{}</p>",
            "",//String::from_utf8(Command::new("python3").arg("../iast235.py").arg(&r.smhita).output().unwrap().stdout).unwrap().replace("\n", " "),
            r.smhita.replace("\n", "<br>"),
            r.crnani
                .iter()
                .map(|c| {
                    c.iter()
                        .map(|p| {
                            format!(
                                "{}<sub> {} <span style='font-family: serif'>{}</span></sub>",
                                match &info.istm {
                                    Some(pdm) if p.mulm == String::from(pdm) =>
                                        format!("<span class='b'>{}</span>", &p.rupm),
                                    _ => format!(
                                        "<a href='{}'>{}</a>",
                                        istnirdesh(&Some(String::from(&p.mulm))),
                                        &p.rupm
                                    ),
                                },
                                p.mulm,
                                vedaweb::drmnamani(&p)
                                    .iter()
                                    .map(|d| d.to_string())
                                    .filter(|d| !["nominal stem", "root", "invariable", "pronoun"]
                                        .iter()
                                        .any(|t| t.to_string() == *d))
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
            .b   { color: blue; font-weight: bold; }
        </style>
    ";

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(format!(
            "{}<p>{}</p>{}{}",
            head,
            match &info.krmh {
                Some(krmh) => format!("<a href='{}'>X</a> ", krmnirdesh(&None)),
                None => String::new(),
            } + &std::fs::read_dir("../")
                .unwrap()
                .map(|entry| entry.unwrap())
                .filter(|entry| entry.path().is_file())
                .map(|entry| entry.file_name().into_string().unwrap())
                .filter(|n| n.ends_with(".krmh"))
                .map(|n| {
                    let nam = std::fs::read_to_string(String::from("../") + &n)
                        .unwrap()
                        .split("\n")
                        .collect::<Vec<&str>>()[0]
                        .to_string();
                    match &info.krmh {
                        Some(krmh) if n == krmh.to_string() => nam,
                        _ => format!("<a href='{}'>{}</a>", krmnirdesh(&Some(n)), nam),
                    }
                })
                .collect::<Vec<String>>()
                .join(" | "),
            match info.istm {
                Some(ref istm) => format!(
                    "<p><a href='{}'>X</a> <b>{}</b> {}</p>",
                    istnirdesh(&None),
                    istm,
                    /*{
                        let pos = data.p.binary_search(&istm.to_string()).unwrap();
                        let it: Vec<usize> = (0..data.c[0].len()).collect();
                        let posj = &data.c[pos];
                        let cos: Vec<f32> = (0..data.p.len())
                            .map(|i| {
                                let ij = &data.c[i];
                                it.iter()
                                    .fold(0_f32, |mn1, j| mn1 as f32 + posj[*j] * ij[*j])
                                    as f32
                                    / data.n[i]
                            })
                            .collect();
                        let mut v: Vec<usize> = (0..data.p.len()).filter(|&i| i != pos).collect();
                        //v.sort_by(|i, j| cos[*j].partial_cmp(&cos[*i]).unwrap());

                        v
                    }
                    .iter()
                    .take(10)
                    .map(|i| String::from(&data.p[*i]))
                    .collect::<Vec<String>>()
                    .join(" ")*/""
                ),
                None => String::new(),
            },
            {
                let sadarnkrmh = {
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
                    Some(ref krmh) => std::fs::read_to_string(String::from("../") + krmh)
                        .expect("krmh?!")
                        .split("\n")
                        .enumerate()
                        .filter(|(i, _)| *i > 0)
                        .map(|(_, l)| {
                            let s: Vec<&str> = l.split(".").collect();
                            let f = |i: usize| s[i].parse::<usize>().unwrap() - 1;
                            (f(0), f(1), f(2))
                        })
                        .collect::<Vec<(usize, usize, usize)>>(),
                    None => sadarnkrmh,
                };

                iter.iter()
                    .map(|(mi, si, ri)| (mi, si, ri, &data.r[*mi][*si][*ri]))
                    .filter(|(_, _, _, r)| match info.istm {
                        Some(ref istm) => r
                            .crnani
                            .iter()
                            .any(|c| c.iter().any(|p| p.mulm == String::from(istm))),
                        None => true,
                    })
                    .map(|(mi, si, ri, r)| {
                        format!(
                            "<a href='{}{1}.{2}.{3}'>{}.{}.{}</a>",
                            vedaweb::VB,
                            mi + 1,
                            si + 1,
                            ri + 1
                        ) + &lekh(&r)
                    })
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
    	let vectortxt: Vec<Vec<String>> = vec![];
        /*let vectortxt: Vec<Vec<String>> = std::fs::read_to_string("../../GloVe/vectors.txt")
            .expect("?!")
            .split("\n")
            .map(|line| line.split(" ").map(|s| String::from(s)).collect())
            .collect();*/
        let mut vectors: Vec<Vec<f32>> = (0..pdmulani.len()).map(|_| Vec::new()).collect();
        for v in vectortxt.iter() {
            match pdmulani.binary_search(&v[0].replace("_", " ")) {
                Ok(i) => {
                    vectors[i] = v
                        .iter()
                        .enumerate()
                        .filter(|(j, _)| *j > 0_usize)
                        .map(|(_, f)| f.parse::<f32>().unwrap())
                        .collect();
                }
                Err(_) => {}
            }
        }
        vectors
    };

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
        let mut norm: Vec<f32> = Vec::new();

        for i in 0..pdmulani.len() {
            /*for j in 0..pdmulani.len() {
                sngh[i][j] = rksnkya as i64 * sngh[i][j] - (pdrgyogh[i] as i64) * (pdrgyogh[j] as i64);
            }*/
            //////////////////// norm replace sngh>vector
            let nn = ((0..vectors[0].len())
                .fold(0_f32, |n1, j| n1 + vectors[i][j] * vectors[i][j]))
            .sqrt();
            if nn == 0.0 {
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
        n: norm,
    });

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .service(web::resource("/").route(web::get().to(hello)))
    })
    .bind(DN)?
    .run()
    .await
}
