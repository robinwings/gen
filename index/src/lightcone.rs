use crate::{File, LightconeMap, PathMap, Write};

pub fn generate(
    lc: LightconeMap,
    pt: &PathMap,
    output_dir: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut output_html = String::from(
        r###"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Lightcone</title>
    <link rel="stylesheet" href="dark.css">
    <style>
        .filter-icon {
            opacity: 0.35;
            cursor: pointer;
            margin-right: 5px;
            height: 38px;
            width: 38px;
        }
        .selected {
            opacity: 1.0;
        }
    </style>
</head>
<body>
    <noscript>
        It seems like you have JavaScript disabled, or your browser does not support it.<br>
        I am sorry but this website cannot function normally without it :)
    </noscript>

    <h1 style="margin-bottom: 0;">robinwings</h1>
    <small>Click ID to see the data :)</small>
    <hr>

    <div style="margin-top: 20px; margin-bottom: 10px;">
        <a class="button-link" style="text-decoration: none;" href="character.html">Character</a>
        <a class="button-link" style="text-decoration: none;" href="#">Lightcone</a>
        <a class="button-link" style="text-decoration: none;" href="relic.html">Relic</a>
        <a class="button-link" style="text-decoration: none;" href="https://github.com/robinwings/gen">Source Code</a>
    </div>

    <input 
        style="margin: 10px 0 20px;" 
        type="text" 
        id="myInput" 
        onkeyup="myFunction()" 
        placeholder="Search for lightcone"
    >

    <div style="margin-bottom: 20px;">
        <!-- Path Icons -->
"###,
    );

    for (_, path) in pt.path_map.iter() {
        if path.text.is_empty() {
            continue;
        }

        let path_icon = format!(
            r#"<img src="https://raw.githubusercontent.com/Mar-7th/StarRailRes/master/{}" class="filter-icon" id="{}-icon" data-path="{}" onclick="toggleFilter(this, 'path')" />
"#,
            path.icon,
            path.text.to_lowercase(),
            path.text.to_lowercase()
        );
        output_html.push_str(&path_icon);
    }

    output_html.push_str(
        r#"
<img src="icon/rarity/5star.png" class="filter-icon" id="5-star-icon" data-rarity="5*" onclick="toggleFilter(this, 'rarity')" />
<img src="icon/rarity/4star.png" class="filter-icon" id="4-star-icon" data-rarity="4*" onclick="toggleFilter(this, 'rarity')" />
<img src="icon/rarity/3star.png" class="filter-icon" id="3-star-icon" data-rarity="3*" onclick="toggleFilter(this, 'rarity')" />

    </div>

    <table id="myTable">
        <tr>
            <th>Lightcone</th>
            <th>Path</th>
            <th>Rarity</th>
            <th>ID</th>
        </tr>"#,
    );

    for (lightcone_id, lightcone) in lc.lightcone_map.iter() {
        let lc_path = match lightcone.path.as_str() {
            "Knight" => "Preservation",
            "Rogue" => "The Hunt",
            "Mage" => "Erudition",
            "Warlock" => "Nihility",
            "Shaman" => "Harmony",
            "Warrior" => "Destruction",
            "Priest" => "Abundance",
            "Memory" => "Remembrance",
            _ => &lightcone.path,
        };

        let row = format!(
            r#"<tr>
                <td>{}</td>
                <td>{}</td>
                <td>{}*</td>
                <td><a class='button-link' style='text-decoration: none;' href='lightcone/{}.html'>{}</a></td>
            </tr>"#,
            lightcone.name, lc_path, lightcone.rarity, lightcone_id, lightcone_id
        );
        output_html.push_str(&row);
    }

    output_html.push_str(r#"
    </table>
<script>
let filters = { element: [], path: [], rarity: [] };

function toggleFilter(e, t) {
    e.classList.toggle("selected");
    let l = e.getAttribute(`data-${t}`),
        r = filters[t].indexOf(l);
    r > -1 ? filters[t].splice(r, 1) : filters[t].push(l);
    applyFilter();
}

function myFunction() {
    applyFilter();
}

function applyFilter() {
    let e = document.getElementById("myInput").value.toUpperCase(),
        t = document.getElementById("myTable").getElementsByTagName("tr"),
        s = 0;

    for (let l = 1; l < t.length; l++) {
        let r = t[l],
            n = r.cells[0],
            o = r.cells[2].innerText.toLowerCase(),
            y = r.cells[1].innerText.toLowerCase(),
            p = r.cells[3].innerText;

        if (n) {
            let f = (n.textContent || n.innerText).toUpperCase().indexOf(e) > -1,
                c = filters.element.length === 0 || filters.element.includes(p),
                d = filters.path.length === 0 || filters.path.includes(y),
                b = filters.rarity.length === 0 || filters.rarity.includes(o);

            f && c && d && b ? (
                r.style.display = "",
                s % 2 == 0 ? r.style.backgroundColor = "var(--table-bg)" : r.style.backgroundColor = "var(--table-bg-alt)",
                s++
            ) : r.style.display = "none";
        }
    }
    applyZebraStripes(t);
}

function applyZebraStripes(e) {
    let t = 0;
    for (let l = 1; l < e.length; l++) {
        let r = e[l];
        r.style.display !== "none" && (t++ % 2 === 0 ? r.style.backgroundColor = "var(--table-bg-alt)" : r.style.backgroundColor = "var(--table-bg)");
    }
}
</script>
</body>
</html>"#);

    let mut file = File::create(format!("{}lightcone.html", output_dir))?;
    file.write_all(output_html.as_bytes())?;

    Ok(())
}
