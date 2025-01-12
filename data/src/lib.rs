use starrailrust::{
    BaseUrl, Client, character::CharacterMap, lightcone::LightconeMap,
    lightcone::LightconePromotionMap, lightcone::LightconeRankMap, misc::ElementMap, misc::ItemMap,
    misc::PathMap, relic::RelicSetMap,
};
use std::fs::File;
use std::io::Write;

mod character;
mod lightcone;
mod relic;

pub async fn generate_data(
    output_dir: &str,
    client: &Client,
) -> Result<(), Box<dyn std::error::Error>> {
    let url = BaseUrl::En;

    let relic_map = RelicSetMap::fetch_map(&url, client).await?;
    let lc_map = LightconeMap::fetch_map(&url, client).await?;
    let lc_r_map = LightconeRankMap::fetch_map(&url, client).await?;
    let lc_p_map = LightconePromotionMap::fetch_map(&url, client).await?;
    let item_map = ItemMap::fetch_map(&url, client).await?;

    relic::generate(relic_map, output_dir)?;
    lightcone::generate(lc_map, lc_r_map, lc_p_map, &item_map, client, output_dir).await?;

    Ok(())
}
