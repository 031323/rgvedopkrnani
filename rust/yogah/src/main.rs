use vedaweb;
use std::process::Command;

fn upsrstkriyarmbah(mndlani: &Vec<Vec<Vec<vedaweb::Rk>>>) {
    for r in mndlani.iter().flatten().flatten() {
        for c in &r.crnani {
            if c.len() > 1 {
                if vedaweb::drmnamani(&c[0]).iter().any(|n| *n == "invariable")
                    && vedaweb::drmnamani(&c[1]).iter().any(|n| *n == "root")
                {
                    /*println!(
                        "{}",
                        c.iter()
                            .map(|p| String::from(&p.rupm))
                            .collect::<Vec<String>>()
                            .join(" ")
                    );*/
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
        //println!("{:?}", pdani.iter().fold(0, |n, m| if pdrgyogh[pdmulani.binary_search(m).unwrap()] > 125 {n+1} else {n} ) as f32 / pdani.len() as f32);
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
    std::fs::write("../crnani2", mndlani.iter().flatten().flatten().map(|r| String::from(&r.smhita)).collect::<Vec<String>>().join("\n\n"));

    let mut pdani=Vec::new();
    for r in mndlani.iter().flatten().flatten() {
        if true || r.strata=="A" {
        for c in &r.crnani {
            for i in 0..c.len()-1 {
                let vstuni = "ajá- amŕ̥ta- aśáni- ádhvan- ádri- áhi- ákṣ- ákṣa- ámatra- ánas- ánna- ánīka- áp- ápas- áśman- íṣ-".to_string().split(" ").map(|v| v.to_string()).collect::<Vec<String>>();
                if (
                        /*(c[i].rupm.ends_with("ā") || c[i].rupm.ends_with("ā́")) && (c[i+1].rupm.starts_with("a") || c[i+1].rupm.starts_with("á"))
                    ||
                        (c[i].rupm.ends_with("ī") || c[i].rupm.ends_with("ī́")) && (c[i+1].rupm.starts_with("i") || c[i+1].rupm.starts_with("í"))
                    ||
                        (c[i].rupm.ends_with("ū") || c[i].rupm.ends_with("ū́")) && (c[i+1].rupm.starts_with("u") || c[i+1].rupm.starts_with("ú"))*/

                        (c[i].rupm.ends_with("ā") || c[i].rupm.ends_with("ā́")) && (c[i+1].rupm.starts_with("ā"))
                    ||
                        (c[i].rupm.ends_with("ī") || c[i].rupm.ends_with("ī́")) && (c[i+1].rupm.starts_with("ī"))
                    ||
                        (c[i].rupm.ends_with("ū") || c[i].rupm.ends_with("ū́")) && (c[i+1].rupm.starts_with("ū"))

                    )
                    && vedaweb::drmnamani(&c[i+1]).iter().any(|&n| String::from(n)=="nominal stem") 
                    //&& vstuni.iter().any(|v| c[i+1].mulm==v.to_string())
                    {
                    match pdani.binary_search(&c[i+1].mulm) {
                        Ok(pos) => {},
                        Err(pos) => pdani.insert(pos, String::from(&c[i+1].mulm)),
                    }
                    //println!("{}", c[i+1].mulm);
                    println!("{}", c.iter().map(|p| String::from(&p.rupm)).collect::<Vec<String>>().join(" "));
                }
            }
        }
        }
    }
    println!("{}", pdani.join(" "));


    std::fs::write("../pdani", mndlani.iter().flatten().flatten().map(|r| r.crnani.iter().map(|c| c.iter().map(|p| String::from(&p.rupm)).collect::<Vec<String>>().join(" ")).collect::<Vec<String>>().join("\n")).collect::<Vec<String>>().join("\n\n"));
    std::fs::write("../crnani", mndlani.iter().flatten().flatten().filter(|r| r.strata=="A").map(|r| String::from(&r.smhita)).collect::<Vec<String>>().join("\n\n"));
    std::fs::write("../nagri2", mndlani.iter().flatten().flatten().map(|r| String::from(&r.nagri)).collect::<Vec<String>>().join("\n\n"));
    std::fs::write("../nagri", mndlani.iter().flatten().flatten().filter(|r| r.strata=="A").map(|r| String::from(&r.nagri)).collect::<Vec<String>>().join("\n\n"));

}

fn grdrkrmh(mndlani: &Vec<Vec<Vec<vedaweb::Rk>>>) {
    /*let suktsngrh: Vec<(usize, usize, usize, )>
    std::fs::write("../grdr.krmh", String::from("गृ॒ध्र॒क्र॒मः") + &mndlani.iter()
    */
}

fn rgvedpath(mndlani: &Vec<Vec<Vec<vedaweb::Rk>>>) {
    crnsngrhnm(&mndlani);
    let uccarnani=String::from_utf8(Command::new("python3").arg("../iast235.py").arg("../crnani"
        ).arg("../nagri").output().unwrap().stdout).unwrap().split("\n").map(|s| String::from(s)).collect::<Vec<String>>();
    let mut i = 0;
    let smrupnm=
    std::fs::write(
        "../2",
        format!("
            <html>
                <head>
                    <link rel='icon' type='image/svg' href='2.svg'>
                    <meta charset='UTF-8'>
                    <meta name='viewport' content='width=device-width, initial-scale=1.0'>
                    <title>ऋ॒ग्वे॒दः</title>
                    <style>
                        html {{ scroll-behavior: smooth; }}
                        body {{ visibility: hidden; }}
                        a    {{ color: blue; text-decoration: none; }}
                        div.fixed {{
                            position:           fixed;
                            bottom:             10%;
                            right:              10%;
                            width:              10%;
                            background-color:   transparent;
                        }}
                        div.ptymanm {{
                            font-weight: bold;
                        }}
                        div.ptniym {{
                            background-color: GreenYellow;
                        }}
                    </style>
                </head>
                <body>
                    <div class='fixed'>
                        <a id='scroll'>
                            <svg width='100%' viewBox='0 0 100 100'>
                                <circle cx='50' cy='50' r='50' fill='indigo' />
                                <circle cx='50' cy='50' r='30' fill='white' />
                                <rect x='40' y='10' width='20' height='80' fill='white' />
                                <circle cx='50' cy='50' r='20' fill='indigo' />
                            </svg>
                        </a>
                    </div>
                    {}
                    <script src='https://031323.github.io/suvak/suvak2.js'></script>
                    <script>
                        suvak_udattpurvanudatth=100;
                        suvak_adisvrh=100;
                        suvak_svr0=120;
                        suvak_svr1=135;
                        suvak_svritodatth=150;
                        udattadnudattsysvrith=true;
                        var ptyte=false;
                        var vakyani=[];
                        var vakykrmh=0;
                        var ptymanm=0;
                        var ptniym=0;
                        website='https://031323.github.io/suvak/';
                        suvagarmbh(
                            function() {{
                                setTimeout(
                                    function() {{
                                        document.body.style.visibility='visible';
                                    }},
                                    100
                                );
                            }},
                            () => {{console.log('reload');}}
                        );
                        function krmnm() {{
                            if(vakykrmh<vakyani.length) {{
                                suvacnarmbh(vakyani[vakykrmh].replace('ळ्ह', 'ढ').replace('ळ','ड'), (t)=>{{}}, krmnm);
                                vakykrmh += 1;
                            }}
                            else rkkrmnm();
                        }}
                        function ptnstitih(purvptymanm, purvptniym, pscatptymanm, pscatptniym) {{
                            if(purvptymanm!=0)
                                document.getElementById(purvptymanm.toString()).className='';
                            if(purvptniym!=0)
                                document.getElementById(purvptniym.toString()).className='';   
                            if(pscatptymanm!=0) {{
                                document.getElementById(pscatptymanm.toString()).className='ptymanm';
                                document.getElementById('scroll').href='#'+pscatptymanm.toString();
                            }}
                            if(pscatptniym!=0)
                                document.getElementById(pscatptniym.toString()).className='ptniym';
                            ptymanm=pscatptymanm;
                            ptniym=pscatptniym;
                        }}
                        function rkkrmnm() {{
                            if(ptniym!=0) {{
                                ptnstitih(ptymanm, ptniym, ptniym, ptniym+1);
                                vakykrmh=0;
                                vakyani=document.getElementById(ptymanm.toString()).
                                    getAttribute('uccarnm').split('। ');
                                console.log(vakyani);
                                krmnm();
                            }}
                            else {{
                                ptnstitih(ptymanm, 0, 0, 0);
                                ptyte=false;
                            }}
                        }}
                        function ptytam(i) {{
                            if(i!=ptymanm && i!=ptniym)
                                ptnstitih(ptymanm, ptniym, ptymanm, i);
                            else
                                ptnstitih(ptymanm, ptniym, ptymanm, 0);
                            if(! ptyte) {{
                                ptyte = true;
                                rkkrmnm();
                            }}
                        }}
                        document.addEventListener('keydown', e=>{{ptnstitih(ptymanm, ptniym, ptymanm, 0);}});
                    </script>
                </body>
            </html>
        ",
        &mndlani
            .iter()
            .enumerate()
            .map(|(mi, m)| {
                m.iter()
                    .enumerate()
                    .map(|(si, s)| {
                        (0..s.len())
                            .filter(|&ri| s[ri].strata == "A")
                            .map(|ri| {
                                i += 1;
                                format!(
                                    "<div class='rk' id='{}' uccarnm='{}'><a href='{}{3}.{4}.{5}' target='_blank'>{}.{}.{}</a><span onclick='ptytam({})'> {}</span></div>",
                                    
                                    i,
                                    uccarnani[i-1],
                                    vedaweb::VB,
                                    mi+1,
                                    si+1,
                                    ri+1,
                                    i,
                                    s[ri].nagri.split("\n").map(|s| String::from(s)).collect::<Vec<String>>()
                                        .join("। ")+"॥"
                                )
                            })
                            .collect::<Vec<String>>()
                            .join("\n")
                    })
                    .collect::<Vec<String>>()
                    .join("\n")
            })
            .collect::<Vec<String>>()
            .join("\n")
        )
    );
}

fn arskrmh(mndlani: &Vec<Vec<Vec<vedaweb::Rk>>>) {
    std::fs::write(
        "../ars.krmh",
        String::from("आ॒र्ष॒क्र॒मः\n")
            + &mndlani
                .iter()
                .enumerate()
                .filter(|(mi, _)| {*mi > 0 && *mi < 7})
                .map(|(mi, m)| {
                    m.iter()
                        .enumerate()
                        .map(|(si, s)| {
                            (0..s.len())
                                .filter(|&ri| {
                                    s[ri].strata == "A"
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
    rgvedpath(&mndlani);
    prtmpuruskrmh(&mndlani);
    grdrkrmh(&mndlani);
    mulpath(&mndlani);
    gntvkrmh(&mndlani);
    pdsrvskrmh(&mndlani);
    arskrmh(&mndlani);
}
