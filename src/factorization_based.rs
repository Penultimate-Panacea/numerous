use prime_factorization::Factorization;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};


pub const TEST_POWERFUL:[i32; 30] = [1, 4, 8, 9, 16, 25, 27, 32, 36, 49,
                                 64, 72, 81, 100, 108, 121, 125, 128, 144, 169,
                                 196, 200, 216, 225, 243, 256, 288, 289, 324, 343];

pub const TEST_ACHILLES:[i32; 30] = [72, 108, 200, 288, 392, 500, 648, 675, 800, 968, 972, 1125, 1323, 1352, 1568, 1800, 1944, 2000, 2700, 2888, 3087, 3200, 3267, 3528, 4000, 4232, 4500, 4563, 4608, 5000];

///
/// A powerful number is a positive integer m such that for every prime number p dividing m, p2 also divides m
/// A001694
/// Informally, given the prime factorization of m, take b to be the product of the prime factors of m that have an odd exponent
/// (if there are none, then take b to be 1). Because m is powerful, each prime factor with an odd exponent has an exponent that 
/// is at least 3, so m/b^3 is an integer. In addition, each prime factor of m/b^3 has an even exponent, so m/b^3 is a perfect square, 
/// so call this a^2; then m = a^2b^3
/// 
///  # Arguements
/// 
///  * `testee` - An i32 number to be tested 
/// 
///  # Example
///  ```
/// let powerful_num: i32 = 200;
/// let not_powerful_num: i32 = 19;
/// assert_eq!(true, is_powerful(powerful_num);
/// assert_eq!(false, is_powerful())
/// ```    
/// 
///  # Todo
///  Upper limit catch, currently panics as i32:MAX integers
/// 
pub fn is_powerful(testee: i32) -> bool {
    if testee < 0 {return false;}
    let unsigned_testee:u64 = testee.unsigned_abs() as u64;
    let factor_factory: Factorization<u64> = Factorization::run(unsigned_testee);
    let prime_factors:Vec<(u64, u32)> = factor_factory.prime_factor_repr();
    let mut b;
    let mut odd_factors = vec![];
    // prime_factors
    //     .into_par_iter()
    //     .for_each(|prime:(u64,u32)|{
    //         if prime.1 % 2 == 1 {odd_factors.push(prime.0);}
    //     });
    for prime in prime_factors{
        if prime.1 % 2 == 1 {odd_factors.push(prime.0);}
    }
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
    let radicand_of_a: u64 = unsigned_testee / b.pow(3);
    let test: u64 = radicand_of_a * b.pow(3);
    if unsigned_testee == test {return true;}
    false
}

#[test]
fn test_is_powerful(){
    rayon::prelude::IntoParallelIterator::into_par_iter(TEST_POWERFUL).for_each(|i| {
        assert_eq!(true, is_powerful(i));
    });
    assert_eq!(false, is_powerful(19));
    assert_eq!(false, is_powerful(-20));
    assert_eq!(false, is_powerful(-25));
}

pub fn is_achilles(testee: i32) -> bool {
    if is_powerful(testee){
        let unsigned_testee:u64 = testee.unsigned_abs() as u64;
        let factor_factory: Factorization<u64> = Factorization::run(unsigned_testee);
        let prime_factors:Vec<(u64, u32)> = factor_factory.prime_factor_repr();
        for factor in prime_factors{
            let factor_squared = factor.0.pow(2);
            if unsigned_testee % factor_squared != 0 {return false;}
        }
        return  true;
    };
    false
}

#[test]
fn test_is_achilles(){
    rayon::prelude::IntoParallelIterator::into_par_iter(TEST_ACHILLES).for_each(|i| {
        assert_eq!(true, is_achilles(i));
    });
    assert_eq!(false, is_achilles(360));
    assert_eq!(false, is_achilles(784));
    assert_eq!(false, is_achilles(-72));
}