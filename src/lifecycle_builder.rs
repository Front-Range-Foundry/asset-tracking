use uuid::Uuid;
use chrono::{offset::{Local}, DateTime};

pub struct LifecycleBuilder {
    pub asset_id: Uuid,
    hatch_date: Option<DateTime<Local>>,
    park_introduction_date: Option<DateTime<Local>>,
    last_checkup_date: Option<DateTime<Local>>,
    death_date: Option<DateTime<Local>>,
}

impl LifecycleBuilder { 
    pub fn new(asset_id: Uuid) -> LifecycleBuilder {
        LifecycleBuilder {
            asset_id,
            ..Default::default()
        }
    }
    pub fn set_hatch_date(&mut self, date: DateTime<Local>) -> () {
        self.hatch_date = Some(date);
    }
    pub fn set_park_introduction_date(&mut self, date: DateTime<Local>) -> () {
        self.park_introduction_date = Some(date);
    }
    pub fn set_last_checkup_date(&mut self, date: DateTime<Local>) -> () {
        self.last_checkup_date = Some(date);
    }
    pub fn set_death_date(&mut self, date: DateTime<Local>) -> () {
        self.death_date = Some(date);
    }
}


impl Default for LifecycleBuilder {
    fn default() -> Self {
        LifecycleBuilder {
            asset_id: Uuid::new_v4(),
            hatch_date: None,
            park_introduction_date: None,
            last_checkup_date: None,
            death_date: None,
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn we_can_create_a_lifecycle_builder() {
        let uuid = Uuid::new_v4();
        let lifecycle_builder = LifecycleBuilder {
            asset_id: uuid,
            ..Default::default()
        };

        assert_eq!(lifecycle_builder.asset_id, uuid);
    }

    #[test]
    fn we_can_set_a_hatch_date() {
        let mut lifecycle_builder = LifecycleBuilder {
            ..Default::default()
        };
        lifecycle_builder.set_hatch_date(Local::now());
        assert!(lifecycle_builder.hatch_date.is_some());
    }

    #[test] 
    fn we_can_set_a_park_introduction_date() {
        let mut lifecycle_builder = LifecycleBuilder {
            ..Default::default()
        };
        lifecycle_builder.set_park_introduction_date(Local::now());
        assert!(lifecycle_builder.park_introduction_date.is_some());
    }

    #[test] 
    fn we_can_set_a_last_checkup_date() {
        let mut lifecycle_builder = LifecycleBuilder {
            ..Default::default()
        };
        lifecycle_builder.set_last_checkup_date(Local::now());
        assert!(lifecycle_builder.last_checkup_date.is_some());
    }

    #[test] 
    fn we_can_set_a_death_date() {
        let mut lifecycle_builder = LifecycleBuilder {
            ..Default::default()
        };
        lifecycle_builder.set_death_date(Local::now());
        assert!(lifecycle_builder.death_date.is_some());
    }
}