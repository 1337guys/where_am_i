extern crate reqwest;
extern crate scraper;
extern crate serde_json;
extern crate serde_derive;
extern crate serde;



// importation syntax
use scraper::{Html, Selector};
use std::panic;
use std::fmt;
#[macro_use]
use serde_derive::{Serialize, Deserialize};
#[macro_use]
use serde_json::{Result, Value};

#[derive(Serialize, Deserialize)]
struct IP {
    ip: String,
}


struct Location {
  country: String,
  region: String,
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Country: {}, Region: {}", self.country, self.region)
    }
}

fn main() {
    match parse_ipify() {
        Some(location) => println!("{}", location),
        None =>
            match parse_ipcim("https://ipcim.com/en/?p=where") {
                Some(location) => println!("{}", location),
                None => 
                    match parse_iplocation("https://www.iplocation.net/") {
                        Some(location) => println!("{}", location),
                        None => println!("Kek!"),
                    },
            }
    }
}

fn parse_iplocation(url: &str) -> Option<Location> {

    let mut resp = reqwest::get(url).unwrap();
    if !resp.status().is_success() {
        return None;
    }

    let body = resp.text().unwrap();

    let fragment = Html::parse_document(&body);

    let country_selector = Selector::parse("div.row:nth-child(11) > div:nth-child(1) > table:nth-child(2) > tbody:nth-child(2) > tr:nth-child(1) > td:nth-child(2)").unwrap();

    let region_selector = Selector::parse("div.row:nth-child(11) > div:nth-child(1) > table:nth-child(2) > tbody:nth-child(2) > tr:nth-child(1) > td:nth-child(3)").unwrap();

    let country = match fragment.select(&country_selector).next() {
        Some(val) => val.text().collect::<Vec<_>>()[0].trim().to_string(),
        None => return None,
    };

    let region = match fragment.select(&region_selector).next() {
        Some(val) => val.text().collect::<Vec<_>>()[0].trim().to_string(),
        None => return None,
    };

    Some(Location {country: country.to_string(), region: region.to_string()})
}

fn parse_ipcim(url: &str) -> Option<Location> {

    let mut resp = reqwest::get(url).unwrap();
    if !resp.status().is_success() {
        return None;
    }

    let body = resp.text().unwrap();

    let fragment = Html::parse_document(&body);

    let geo_selector = Selector::parse("#geoinfo").unwrap();

    match fragment.select(&geo_selector).next() {
        Some(val) => return Some(Location { country: val.text().collect::<Vec<_>>()[4].trim().to_string() , region: val.text().collect::<Vec<_>>()[6].trim().to_string()}),
        None => return None,
    };
}



fn parse_ipify() -> Option<Location> {

    let mut resp = reqwest::get("https://api.ipify.org?format=json").unwrap();
    if !resp.status().is_success() {
        return None;
    }

    let body = resp.text().unwrap();

    let v: IP = serde_json::from_str(&body).unwrap();

    let mut resp = reqwest::get(&format!("http://ip-api.com/json/{}", v.ip)).unwrap();
    if !resp.status().is_success() {
        return None;
    }

    let body = resp.text().unwrap();

    let v: Value = serde_json::from_str(&body).unwrap();

    Some(Location { country: v["country"].to_string() , region: v["city"].to_string()})
}