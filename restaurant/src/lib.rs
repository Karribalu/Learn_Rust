mod front_of_house;
pub use crate::front_of_house::hosting;
fn deliver_order() {}
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }
    fn cook_order() {}
}
pub fn eat_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("dried");
    meal.toast = String::from("Wheat");
    println!("I like {}", meal.toast);
    // meal.seasonal_fruit = String::from("blueberries");
}
