mod drmsngrhh;
use crate::drmsngrhh::DRMANI;
use std::fmt::{self, Display, Formatter};
use std::io::{Read, Write};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Pdm {
    pub rupm: String,
    pub mulm: String,
    drmani: u64,
}

pub fn drmani(p: &Pdm) -> Vec<&str> {
    DRMANI
        .iter()
        .enumerate()
        .filter(|&(i, _)| (1_u64 << i) & p.drmani != 0)
        .map(|(_, &d)| d)
        .collect::<Vec<&str>>()
}

impl Display for Pdm {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}({}{})",
            self.rupm,
            self.mulm,
            drmani(self).join(" ")
        )
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Rk {
    pub smhita: String,
    pub crnani: Vec<Vec<Pdm>>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Rgvedh(pub Vec<Vec<Vec<Rk>>>);

impl Display for Rk {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}\n{}",
            self.smhita,
            self.crnani
                .iter()
                .map(|c| c
                    .iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(" "))
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}

pub fn aropnm(c_salt_dir: &str) -> Result<Rgvedh, std::io::Error>  {
    fn bincode() -> Result<Rgvedh, std::io::Error> {
        let mut file = std::fs::File::open("rgvedh_bincode")?;
        let mut buffer = Vec::<u8>::new();
        file.read_to_end(&mut buffer)?;
        Ok(Rgvedh(bincode::deserialize(&buffer[..]).unwrap()))
    }
    match bincode() {
        Ok(R) => Ok(R),
        Err(E) => {
        
        let R = Rgvedh((1..11)
        .map(|m| {
            let snkyah = [
                "prathamám",
                "dvitī́yam",
                "tṛtī́yam",
                "caturthám",
                "pañcamám",
                "ṣaṣṭhám",
                "saptamám",
                "aṣṭamám",
                "navamám",
                "daśamám",
            ];
            println!("{} máṇḍalam ā́ropyate.", snkyah[m - 1]);
            roxmltree::Document::parse(
                &std::fs::read_to_string(format!("{}rv_book_{:0>2}.tei", c_salt_dir, m)).unwrap(),
            )
            .unwrap()
            .descendants()
            .filter(|d| d.attribute("type") == Some("hymn"))
            .map(|s| {
                s.descendants()
                    .filter(|d| d.attribute("type") == Some("stanza"))
                    .map(|r| Rk {
                        smhita: match r
                            .descendants()
                            .find(|d| d.attribute("source") == Some("vnh"))
                        {
                            Some(vnh) => vnh
                                .descendants()
                                .filter(|d| d.has_tag_name("l"))
                                .map(|d| d.text().unwrap().to_string())
                                .collect::<Vec<String>>()
                                .join("\n"),
                            None => r
                                .descendants()
                                .find(|d| d.attribute("source") == Some("zurich"))
                                .unwrap()
                                .descendants()
                                .filter(|d| d.has_tag_name("l"))
                                .enumerate()
                                .filter(|&(i, _)| i % 2 == 0)
                                .map(|(_, d)| d.text().unwrap().to_string())
                                .collect::<Vec<String>>()
                                .join("\n"),
                        },
                        crnani: r
                            .descendants()
                            .find(|d| d.attribute("source") == Some("zurich"))
                            .unwrap()
                            .descendants()
                            .filter(|d| d.has_tag_name("l"))
                            .enumerate()
                            .filter(|&(i, _)| i % 2 == 1)
                            .map(|(_, c)| {
                                c.descendants()
                                    .filter(|d| d.has_tag_name("fs") && d.attribute("type") == Some("zurich_info"))
                                    .map(|p| {
                                        let rupm = p
                                            .descendants()
                                            .find(|d| d.attribute("name") == Some("surface"))
                                            .unwrap()
                                            .descendants()
                                            .find(|d| d.has_tag_name("string"))
                                            .unwrap()
                                            .text()
                                            .unwrap();
                                        Pdm {
                                            rupm: rupm.to_string(),
                                            mulm: match p
                                                .descendants()
                                                .find(|d| d.attribute("name") == Some("gra_lemma"))
                                            {
                                                Some(d) => d
                                                    .descendants()
                                                    .find(|d| d.has_tag_name("string"))
                                                    .unwrap()
                                                    .text()
                                                    .unwrap()
                                                    .to_string(),
                                                None => rupm.to_string(),
                                            },
                                            drmani: p
                                                .descendants()
                                                .filter(|d| d.has_tag_name("symbol"))
                                                .fold(0, |ds, d| {
                                                    ds | (1_u64
                                                        << DRMANI
                                                            .iter()
                                                            .position(|&s| {
                                                                s == d.attribute("value").unwrap()
                                                            })
                                                            .unwrap())
                                                }),
                                        }
                                    })
                                    .collect()
                            })
                            .collect(),
                    })
                    .collect()
            })
            .collect()
        })
        .collect());
        
        let mut file = std::fs::File::create("rgvedh_bincode")?;
        file.write_all(&bincode::serialize(&R).unwrap())?;
        Ok(R)
        
        },
        }
}
