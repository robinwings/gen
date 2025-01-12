#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let is_mini = if args.len() == 2 {
        args[1] == "mini"
    } else {
        false
    };

    let output_dir = if is_mini {
        create_folder_structure(&std::path::Path::new("./__output_minified__/"))?;
        "__output_minified__/"
    } else {
        create_folder_structure(&std::path::Path::new("./__output__/"))?;
        "__output__/"
    };

    let client = starrailrust::Client::new();

    index::generate_index(output_dir, &client).await?;
    data::generate_data(output_dir, &client).await?;

    if is_mini {
        let walker = walkdir::WalkDir::new(output_dir).into_iter();

        for entry in walker.filter_map(Result::ok) {
            if entry
                .path()
                .extension()
                .map(|ext| ext == "html")
                .unwrap_or(false)
            {
                let html_content = std::fs::read_to_string(entry.path())?;
                let minified_html =
                    minify_html::minify(html_content.as_bytes(), &minify_html::Cfg {
                        keep_spaces_between_attributes: true,
                        minify_css: true,
                        minify_js: true,
                        ..Default::default()
                    });
                std::fs::write(entry.path(), minified_html)?;
            }
        }
    }

    let output_dir = format!("{}icon/", output_dir);

    icon::resize_images_in_directory(&output_dir, &output_dir)?;

    Ok(())
}

fn create_folder_structure(root: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
    let folders = [
        "character",
        "icon/character",
        "icon/element",
        "icon/item",
        "icon/path",
        "icon/rarity",
        "icon/relic",
        "icon/relic_filter",
        "lightcone",
        "relic",
    ];

    for folder in &folders {
        let folder_path = root.join(folder);
        if !folder_path.exists() {
            std::fs::create_dir_all(&folder_path)?;
        }
    }

    Ok(())
}
