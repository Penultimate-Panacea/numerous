#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![warn(clippy::cargo)]

mod factorization_based;


fn main() {
    println!("{}", factorization_based::is_powerful(1)); // First Powerful
    println!("{}", factorization_based::is_powerful(4)); // Second Powerful 2^2
    println!("{}", factorization_based::is_powerful(8)); // Third Powerful 2^3
    println!("{}", factorization_based::is_powerful(343)); // 7^3
    println!("{}", factorization_based::is_powerful(441)); // 7^2*3^2
}
