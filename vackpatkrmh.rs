use std::fs;

fn main() {
	const VRNAH: &str = "_kKgGNcCjJYwWqQRtTdDnpPbBmyrlvSzshLaAEOeoiIuUfFHM~";
	const AKSRSNKYA: usize = VRNAH.len() * VRNAH.len();
	type Aksrani = [u8; AKSRSNKYA];
	
	let mut vrnkrmh: Vec<char> = VRNAH.chars().collect();
	vrnkrmh.sort();

	let iast: Vec<String> = fs::read_to_string("crnani")
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
			let sc: Vec<char> = ("_".to_string() + s + "_").chars().collect();
			for i in 0..sc.len()-1 {
				let c = sc[i];
				if i < s.len() - 1 {
					match vrnkrmh.binary_search(&c) {
						Ok(n) => match vrnkrmh.binary_search(&sc[i+1]) {
							Ok(m) => aksrani[n*VRNAH.len() + m] += 1,
							Err(..) => {},
						},
						Err(..) => {},
					}
				}
			}
			Crnm { ptitm: false, aksrani: aksrani , primanm: sc.iter().fold(0, |l, &c| if c == '3' || c == '5' {l} else {l+1}) }
		})
		.collect();

	let mut m = 0;
	let mut ptitani = 0;
	let mut aksrani = [0_u8; AKSRSNKYA];
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
						aksrani[i] = match aksrani[i].checked_add(slp[D].aksrani[i]) {
							Some(a) => a,
							None => aksrani[i],
						};
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
				println!("{} {}", vrnkrmh[i/VRNAH.len()], i%VRNAH.len());
			}
		}
		m += 1;
	}
}
