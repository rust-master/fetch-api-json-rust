use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct Pancakeswap {
    updated_at: isize,
    data: Data,
}

#[derive(Debug, Serialize, Deserialize)]
struct Data {
    #[serde(rename = "name")]
    name: String,
    symbol: String,
    price: String,
    #[serde(rename = "price_BNB")]
    price_bnb: String,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error>{
    let data: Pancakeswap = reqwest::Client::new()
        .get("https://api.pancakeswap.info/api/v2/tokens/0x0E09FaBB73Bd3Ade0a17ECC321fD13a19e81cE82")
        .send()
        .await?
        .json()
        .await?;

    println!("{:#?}", data);
   
    Ok(())
}
