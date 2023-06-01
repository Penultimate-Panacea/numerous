use prime_factorization::Factorization;

pub const TEST_POWERFUL:[i64; 53] = [1, 4, 8, 9, 16, 25, 27, 32, 36, 49, 64, 72, 81, 100, 108, 121, 125, 128, 144, 169, 196, 200, 216, 225, 243, 256, 288, 289, 324, 343, 361, 392, 400, 432, 441, 484, 500, 512, 529, 576, 625, 648, 675, 676, 729, 784, 800, 841, 864, 900, 961, 968, i64::MAX];


pub fn is_powerful(testee: i64) -> bool {
    let unsigned_testee = testee.unsigned_abs();
    let factor_factory = Factorization::run(unsigned_testee);
        for factor in factor_factory.factors{
        if factor * factor % unsigned_testee == 0  { //A powerful number is a positive integer m such that for every prime number p dividing m, p2 also divides m.
            return true
        }
    }
    false
}

