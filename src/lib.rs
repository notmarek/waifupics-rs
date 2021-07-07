#[derive(thiserror::Error, Debug)]
pub enum WaifuPicsError {
    #[error("reqwest error")]
    ReqwestError(#[from] reqwest::Error),

    #[error("no picture found")]
    NoPicture,
}
pub enum CategorySFW {
    Waifu,
    Neko,
    Shinobu,
    Megumin,
    Bully,
    Cuddle,
    Cry,
    Hug,
    Awoo,
    Kiss,
    Lick,
    Pat,
    Smug,
    Bonk,
    Yeet,
    Blush,
    Smile,
    Wave,
    Highfive,
    Handhold,
    Nom,
    Bite,
    Glomp,
    Slap,
    Kill,
    Kick,
    Happy,
    Wink,
    Poke,
    Dance,
    Cringe,
}

impl std::fmt::Display for CategorySFW {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CategorySFW::Waifu => "sfw/waifu",
            CategorySFW::Neko => "sfw/neko",
            CategorySFW::Shinobu => "sfw/shinobu",
            CategorySFW::Megumin => "sfw/megumin",
            CategorySFW::Bully => "sfw/bully",
            CategorySFW::Cuddle => "sfw/cuddle",
            CategorySFW::Cry => "sfw/cry",
            CategorySFW::Hug => "sfw/hug",
            CategorySFW::Awoo => "sfw/awoo",
            CategorySFW::Kiss => "sfw/kiss",
            CategorySFW::Lick => "sfw/lick",
            CategorySFW::Pat => "sfw/pat",
            CategorySFW::Smug => "sfw/smug",
            CategorySFW::Bonk => "sfw/bonk",
            CategorySFW::Yeet => "sfw/smile",
            CategorySFW::Blush => "sfw/blush",
            CategorySFW::Smile => "sfw/smile",
            CategorySFW::Wave => "sfw/wave",
            CategorySFW::Highfive => "sfw/highfive",
            CategorySFW::Handhold => "sfw/handhold",
            CategorySFW::Nom => "sfw/nom",
            CategorySFW::Bite => "sfw/bite",
            CategorySFW::Glomp => "sfw/glomp",
            CategorySFW::Slap => "sfw/slap",
            CategorySFW::Kill => "sfw/kill",
            CategorySFW::Kick => "sfw/kick",
            CategorySFW::Happy => "sfw/happy",
            CategorySFW::Wink => "sfw/wink",
            CategorySFW::Poke => "sfw/poke",
            CategorySFW::Dance => "sfw/dance",
            CategorySFW::Cringe => "sfw/cringe",
        }
        .fmt(f)
    }
}

pub enum CategoryNSFW {
    Waifu,
    Neko,
    Trap,
    Blowjob,
}
impl std::fmt::Display for CategoryNSFW {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CategoryNSFW::Waifu => "nsfw/waifu",
            CategoryNSFW::Neko => "nsfw/neko",
            CategoryNSFW::Trap => "nsfw/trap",
            CategoryNSFW::Blowjob => "nsfw/blowjob",
        }
        .fmt(f)
    }
}

pub const API_URL: &str = "https://api.waifu.pics";

pub async fn get_with_client(
    client: &reqwest::Client,
    category: impl std::fmt::Display,
    many: bool,
) -> Result<Vec<String>, reqwest::Error> {
    use serde::Deserialize;
    #[derive(Deserialize)]
    struct WaifuPicture {
        url: String,
    }
    #[derive(Deserialize)]
    struct WaifuPictures {
        files: Vec<String>,
    }

    if many {
        let req_uri = format!("{}/many/{}", API_URL, category);
        let r = client
            .post(&req_uri)
            .body("{}")
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .send()
            .await?
            .json::<WaifuPictures>()
            .await?;
        Ok(r.files)
    } else {
        let req_uri = format!("{}/{}", API_URL, category);
        let r = client
            .get(&req_uri)
            .send()
            .await?
            .json::<WaifuPicture>()
            .await?;
        Ok(vec![r.url])
    }
}

pub async fn get_single_with_client(
    client: &reqwest::Client,
    category: impl std::fmt::Display,
) -> Result<String, WaifuPicsError> {
    Ok(get_with_client(client, category, false)
        .await?
        .pop()
        .ok_or(WaifuPicsError::NoPicture)?)
}

pub async fn get(category: impl std::fmt::Display) -> Result<String, WaifuPicsError> {
    get_single_with_client(&reqwest::Client::new(), category).await
}

pub async fn get_many_with_client(
    client: &reqwest::Client,
    category: impl std::fmt::Display,
) -> Result<Vec<String>, WaifuPicsError> {
    Ok(get_with_client(client, category, true).await?)
}

pub async fn get_many(category: impl std::fmt::Display) -> Result<Vec<String>, WaifuPicsError> {
    get_many_with_client(&reqwest::Client::new(), category).await
}
