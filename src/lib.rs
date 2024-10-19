mod enums;

use std::error::Error;
use enums::ParseError;
use reqwest;
use scraper::{self, Html, Selector};

pub struct ParsedData{
    pub title: String,
    pub headings: Vec<String>,
    pub links: Vec<String>
}
pub async fn fetch_page(url: &str)-> Result<String, Box<dyn Error>>{
    let response = reqwest::get(url).await?;
    if response.status().is_success(){
        return Ok(response.text().await?)
    }
    Err("Failed to fetch the page".into())


} 
pub fn parse_html(html_content: &str)-> Result<ParsedData, ParseError>{
    let document = Html::parse_document(html_content);
    let title_selector = Selector::parse("title").unwrap();

    let title = document.select(&title_selector)
                    .next()
                    .map(|t| t.text().collect::<String>())
                    .ok_or(ParseError::MissingField("Title not found".into()))?;


    let mut headings = Vec::new();
        for i in 1..=3 { 
            let selector = format!("h{}", i);
            let h_selector = Selector::parse(&selector).unwrap();
            for element in document.select(&h_selector) {
                headings.push(element.text().collect::<String>());
                 }
         }
    
         let links: Vec<String> = document.select(&Selector::parse("a").unwrap())
         .filter_map(|element| element.value().attr("href").map(|s| s.to_string()))
         .collect();

        Ok(ParsedData{ title, headings, links })

}