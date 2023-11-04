mod front_of_house;

mod back_of_house;

pub use crate::back_of_house::*;
pub use crate::front_of_house::hosting;
pub use crate::front_of_house::serving;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    serving::take_order();
    let mut meal = Breakfast::summer("Rye");
    let appetizer = Appetizer::Soup;
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    println!("Today's seasonal fruit is {}", meal.get_seasonal_fruit());
    match appetizer {
        Appetizer::Soup => println!("Appetizer is Soup!"),
        Appetizer::Salad => println!("Appetizer is Salad!"),
    }
    // uncomment for compiler error, seasonal_fruit is private.
    // meal.seasonal_fruit = String::from("blueberries");
}
