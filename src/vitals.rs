
pub mod vitals {
	pub struct Vitals {
	}

	impl Vitals {
		pub fn new() -> Vitals {
			Vitals {
			}
		}
	}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vitals_can_be_created() {
		let vitals = vitals::Vitals {};
		assert_eq!(vitals, vitals::Vitals {});
	}
}