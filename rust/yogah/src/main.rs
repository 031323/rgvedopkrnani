use vedaweb;

fn upsrstkriyarmbah(mndlani: &Vec<Vec<Vec<vedaweb::Rk>>>) {
    for r in mndlani.iter().flatten().flatten() {
        for c in &r.crnani {
            if c.len() > 1 {
                if vedaweb::drmnamani(&c[0]).iter().any(|n| *n == "invariable")
                    && vedaweb::drmnamani(&c[1]).iter().any(|n| *n == "root")
                {
                    println!(
                        "{}",
                        c.iter()
                            .map(|p| String::from(&p.rupm))
                            .collect::<Vec<String>>()
                            .join(" ")
                    );
                }
            }
        }
    }
}

fn pdsrvskrmh(mndlani: &Vec<Vec<Vec<vedaweb::Rk>>>) {
    let (pdmulani, pdrgyogh) = vedyogah::pdmulani(&mndlani);
    let srvh = |su: &(usize, usize, &Vec<vedaweb::Rk>)| -> usize {
        let mut pdani: Vec<String> = Vec::new();
        for r in su.2 {
            for p in r.crnani.iter().flatten() {
                pdani.push(String::from(&p.mulm));
            }
        }
        //println!("{:?}", pdrgyogh[pdmulani.binary_search(&pdani[pdani.len()/5]).unwrap()]);
        println!("{:?}", pdani.iter().fold(0, |n, m| if pdrgyogh[pdmulani.binary_search(m).unwrap()] > 125 {n+1} else {n} ) as f32 / pdani.len() as f32);
        pdani.iter().fold(0, |n, m| if pdrgyogh[pdmulani.binary_search(m).unwrap()] > 125 {n+1} else {n} ) * 100 / pdani.len()

        //pdani.sort_by_key(|m| pdrgyogh[pdmulani.binary_search(m).unwrap()]);
        //pdrgyogh[pdmulani.binary_search(&pdani[pdani.len()/5]).unwrap()]

    };

    let mut suktani: Vec<(usize, usize, &Vec<vedaweb::Rk>)> = mndlani.iter().enumerate().map(|(mi, m)| {
        m.iter().enumerate().filter(|(_, s)| s.iter().all(|r| r.strata=="A")).map(|(si, s)| (mi, si, s)).collect::<Vec<(usize, usize, &Vec<vedaweb::Rk>)>>()
    }).flatten().collect();
    suktani.sort_by(|a, b| srvh(b).partial_cmp(&srvh(a)).unwrap());

    std::fs::write("../srvs.krmh", String::from("प॒द॒श्र॒व॒स्क्र॒मः\n") + &suktani.iter().map(|s| (0..s.2.len()).map(|ri| format!("{}.{}.{}", s.0 + 1, s.1 + 1, ri + 1)).collect::<Vec<String>>()).flatten().collect::<Vec<String>>().join("\n"));
}

fn gntvkrmh(mndlani: &Vec<Vec<Vec<vedaweb::Rk>>>) {
    fn gntvm(su: &(usize, usize, &Vec<vedaweb::Rk>)) -> f32 {
        let mut pdani: Vec<String> = Vec::new();
        let mut pdsnkya = 0;
        for r in su.2 {
            for p in r.crnani.iter().flatten() {
                match pdani.binary_search(&p.mulm) {
                    Ok(n) => {},
                    Err(n) => {pdani.insert(n, String::from(&p.mulm));},
                }
                pdsnkya += 1;
            }
        }
        pdani.len() as f32 / pdsnkya as f32
    }

    let mut suktani: Vec<(usize, usize, &Vec<vedaweb::Rk>)> = mndlani.iter().enumerate().map(|(mi, m)| {
        m.iter().enumerate().filter(|(_, s)| s[0].strata=="A").map(|(si, s)| (mi, si, s)).collect::<Vec<(usize, usize, &Vec<vedaweb::Rk>)>>()
    }).flatten().collect();
    suktani.sort_by(|a, b| gntvm(a).partial_cmp(&gntvm(b)).unwrap());

    std::fs::write("../gntv.krmh", String::from("घ॒न॒त्व॒क्र॒मः\n") + &suktani.iter().map(|s| (0..s.2.len()).map(|ri| format!("{}.{}.{}", s.0 + 1, s.1 + 1, ri + 1)).collect::<Vec<String>>()).flatten().collect::<Vec<String>>().join("\n"));
}

fn crnsngrhnm(mndlani: &Vec<Vec<Vec<vedaweb::Rk>>>) {
    std::fs::write("../crnani", mndlani.iter().flatten().flatten().map(|r| String::from(&r.smhita)).collect::<Vec<String>>().join("\n"));
}

fn grdrkrmh(mndlani: &Vec<Vec<Vec<vedaweb::Rk>>>) {
    /*let suktsngrh: Vec<(usize, usize, usize, )>
    std::fs::write("../grdr.krmh", String::from("गृ॒ध्र॒क्र॒मः") + &mndlani.iter()
    */
}

fn prtmpuruskrmh(mndlani: &Vec<Vec<Vec<vedaweb::Rk>>>) {
    std::fs::write(
        "../prtmpurus.krmh",
        String::from("प्र॒थ॒म॒पु॒रु॒ष॒क्र॒मः\n")
            + &mndlani
                .iter()
                .enumerate()
                .map(|(mi, m)| {
                    m.iter()
                        .enumerate()
                        .filter(|(_, s)| {
                            s.iter().any(|r| r.crnani.iter().flatten().any(|p| p.mulm == "pitú-"))
                        })
                        /*.filter(|(_, s)| {
                            s.iter().all(|r| {
                                r.strata == "A"
                                  && r.crnani.iter().flatten().all(|p| {
                                            vedaweb::drmnamani(&p).iter().all(|&n| {
                                                n != "2"
                                                    && n != "1"
                                                    && n != "VOC"
                                            }) && p.mulm != "tvám"
                                                && p.mulm != "ahám"
                                        })
                            })
                        })*/
                        .map(|(si, s)| {
                            (0..s.len())
                                .filter(|&ri| {
                                    s[ri].strata == "A"
                                        && s[ri].crnani.iter().flatten().all(|p| {
                                            vedaweb::drmnamani(&p).iter().all(|&n| {
                                                /*n != "2"
                                                    &&*/ n != "1"
                                                    && n != "VOC"
                                            }) && p.mulm != "tvám"
                                                && p.mulm != "ahám"
                                        })
                                })
                                .map(|ri| format!("{}.{}.{}", mi + 1, si + 1, ri + 1))
                                .collect::<Vec<String>>()
                                .join("\n")
                        })
                        .filter(|ss| ss != "")
                        .collect::<Vec<String>>()
                        .join("\n")
                })
                .filter(|ms| ms != "")
                .collect::<Vec<String>>()
                .join("\n"),
    );

    //std::fs::write("../prtmpuruskrmh", mndlani.iter().enumerate().map(|(mi, m)| m.iter().enumerate().filter(|(_, s)| s.iter().all(|r| r.strata=="A" && r.crnani.iter().flatten().all(|p| vedaweb::drmnamani(&p).iter().all(|&n| n != "2" && n != "1") && p.mulm != "tvám" && p.mulm != "ahám"))).map(|(si, s)| (0..s.len()).map(|ri| format!("{}.{}.{}", mi+1, si+1, ri+1)).collect::<Vec<String>>().join("\n")).collect::<Vec<String>>().join("\n")).filter(|ms| ms != "").collect::<Vec<String>>().join("\n"));
}

fn mulpath(mndlani: &Vec<Vec<Vec<vedaweb::Rk>>>) {
    std::fs::write(
        "../rvmulani",
        mndlani
            .iter()
            .flatten()
            .map(|s| {
                s.iter().fold(String::from(""), |s, r| {
                    s + " "
                        + &r.crnani
                            .iter()
                            .flatten()
                            .map(|p| String::from(&p.mulm).replace(" ", "_"))
                            .collect::<Vec<String>>()
                            .join(" ")
                })
            })
            .collect::<Vec<String>>()
            .join("\n"),
    )
    .expect("!?");
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 2 {
        println!("c-salt_vedaweb_tei ítyasyá sthā́nam ápekṣyate.");
        std::process::exit(0);
    }

    let mndlani = vedaweb::aropnm(&args[1]).unwrap().0;

    prtmpuruskrmh(&mndlani);
    grdrkrmh(&mndlani);
    mulpath(&mndlani);
    crnsngrhnm(&mndlani);
    gntvkrmh(&mndlani);
    pdsrvskrmh(&mndlani);
}
