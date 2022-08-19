use uuid::Uuid;

pub struct Vitals {
	pub id: Uuid,
}

impl Vitals {
	pub fn new(id: Uuid) -> Vitals {
		Vitals {
			id,
		}
	}
}

#[cfg(test)]
mod tests {
    use super::*;

		#[test]
		fn we_can_create_vitals() {
			let uuid = Uuid::new_v4();
			let vitals = Vitals::new(uuid);
			assert_eq!(vitals.id, uuid);
		}
}