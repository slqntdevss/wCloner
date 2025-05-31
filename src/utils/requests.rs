use reqwest::{Body, Response};

pub async fn get(url: String) -> Result<String, reqwest::Error>  {
    let client = reqwest::Client::new();
    return client.get(url).send().await?.text().await
}
pub async fn get_with_no_text(url: String) -> Result<Response, reqwest::Error>  {
    let client = reqwest::Client::new();
    return Ok(client.get(url).send().await?)
}
pub async fn post(url: &str, body: Body) -> Result<String, reqwest::Error>  {
    let client = reqwest::Client::new();
    return client.post(url).send().await?.text().await
}