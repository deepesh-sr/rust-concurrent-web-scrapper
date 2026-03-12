use scraper::{Html, Selector};



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    //vector of urls
    let urls = vec!["https://solana.com","https://amazon.com","https://myntra.com"];

    let mut handles = vec![];
    for url in urls {
        let handle = tokio::spawn(scrape(url.to_string()));
        handles.push(handle);
    }

    for handle in handles { 
        // this is good for learning but not for production 
        // let links = handle.await.unwrap().unwrap();
        // println!("{:?}", links);

        // error handling with pattern matching 
        match  handle.await {
            Ok(Ok(links))=>println!("{:?}",links),
            Ok(Err(e))=> eprintln!("{:?}",e),
            Err(e)=> eprintln!("{:?}",e)
        }
    }
    Ok(())
}

async fn fetch_page(url: &String)-> Result<String, Box<dyn  std::error::Error + Send + Sync>> { 
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

async fn scrape(url : String) -> Result<Vec<String>,Box<dyn std::error::Error + Send + Sync >>{
let html = fetch_page(&url).await?;
  Ok(parse_links(&html))
}