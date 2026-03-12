use scraper::{Html, Selector};



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");
    let url = String::from("https://solana.com");
    let html = fetch_page(&url).await?;
    let parsed_links = parse_links(&html);
    println!("{parsed_links:?}");

    Ok(())
}

async fn fetch_page(url: &String)-> Result<String, Box<dyn  std::error::Error>> { 
    let resp = reqwest::get(url).await?.text().await?;
    Ok(resp)
}

fn parse_links(html : &str)-> Vec<String>{
    let mut links = vec![];
    let parsed_link = Html::parse_document(html);
    let selector = Selector::parse("a").unwrap();
   let elements =  parsed_link.select(&selector);   
    for element in elements{
       if let Some(href) = element.value().attr("href"){
        links.push(href.to_string());
       }
    }
    links

}