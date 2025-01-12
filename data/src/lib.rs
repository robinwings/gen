use starrailrust::{
    BaseUrl, Client, character::CharacterMap, character::CharacterPromotionMap,
    character::CharacterRankMap, character::CharacterSkillMap, character::CharacterSkillTreeMap,
    lightcone::LightconeMap, lightcone::LightconePromotionMap, lightcone::LightconeRankMap,
    misc::ElementMap, misc::ItemMap, misc::PathMap, relic::RelicSetMap,
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

    let ch_map = CharacterMap::fetch_map(&url, client).await?;
    let ch_p_map = CharacterPromotionMap::fetch_map(&url, client).await?;
    let ch_r_map = CharacterRankMap::fetch_map(&url, client).await?;
    let ch_s_map = CharacterSkillMap::fetch_map(&url, client).await?;
    let ch_st_map = CharacterSkillTreeMap::fetch_map(&url, client).await?;

    relic::generate(relic_map, output_dir)?;
    lightcone::generate(lc_map, lc_r_map, lc_p_map, &item_map, client, output_dir).await?;
    character::generate(ch_map, ch_p_map, ch_r_map, ch_s_map, ch_st_map, &item_map, client, output_dir).await?;

    Ok(())
}

pub async fn mtest() -> Result<(), Box<dyn std::error::Error>> {
    let output_dir = "";
    let client = &Client::new();
    let url = BaseUrl::En;
    let item_map = ItemMap::fetch_map(&url, client).await?;
    let ch_map = CharacterMap::fetch_map(&url, client).await?;
    let ch_p_map = CharacterPromotionMap::fetch_map(&url, client).await?;
    let ch_r_map = CharacterRankMap::fetch_map(&url, client).await?;
    let ch_s_map = CharacterSkillMap::fetch_map(&url, client).await?;
    let ch_st_map = CharacterSkillTreeMap::fetch_map(&url, client).await?;

    character::generate(ch_map, ch_p_map, ch_r_map, ch_s_map, ch_st_map, &item_map, client, output_dir).await?;

    Ok(())
}

fn warning(x: &str, y: &str) {
    println!("couldnt find {} for id {}, skipping gen", x, y);
}