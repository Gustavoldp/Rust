use crate::back_of_house::Appetizer;
use std::collections::HashMap;
use std::fmt;
use std::io as IoResult;
//brings the hosting module to the root scope
//the pub keyword is used for re-exporting
//we are using it to bring the item to the scope and making
//it available for others to bring it too.
//pub use crate::front_of_house::hosting;
//use self::front_of_house::hosting;


mod front_of_house;
pub use crate::front_of_house::hosting;

mod back_of_house{
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast{
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
    //when we make a enum public, all its variants are then public
    pub enum Appetizer{
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant(){
    //Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    //Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    //The next line won't compile if we uncomment it; we're not alowed
    //to see or modify the seasonal fruit that comes with the meal
    //meal.seasonal_fruit = String::from("blueberries");

    //we can then use the enums
    let order1 = back_of_house::Appetizer::Soup;
    let order2: Appetizer = back_of_house::Appetizer::Salad;

    //the hosting module is on the scope now.
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();

    //abosolute path
    crate::front_of_house::hosting::add_to_waitlist();

    //relative path
    front_of_house::hosting::add_to_waitlist();
}

//this is how to declare two results that have a different parent module
/*
fn function1() -> fmt::Result{
 //--snip--
}
//using io::Result will have the same effect
fn function2() -> IoResult{
    //--snip
}
 */


fn main(){
    let mut map = HashMap::new();
    map.insert(1, 2);
}