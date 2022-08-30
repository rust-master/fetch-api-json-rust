
#[tokio::main]
async fn main() -> Result<(), reqwest::Error>{
    let data = reqwest::Client::new()
        .get("https://api.pancakeswap.info/api/v2/tokens/0x0E09FaBB73Bd3Ade0a17ECC321fD13a19e81cE82")
        .send()
        .await?
        .text()
        .await?;

    println!("{:#?}", data);
   
    Ok(())
}
