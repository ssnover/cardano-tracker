use reqwest::blocking::Client;

fn main() -> std::io::Result<()> {
    let query_url = format!("https://api.coingecko.com/api/v3/coins/cardano?localization=false");
    let client = Client::new();
    let response = client
        .get(&query_url)
        .header("accept", "application/json")
        .send()
        .unwrap()
        .text()
        .unwrap();
    let response_json: serde_json::Value = serde_json::from_str(&response)?;

    println!(
        "1.00 ADA => {} USD",
        response_json["market_data"]["current_price"]["usd"]
    );

    Ok(())
}
