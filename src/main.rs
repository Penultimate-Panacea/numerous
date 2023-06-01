#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![warn(clippy::cargo)]

mod factorization_based;


fn main() {
    let mut j = 1;
    for i in factorization_based::TEST_POWERFUL{
        let a = factorization_based::is_powerful(i);
        println!("{a}");
    };
    while j < 1000 {
        let b = factorization_based::is_powerful(j);
        println!("Is {j} powerful?:  {b}");
        j += 1;
    }
}
