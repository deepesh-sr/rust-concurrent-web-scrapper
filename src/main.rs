use scraper::{Html, Selector};



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");
    let url = String::from("https://solana.com");
    let html = fetch_page(&url).await?;


    Ok(())
}

async fn fetch_page(url: &String)-> Result<String, Box<dyn  std::error::Error>> { 
    let resp = reqwest::get(url).await?.text().await?;
    Ok(resp)
}
