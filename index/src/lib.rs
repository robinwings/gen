use starrailrust::{BaseUrl, Client, character::CharacterMap, misc::ElementMap, misc::PathMap};

mod character;

pub async fn generate_index(output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let url = BaseUrl::En;
    let client = Client::new();

    let character_map = CharacterMap::fetch_map(&url, &client).await?;
    let element_map = ElementMap::fetch_map(&url, &client).await?;
    let path_map = PathMap::fetch_map(&url, &client).await?;

    character::generate(character_map, element_map, path_map, output_path)?;

    Ok(())
}
