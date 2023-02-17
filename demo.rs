// This will not compile, this is just a snippet.
// TODO: Maybe making result mutable and += (hi > !lo) as u64
// and lo = r.wrapping_mul(n) * (hi == lo) as u64 might be faster
// because there'd be no branches?
fn gen_mod_u64(n: u64) -> u64 {
        // Code snippet adapted from smol-rs/fastrand, which was itself adapted from Lemire's algorithm.
        let mut r = gen_u64();
        let result = mul_high_u64(r, n);
        let mut hi: u64;
        let mut lo = r.wrapping_mul(n);
        while lo > n.wrapping_neg() {
            r = gen_u64();
            hi = mul_high_u64(r, n);
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

// I'll have to benchmark this variant as well, it avoids calculating
// !lo multiple times, but has a bit more complicated loop condition
fn alt_mod_u64(n: u64) -> u64 {
        // Code snippet adapted from smol-rs/fastrand, which was itself adapted from Lemire's algorithm.
        let mut r = gen_u64();
        let result = mul_high_u64(r, n);
        let mut hi: u64;
        let mut not_lo = !r.wrapping_mul(n);
        while not_lo < (n - 1) {
            r = gen_u64();
            hi = mul_high_u64(r, n);
            if hi > not_lo {
              return result + 1;
            }
            if hi < not_lo {
              return result;
            }
            not_lo = !r.wrapping_mul(n);
        }
        result
    }

