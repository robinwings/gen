#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let is_mini = if args.len() == 2 {
        args[1] == "mini"
    } else {
        false
    };

    let output_dir = if is_mini {
        "__output_minified__/"
    } else {
        "__output__/"
    };

    index::generate_index(output_dir).await?;
    data::generate_data(output_dir).await?;

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
                    minify_html::minify(html_content.as_bytes(), &minify_html::Cfg::default());
                std::fs::write(entry.path(), minified_html)?;
            }
        }
    }

    Ok(())
}
