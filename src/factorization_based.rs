use primal::is_prime;

pub const TEST_PRIMES: [i64; 6] = [2,3,7_789,14_221,891_248_191, 10_000_000_019];

    /// Returns a boolean value, true if prime, false if not.
    /// Extends the use of the `primal::is_prime` by handling i64 numbers, and will be expanded in the future to handle i128 
    /// # Arguments 
    /// 
    /// * 'testee' - An i64 number to be checked against the primal crates's `is_prime` function. 
    /// 
pub fn is_prime_extended(testee: i64) -> bool {

    if testee < 0{
        false
    }
    else {
        let unsigned_testee:u64 = testee.unsigned_abs();
        is_prime(unsigned_testee)
    }
}


fn is_powerful(testee: i64) -> bool {
    primal::
}