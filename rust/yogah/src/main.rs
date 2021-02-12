use vedaweb;

fn upsrstkriyarmbah(mndlani: Vec<Vec<Vec<vedaweb::Rk>>>) {
    for r in mndlani.iter().flatten().flatten(){
        for c in &r.crnani {
            if c.len() > 1 {
                if vedaweb::drmnamani(&c[0]).iter().any(|n| *n=="invariable") && vedaweb::drmnamani(&c[1]).iter().any(|n| *n=="root") {
                    println!("{}", c.iter().map(|p| String::from(&p.rupm)).collect::<Vec<String>>().join(" "));
                }
            }
        }
    }
}

fn grdrkrmh(mndlani: Vec<Vec<Vec<vedaweb::Rk>>>) {
    /*let suktsngrh: Vec<(usize, usize, usize, )>
    std::fs::write("../grdr.krmh", String::from("गृ॒ध्र॒क्र॒मः") + &mndlani.iter()
    */
}

fn prtmpuruskrmh(mndlani: Vec<Vec<Vec<vedaweb::Rk>>>) {
    std::fs::write("../prtmpurus.krmh", String::from("प्र॒थ॒म॒पु॒रु॒ष॒क्र॒मः\n") + &mndlani.iter().enumerate().map(|(mi, m)| m.iter().enumerate().map(|(si, s)| (0..s.len()).filter(|&ri| s[ri].strata=="A" && s[ri].crnani.iter().flatten().all(|p| vedaweb::drmnamani(&p).iter().all(|&n| n != "2" && n != "1" && n != "VOC" && n != "IMP" && n != "INJ"  && n != "OPT"  && n != "SBJV") && p.mulm != "tvám" && p.mulm != "ahám")).map(|ri| format!("{}.{}.{}", mi+1, si+1, ri+1)).collect::<Vec<String>>().join("\n")).filter(|ss| ss!= "").collect::<Vec<String>>().join("\n")).filter(|ms| ms != "").collect::<Vec<String>>().join("\n"));
    
    //std::fs::write("../prtmpuruskrmh", mndlani.iter().enumerate().map(|(mi, m)| m.iter().enumerate().filter(|(_, s)| s.iter().all(|r| r.strata=="A" && r.crnani.iter().flatten().all(|p| vedaweb::drmnamani(&p).iter().all(|&n| n != "2" && n != "1") && p.mulm != "tvám" && p.mulm != "ahám"))).map(|(si, s)| (0..s.len()).map(|ri| format!("{}.{}.{}", mi+1, si+1, ri+1)).collect::<Vec<String>>().join("\n")).collect::<Vec<String>>().join("\n")).filter(|ms| ms != "").collect::<Vec<String>>().join("\n"));
}



fn mulpath(mndlani: Vec<Vec<Vec<vedaweb::Rk>>>) {
    std::fs::write("../rvmulani", mndlani.iter().flatten().map(|s| s.iter().fold(String::from(""), |s, r| s + " " + &r.crnani.iter().flatten().map(|p| String::from(&p.mulm).replace(" ", "_")).collect::<Vec<String>>().join(" "))).collect::<Vec<String>>().join("\n")).expect("!?");
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 2 {
        println!("c-salt_vedaweb_tei ítyasyá sthā́nam ápekṣyate.");
        std::process::exit(0);
    }

    let mndlani = vedaweb::aropnm(&args[1]).unwrap().0;
    
    mulpath(mndlani);
}
