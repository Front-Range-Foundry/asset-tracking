

pub struct Vitals {
}

impl Vitals {
	pub fn new() -> Vitals {
		Vitals {
		}
	}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vitals_can_be_created() {
		let vitals = Vitals {};
		assert_eq!(vitals, Vitals {});
	}
}