use crate::{CharacterMap, ElementMap, File, PathMap, Write};

pub fn generate(
    ch: CharacterMap,
    el: ElementMap,
    pt: &PathMap,
    output_dir: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut output_html = String::from(
        r###"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Character</title>
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
        <a class="button-link" style="text-decoration: none;" href="#">Character</a>
        <a class="button-link" style="text-decoration: none;" href="lightcone.html">Lightcone</a>
        <a class="button-link" style="text-decoration: none;" href="relic.html">Relic</a>
        <a class="button-link" style="text-decoration: none;" href="https://github.com/robinwings/gen">Source Code</a>
    </div>

    <input 
        style="margin: 10px 0 20px;" 
        type="text" 
        id="myInput" 
        onkeyup="myFunction()" 
        placeholder="Search for character"
    >

    <div style="margin-bottom: 20px;">
        <!-- Element Icons -->
"###,
    );

    // Add element icons
    for (_, element) in el.element_map.iter() {
        let element_icon = format!(
            r#"<img src="https://raw.githubusercontent.com/Mar-7th/StarRailRes/master/{}" class="filter-icon" id="{}-icon" data-element="{}" onclick="toggleFilter(this, 'element')" />
"#,
            element.icon,
            element.name.to_lowercase(),
            element.name.to_lowercase()
        );
        output_html.push_str(&element_icon);
    }

    output_html.push_str(
        r#"
        <!-- Path Icons -->
"#,
    );

    // Add path icons
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

    </div>

    <table id="myTable">
        <tr>
            <th>Character</th>
            <th>Path</th>
            <th>Element</th>
            <th>Rarity</th>
            <th>ID</th>
        </tr>"#,
    );

    // Add character rows
    for (character_id, character) in ch.character_map.iter() {
        let ch_name = match character.tag.as_str() {
            "mar7th" => "March 7th",
            "mar7th2" => "Swordmaster March 7th",

            "playerboy" => "Physical Caelus",
            "playerboy2" => "Fire Caelus",
            "playerboy3" => "Imaginary Caelus",
            "playerboy4" => "Ice Caelus",

            "playergirl" => "Physical Stelle",
            "playergirl2" => "Fire Stelle",
            "playergirl3" => "Imaginary Stelle",
            "playergirl4" => "Ice Stelle",

            _ => &character.name,
        };

        let ch_element = match character.element.as_str() {
            "Thunder" => "Lightning",
            _ => &character.element,
        };

        let ch_path = match character.path.as_str() {
            "Knight" => "Preservation",
            "Rogue" => "The Hunt",
            "Mage" => "Erudition",
            "Warlock" => "Nihility",
            "Shaman" => "Harmony",
            "Warrior" => "Destruction",
            "Priest" => "Abundance",
            "Memory" => "Remembrance",
            _ => &character.path,
        };

        let row = format!(
            r#"<tr>
                <td>{}</td>
                <td>{}</td>
                <td>{}</td>
                <td>{}*</td>
                <td><a class='button-link' style='text-decoration: none;' href='character/{}.html'>{}</a></td>
            </tr>"#,
            ch_name, ch_path, ch_element, character.rarity, character_id, character_id
        );
        output_html.push_str(&row);
    }

    output_html.push_str(r#"
    </table>
<script>
function reloadCSS() {
    var e = document.querySelector('link[href*="dark.css"]');
    if (e) {
        var t = e.getAttribute("href").split("?")[0] + "?v=" + new Date().getTime();
        e.setAttribute("href", t);
    }
}

let filters = {
    element: [],
    path: [],
    rarity: []
};

function toggleFilter(e, t) {
    e.classList.toggle("selected");
    let l = e.getAttribute(`data-${t}`),
        r = filters[t].indexOf(l);
    r > -1 ? filters[t].splice(r, 1) : filters[t].push(l);
    applyFilter();
}

function applyFilter() {
    let input = document.getElementById("myInput").value.toUpperCase();
    let table = document.getElementById("myTable");
    let rows = table.getElementsByTagName("tr");
    let count = 0;

    for (let i = 1; i < rows.length; i++) {
        let row = rows[i];
        let cell0 = row.cells[0];
        let cell1 = row.cells[1];
        let cell2 = row.cells[2];
        let cell3 = row.cells[3];
        if (cell0 && cell1 && cell2 && cell3) {
            let text0 = cell0.innerText.toUpperCase();
            let text1 = cell1.innerText.toLowerCase();
            let text2 = cell2.innerText.toLowerCase();
            let text3 = cell3.innerText;

            let matchesSearch = text0.indexOf(input) > -1;
            let matchesElement = filters.element.length === 0 || filters.element.includes(text2);
            let matchesPath = filters.path.length === 0 || filters.path.includes(text1);
            let matchesRarity = filters.rarity.length === 0 || filters.rarity.includes(text3);

            if (matchesSearch && matchesElement && matchesPath && matchesRarity) {
                row.style.display = "";
                count++ % 2 === 0 ? row.style.backgroundColor = "var(--table-bg-alt)" : row.style.backgroundColor = "var(--table-bg)";
            } else {
                row.style.display = "none";
            }
        }
    }
}

function myFunction() {
    applyFilter();
}
</script>
</body>
</html>"#);

    let mut file = File::create(format!("{}character.html", output_dir))?;
    file.write_all(output_html.as_bytes())?;

    Ok(())
}
