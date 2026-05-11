#[derive(Clone)]
pub struct NavLink {
    pub display: String,
    pub target: String,
    pub source: LinkSource,
}

#[derive(Clone)]
pub enum LinkSource{
    InfboxFlatField,
    InfboxStage,
    Body,
}