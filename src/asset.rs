use uuid::Uuid;
use chrono::{offset::{Local}, DateTime};
use crate::vitals;

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
    pub species: Species,
    pub security_level: u8,
    pub vitals: vitals::Vitals,
    // pub diet
    // pub location
    pub paddock_id: Option<Uuid>,
    // some dates
    pub last_seen: Option<DateTime<Local>>, // this is the perfect use case for the builder.
    pub egg_laid: Option<DateTime<Local>>,
    pub hatched: Option<DateTime<Local>>,
    pub released: Option<DateTime<Local>>,
    pub died: Option<DateTime<Local>>,
    pub last_veterinary_checkup: Option<DateTime<Local>>,
    pub record_created: DateTime<Local>,
    // some flags;
    pub is_alive: bool,
    pub is_contained: bool,
    pub is_in_transit: bool,
    pub is_in_paddock: bool,
    pub needs_status_check: bool,
    pub needs_veterinary_care: bool,
    pub needs_feeding: bool,
}

impl Asset {
    pub fn new(species: Species, name: String) -> Asset {
        Asset {
            id: Uuid::new_v4(),
            name: String::from(name),
            species,
            security_level: 1,
            vitals: vitals::Vitals::new(Uuid::new_v4()),
            paddock_id: None,

            // initialize dates with nothing
            last_seen: None,
            egg_laid: None,
            hatched: None,
            released: None,
            died: None,
            last_veterinary_checkup: None,
            record_created: Local::now(),
            is_alive: true,
            is_contained: true,
            is_in_transit: false,
            is_in_paddock: false,
            needs_feeding: false,
            needs_status_check: false,
            needs_veterinary_care: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn we_can_create_an_asset() {
        let asset = Asset::new(Species::BRCH, String::from("BigBrachy"));
        assert_eq!(asset.name, "BigBrachy");
        assert!(matches!(asset.species, Species::BRCH));
    }
}