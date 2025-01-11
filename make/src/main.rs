#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(index::generate_index("__output__/").await?)
}
