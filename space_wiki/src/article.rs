use crate::types::{Article, ArticleType, Block, Document, Infobox, Segment};
use std::fs::read_to_string;
use std::path::Path;
use crate::types::{EngineSubtype, LaunchVehicleSubtype, SpacecraftSubtype};
use indexmap::IndexMap;

pub fn load_article(path: &Path) -> Result<Article, String>{
    let contents = read_to_string(path).map_err(|e| e.to_string())?;
    let value: toml::Value = toml::from_str(&contents).map_err(|e| e.to_string())?;
    let meta = value.get("meta")
        .and_then(|m| m.as_table())
        .ok_or("Missing meta".to_string())?;
    let title = meta.get("title")
    .and_then(|t| t.as_str())
    .ok_or("Missing title".to_string())?
    .to_string();
    let type_str = meta.get("type")
        .and_then(|t| t.as_str())
        .ok_or("Missing type".to_string())?;
    let subtype_str = meta.get("subtype")
        .and_then(|t| t.as_str())
        .ok_or("Missing subtype".to_string())?;

    let article_type = match (type_str, subtype_str) {
        ("engine", "cryogenic") => ArticleType::Engine(EngineSubtype::Cryogenic),
        ("engine", "liquid") => ArticleType::Engine(EngineSubtype::Liquid),
        ("engine", "solid") => ArticleType::Engine(EngineSubtype::Solid),
        ("engine", "nuclear") => ArticleType::Engine(EngineSubtype::Nuclear),
        ("engine", "electric") => ArticleType::Engine(EngineSubtype::Electric),
        ("launch_vehicle", "small_lift") => ArticleType::LaunchVehicle(LaunchVehicleSubtype::SmallLift),
        ("launch_vehicle", "medium_lift") => ArticleType::LaunchVehicle(LaunchVehicleSubtype::MediumLift),
        ("launch_vehicle", "heavy_lift") => ArticleType::LaunchVehicle(LaunchVehicleSubtype::HeavyLift),
        ("launch_vehicle", "super_heavy_lift") => ArticleType::LaunchVehicle(LaunchVehicleSubtype::SuperHeavyLift),
        ("launch_vehicle", "upper_stage") => ArticleType::LaunchVehicle(LaunchVehicleSubtype::UpperStage),
        ("spacecraft", "crew_capsule") => ArticleType::Spacecraft(SpacecraftSubtype::CrewCapsule),
        ("spacecraft", "cargo_capsule") => ArticleType::Spacecraft(SpacecraftSubtype::CargoCapsule),
        ("spacecraft", "lander") => ArticleType::Spacecraft(SpacecraftSubtype::Lander),
        ("spacecraft", "satellite") => ArticleType::Spacecraft(SpacecraftSubtype::Satellite),
        ("spacecraft", "spacestation") => ArticleType::Spacecraft(SpacecraftSubtype::Spacestation),
        ("spacecraft", "probe") => ArticleType::Spacecraft(SpacecraftSubtype::Probe),
        _ => return Err("Unknown type/subtype".to_string()),
    };

    let infobox = value.get("infobox")
        .and_then(|i| i.as_table())
        .ok_or("Missing infobox".to_string())?;
    let mut fields = IndexMap::new();
    for(key, val) in infobox.iter(){
        if let Some(s) = val.as_str(){
            fields.insert(key.clone(), s.to_string());

        }
    }
    let infobox = Infobox{
        fields,
    };

    let body = value.get("body")
        .and_then(|i| i.as_table())
        .ok_or("Missing body".to_string())?;
    let body_text = body.get("text")
    .and_then(|t| t.as_str())
    .ok_or("Missing body text".to_string())?;
    //TODO implement parsing logic to get the heading with the body text for document.
    //This is a primative implementation to test the function building.
    let document = vec![Block::Paragraph {segments: vec![Segment::Text(body_text.to_string())]}];
    let document = Document{ blocks: document };
    let article = Article{
        title,
        article_type,
        infobox,
        body: document
    };
    Ok(article)
}