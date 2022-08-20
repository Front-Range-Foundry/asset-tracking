use uuid::Uuid;

use crate::asset::Species;

pub enum FoodType {
  MEAT,
  GOAT,
  CYCAD,
  BERRY,
  TUBER,
  FISH,
  FRUIT,
  NUT,
  HAY
}

pub struct DietTracker {
  pub asset_id: Uuid,
  pub food_types: Vec<FoodType>
}

impl DietTracker {
  pub fn new(asset_id: Uuid, species: Species) -> DietTracker {
    let diet_components = get_diet_components(species);
    DietTracker {
      asset_id,
      food_types: diet_components,
    }
  }
}

fn get_diet_components(species: Species) -> Vec<FoodType> {
  match species {
    Species::BRCH => {
      vec![FoodType::CYCAD, FoodType::FRUIT, FoodType::NUT]
    },
    Species::GALL => {
      vec![FoodType::BERRY, FoodType::HAY, FoodType::NUT, FoodType::FRUIT]
    },
    Species::BRNX => {
      vec![FoodType::FISH]
    },
    Species::COEL => {
      vec![FoodType::MEAT, FoodType::GOAT, FoodType::FISH]
    },
    Species::DILO => {
      vec![FoodType::MEAT, FoodType::GOAT, FoodType::FISH]
    },
    Species::HERR => {
      vec![FoodType::MEAT, FoodType::GOAT, FoodType::FISH]
    },
    Species::MTCN => {
      vec![FoodType::MEAT, FoodType::GOAT]
    },
    Species::PARA => {
      vec![FoodType::CYCAD, FoodType::HAY, FoodType::FRUIT]
    },
    Species::PROC => {
      vec![FoodType::MEAT, FoodType::GOAT, FoodType::FISH]
    },
    Species::TREX => {
      vec![FoodType::GOAT]
    },
    Species::TRIK => {
      vec![FoodType::HAY, FoodType::FRUIT, FoodType::NUT, FoodType::CYCAD]
    },
    Species::VELO => {
      vec![FoodType::MEAT, FoodType::GOAT]
    }
  }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn we_can_create_diet_tracker() {
      let asset_id = Uuid::new_v4();
        let diet_tracker = DietTracker::new(asset_id, Species::BRCH);
        assert_eq!(diet_tracker.asset_id, asset_id);
    }
  
    #[test]
    fn it_has_the_right_number_of_food_types() {
      let asset_id = Uuid::new_v4();
      let diet_tracker = DietTracker::new(asset_id, Species::BRCH);
      assert_eq!(diet_tracker.food_types.len(), 3);
    }

    #[test]
    fn it_generates_the_correct_diet() {
      let asset_id = Uuid::new_v4();
      let diet_tracker = DietTracker::new(asset_id, Species::BRCH);
      assert!(matches!(diet_tracker.food_types[0], FoodType::CYCAD));
      assert!(matches!(diet_tracker.food_types[1], FoodType::FRUIT));
      assert!(matches!(diet_tracker.food_types[2], FoodType::NUT));
    }
}