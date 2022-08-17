use uuid::Uuid;

pub enum Species {
    BRCH,
    GALL,
    TRIK,
    PARA,
    PROC,
    MTCN,
    COEL,
    TREX,
    VELO,
    DILO,
    HERR,
    BRNX,
}

impl Species {
    pub fn as_str(&self) -> &'static str {
        match self {
            Species::BRCH => "BRCH",
            Species::GALL => "GALL",
            Species::TRIK => "TRIK",
            Species::PARA => "PARA",
            Species::PROC => "PROC",
            Species::MTCN => "MTCN",
            Species::COEL => "COEL",
            Species::TREX => "TREX",
            Species::VELO => "VELO",
            Species::DILO => "DILO",
            Species::HERR => "HERR",
            Species::BRNX => "BRNX",
        }
    }
}

pub struct Asset {
    pub id: Uuid,
    pub name: String,
    pub species: Species
}
