use html_scraper::{fetch_page, parse_html};

#[tokio::main]
async fn main() {
    let url = "enter your url here";

    match fetch_page(url).await {
        Ok(html_content) => match parse_html(&html_content) {
            Ok(parsed_data) => {
                println!("Title: {}", parsed_data.title);
                println!("Headings: {:#?}", parsed_data.headings);
                println!("Links: {:#?}", parsed_data.links);
            }
            Err(err) => {
                eprintln!("Error parsing HTML: {:?}", err);
            }
        },
        Err(err) => {
            eprintln!("Error fetching page: {:?}", err);
        }
    }
}
