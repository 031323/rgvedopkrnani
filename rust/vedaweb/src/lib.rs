mod drmsngrhh;
use crate::drmsngrhh::DRMANI;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter};
use std::io::{Read, Write};

pub const VB: &str = "https://vedaweb.uni-koeln.de/rigveda/view/id/";

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Pdm {
    pub rupm: String,
    pub mulm: String,
    drmani: u64,
}

pub fn drmnamani(p: &Pdm) -> Vec<&str> {
    DRMANI
        .iter()
        .enumerate()
        .filter(|&(i, _)| (1_u64 << i) & p.drmani != 0)
        .map(|(_, &d)| d)
        .collect::<Vec<&str>>()
}

pub fn pdbedh(p: &Pdm) -> String {
    for b in ["nominal stem", "root", "invariable", "pronoun"].iter() {
        for d in drmnamani(p) {
            if b.to_string() == d.to_string() {
                return b.to_string();
            }
        }
    }
    "".to_string()
}

impl Display for Pdm {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}({}{})",
            self.rupm,
            self.mulm,
            drmnamani(self).join(" ")
        )
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Rk {
    pub strata: String,
    pub smhita: String,
    pub nagri: String,
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

pub fn aropnm(c_salt_dir: &str) -> Result<Rgvedh, std::io::Error> {
    fn bincode() -> Result<Rgvedh, std::io::Error> {
        let mut file = std::fs::File::open("rgvedh_bincode")?;
        let mut buffer = Vec::<u8>::new();
        file.read_to_end(&mut buffer)?;
        Ok(Rgvedh(bincode::deserialize(&buffer[..]).unwrap()))
    }
    match bincode() {
        Ok(r) => Ok(r),
        Err(_) => {
            let r = Rgvedh(
                (1..11)
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
                            &std::fs::read_to_string(format!(
                                "{}/rigveda/TEI/rv_book_{:0>2}.tei",
                                c_salt_dir, m
                            ).replace("//","/"))
                            .unwrap(),
                        )
                        .unwrap()
                        .descendants()
                        .filter(|d| d.attribute("type") == Some("hymn"))
                        .map(|s| {
                            s.descendants()
                                .filter(|d| d.attribute("type") == Some("stanza"))
                                .map(|r| Rk {
                                    strata: r
                                        .descendants()
                                        .find(|d| {
                                            d.has_tag_name("f")
                                                && d.attribute("name") == Some("strata")
                                        })
                                        .unwrap()
                                        .text()
                                        .unwrap()
                                        .to_string(),
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
                                    nagri: r
                                        .descendants()
                                        .find(|d| d.attribute("source") == Some("eichler"))
                                        .unwrap()
                                        .descendants()
                                        .filter(|d| d.has_tag_name("l"))
                                        .map(|d| d.text().unwrap().to_string()
                                            .replace("१॒॑ः", "ः१॒॑")
                                            .replace("१॒॑ं", "ं१॒॑")
                                            .replace("३॒॑ः", "ः३॒॑")
                                            .replace("३॒॑ं", "ं३॒॑")
                                        )
                                        .collect::<Vec<String>>()
                                        .join("\n"),
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
                                                .filter(|d| {
                                                    d.has_tag_name("fs")
                                                        && d.attribute("type")
                                                            == Some("zurich_info")
                                                })
                                                .map(|p| {
                                                    let rupm = p
                                                        .descendants()
                                                        .find(|d| {
                                                            d.attribute("name") == Some("surface")
                                                        })
                                                        .unwrap()
                                                        .descendants()
                                                        .find(|d| d.has_tag_name("string"))
                                                        .unwrap()
                                                        .text()
                                                        .unwrap();
                                                    Pdm {
                                                        rupm: rupm.to_string(),
                                                        mulm: match p.descendants().find(|d| {
                                                            d.attribute("name") == Some("gra_lemma")
                                                        }) {
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
                                                                            /*println!("{}",d
                                                                                .attribute("value")
                                                                                .unwrap());*/
                                                                            s == d
                                                                                .attribute("value")
                                                                                .unwrap()
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
                    .collect(),
            );

            let mut file = std::fs::File::create("rgvedh_bincode")?;
            file.write_all(&bincode::serialize(&r).unwrap())?;
            Ok(r)
        }
    }
}
