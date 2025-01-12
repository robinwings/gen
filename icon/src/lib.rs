use fast_image_resize::Resizer;
use fast_image_resize::images::Image;
use image::GenericImageView;
use image::{DynamicImage, ImageReader};
use image::{ImageEncoder, codecs::png::PngEncoder};
use starrailrust::Client;
use std::error::Error;
use std::io::Write;
use std::path::Path;
use std::{fs, io::BufWriter};
use walkdir::WalkDir;

pub enum IconType {
    Character,
    Element,
    Item,
    Path,
    Relic,
}

const BASE_URL: &str = "https://raw.githubusercontent.com/Mar-7th/StarRailRes/master/";

pub fn write_rarity_and_planar_icon(output_dir: &str) -> Result<(), Box<dyn Error>> {
    let assets = [
        (include_bytes!("./3star.png") as &[u8], "rarity/3star.png"),
        (include_bytes!("./4star.png"), "rarity/4star.png"),
        (include_bytes!("./5star.png"), "rarity/5star.png"),
        (include_bytes!("./planar.png"), "relic_filter/planar.png"),
    ];

    fn write_file(output_dir: &str, sub_path: &str, content: &[u8]) -> std::io::Result<()> {
        let full_path = format!("{}icon/{}", output_dir, sub_path);
        let path = Path::new(&full_path);

        if path.exists() {
            return Ok(());
        }

        let mut file = fs::File::create(path)?;
        file.write_all(content)?;
        Ok(())
    }

    for (content, sub_path) in assets {
        write_file(output_dir, sub_path, content)?;
    }

    Ok(())
}

pub async fn download_image(
    client: &Client,
    icon: &str,
    itype: &IconType,
    output_dir: &str,
) -> Result<(), Box<dyn Error>> {
    let icon_file_name = icon.split("/").collect::<Vec<&str>>()[2];

    let output_file = match itype {
        IconType::Character => format!("{}icon/character/{}", output_dir, icon_file_name),
        IconType::Element => format!("{}icon/element/{}", output_dir, icon_file_name),
        IconType::Item => format!("{}icon/item/{}", output_dir, icon_file_name),
        IconType::Path => format!("{}icon/path/{}", output_dir, icon_file_name),
        IconType::Relic => format!("{}icon/relic/{}", output_dir, icon_file_name),
    };

    let path = Path::new(&output_file);
    if path.exists() {
        return Ok(());
    }

    let image_url = format!("{}{}", BASE_URL, icon);
    let response = client.get(&image_url).send().await?;

    if response.status().is_success() {
        let bytes = response.bytes().await?;
        fs::write(&output_file, bytes)?;
    } else {
        eprintln!("Failed to download image: {}", response.status());
    }

    Ok(())
}

pub fn resize_images_in_directory(
    input_dir: &str,
    output_dir: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    for entry in WalkDir::new(input_dir).into_iter().filter_map(Result::ok) {
        if entry.file_type().is_file() {
            let input_path = entry.path();
            if let Some(ext) = input_path.extension().and_then(|e| e.to_str()) {
                if ext.eq_ignore_ascii_case("png")
                    || ext.eq_ignore_ascii_case("webp")
                    || ext.eq_ignore_ascii_case("jpg")
                {
                    let output_path =
                        Path::new(output_dir).join(input_path.strip_prefix(input_dir)?);

                    if let Some(parent) = output_path.parent() {
                        if !parent.exists() {
                            fs::create_dir_all(parent)?;
                        }
                    }

                    if let Err(e) = resize_image(input_path, &output_path) {
                        eprintln!("Failed to resize image {}: {}", input_path.display(), e);
                    }
                }
            }
        }
    }

    Ok(())
}

fn resize_image(input_path: &Path, output_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(file_name) = input_path.file_name() {
        // not U8x4
        if file_name == "planar.png" {
            return Ok(());
        }
    }

    let src_image = ImageReader::open(input_path)?.decode()?;

    let (width, height) = src_image.dimensions();

    if width == 38 && height == 38 {
        return Ok(());
    }

    let dst_width = 38;
    let dst_height = 38;

    let mut resizer = Resizer::new();

    let mut dst_image = Image::new(dst_width, dst_height, fast_image_resize::PixelType::U8x4);

    resizer.resize(&src_image, &mut dst_image, None)?;

    let mut result_buf = BufWriter::new(Vec::new());

    PngEncoder::new(&mut result_buf).write_image(
        dst_image.buffer(),
        dst_width,
        dst_height,
        image::ColorType::Rgba8.into(),
    )?;

    let result_data = result_buf.into_inner()?;
    fs::write(output_path, result_data)?;

    Ok(())
}
