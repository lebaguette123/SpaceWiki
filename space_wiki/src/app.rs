use std::fs::read_dir;
use std::collections::HashMap;
use crate::types::{Article, ArticleType, EngineSubtype, LaunchVehicleSubtype, SidebarEntry, SpacecraftSubtype};
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
            let index = self.sidebar_entries()
                .iter()
                .position(|entry| matches!(entry, SidebarEntry::Article{ key, ..} if key == name)).unwrap();
            self.selected_sidebar_index = index;
            Ok(())
        } else {
            Err("Article not found".to_string())
        }
    }


    pub fn move_sidebar_down(&mut self){
        let current_list = self.sidebar_entries();
        let next = current_list
            .iter()
            .enumerate()
            .find(|(i, entry)| i > &self.selected_sidebar_index && matches!(entry, SidebarEntry::Article{..}));

        if let Some((i, _ )) = next{
            self.selected_sidebar_index = i;
        }
    }

    pub fn move_sidebar_up(&mut self){
        let current_list = self.sidebar_entries();
        let before = current_list
            .iter()
            .enumerate()
            .rev().find(|(i, entry)| i < &self.selected_sidebar_index && matches!(entry, SidebarEntry::Article{..}));
        if let Some((i, _ )) = before{
            self.selected_sidebar_index = i;
        }
    }

    pub fn sidebar_entries(&self) -> Vec<SidebarEntry>{
        let mut articles: Vec<(&String, &Article)> = self.loaded_articles.iter().collect();
        articles.sort_by(|(_,a), (_, b)| a.title.cmp(&b.title));
        articles.sort_by_key(|(_, article)| type_order(&article.article_type));
        let mut entries: Vec<SidebarEntry> = Vec::new();
        let mut last_type: Option<u8> = None;
        let mut last_subtype: Option<u8> = None;

        for (stem, article) in articles {
            let (t, st) = type_order(&article.article_type);
            if Some(t) != last_type{
                entries.push(SidebarEntry::TypeHeading(type_display(&article.article_type).0.to_string()));
                last_subtype = None;
            }
            if Some(st) != last_subtype{
                entries.push(SidebarEntry::SubtypeHeading(type_display(&article.article_type).1.to_string()));
            }
            entries.push(SidebarEntry::Article{ title: article.title.clone(), key: stem.clone() });
            last_type = Some(t);
            last_subtype = Some(st);
        }
        entries
    }
}

fn type_order(t: &ArticleType) -> (u8, u8){
    match t{
        ArticleType::LaunchVehicle(st) =>{
            let subtype = match st{
                LaunchVehicleSubtype::SuperHeavyLift => 0,
                LaunchVehicleSubtype::HeavyLift => 1,
                LaunchVehicleSubtype::MediumLift => 2,
                LaunchVehicleSubtype::SmallLift => 3,
                LaunchVehicleSubtype::UpperStage => 4,
            };
            (0, subtype)
        } ,
        ArticleType::Spacecraft(st) => {
            let subtype = match st{
                SpacecraftSubtype::CrewCapsule =>0,
                SpacecraftSubtype::CargoCapsule =>1,
                SpacecraftSubtype::Lander => 2,
                SpacecraftSubtype::Satellite => 3,
                SpacecraftSubtype::Probe => 4,
                SpacecraftSubtype::Spacestation => 5,
            };
            (1, subtype)

        },
        ArticleType::Engine(st) => {
            let subtype = match st{
                EngineSubtype::Cryogenic=> 0,
                EngineSubtype::Liquid => 1,
                EngineSubtype::Solid => 2,
                EngineSubtype::Nuclear => 3,
                EngineSubtype::Electric => 4,

            };
            (2, subtype)
        },
    }
}

pub fn type_display(t: &ArticleType) -> (&str, &str){
    match t {
        ArticleType::LaunchVehicle(st) => {
            let subtype = match st {
                LaunchVehicleSubtype::SuperHeavyLift => "SUPER HEAVY LIFT",
                LaunchVehicleSubtype::HeavyLift => "HEAVY LIFT",
                LaunchVehicleSubtype::MediumLift => "MEDIUM LIFT",
                LaunchVehicleSubtype::SmallLift => "SMALL LIFT",
                LaunchVehicleSubtype::UpperStage => "UPPER STAGE",
            };
            ("LAUNCH VEHICLES", subtype)
        },
        ArticleType::Spacecraft(st) => {
            let subtype = match st {
                SpacecraftSubtype::CrewCapsule => "CREW CAPSULE",
                SpacecraftSubtype::CargoCapsule => "CARGO CAPSULE",
                SpacecraftSubtype::Lander => "LANDER",
                SpacecraftSubtype::Satellite => "SATELLITE",
                SpacecraftSubtype::Probe => "PROBE",
                SpacecraftSubtype::Spacestation => "SPACE STATION",
            };
            ("SPACECRAFT", subtype)
        },
        ArticleType::Engine(st) => {
            let subtype = match st {
                EngineSubtype::Cryogenic => "CRYOGENIC",
                EngineSubtype::Liquid => "LIQUID",
                EngineSubtype::Solid => "SOLID",
                EngineSubtype::Nuclear => "NUCLEAR",
                EngineSubtype::Electric => "ELECTRIC",
            };
            ("ENGINES", subtype)
        },
    }
}