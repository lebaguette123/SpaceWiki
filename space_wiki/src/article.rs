use crate::types::{Article, ArticleType, Infobox, EngineSubtype, LaunchVehicleSubtype, SpacecraftSubtype, Table, Segment, Stage};
use std::fs::read_to_string;
use std::path::Path;
use indexmap::IndexMap;
use crate::parser::{parse_body, segments_from_str};

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
        ("spacecraft", "space_station") => ArticleType::Spacecraft(SpacecraftSubtype::Spacestation),
        ("spacecraft", "probe") => ArticleType::Spacecraft(SpacecraftSubtype::Probe),
        _ => return Err("Unknown type/subtype".to_string()),
    };

    let mut subtables = Vec::new();
    let mut stages = Vec::new();

    if let Some(infobox_root) = value.get("infobox").and_then(|v| v.as_table()){
        for (name, content) in infobox_root{
            if let Some(fields_table) = content.as_table(){
                let mut fields = IndexMap::new();

                for (k, v) in fields_table{
                    if let Some(val_str) = v.as_str(){
                        fields.insert(k.clone(), val_str.to_string());
                    }
                }

                subtables.push(Table{
                    title: name.clone(),
                    fields,
                });
            }
        }
    }

    if let Some(stage_root) = value.get("stage").and_then(|v| v.as_table()){
        for(number_key, content) in stage_root{
            if let Some(s) = content.as_table(){

                let number = number_key.parse::<usize>().map_err(|_| format!("Stage header [{}] must be a number", number_key))?;
                let name_str = s.get("name")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Unknown Stage");
                let name = segments_from_str(name_str)
                    .into_iter()
                    .next()
                    .unwrap_or(Segment::Text(name_str.to_string()));
                let description = s.get("description")
                    .and_then(|v| v.as_str())
                    .map(segments_from_str)
                    .unwrap_or_default();
                let engines = s.get("engines").and_then(|v| v.as_str()).map(|e_str|{
                    e_str.split('•')
                        .map(|part| segments_from_str(part.trim()))
                        .collect()
                }).unwrap_or_default();

                stages.push(Stage{
                    number,
                    name,
                    description,
                    engines,
                    propellant: s.get("propellant").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                    bespoke: s.get("bespoke").and_then(|v| v.as_bool()).unwrap_or(false)
                });
            }
        }
    }

    stages.sort_by_key(|s| s.number);

    let infobox = Infobox{
        subtables,
        stages,
    };

    let body = value.get("body")
        .and_then(|i| i.as_table())
        .ok_or("Missing body".to_string())?;
    let body_text = body.get("text")
    .and_then(|t| t.as_str())
    .ok_or("Missing body text".to_string())?;
    let document = parse_body(body_text);
    let article = Article{
        title,
        article_type,
        infobox,
        body: document
    };
    Ok(article)
}