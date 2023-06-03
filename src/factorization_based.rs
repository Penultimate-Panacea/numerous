use prime_factorization::Factorization;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
pub const TEST_POWERFUL:[i64; 53] = [1, 4, 8, 9, 16, 25, 27, 32, 36, 49, 64, 72, 81, 100, 108, 121, 125, 128, 144, 169, 196, 200, 216, 225, 243, 256, 288, 289, 324, 343, 361, 392, 400, 432, 441, 484, 500, 512, 529, 576, 625, 648, 675, 676, 729, 784, 800, 841, 864, 900, 961, 968, i64::MAX];

///
/// A powerful number is a positive integer m such that for every prime number p dividing m, p2 also divides m
/// 
/// 
pub fn is_powerful(testee: i64) -> bool {
    if testee < 0 {return false;}
    let unsigned_testee:u64 = testee.unsigned_abs();
    let factor_factory: Factorization<u64> = Factorization::run(unsigned_testee);
    let prime_factors:Vec<(u64, u32)> = factor_factory.prime_factor_repr();
    let mut num_of_squareful_factors = 0;
    for prime in &prime_factors{
        let factor = prime.0;
        if factor.pow(2) % unsigned_testee == 0 {
            num_of_squareful_factors += 1;
        }
    }
    if prime_factors.len() == num_of_squareful_factors {
        return true;
    }
    for prime in &prime_factors{
        let power = prime.1;
        if power == 3 {
            return true;
        }
    }
    false
}

///
/// 
/// A001694
/// 
/// 
pub fn is_powerful_candiate_function(testee: i64) -> bool {
    if testee < 0 {return false;}
    let unsigned_testee:u64 = testee.unsigned_abs();
    let factor_factory: Factorization<u64> = Factorization::run(unsigned_testee);
    let prime_factors:Vec<(u64, u32)> = factor_factory.prime_factor_repr();
    let mut b;
    let mut odd_factors = vec![];
    prime_factors
        .into_par_iter()
        .for_each(|prime:(u64,u32)|{
            if prime.1 % 2 == 1 {odd_factors.push(prime.0);}
        });
    if odd_factors.is_empty() {
        b = 1;
    }
    else {
        b = odd_factors[0];
        odd_factors.remove(0);
        for odd in odd_factors {
            b *= odd;
        }
    };
    let radicand_of_a = unsigned_testee / b.pow(3);
    let test = radicand_of_a * b.pow(3);
    if unsigned_testee == test {return true;}
    false
}
