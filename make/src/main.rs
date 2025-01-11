#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    index::generate_index("__output__/").await?;
    data::generate_data("__output__/").await?;
    Ok(())
}
