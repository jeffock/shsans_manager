use serde_json::Value;

use crate::paperstruct::Paper;

pub async fn fetch_paper(doi: &str) -> Result<Paper, Box<dyn std::error::Error>> {
    let url = format!("https://api.semanticscholar.org/v1/paper/{}", doi);
    let response = reqwest::get(&url).await?;

    if response.status().is_success() {
        let json: Value = response.json().await?;
        let title = json["title"].as_str().unwrap_or("").to_string();
        let keywords: Vec<String> = json["keywords"]
            .as_array()
            .unwrap_or(&vec![])
            .iter()
            .filter_map(|v| v.as_str().map(String::from))
            .collect();

        let paper_out = Paper::new(doi.to_string(), title, keywords);
        Ok(paper_out)
    } else {
        Err(format!("Failed to fetch paper with DOI {}", doi).into())
    }
}
