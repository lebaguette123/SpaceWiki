pub struct UnifiedLink{
    pub display: String,
    pub target: String,
    pub source: LinkSource,
}

pub enum LinkSource{
    InfboxFlatField,
    InfboxStage,
    Body,
}