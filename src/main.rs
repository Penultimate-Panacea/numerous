#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![warn(clippy::cargo)]

use rayon::prelude::*;

mod factorization_based;


fn main() {
    factorization_based::TEST_POWERFUL.into_par_iter().for_each(|i| {
        println!("Is {i} powerful?: {}", factorization_based::is_powerful(i));
    });
    println!("");
    println!("");
    println!("ACHILLES");
    println!("");
    println!("");
    factorization_based::TEST_ACHILLES.into_par_iter().for_each(|i| {
        println!("Is {i} achilles?: {}", factorization_based::is_achilles(i));
    });
    println!("");
    println!("");
    println!("PERFECT");
    println!("");
    println!("");
    factorization_based::TEST_PERFECT.into_par_iter().for_each(|i| {
        println!("Is {i} powerful?: {}", factorization_based::is_perfect_power(i));
    });
}
