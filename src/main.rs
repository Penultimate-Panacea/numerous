#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![warn(clippy::cargo)]

mod factorization;


fn main() {
    for i in factorization::TEST_PRIMES{
        let a = factorization::is_prime_extended(i);
        println!("{a}");
    };
}
