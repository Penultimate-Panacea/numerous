use prime_factorization::Factorization;


pub const TEST_POWERFUL:[i32; 52] = [1, 4, 8, 9, 16, 25, 27, 32, 36, 49, 64, 72, 81,
                                    100, 108, 121, 125, 128, 144, 169, 196, 200, 216, 
                                    225, 243, 256, 288, 289, 324, 343, 361, 392, 400, 
                                    432, 441, 484, 500, 512, 529, 576, 625, 648, 675, 
                                    676, 729, 784, 800, 841, 864, 900, 961, 968];

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
/// TEST_POWERFUL.into_par_iter().for_each(|i| {
/// assert_eq!(true, is_powerful(i));};
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
    rayon::prelude::ParallelIterator::for_each(rayon::prelude::IntoParallelIterator::into_par_iter(TEST_POWERFUL), |i| {
        assert_eq!(true, is_powerful(i));
    });
}
