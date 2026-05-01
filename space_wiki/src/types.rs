use std::collections::HashMap;

pub struct Article{
    pub title: String,
    pub article_type: ArticleType,
    pub infobox: Infobox,
    pub body: Document,
}

pub struct Infobox{
    pub flat_fields: HashMap<String, String>,
    pub field_order: Vec<String>,
}

pub struct Document{
    pub blocks: Vec<Block>,
}

pub enum ArticleType {
    Engine(EngineSubtype),
    LaunchVehicle(LaunchVehicleSubtype),
    Spacecraft(SpacecraftSubtype),
}

pub enum EngineSubtype {
    Cryogenic,
    Liquid,
    Solid,
    Nuclear,
    Electric,

}

pub enum LaunchVehicleSubtype {
    SuperHeavyLift,
    HeavyLift,
    MediumLift,
    SmallLift,
}

pub enum SpacecraftSubtype {
    Capsule,
    Lander,
    Satellite,
    Spacestation,
    Probe,
}
pub enum Block{
    Heading { level: u8, text: String },
    Paragraph { segments: Vec<Segment> },
}

pub enum Segment{
    Text(String),
}