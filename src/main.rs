extern crate rand;

use rand::Rng;

static FOOD_PLACES: [&'static str; 8] = [
    "Pumpkin Brown",
    "Let us eat",
    "Pizza Express",
    "Redbox",
    "Generic Burrito place on Lothian Rd",
    "Taquito",
    "Wagamama",
    "Söderberg"
];

fn main() {
    let mut rng = rand::thread_rng();
    let choice = rng.choose(&FOOD_PLACES);
    println!("Choice is {}", choice.unwrap());
}
