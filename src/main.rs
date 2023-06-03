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
    println!("CANDIDATE FUNCTION");
    println!("");
    println!("");
    factorization_based::TEST_POWERFUL.into_par_iter().for_each(|i| {
        assert_eq!(true, factorization_based::is_powerful(i));
    });
}
