#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let img_url: Vec<String> = waifupics_rs::get_with_client(&reqwest::Client::new(), waifupics_rs::CategorySFW::Bonk, false).await?;
    println!("{:#?}", img_url);
    Ok(())
}