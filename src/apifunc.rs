use serde_json::Value;

#[path = "paperstruct.rs"] mod paperstruct;
use paperstruct::Paper;

pub async fn fetch_paper(doi: &str) -> Result<Paper, Error> {
    Ok(())
}
