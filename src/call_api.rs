pub async fn get_cat_fact() -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let body = client
        .get("https://httpbin.org/get")
        .send()
        .await?
        .text()
        .await?;

    Ok(body)
}
