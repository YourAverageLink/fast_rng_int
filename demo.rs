// This will not compile, this is just a snippet.
fn gen_mod_u64(n: u64) -> u64 {
        // Code snippet adapted from smol-rs/fastrand, which was itself adapted from Lemire's algorithm.
        let mut r = gen_u64();
        let result = mul_high_u64(r, n);
        let mut hi: u64;
        let mut lo = r.wrapping_mul(n);
        while lo > n.wrapping_neg() {
            r = gen_u64();
            hi = r.mul_high_u64(r, n);
            if hi > !lo {
              return result + 1;
            }
            if hi < !lo {
              return result;
            }
            lo = r.wrapping_mul(n);
        }
        result
    }
