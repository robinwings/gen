use crate::{RelicSetMap, File, Write};

pub fn generate(rl: RelicSetMap, output_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut output_html = String::from(
        r###"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Relic</title>
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
        <a class="button-link" style="text-decoration: none;" href="lightcone.html">Lightcone</a>
        <a class="button-link" style="text-decoration: none;" href="#">Relic</a>
        <a class="button-link" style="text-decoration: none;" href="https://github.com/robinwings/gen">Source Code</a>
    </div>

    <input 
        style="margin: 10px 0 20px;" 
        type="text" 
        id="myInput" 
        onkeyup="myFunction()" 
        placeholder="Search for relic"
    >

    <div style="margin-bottom: 20px;">
<img src="icon/relic/planar.webp" class="filter-icon" id="planar-icon" data-type="Planar" onclick="toggleFilter(this, 'type')" />

    </div>

    <table id="myTable">
        <tr>
            <th>Relic</th>
            <th>Type</th>
            <th>ID</th>
        </tr>"###,
    );

    for (relic_id, relic) in rl.relic_set_map.iter() {
        let relic_type = if relic.desc.len() != 1 {
            "Relic"
        } else {
            "Planar"
        };

        let row = format!(
            r#"<tr>
                <td>{}</td>
                <td>{}</td>
                <td><a class='button-link' style='text-decoration: none;' href='relic/{}.html'>{}</a></td>
            </tr>"#,
            relic.name, relic_type, relic_id, relic_id
        );
        output_html.push_str(&row);
    }

    output_html.push_str(r#"
    </table>
<script>
let filters = { type: [], rarity: [] };

function toggleFilter(element, filterType) {
    element.classList.toggle("selected");
    let value = element.getAttribute(`data-${filterType}`).toLowerCase();
    let index = filters[filterType].indexOf(value);
    if (index > -1) {
        filters[filterType].splice(index, 1);
    } else {
        filters[filterType].push(value);
    }
    myFunction();
}

function showAllRows() {
    let rows = document.getElementById("myTable").getElementsByTagName("tr");
    for (let i = 1; i < rows.length; i++) {
        rows[i].style.display = "";
    }
    applyZebraStripes(rows);
}

function myFunction() {
    let input = document.getElementById("myInput").value.toUpperCase();
    let rows = document.getElementById("myTable").getElementsByTagName("tr");
    let count = 0;
    for (let i = 1; i < rows.length; i++) {
        let row = rows[i];
        let cell0 = row.cells[0];
        let cell1 = row.cells[1];
        let cell2 = row.cells[2];
        if (cell0 && cell1 && cell2) {
            let text0 = cell0.innerText.toLowerCase();
            let text1 = cell1.innerText.toLowerCase();
            let text2 = cell2.innerText.trim();
            let matchesSearch = text0.toUpperCase().indexOf(input) > -1;
            let matchesType = filters.type.length === 0 || filters.type.includes(text1);
            let matchesRarity = filters.rarity.length === 0 || filters.rarity.includes(text2);
            if (matchesSearch && matchesType && matchesRarity) {
                row.style.display = "";
                count++ % 2 === 0 ? row.style.backgroundColor = "var(--table-bg-alt)" : row.style.backgroundColor = "var(--table-bg)";
            } else {
                row.style.display = "none";
            }
        }
    }
    applyZebraStripes(rows);
}

function applyZebraStripes(rows) {
    let count = 0;
    for (let i = 1; i < rows.length; i++) {
        let row = rows[i];
        if (row.style.display !== "none") {
            count++ % 2 === 0 ? row.style.backgroundColor = "var(--table-bg-alt)" : row.style.backgroundColor = "var(--table-bg)";
        } else {
            row.style.backgroundColor = "";  // Clear the background color if the row is hidden
        }
    }
}
</script>
</body>
</html>"#);

    let mut file = File::create(format!("{}relic.html", output_dir))?;
    file.write_all(output_html.as_bytes())?;

    Ok(())
}
