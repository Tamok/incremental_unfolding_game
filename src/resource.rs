/*
This file will contain the Resources struct, which will hold the game's resources,
like energy and currency.
*/
pub struct Resources {
    pub name: String,
    pub amount: u64,
    pub rate: u64,
}

impl Resources {
    pub fn new(name: &str, initial_amount: u64, initial_rate: u64) -> Self {
        Self {
            name: String::from(name),
            amount: initial_amount,
            rate: initial_rate,
        }
    }

    pub fn generate(&mut self) {
        self.amount += self.rate;
    }
}