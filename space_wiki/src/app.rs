use std::fs::read_dir;
use std::collections::HashMap;
use crate::types::Article;
use crate::article::load_article;

pub struct App{
    pub loaded_articles: HashMap<String, Article>,
    pub current_article: Option<String>,
    pub selected_sidebar_index: usize,
}
impl App{
    pub fn new() -> Result<App, Box<dyn std::error::Error>>{
        let mut loaded_articles = HashMap::new();

        for entry in read_dir("articles/")? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("toml") {
                let article = load_article(&path)?;
                if let Some(stem) = path.file_stem().and_then(|s| s.to_str()){
                    loaded_articles.insert(stem.to_string(), article);
                }
            }
        }

        Ok(App {
            loaded_articles,
            current_article: None,
            selected_sidebar_index: 0,
        })
    }

    pub fn open_article(&mut self, name: &str) -> Result<(), String>{
        if self.loaded_articles.contains_key(name){
            self.current_article = Some(name.to_string());
            self.selected_sidebar_index = 0;
            Ok(())
        } else {
            Err("Article not found".to_string())
        }
    }
}