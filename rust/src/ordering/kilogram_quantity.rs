#[derive(PartialEq, Debug)]
pub struct KilogramQuantity(f32);

impl KilogramQuantity {
    pub fn create(qty: f32) -> Result<KilogramQuantity, String> {
        if qty < 0.05 {
            Err(String::from("`KilogramQuantity` cannot be less than 0.05"))
        } else if qty > 1000.0 {
            Err(String::from(
                "`KilogramQuantity` cannot be more than 1000.00",
            ))
        } else {
            Ok(KilogramQuantity(qty))
        }
    }

    pub fn value(&self) -> f32 {
        self.0
    }

    pub fn minimum() -> KilogramQuantity {
        KilogramQuantity(0.05)
    }
}
