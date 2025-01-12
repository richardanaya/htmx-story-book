use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Book {
    pub id: u32,
    pub title: String,
    pub summary: String,
    pub pages: Vec<Page>,
    pub starting_page: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Page {
    pub id: u32,
    pub content: String,
    pub choices: Vec<Choice>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Choice {
    pub text: String,
    pub target_page_id: u32,
}
