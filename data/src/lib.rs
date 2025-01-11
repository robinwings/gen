use starrailrust::{
    BaseUrl, Client, character::CharacterMap, lightcone::LightconeMap, misc::ElementMap,
    misc::PathMap, relic::RelicSetMap,
};
use std::fs::File;
use std::io::Write;

mod character;
mod lightcone;
mod relic;

pub async fn generate_data(output_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    let url = BaseUrl::En;
    let client = Client::new();

    let relic_map = RelicSetMap::fetch_map(&url, &client).await?;

    relic::generate(relic_map, output_dir)?;

    Ok(())
}
