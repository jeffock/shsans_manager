use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Paper {
    pub doi: String,
    pub title: String,
    pub tags: Vec<String>,
}

// impl constructor //
impl Paper {
    pub fn new(doi: String, title: String, tags: Vec<String>) -> Paper {
        Paper { doi, title, tags }
    }
}
