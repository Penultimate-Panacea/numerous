#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![warn(clippy::cargo)]

mod factorization_based;


fn main() {
    for i in factorization_based::TEST_PRIMES{
        let a = factorization_based::is_prime_extended(i);
        println!("{a}");
    };
}
