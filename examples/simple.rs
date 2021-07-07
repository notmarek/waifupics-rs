#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let img: String = waifupics_rs::get(waifupics_rs::CategorySFW::Bonk).await?;
    println!("Single image: {}", img);
    let imgs: Vec<String> = waifupics_rs::get_many(waifupics_rs::CategorySFW::Bonk).await?;
    println!("Vec of images: {:#?}", imgs);
    Ok(())
}