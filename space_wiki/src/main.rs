mod types;

use std::collections::HashMap;
use std::io::stdout;
use crossterm::event::{self, Event, KeyCode};
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use ratatui::widgets::Paragraph;
use crate::types::ArticleType::Engine;
use std::fs::read_to_string;
use std::path::Path;
use crate::types::{Article, ArticleType, Block, Document, Infobox, Segment};

fn cleanup(){
    execute!(stdout(),LeaveAlternateScreen).unwrap();
    disable_raw_mode().unwrap();
}

fn load_article(path: &Path) -> Result<Article, String>{
    let contents = read_to_string(path).map_err(|e| e.to_string())?;
    let value: toml::Value = toml::from_str(&contents).map_err(|e| e.to_string())?;
    let meta = value.get("meta")
        .and_then(|m| m.as_table())
        .ok_or("Missing meta".to_string())?;
    let title = meta.get("title")
        .and_then(|t| t.as_str())
        .ok_or("Missing title".to_string())?;
    let title = title.to_string();
    let type_str = meta.get("type")
        .and_then(|t| t.as_str())
        .ok_or("Missing type".to_string())?;
    let subtype_str = meta.get("subtype")
        .and_then(|t| t.as_str())
        .ok_or("Missing subtype".to_string())?;

    let article_type = match (type_str, subtype_str) {
        ("engine", "cryogenic") => ArticleType::Engine(types::EngineSubtype::Cryogenic),
        ("engine", "liquid") => ArticleType::Engine(types::EngineSubtype::Liquid),
        ("engine", "solid") => ArticleType::Engine(types::EngineSubtype::Solid),
        ("engine", "nuclear") => ArticleType::Engine(types::EngineSubtype::Nuclear),
        ("engine", "electric") => ArticleType::Engine(types::EngineSubtype::Electric),
        ("launch_vehicle", "small_lift") => ArticleType::LaunchVehicle(types::LaunchVehicleSubtype::SmallLift),
        ("launch_vehicle", "medium_lift") => ArticleType::LaunchVehicle(types::LaunchVehicleSubtype::MediumLift),
        ("launch_vehicle", "heavy_lift") => ArticleType::LaunchVehicle(types::LaunchVehicleSubtype::HeavyLift),
        ("launch_vehicle", "super_heavy_lift") => ArticleType::LaunchVehicle(types::LaunchVehicleSubtype::SuperHeavyLift),
        ("spacecraft", "crew_capsule") => ArticleType::Spacecraft(types::SpacecraftSubtype::CrewCapsule),
        ("spacecraft", "cargo_capsule") => ArticleType::Spacecraft(types::SpacecraftSubtype::CargoCapsule),
        ("spacecraft", "lander") => ArticleType::Spacecraft(types::SpacecraftSubtype::Lander),
        ("spacecraft", "satellite") => ArticleType::Spacecraft(types::SpacecraftSubtype::Satellite),
        ("spacecraft", "spacestation") => ArticleType::Spacecraft(types::SpacecraftSubtype::Spacestation),
        ("spacecraft", "probe") => ArticleType::Spacecraft(types::SpacecraftSubtype::Probe),
        _ => return Err("Unknown type/subtype".to_string()),
    };

    let infobox = value.get("infobox")
        .and_then(|i| i.as_table())
        .ok_or("Missing infobox".to_string())?;
    let mut flat_fields = HashMap::new();
    let mut field_order = Vec::new();
    for(key, val) in infobox.iter(){
        if let Some(s) = val.as_str(){
            field_order.push(key.clone());
            flat_fields.insert(key.clone(), s.to_string());

        }
    }
    let infobox = Infobox{
        flat_fields,
        field_order,

    };

    let body = value.get("body")
        .and_then(|i| i.as_table())
        .ok_or("Missing body".to_string())?;
    //TODO implement parsing logic to get the heading with the body text for document.
    //This is a primative implementation to test the function building.
    let document = vec![Block::Paragraph {segments: vec![Segment::Text(body.to_string())]}];
    let document = Document{ blocks: document };
    let article = Article{
        title,
        article_type,
        infobox,
        body: document
    };
    Ok(article)


}
fn main() {
    std::panic::set_hook(Box::new(|_| cleanup()));
    enable_raw_mode().unwrap();
    execute!(stdout(), EnterAlternateScreen).unwrap();

    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend).unwrap();
    let flat_fields = HashMap::new();
    let field_order = Vec::<String>::new();
    let infobox = types::Infobox{
        flat_fields,
        field_order,
    };

    let block = vec![
        types::Block::Heading { level: 1, text: String::from("RS-25") },
        types::Block::Paragraph { segments: vec![types::Segment::Text(String::from("The RS-25, also known as the Space Shuttle Main Engine (SSME), is a liquid-fuel rocket engine that was used on the Space Shuttle and is now being repurposed for NASA's Space Launch System (SLS). It is a cryogenic engine that burns liquid hydrogen and liquid oxygen, and it is known for its high performance and reliability. The RS-25 has a long history of successful missions and continues to be an important part of NASA's space exploration efforts."))] },
    ];


    let document = types::Document{
      blocks: block,
    };

    let article = types::Article{
        title: String::from("RS-25"),
        article_type: Engine(types::EngineSubtype::Cryogenic),
        infobox,
        body: document,


    };

    loop{
        terminal.draw(|frame| {
            let area = frame.area();
            let paragraph = Paragraph::new("SpaceWiki - press q to quit");
            frame.render_widget(paragraph, area);
        }).unwrap();
       if let Ok(Event::Key(key)) = event::read() {
           if key.code == KeyCode::Char('q') {
               cleanup();
               break;
           }
       }
    }


}
