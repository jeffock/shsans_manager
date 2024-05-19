#[derive(Debug)]
pub struct Paper<'a> {
    pub doi: &'a String,
    pub title: &'a String,
    pub tags: Vec<String>,
}

// impl constructor //
impl<'a> Paper<'a> {
    pub fn new(doi: &'a String, title: &'a String, tags: Vec<String>) -> Paper<'a> {
        Paper { doi, title, tags }
    }
}
