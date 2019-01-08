extern crate reqwest;
extern crate scraper;

// importation syntax
use scraper::{Html, Selector};

fn main() {
    parse_location("https://www.iplocation.net/");
}

fn parse_location(url: &str) {

   let mut resp = reqwest::get(url).unwrap();
   assert!(resp.status().is_success());

   let body = resp.text().unwrap();
   // parses string of HTML as a document
   let fragment = Html::parse_document(&body);
   // parses based on a CSS selector
   let country_selector = Selector::parse("div.row:nth-child(11) > div:nth-child(1) > table:nth-child(2) > tbody:nth-child(2) > tr:nth-child(1) > td:nth-child(2)").unwrap();
   let region_selector = Selector::parse("div.row:nth-child(11) > div:nth-child(1) > table:nth-child(2) > tbody:nth-child(2) > tr:nth-child(1) > td:nth-child(3)").unwrap();
   
   let country_html = fragment.select(&country_selector).next().unwrap();
   let region_html = fragment.select(&region_selector).next().unwrap();
 
   println!("Country: {}, region: {}", country_html.text().collect::<Vec<_>>()[0].trim(), region_html.text().collect::<Vec<_>>()[0].trim());
   // iterate over elements matching our selector
   //for story in fragment.select(&stories) {
        // grab the headline text and place into a vector
   //     let story_txt = story.text().collect::<Vec<_>>();
   //     println!("{:?}", story_txt);
   // }
}
