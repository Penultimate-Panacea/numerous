use prime_factorization::Factorization;
pub const TEST_POWERFUL:[i64; 53] = [1, 4, 8, 9, 16, 25, 27, 32, 36, 49, 64, 72, 81, 100, 108, 121, 125, 128, 144, 169, 196, 200, 216, 225, 243, 256, 288, 289, 324, 343, 361, 392, 400, 432, 441, 484, 500, 512, 529, 576, 625, 648, 675, 676, 729, 784, 800, 841, 864, 900, 961, 968, i64::MAX];

///
/// A powerful number is a positive integer m such that for every prime number p dividing m, p2 also divides m
/// 
/// 
pub fn is_powerful(testee: i64) -> bool {
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

