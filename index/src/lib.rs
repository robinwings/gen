use starrailrust::{
    BaseUrl, Client, character::CharacterMap, lightcone::LightconeMap, misc::ElementMap,
    misc::PathMap, relic::RelicSetMap,
};

mod character;
mod lightcone;
mod relic;

pub async fn generate_index(output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let url = BaseUrl::En;
    let client = Client::new();

    let character_map = CharacterMap::fetch_map(&url, &client).await?;
    let element_map = ElementMap::fetch_map(&url, &client).await?;
    let path_map = PathMap::fetch_map(&url, &client).await?;
    let lightcone_map = LightconeMap::fetch_map(&url, &client).await?;
    let relic_map = RelicSetMap::fetch_map(&url, &client).await?;

    character::generate(character_map, element_map, &path_map, output_path)?;
    lightcone::generate(lightcone_map, &path_map, output_path)?;
    relic::generate(relic_map, output_path)?;

    Ok(())
}
