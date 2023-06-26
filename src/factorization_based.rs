use gcd::Gcd;
use prime_factorization::Factorization;
use integer_sqrt::IntegerSquareRoot;

pub const TEST_POWERFUL:[i32; 30] = [1, 4, 8, 9, 16, 25, 27, 32, 36, 49,
                                     64, 72, 81, 100, 108, 121, 125, 128, 144, 169,
                                     196, 200, 216, 225, 243, 256, 288, 289, 324, 343];
pub const TEST_ACHILLES:[i32; 30] = [72, 108, 200, 288, 392, 500, 648, 675, 800, 968,
                                    972, 1125, 1323, 1352, 1568, 1800, 1944, 2000, 2700, 2888,
                                     3087, 3200, 3267, 3528, 4000, 4232, 4500, 4563, 4608, 5000];
pub const TEST_PERFECT:[i32; 30] = [4, 8, 9, 16, 25, 27, 32, 36, 49, 64,
                                    81, 100, 121, 125, 128, 144, 169, 196, 216, 225, 
                                    243, 256, 289, 324, 343, 361, 400, 441, 484, 512];
pub const TEST_SEMIPRIME:[i32; 30] = [4, 6, 9, 10, 14, 15, 21, 22, 25, 26,
                                      33, 34, 35, 38, 39, 46, 49, 51, 55, 57,
                                      58, 62, 65, 69, 74, 77, 82, 85, 86, 87];
pub const TEST_SPHENIC: [i32; 30] = [30, 42, 66, 70, 78, 102, 105, 110, 114, 130,
                                    138, 154, 165, 170, 174, 182, 186, 190, 195, 222,
                                    230, 231, 238, 246, 255, 258, 266, 273, 282, 285];
pub const TEST_SQUAREFREE: [i32; 30] = [1, 2, 3, 5, 6, 7, 10, 11, 13, 14, 
                                        15, 17, 19, 21, 22, 23, 26, 29, 30, 31, 
                                        33, 34, 35, 37, 38, 39, 41, 42, 43, 46];
pub const TEST_PRONIC: [i32; 30] = [2, 6, 12, 20, 30, 42, 56, 72, 90,
                                    110, 132, 156, 182, 210, 240, 272, 306, 342, 380,
                                    420, 462, 506, 552, 600, 650, 702, 756, 812, 870, 930];
///
/// A powerful number is a positive integer m such that for every prime number p dividing m, p2 also divides m. 
/// Behavior described in OEIS A001694. s
/// 
/// For this test we take the informal definition of a powerful number as described on wikipedia: 
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
///  ``` rust
/// let powerful_num: i32 = 200;
/// let not_powerful_num: i32 = 19;
/// assert!(is_powerful(powerful_num);
/// assert!(!is_powerful(not_powerful_num));
/// ```    
/// 
///  # Todo
///  Upper limit catch, currently panics as i32:MAX integers
/// 
pub fn is_powerful(testee: i32) -> bool {
    if testee < 0 {return false;}
    let unsigned_testee:u64 = u64::from(testee.unsigned_abs());
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
    let test: u64 = radicand_of_a * b.pow(3); // IS THIS USELESS CODE?
    if unsigned_testee == test {return true;}
    false
}

#[test]
fn test_is_powerful(){
    rayon::prelude::ParallelIterator::for_each(rayon::prelude::IntoParallelIterator::into_par_iter(TEST_POWERFUL), |i| {
        assert!(is_powerful(i));
    });
    assert!(!is_powerful(19));
    assert!(!is_powerful(-20));
    assert!(!is_powerful(-25));
}

///
/// An achilles number is a powerful number which is not a perfect power. 
/// Behavior described in OEIS A052486. 
/// 
/// From Wikipedia:  positive integer n is a powerful number if, for every prime factor p of n, p2 is also a divisor. In other words, every prime factor appears at least squared in the factorization. All Achilles numbers are powerful. However, not all powerful numbers are Achilles numbers: only those that cannot be represented as mk, where m and k are positive integers greater than 1.
/// 
///  # Arguements
/// 
///  * `testee` - An i32 number to be tested 
/// 
///  # Example
///  ``` rust
/// let achilles_num: i32 = 500;
/// let not_achilles_num: i32 = 784;
/// assert!(is_achilles(achilles_num);
/// assert!(!is_achilles(not_achilles_num));
/// ```    
/// 
///  # Todo
///  Upper limit catch, currently panics as i32:MAX integers
/// 
pub fn is_achilles(testee: i32) -> bool {
    if is_perfect_power(testee) {return false;}
    if is_powerful(testee){
        if is_perfect_power(testee) {return false;}
        let unsigned_testee:u64 = u64::from(testee.unsigned_abs());
        let factor_factory: Factorization<u64> = Factorization::run(unsigned_testee);
        let prime_factors:Vec<(u64, u32)> = factor_factory.prime_factor_repr();
        for factor in prime_factors{
            let factor_squared = factor.0.pow(2);
            if unsigned_testee % factor_squared != 0 {return false;} // if any factor squared does not divide testee, f
        }
        return  true;
    };
    false
}

#[test]
fn test_is_achilles(){
    rayon::prelude::ParallelIterator::for_each(rayon::prelude::IntoParallelIterator::into_par_iter(TEST_ACHILLES), |i| {
        assert!(is_achilles(i));
    });
    assert!(!is_achilles(360));
    assert!(!is_achilles(784));
    assert!(!is_achilles(-72));
}

///
/// An pefect power is a is a natural number that can be described as a n-th power of a single natural number
/// Behavior described in OEIS A052486. 
/// 
/// From Wikipedia: n is a perfect power if there exist natural numbers m > 1, and k > 1 such that mk = n. In this case, n may be called a perfect kth power. If k = 2 or k = 3, then n is called a perfect square or perfect cube, respectively. Sometimes 0 and 1 are also considered perfect powers (0k = 0 for any k > 0, 1k = 1 for any k). 
/// 
///  # Arguements
/// 
///  * `testee` - An i32 number to be tested 
/// 
///  # Example
///  ``` rust
/// let perfect: i32 = 500;
/// let not_perfect: i32 = 500;
/// assert!(is_perfect_power(perfect));
/// assert!(!is_perfect_power(not_perfect));
/// ```    
/// 
///  # Todo
///  Upper limit catch, currently panics as i32:MAX integers
/// 
pub fn is_perfect_power(testee: i32) -> bool {
    if !is_powerful(testee) {return false;};
    let unsigned_testee: u64 = u64::from(testee.unsigned_abs());
    let factor_factory: Factorization<u64> = Factorization::run(unsigned_testee);
    let prime_factors:Vec<(u64, u32)> = factor_factory.prime_factor_repr();
    let mut gcd_of_prime_powers = prime_factors[0].1;
    for prime_power in &prime_factors[1..] {
        gcd_of_prime_powers = gcd_of_prime_powers.gcd(prime_power.1);
    }
    if gcd_of_prime_powers > 1 {
        return true;
    }
false
}

#[test]
fn test_is_perfect_power(){
    rayon::prelude::ParallelIterator::for_each(rayon::prelude::IntoParallelIterator::into_par_iter(TEST_PERFECT), |i| {
        assert!(is_perfect_power(i));
    });
    assert!(!is_perfect_power(360));
    assert!(!is_perfect_power(785));
    assert!(!is_perfect_power(-72));    

}

///
/// A semiprime is the product of exactly two primes
/// Behavior described in OEIS A001358. 
/// 
///  # Arguements
/// 
///  * `testee` - An i32 number to be tested 
/// 
///  # Example
///  ``` rust
/// let semiprime: i32 = 9;
/// let not_semiprime: i32 = 500;
/// assert!(is_semiprime(semiprime));
/// assert!(!is_semiprime(not_semiprime));
/// ```    
/// 
///  # Todo
///  Upper limit catch, currently panics as i32:MAX integers
/// 
pub fn is_semiprime(testee: i32) -> bool{
    if testee < 4 {return false;}
    let unsigned_testee:u64 = u64::from(testee.unsigned_abs());
    let factor_factory: Factorization<u64> = Factorization::run(unsigned_testee);
    let prime_factors:Vec<(u64, u32)> = factor_factory.prime_factor_repr();
    if prime_factors.len() == 2{
        return true;
    }
    if prime_factors.len() == 1 && prime_factors[0].1 == 2 {
        return true;
    }    
false
}

#[test]
fn test_is_semiprime(){
    rayon::prelude::ParallelIterator::for_each(rayon::prelude::IntoParallelIterator::into_par_iter(TEST_SEMIPRIME), |i| {
    assert!(is_semiprime(i));
    assert!(!is_semiprime(19));
assert!(!is_semiprime(786));
assert!(!is_semiprime(-72));   
});
}


///
/// A sphenic integer is the product of three distince primes
/// Behavior described in OEIS A007304. 
/// 
///  # Arguements
/// 
///  * `testee` - An i32 number to be tested 
/// 
///  # Example
///  ``` rust
/// let sphenic: i32 = 30;
/// let not_sphenic: i32 = 24;
/// assert!(is_sphenic(sphenic));
/// assert!(!is_shphenic(not_sphenic));
/// ```    
/// 
///  # Todo
///  Upper limit catch, currently panics as i32:MAX integers
pub fn is_sphenic(testee: i32) -> bool {
    if testee < 4 {return false;}
    let unsigned_testee:u64 = u64::from(testee.unsigned_abs());
    let factor_factory: Factorization<u64> = Factorization::run(unsigned_testee);
    let prime_factors:Vec<(u64, u32)> = factor_factory.prime_factor_repr();
    if prime_factors.len() == 3{
        return true;
    } 
false   
}

#[test]
fn test_is_sphenic(){
    rayon::prelude::ParallelIterator::for_each(rayon::prelude::IntoParallelIterator::into_par_iter(TEST_SPHENIC), |i| {
    assert!(is_sphenic(i));
    assert!(!is_sphenic(19));
    assert!(is_sphenic(786));
    assert!(!is_sphenic(788));
    assert!(!is_sphenic(72));   
});}
//
/// A sqaurefree integer is the product of unique prime factors, no prime factor has a power greater than one 
/// Behavior described in OEIS A005117. 
/// 
///  # Arguements
/// 
///  * `testee` - An i32 number to be tested 
/// 
///  # Example
///  ``` rust
/// let squarefree: i32 = 10;
/// let not_squarefree: i32 = 18;
/// assert!(is_squarefree(squarefree));
/// assert!(!is_squarefree(not_squarefree));
/// ```    
/// 
///  # Todo
///  Upper limit catch, currently panics as i32:MAX integers
/// 
///  I do not like how the tail for this function returns true, while all other test functions' tails return false.
/// 
pub fn is_squarefree(testee: i32) -> bool {
    let unsigned_testee:u64 = u64::from(testee.unsigned_abs());
    let factor_factory: Factorization<u64> = Factorization::run(unsigned_testee);
    let prime_factors:Vec<(u64, u32)> = factor_factory.prime_factor_repr();
    for prime in prime_factors{
        if prime.1 > 1 {return false;}
    }
    true
}

#[test]
fn test_is_squarefree(){
    rayon::prelude::ParallelIterator::for_each(rayon::prelude::IntoParallelIterator::into_par_iter(TEST_SQUAREFREE), |i| {
        assert!(is_squarefree(i));
        assert!(!is_squarefree(8));
        assert!(!is_squarefree(9));
});}

/// Checks if a number is a pronic number.
///
/// A pronic number, also known as an oblong number or rectangular number,
/// is a number that is the product of two consecutive integers.
/// Described in OEIS A002378
/// 
/// # Arguments
///
/// * `testee` - An i32 number to be tested.
///
/// # Example
///
/// ``` rust
/// let pronic: i32 = 6;
/// let not_pronic: i32 = 10;
/// assert!(is_pronic(pronic));
/// assert!(!is_pronic(not_pronic));
/// ```
///
pub fn is_pronic(testee: i32) -> bool {
    let mut sqrt: i32 = 2;
    if testee < 0 {return false;}
    if testee == 0 {return true;}
    if testee > 0 {sqrt = testee.integer_sqrt();}

    for i in 0..=sqrt {
        if i * (i + 1) == testee {
            return true;
        }
    }
    false
}

#[test]
fn test_is_pronic(){
    rayon::prelude::ParallelIterator::for_each(rayon::prelude::IntoParallelIterator::into_par_iter(TEST_PRONIC), |i| {
        assert!(is_pronic(i));});
        assert!(!is_pronic(8));
        assert!(!is_pronic(10));
}
