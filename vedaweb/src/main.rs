enum Purush {
	Prtmh,
	Mdymh,
	Uttmh,
}

enum Vibktih {
	Krta,
	Krm,
	Krnm,
	Smprdanm,
	Apadanm,
	Smbndh,
	Adikrnm,
	Smbodnm,
}

enum Vcnm {
	Ekm,
	Dve,
	Bhuni,
}

enum Vacym {
	Krm,
	Krta,
	Bavh,
}

enum Lingm {
	Stri,
	Puman,
}

struct rc {
		
}

fn main() {
	let args: Vec<_> = std::env::args().collect();

	if args.len() != 2 {
		std::process::exit(1);
	}

	let text = std::fs::read_to_string(&args[1]).unwrap();

	match roxmltree::Document::parse(&text) {
		Ok(doc) => { println!("Printing..."); println!("{:?}", doc);},
		Err(e) => println!("Error: {}", e),
	}
}
