use uuid::Uuid;
use chrono::{offset::{Local}, DateTime};
use std::collections::HashMap;

struct Event {
    name: String,
    recorded_by: Uuid,
    recorded_on: DateTime<Local>,
}

pub struct LifecycleBuilder {
    pub asset_id: Uuid,
    hatch_date: Option<DateTime<Local>>,
    park_introduction_date: Option<DateTime<Local>>,
    last_checkup_date: Option<DateTime<Local>>,
    death_date: Option<DateTime<Local>>,
    other_dates: Vec<Event>,
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
    pub fn add_other_event(&mut self, name: String, recorded_by: Uuid, recorded_on: DateTime<Local>) -> () {
        self.other_dates.push(Event {
            name,
            recorded_by,
            recorded_on,
        });
    }
    pub fn get_all_dates(&self) -> HashMap<String, DateTime<Local>> { 
        let mut dates = HashMap::new();
        if let Some(date) = self.hatch_date {
            dates.insert("hatch_date".to_string(), date);
        }
        if let Some(date) = self.park_introduction_date {
            dates.insert("park_introduction_date".to_string(), date);
        }
        if let Some(date) = self.last_checkup_date {
            dates.insert("last_checkup_date".to_string(), date);
        }
        if let Some(date) = self.death_date {
            dates.insert("death_date".to_string(), date);
        }
        for event in &self.other_dates {
            dates.insert(event.name.clone(), event.recorded_on);
        }
        dates
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
            other_dates: Vec::new(),
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

    #[test]
    fn we_can_set_additional_dates_in_the_lifecycle() {
        let mut lifecycle_builder = LifecycleBuilder {
            ..Default::default()
        };
        let id = Uuid::new_v4();
        let now = Local::now();
        lifecycle_builder.add_other_event(String::from("Brach Gave Birth"), id, now);
        assert_eq!(lifecycle_builder.other_dates.len(), 1);
        assert_eq!(lifecycle_builder.other_dates[0].name, "Brach Gave Birth");
        assert_eq!(lifecycle_builder.other_dates[0].recorded_by, id);
        assert_eq!(lifecycle_builder.other_dates[0].recorded_on, now);
    }

    #[test] 
    fn we_can_retrieve_all_dates() {
        let mut lifecycle_builder = LifecycleBuilder {
            ..Default::default()
        };
        lifecycle_builder.set_hatch_date(Local::now());
        lifecycle_builder.set_park_introduction_date(Local::now());
        lifecycle_builder.set_last_checkup_date(Local::now());
        lifecycle_builder.set_death_date(Local::now());
        lifecycle_builder.add_other_event(String::from("Brach Gave Birth"), Uuid::new_v4(), Local::now());
        
        let dates = lifecycle_builder.get_all_dates();
        assert_eq!(dates.len(), 5);
    }
}