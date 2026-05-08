use indexmap::IndexMap;

pub struct Article{
    pub title: String,
    pub article_type: ArticleType,
    pub infobox: Infobox,
    pub body: Document,
}

pub struct Infobox{
    pub fields: IndexMap<String, String>,
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
    UpperStage,
}

pub enum SpacecraftSubtype {
    CrewCapsule,
    CargoCapsule,
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
    Link { target: String, display: Option<String> },
}

pub enum SidebarEntry {
    TypeHeading(String),
    SubtypeHeading(String),
    Article{ title: String, key: String },
}

#[allow(dead_code)]
pub enum SidebarMode {
    Sections,
    OnThisPage,
    History,
}