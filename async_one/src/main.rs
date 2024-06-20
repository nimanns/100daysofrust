
use reqwest::Error;
use tokio; 

async fn fetch_data() -> Result<String, Error> {
    let url = "https://jsonplaceholder.typicode.com/posts/1"; 
    let response = reqwest::get(url).await?; 

        Ok(response.text().await?)
}

#[tokio::main]
async fn main() {
    match fetch_data().await {
        Ok(data) => println!("Fetched data: {}", data),
        Err(e) => eprintln!("Error fetching data: {}", e),
    }
}

