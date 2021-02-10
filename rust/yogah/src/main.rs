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

fn mulpath(mndlani: Vec<Vec<Vec<vedaweb::Rk>>>) {
    std::fs::write("../rvmulani", mndlani.iter().flatten().map(|s| s.iter().fold(String::from(""), |s, r| s + " " + &r.crnani.iter().flatten().map(|p| String::from(&p.mulm).replace(" ", "")).collect::<Vec<String>>().join(" "))).collect::<Vec<String>>().join("\n")).expect("!?");
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 2 {
        println!("c-salt_vedaweb_tei ítyasyá sthā́nam ápekṣyate.");
        std::process::exit(0);
    }

    let mndlani = vedaweb::aropnm(&args[1]).unwrap().0;
    
    upsrstkriyarmbah(mndlani);
}
