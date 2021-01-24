use std::fs;

fn main() {
	const VYNJNANI: &str = "kKgGNcCjJYwWqQRtTdDnpPbBmyrlvSzshx";
	const SVRGNAH: [&str; 4] = ["aAeEoO", "iI", "uU", "fF"];
	const AKSRSNKYA: usize = VYNJNANI.len() * SVRGNAH.len();
	type Aksrani = [u8; AKSRSNKYA];
	
	let mut vynjnkrmh: Vec<char> = VYNJNANI.chars().collect();
	vynjnkrmh.sort();
	let mut svrkrmh: Vec<char> = SVRGNAH.join("").chars().collect();
	svrkrmh.sort();

	let gnh: Vec<usize> = svrkrmh.iter()
		.map(|c| {
			for i in 1..SVRGNAH.len() {
				if SVRGNAH[i].chars().any(|d| d == *c) {
					return i;
				}
			}
			0
		})
		.collect();
	
	let iast: Vec<String> = fs::read_to_string("iast2")
		.expect("file?!")
		.split('\n')
		.map(|s: &str| s.to_string())
		.collect();

	#[derive(Debug)]
	struct Crnm {
		ptitm: bool,
		aksrani: Aksrani,
		primanm: usize,
	}

	let mut slp: Vec<Crnm> = fs::read_to_string("slp")
		.expect("file?!")
		.split('\n')
		.map(|s| {
			let mut aksrani = [0; AKSRSNKYA];
			let sc: Vec<char> = s.chars().collect();
			for i in 0..sc.len()-1 {
				let c = sc[i];
				if i < s.len() - 1 {
					match vynjnkrmh.binary_search(&c) {
						Ok(n) => match svrkrmh.binary_search(&sc[i+1]) {
							Ok(m) => aksrani[n*SVRGNAH.len() + gnh[m]] += 1,
							Err(..) => {},
						},
						Err(..) => {},
					}
				}
			}
			Crnm { ptitm: false, aksrani: aksrani , primanm: sc.len() }
		})
		.collect();

	let mut m = 0;
	let mut ptitani = 0;
	let mut aksrani = [0; AKSRSNKYA];
	while ptitani < slp.len() {
		let mut nvtvm = 1_f64;
		while nvtvm > 0_f64 {
			let nC = (0..slp.len()).fold((0_f64, None), |(M, C), D| {
				if slp[D].ptitm {
					(M, C)
				}
				else {
					let N = (0..AKSRSNKYA).fold(0_usize, |n, i| {
						n + if slp[D].aksrani[i] > 0 && aksrani[i] == m
							{ 1 } else { 0 }
					}) as f64 / slp[D].primanm as f64;

					if N > M {
						(N, Some(D))
					}
					else {
						(M, C)
					}
				}
			});
			nvtvm = nC.0;
			let C = nC.1;
			match C {
				Some(D) => {
					ptitani += 1;
					slp[D].ptitm = true;
					for i in 0..AKSRSNKYA {
						aksrani[i] += slp[D].aksrani[i];
					}
					println!("{}", iast[D]);
					/*println!("{} {} {}", ptitani, aksrani.iter().fold(0, |n, x| {
						n + if *x > m { 1 } else { 0 }
					}), m);*/
				},
				None => {},
			}
		}
		for i in 0..AKSRSNKYA {
			if aksrani[i] == 0 {
				println!("{} {}", vynjnkrmh[i/SVRGNAH.len()], i%SVRGNAH.len());
			}
		}
		m += 1;
	}
}