use std::fs::read_dir;
use std::collections::HashMap;
use crate::types::{Article, ArticleType, Block, EngineSubtype, LaunchVehicleSubtype, Segment, SidebarEntry, SpacecraftSubtype};
use crate::article::load_article;
use crate::link::{LinkSource, NavLink};
use crate::parser::segments_from_str;

pub struct App{
    pub loaded_articles: HashMap<String, Article>,
    pub current_article: Option<String>,
    pub selected_sidebar_index: usize,
    pub nav_links: Vec<NavLink>,
    pub infobox_link_count: usize,
    pub focused_link: Option<usize>,
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
            nav_links: Vec::new(),
            infobox_link_count: 0,
            focused_link: None,
        })
    }

    pub fn open_article(&mut self, name: &str) -> Result<(), String>{
        if self.loaded_articles.contains_key(name){
            self.current_article = Some(name.to_string());
            let index = self.sidebar_entries()
                .iter()
                .position(|entry| matches!(entry, SidebarEntry::Article{ key, ..} if key == name)).unwrap();
            self.selected_sidebar_index = index;
            self.build_nav_links();
            self.focused_link = None;
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

    pub fn cycle_link_next(&mut self){
        if self.nav_links.is_empty(){
            return;
        }
        match self.focused_link{
            None => self.focused_link = Some(0),
            Some(i) if i + 1 == self.nav_links.len() => self.focused_link = Some(0),
            Some(i) => self.focused_link = Some(i + 1),
        }
    }

    pub fn cycle_link_prev(&mut self){
        if self.nav_links.is_empty(){
            return;
        }
        match self.focused_link{
            None | Some(0) => self.focused_link = Some(self.nav_links.len() - 1),
            Some(i) => self.focused_link = Some(i - 1),
        }
    }

    pub fn link_is_valid(&self, target: &str) -> bool{
        self.loaded_articles.contains_key(target)
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
    pub fn build_nav_links(&mut self) {
        if self.current_article.is_none(){
            self.nav_links = Vec::new();
            self.infobox_link_count = 0;
            return;
        }
        let mut links: Vec<NavLink> = Vec::new();
        let mut infobox_count: usize = 0;
        let key = self.current_article.as_ref().unwrap().clone();
        let article = self.loaded_articles.get(&key).unwrap();
        let fields = article.infobox.fields.clone();
        let body = article.body.blocks.clone();

        for (_, value) in fields.iter(){
            for seg in segments_from_str(value) {
                match seg{
                    Segment::Link{target, display} =>{
                        links.push(NavLink{target: target.clone(), source: LinkSource::InfboxFlatField, display: display.unwrap_or(target) })
                    },
                    _ => ()
                }
            }
        }
        infobox_count = links.len();
        for block in body{
            if let Block::Paragraph {segments} = block{
                for seg in segments{
                    if let Segment::Link{target, display} = seg{
                        links.push(NavLink{target: target.clone(), source: LinkSource::Body, display: display.unwrap_or(target) })
                    }
                }
            }
        }

        self.nav_links = links;
        self.infobox_link_count = infobox_count;
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