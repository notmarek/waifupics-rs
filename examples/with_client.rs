#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let img: String = waifupics_rs::get_single_with_client(&reqwest::Client::new(), waifupics_rs::CategorySFW::Bonk).await?;
    println!("{}", img);
    Ok(())
}