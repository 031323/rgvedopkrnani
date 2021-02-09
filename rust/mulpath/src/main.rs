use vedaweb;

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 2 {
        println!("c-salt_vedaweb_tei ítyasyá sthā́nam ápekṣyate.");
        std::process::exit(0);
    }

    let mndlani = vedaweb::aropnm(&args[1]).unwrap().0;
    
    std::fs::write("../rvmulani", mndlani.iter().flatten().map(|s| s.iter().fold(String::from(""), |s, r| s + " " + &r.crnani.iter().flatten().map(|p| String::from(&p.mulm).replace(" ", "")).collect::<Vec<String>>().join(" "))).collect::<Vec<String>>().join("\n")).expect("!?");
}
