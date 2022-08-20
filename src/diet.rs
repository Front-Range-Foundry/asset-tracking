use uuid::Uuid;

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
  pub fn new(asset_id: Uuid) -> DietTracker {
    DietTracker {
      asset_id,
      food_types: Vec::new()
    }
  }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn we_can_create_diet_tracker() {
      let asset_id = Uuid::new_v4();
        let diet_tracker = DietTracker::new(asset_id);
        assert_eq!(diet_tracker.asset_id, asset_id);
    }
  
    #[test]
    fn it_has_no_food_types() {
      let asset_id = Uuid::new_v4();
      let diet_tracker = DietTracker::new(asset_id);
      assert_eq!(diet_tracker.food_types.len(), 0);
    }
}