pub fn pdmulani(mndlani: &Vec<Vec<Vec<vedaweb::Rk>>>) -> (Vec<String>, Vec<i32>) {
        let mut pdmulani: Vec<String> = Vec::new();
        let mut pdrgyogh: Vec<i32> = Vec::new();

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
        krmh.sort_by_key(|&k| -pdrgyogh[k]);
        for k in krmh {
          println!("{}\t{}", pdmulani[k], pdrgyogh[k]);
        }
        (pdmulani, pdrgyogh)
    }
