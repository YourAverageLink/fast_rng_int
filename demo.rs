// This will not compile, this is just a snippet.
fn gen_mod_u64(n: u64) -> u64 {
        // Code snippet adapted from smol-rs/fastrand, which was itself adapted from Lemire's algorithm.
        let mut r = gen_u64();
        let mut result = mul_high_u64(r, n);
        let mut hi: u64;
        let mut lo = r.wrapping_mul(n);
        // Determine if the number is "binary borderline", that is, extra bits could increment the result
        while lo > n.wrapping_neg() {
            r = gen_u64();
            hi = mul_high_u64(r, n);
            // hi > !lo implies the new bits *would* increment the original result, so add to the result
            result += (hi > !lo);
            // hi != lo implies the new bits definitely do (hi > !lo) or do not (hi < !lo) increment the result
            // set lo to 0 to exit the loop branchlessly if so.
            lo = r.wrapping_mul(n) * (hi == !lo) as u64;
        }
        result
    }

// I'll have to benchmark this variant as well, it avoids calculating
// !lo multiple times, but has a bit more complicated loop condition
fn alt_mod_u64(n: u64) -> u64 {
        // Code snippet adapted from smol-rs/fastrand, which was itself adapted from Lemire's algorithm.
        let mut r = gen_u64();
        let mut result = mul_high_u64(r, n);
        let mut hi: u64;
        let mut not_lo = !r.wrapping_mul(n);
        while not_lo < (n - 1) {
            r = gen_u64();
            hi = mul_high_u64(r, n);
            result += (hi > not_lo) as u64;
            not_lo = !r.wrapping_mul(n) * (hi == not_lo) as u64;
        }
        result
    }

