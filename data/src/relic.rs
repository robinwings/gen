use crate::{File, RelicSetMap, Write};

pub fn generate(rl: RelicSetMap, output_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    for (relic_id, relic) in rl.relic_set_map.iter() {
        let mut output_html = format!(
            r###"<!DOCTYPE html>
<html>
    <head>
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <link rel="stylesheet" href="../dark.css">
        <title>{}</title>
    </head>
    <body>
        <noscript>
            It seems like you have JavaScript disabled, or your browser does not support it.<br>
            I am sorry but this website cannot function normally without it :)
        </noscript>
        <h1 style="margin-bottom:0px">robinwings</h1>
        <small>Some scalings are in decimals instead of percentage :)</small>
        <hr>
        <div style="margin-top:20px; margin-bottom:10px">
            <a class="button-link" style="text-decoration: none;" href="../character.html">Character</a>
            <a class="button-link" style="text-decoration: none;" href="../lightcone.html">Lightcone</a>
            <a class="button-link" style="text-decoration: none;" href="../relic.html">Relic</a>
            <a class="button-link" style="text-decoration: none;" href="https://github.com/robinwings/gen">Source Code</a>
        </div>
        <h1>{}</h1>
        <h2>Ability</h2>
        <ul>
"###,
            relic.name, relic.name
        );

        if relic.desc.len() == 1 {
            let rel_uint = relic_id.parse::<u32>()?;
            let orb_id = (rel_uint * 10) + 60005;
            let rope_id = (rel_uint * 10) + 60006;

            let html = format!(
                r#"
            <li><strong>2-Piece:</strong> {}</li>
        </ul>
        <h2>ID</h2>
        <p><strong>6XXXX 5*, 5XXXX 4*, 4XXXX 3*, 3XXXX 2*</strong></p>
        <ul>
            <li><strong>Orb:</strong> {}</li>
            <li><strong>Rope:</strong> {}</li>
        </ul>
                "#,
                relic.desc[0], orb_id, rope_id
            );

            output_html.push_str(&html);
        } else if relic.desc.len() == 2 {
            let rel_uint = relic_id.parse::<u32>()?;
            let head_id = (rel_uint * 10) + 60001;
            let hand_id = (rel_uint * 10) + 60002;
            let body_id = (rel_uint * 10) + 60003;
            let foot_id = (rel_uint * 10) + 60004;

            let html = format!(
                r#"
            <li><strong>2-Piece:</strong> {}</li>
            <li><strong>4-Piece:</strong> {}</li>
        </ul>
        <h2>ID</h2>
        <p><strong>6XXXX 5*, 5XXXX 4*, 4XXXX 3*, 3XXXX 2*</strong></p>
        <ul>
            <li><strong>Head:</strong> {}</li>
            <li><strong>Hand:</strong> {}</li>
            <li><strong>Body:</strong> {}</li>
            <li><strong>Foot:</strong> {}</li>
        </ul>
    "#,
                relic.desc[0], relic.desc[1], head_id, hand_id, body_id, foot_id
            );

            output_html.push_str(&html);
        }

        output_html.push_str(
            "</body>
</html>",
        );

        let mut file = File::create(format!("{}/relic/{}.html", output_dir, relic_id))?;
        file.write_all(output_html.as_bytes())?;
    }

    Ok(())
}
