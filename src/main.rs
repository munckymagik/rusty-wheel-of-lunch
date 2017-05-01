extern crate rand;

use rand::Rng;

static FOOD_PLACES: [&str; 14] = [
    "Bar Burrito",
    "Bento Ya",
    "Let us eat",
    "Maki and Ramen",
    "Mama's",
    "Pizza Express",
    "Pumpkin Brown",
    "Redbox",
    "Söderberg",
    "Taquito",
    "The Red Squirrel",
    "Ting Thai Caravan",
    "Wagamama",
    "Wildman Wood",
];

fn main() {
    let mut rng = rand::thread_rng();
    let choice = rng.choose(&FOOD_PLACES);
    println!("Choice is {}", choice.unwrap());
}
