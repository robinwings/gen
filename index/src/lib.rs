use starrailrust::{
    BaseUrl, Client, character::CharacterMap, lightcone::LightconeMap, misc::ElementMap,
    misc::PathMap, relic::RelicSetMap,
};
use std::fs::File;
use std::io::Write;

mod character;
mod lightcone;
mod relic;

pub async fn generate_index(output_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    let url = BaseUrl::En;
    let client = Client::new();

    let mut index_file = File::create(format!("{}index.html", output_dir))?;
    let index_html = include_str!("./index.html");

    index_file.write_all(index_html.as_bytes())?;

    let character_map = CharacterMap::fetch_map(&url, &client).await?;
    let element_map = ElementMap::fetch_map(&url, &client).await?;
    let path_map = PathMap::fetch_map(&url, &client).await?;
    let lightcone_map = LightconeMap::fetch_map(&url, &client).await?;
    let relic_map = RelicSetMap::fetch_map(&url, &client).await?;

    character::generate(character_map, element_map, &path_map, output_dir)?;
    lightcone::generate(lightcone_map, &path_map, output_dir)?;
    relic::generate(relic_map, output_dir)?;

    Ok(())
}
