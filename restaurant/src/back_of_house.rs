pub struct Breakfast {
    pub toast: String,
    seasonal_fruit: String,
}

pub enum Appetizer {
    Soup,
    Salad,
}

impl Breakfast {
    pub fn summer(toast: &str) -> Self {
        Self {
            toast: String::from(toast),
            seasonal_fruit: String::from("peaches"),
        }
    }
    pub fn get_seasonal_fruit(&self) -> &str {
        &self.seasonal_fruit
    }
}
