pub fn pdmulani(mndlani: &Vec<Vec<Vec<vedaweb::Rk>>>) -> (Vec<String>, Vec<i32>) {
        let mut pdmulani: Vec<String> = Vec::new();
        let mut pdani: Vec<String> = Vec::new();
        let mut pdrgyogh: Vec<i32> = Vec::new();
        let mut pdsuktyogh: Vec<i32> = Vec::new();

        for s in mndlani[0..9].iter().flatten() {
            let v = {
                let mut v: Vec<String> = s
                    .iter().map(|r| if r.strata == "P" || r.strata == "p" || r.strata == "C" || r.strata == "c" {vec![]} else { r.crnani.iter().flatten().map(|pdm| String::from(&pdm.rupm) + "\t" + &pdm.mulm)
                    .collect::<Vec<String>>()}).flatten().collect();
                v.sort();
                v.dedup();
                v
            };
            for p in v {
                match pdani.binary_search(&p) {
                    Ok(pos) => {
                        pdsuktyogh[pos] += 1;
                    }
                    Err(pos) => {
                        pdani.insert(pos, p);
                        pdsuktyogh.insert(pos, 1);
                    }
                }
            }
        }

        for r in mndlani.iter().flatten().flatten() {
            if r.strata != "A" {continue;}
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
        let mut krmh: Vec<usize> = (0..pdmulani.len()).collect();
        let mut pdkrmh: Vec<usize> = (0..pdani.len()).collect();
        pdkrmh.sort_by_key(|&k| -pdsuktyogh[k]);
        krmh.sort_by_key(|&k| -pdrgyogh[k]);
        println!("{:?}", krmh.len());
        for k in krmh {
          //println!("{}\t{}", pdmulani[k], pdrgyogh[k]);
        }
        for k in pdkrmh {
          println!("{}\t{}", pdani[k], pdsuktyogh[k]);
        }
        (pdmulani, pdrgyogh)
    }
