#[derive(PartialEq, Debug)]
pub struct UnitQuantity(u32);

impl UnitQuantity {
    pub fn create(qty: u32) -> Result<UnitQuantity, String> {
        match qty {
            0 => Err(String::from("`UnitQuantity` cannot be less than 1")),
            1...1000 => Ok(UnitQuantity(qty)),
            _ => Err(String::from("`UnitQuantity` cannot be greater than 1000")),
        }
    }

    pub fn value(&self) -> u32 {
        self.0
    }

    pub fn minimum() -> UnitQuantity {
        UnitQuantity(1)
    }
}
