use crate::{
    CharacterMap,
    CharacterPromotionMap,
    CharacterRankMap,
    CharacterSkillMap,
    CharacterSkillTreeMap,
    ItemMap,
    File,
    Write,
    Client,
    warning,
};

macro_rules! max_stat {
    ($name:ident, $promo:expr, $stat:ident) => {
        let $name = $promo.$stat.base + ($promo.$stat.step * 79f64);
    };
}

macro_rules! max_promo {
    ($name:ident, $map:expr, $key:expr, $label:expr) => {
        let $name = match $map.get_value_by_key($key) {
            Some(v) => v.values[6].clone(),
            _ => {
                warning($label, $key);
                continue;
            }
        };
    };
}

macro_rules! get_v_by_k{
    ($name:ident, $map:expr, $label:expr) => {
        let $name = |x: &str| match $map.get_value_by_key(x) {
            Some(v) => v,
            _ => panic!("cant find {} with id: {}", $label, x),
        };
    };
}

macro_rules! get_vf_by_k{
    ($name:ident, $map:expr, $field:ident, $label:expr) => {
        let $name = |x: &str| match $map.get_value_by_key(x) {
            Some(v) => v.$field,
            _ => panic!("cant find {} with id: {}", $label, x),
        };
    };
}

const JS_SCRIPT: &str = r###"function updateSkill(skillId, level) {
    var skillDataElement = document.getElementById(skillId + '_data');
    
    var stats = skillDataElement.getAttribute('data-level' + level);
    console.log('Selected level:', level);
    console.log('Stats for selected level:', stats);
    
    var statsArray = stats.split(',').map(stat => parseFloat(stat.trim()));
    console.log('Parsed stats array:', statsArray);
    
    var skillDescriptionElement = document.getElementById(skillId + '_description');
    
    var originalDescription = skillDescriptionElement.getAttribute('data-original-description');
    console.log('Original description:', originalDescription);
    
    var updatedDescription = originalDescription;
    statsArray.forEach((stat, index) => {
        var placeholderRegExp = new RegExp(`#${index + 1}\\[([if1]+)\\](%?)`, 'g');

        updatedDescription = updatedDescription.replace(placeholderRegExp, (match, p1, p2) => {
            var formattedStat = stat;
            console.log('Processing placeholder:', match, 'with stat:', stat, 'and format:', p1, 'percentage sign:', p2);
            
            if (p2 === '%') {
                formattedStat = (stat * 100).toFixed(stat < 0.01 ? 2 : 0) + '%';
            } else {
                if (stat % 1 !== 0) {
                    formattedStat = stat.toFixed(stat % 1 !== 0 ? 2 : 0);
                }
            }
            console.log('Replacing placeholder:', match, 'with formatted stat:', formattedStat);
            return formattedStat;
        });
    });
    
    console.log('Updated description:', updatedDescription);
    skillDescriptionElement.innerHTML = updatedDescription;
}
function initializeSkills() {
    var dropdowns = document.querySelectorAll('select[onchange^="updateSkill"]');
    dropdowns.forEach(dropdown => {
        dropdown.value = 1;
        dropdown.dispatchEvent(new Event('change'));
    });
}
"###;

pub async fn generate(
    ch_map: CharacterMap,
    ch_p_map: CharacterPromotionMap,
    ch_r_map: CharacterRankMap,
    ch_s_map: CharacterSkillMap,
    ch_st_map: CharacterSkillTreeMap,
    i_map: &ItemMap,
    cl: &Client,
    output_dir: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    for (c_id, ch) in ch_map.character_map.iter() {
        // anon functions
        get_v_by_k!(get_ch_s, ch_s_map, "ch skill");
        get_v_by_k!(get_ch_st, ch_st_map, "ch skilltree");
        get_v_by_k!(get_ch_r, ch_r_map, "ch rank");
        get_vf_by_k!(get_i_icon, i_map, icon, "item icon");
        get_vf_by_k!(get_i_name, i_map, name, "item name");

        get_ch_s("100101");
        get_ch_st("1001001");
        get_ch_r("100101");
        get_i_icon("2");
        get_i_name("2");

        // variables
        max_promo!(c_max_promo, ch_p_map, c_id, "ch promo");
        max_stat!(c_max_hp, c_max_promo, hp);
        max_stat!(c_max_atk, c_max_promo, atk);
        max_stat!(c_max_def, c_max_promo, def);
        max_stat!(c_max_spd, c_max_promo, spd);
        max_stat!(c_max_taunt, c_max_promo, taunt);
        max_stat!(c_max_cr, c_max_promo, crit_rate);
        max_stat!(c_max_cd, c_max_promo, crit_dmg);

        println!("{}", &ch.name);
        println!("{}", c_max_hp);
        println!("{}", c_max_atk);
        println!("{}", c_max_def);
        println!("{}", c_max_spd);
        println!("{}", c_max_taunt);
        println!("{}", c_max_cr);
        println!("{}", c_max_cd);

        // let mut output_html = String::new();

        // let mut file = File::create(format!("{}/character/{}.html", output_dir, c_id))?;
        // file.write_all(output_html.as_bytes())?;
    }

    Ok(())
}