use crate::{
    Client, File, ItemMap, LightconeMap, LightconePromotionMap, LightconeRankMap, Write, warning,
};

const JS_SCRIPT: &str = r###"
function updateRefinements(weaponId, level) {
    var refinementDataElement = document.getElementById(weaponId + '_data');
    var stats = refinementDataElement.getAttribute('data-level' + level);
    
    var statsArray = stats.split(',').map(stat => parseFloat(stat.trim()));
    
    var refinementsElement = document.getElementById(weaponId + '_refinements');
    var originalDescription = refinementsElement.getAttribute('data-original-description');
    
    statsArray.forEach((stat, index) => {
        var placeholderRegExp = new RegExp(`#${index + 1}\\[([if1]+)\\](%?)`, 'g');

        originalDescription = originalDescription.replace(placeholderRegExp, (match, p1, p2) => {
            var formattedStat = stat;
            // console.log('Processing placeholder:', match, 'with stat:', stat, 'and format:', p1, 'percentage sign:', p2);
        
            if (p2 === '%') {
                const s = (stat * 100).toFixed(stat < 0.01 ? 2 : 0) + '%';
                formattedStat = `<b>${s}</b>`;
            } else {
                if (stat % 1 !== 0) {
                    const s = stat.toFixed(stat % 1 !== 0 ? 2 : 0);
                    formattedStat = `<b>${s}</b>`;
                }
            }
            // console.log('Replacing placeholder:', match, 'with formatted stat:', formattedStat);
            return formattedStat;
        });
    });

    refinementsElement.innerHTML = originalDescription;
}
function initializeSkills() {
    var dropdowns = document.querySelectorAll('select[onchange^="updateRefinements"]');
    dropdowns.forEach(dropdown => {
        dropdown.value = 1;
        dropdown.dispatchEvent(new Event('change'));
    });
}"###;

pub async fn generate(
    l: LightconeMap,
    r: LightconeRankMap,
    p: LightconePromotionMap,
    i: &ItemMap,
    cl: &Client,
    output_dir: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    for (lightcone_id, lightcone) in r.lightcone_rank_map.iter() {
        let lc_info = match l.get_value_by_key(lightcone_id) {
            Some(v) => v,
            _ => {
                warning("lc info", lightcone_id);
                continue;
            }
        };

        let promo_info = match p.get_value_by_key(lightcone_id) {
            Some(v) => v,
            _ => {
                warning("promo", lightcone_id);
                continue;
            }
        };

        let get_item_icon = |x: &str| match i.get_value_by_key(x) {
            Some(v) => v.icon,
            _ => {
                warning("item icon", lightcone_id);
                String::new()
            }
        };

        let get_item_name = |x: &str| match i.get_value_by_key(x) {
            Some(v) => v.name,
            _ => {
                warning("item name", lightcone_id);
                String::new()
            }
        };

        let lc_name = lc_info.name;
        let lc_rarity = lc_info.rarity;
        let lc_path = match lc_info.path.as_str() {
            "Knight" => "Preservation",
            "Rogue" => "The Hunt",
            "Mage" => "Erudition",
            "Warlock" => "Nihility",
            "Shaman" => "Harmony",
            "Warrior" => "Destruction",
            "Priest" => "Abundance",
            "Memory" => "Remembrance",
            _ => &lc_info.path,
        };
        let lc_desc = lightcone.desc.replace("\n", "<br>");

        let (max_hp, max_atk, max_def) = {
            let max_promo = &promo_info.values[6];
            (
                (max_promo.hp.base + (max_promo.hp.step * 79f64)),
                (max_promo.atk.base + (max_promo.atk.step * 79f64)),
                (max_promo.def.base + (max_promo.def.step * 79f64)),
            )
        };

        let mut output_html = format!(
            r###"<!DOCTYPE html>
<html>
    <head>
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <link rel="stylesheet" href="../dark.css">
        <title>{}</title>
        <script>
        {}
        </script>
    </head>
    <body onload="initializeSkills()">
        <noscript>
            It seems like you have JavaScript disabled, or your browser does not support it.<br>
            I am sorry but this website cannot function normally without it :)
        </noscript>
        <h1 style="margin-bottom:0px">robinwings</h1>
        <small>If you found a bug or anything like that, please report it <a href="https://github.com/robinwings/gen/issues">here.</a></small>
        <hr>
        <div style="margin-top:20px; margin-bottom:10px">
            <a class="button-link" style="text-decoration: none;" href="../character.html">Character</a>
            <a class="button-link" style="text-decoration: none;" href="../lightcone.html">Lightcone</a>
            <a class="button-link" style="text-decoration: none;" href="../relic.html">Relic</a>
            <a class="button-link" style="text-decoration: none;" href="https://github.com/robinwings/gen">Source Code</a>
        </div>
        <h1>{}</h1>
        <p><strong>{} | {}* {}</strong></p>

        <h2>Stats (Lv. 80)</h2>
        <div style="display: flex; margin:0px; padding:0px;">
            <ul>
                <li><strong>HP:</strong> {}</li>
                <li><strong>ATK:</strong> {}</li>
                <li><strong>DEF:</strong> {}</li>
            </ul>
        </div>

        <h2>Ability</h2>
        <ul>
            <li>
                <strong>{}:</strong> 
                <select onchange="updateRefinements('{}', this.value)">
                    <option value="1">1</option>
                    <option value="2">2</option>
                    <option value="3">3</option>
                    <option value="4">4</option>
                    <option value="5">5</option>
                </select>
                <div id="{}_refinements" data-original-description="{}">
                    {}
                </div>
                <div id="{}_data" style="display:none;" "###,
            lc_name,
            JS_SCRIPT,
            lc_name,
            lightcone_id,
            lc_rarity,
            lc_path,
            max_hp,
            max_atk,
            max_def,
            lightcone.skill,
            lightcone_id,
            lightcone_id,
            lc_desc,
            lc_desc,
            lightcone_id
        );

        for (i, param) in lightcone.params.iter().enumerate() {
            let data_level = format!(
                r#"data-level{}="{}" "#,
                i + 1,
                param
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            );
            output_html.push_str(&data_level);
        }

        // // The JSON doesnt have hidden values, unfortunate.

        // output_html.push_str(
        //     "></div>
        //     </li>",
        // );

        // output_html.push_str(
        //     "<li>
        //         <strong>Hidden Values:</strong>
        //         <ul>",
        // )

        // for (i, prop) in lightcone.properties.iter().enumerate() {
        //     output_html.push_str(&format!("<li><strong>S{}:</strong> {:?}</li>", i, prop).replace("LightconeRankProperty ", ""))
        // }

        // output_html.push_str("</ul></li>");

        output_html.push_str(
            "></div></li></ul><h2>Ascension Materials</h2><table><tr><th>Material</th><th>Amount</th></tr>"
        );

        let mut material_counts: std::collections::BTreeMap<String, u32> =
            std::collections::BTreeMap::new();

        for mats in promo_info.materials {
            for mat in mats {
                let id = mat.id;
                let num = mat.num;
                *material_counts.entry(id).or_insert(0) += num;
            }
        }

        for (mat_id, total_num) in material_counts.iter() {
            let material_icon = get_item_icon(&mat_id);

            icon::download_image(cl, &material_icon, &icon::IconType::Item, output_dir).await?;

            let mat_name = get_item_name(&mat_id);
            output_html.push_str(&format!(
                r#"
    <tr>
        <td style='display: flex; align-items: center;'>
            <img style='height: 38px; width: auto; margin-right: 1rem;' src='../{}' />
            {}
        </td>
        <td>{}</td>
    </tr>"#,
                material_icon, mat_name, total_num
            ));
        }

        output_html.push_str("</table></body></html>");

        let mut file = File::create(format!("{}/lightcone/{}.html", output_dir, lightcone_id))?;
        file.write_all(output_html.as_bytes())?;
    }

    Ok(())
}
