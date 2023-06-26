#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![warn(clippy::cargo)]

use rayon::prelude::*;

mod factorization_based;


fn main() {
    factorization_based::TEST_POWERFUL.into_par_iter().for_each(|i| {
        println!("Is {i} powerful?: {}", factorization_based::is_powerful(i));
    });
    println!("ACHILLES");
    factorization_based::TEST_ACHILLES.into_par_iter().for_each(|i| {
        println!("Is {i} achilles?: {}", factorization_based::is_achilles(i));
    });
    println!("PERFECT");
    factorization_based::TEST_PERFECT.into_par_iter().for_each(|i| {
        println!("Is {i} perfect powerful?: {}", factorization_based::is_perfect_power(i));
    });
    println!("SEMIPRIME");
    factorization_based::TEST_SEMIPRIME.into_par_iter().for_each(|i| {
        println!("Is {i} semiprime?: {}", factorization_based::is_semiprime(i));
    });
    println!("SPHENIC");
    factorization_based::TEST_SPHENIC.into_par_iter().for_each(|i| {
        println!("Is {i} sphenic?: {}", factorization_based::is_sphenic(i));
    });
    println!("SQUAREFREE");
    factorization_based::TEST_SQUAREFREE.into_par_iter().for_each(|i| {
        println!("Is {i} sphenic?: {}", factorization_based::is_squarefree(i));
    });
    println!("SQUAREFREE");
    factorization_based::TEST_PRONIC.into_par_iter().for_each(|i| {
        println!("Is {i} sphenic?: {}", factorization_based::is_pronic(i));
    });
}
