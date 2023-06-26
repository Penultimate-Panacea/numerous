#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![warn(clippy::cargo)]


mod factorization_based;


fn main() {
    for i in &factorization_based::TEST_POWERFUL{
        println!("Is {i} powerful?: {}", factorization_based::is_powerful(*i));
    };
    println!("ACHILLES");
    for i in &factorization_based::TEST_ACHILLES{
        println!("Is {i} achilles?: {}", factorization_based::is_achilles(*i));
    };
    println!("PERFECT");
    for i in &factorization_based::TEST_PERFECT{
        println!("Is {i} perfect powerful?: {}", factorization_based::is_perfect_power(*i));
    };
    println!("SEMIPRIME");
    for i in &factorization_based::TEST_SEMIPRIME {
        println!("Is {i} semiprime?: {}", factorization_based::is_semiprime(*i));
    };
    println!("SPHENIC");
    for i in &factorization_based::TEST_SPHENIC {
        println!("Is {i} sphenic?: {}", factorization_based::is_sphenic(*i));
    };
    println!("SQUAREFREE");
    for i in &factorization_based::TEST_SQUAREFREE{
        println!("Is {i} squarefree?: {}", factorization_based::is_squarefree(*i));
    };
    println!("PRONIC");
    for i in &factorization_based::TEST_PRONIC {
        println!("Is {i} pronic?: {}", factorization_based::is_pronic(*i));
    };
    println!("PRONIC");
    for i in &factorization_based::TEST_PRONIC{
        println!("Is {i} unusual?: {}", factorization_based::is_unusual(*i));
    };
    println!("PRONIC");
    for i in &factorization_based::TEST_PRONIC{
        println!("Is {i} 5-smooth?: {}", factorization_based::is_smooth(*i,5));
    };
    println!("PRONIC");
    for i in &factorization_based::TEST_PRONIC{
        println!("Is {i} rough-3?: {}", factorization_based::is_rough(*i,3));
    };
}
