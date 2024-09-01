#![allow(dead_code)]

use serde::Deserialize;
use serde_json;

const URL_BASE: &str = "https://www.ietf.org/rfc/";

#[derive(Debug, Deserialize)]
pub struct IetfRfc {
    pub id: usize,
    pub date: String,
    pub title: String,
}

impl IetfRfc {
    pub fn url(&self) -> String {
        format!("{base}rfc{id}.txt", base = URL_BASE, id = self.id)
    }
    pub fn doi(&self) -> String {
        format!("10.17487/RFC{}", self.id)
    }
    pub fn from_file(filename: &str) -> Vec<IetfRfc> {
        let file = std::fs::File::open(filename).expect("Failed to open file");
        let reader = std::io::BufReader::new(file);
        serde_json::from_reader(reader).expect("Failed to parse JSON")
    }
}
