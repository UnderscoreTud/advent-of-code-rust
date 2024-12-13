pub mod template;

// https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm
pub fn extended_gcd(a: i64, b: i64) -> (i64, (i64, i64)) {
    let (mut old_s, mut s) = (1, 0);
    let (mut old_r, mut r) = (a, b);

    while r != 0 {
        let quotient = old_r / r;
        (old_r, r) = (r, old_r - quotient * r);
        (old_s, s) = (s, old_s - quotient * s);
    }

    let bezout_t = if b == 0 { 0 } else { (old_r - old_s * a) / b };
    (old_r, (old_s, bezout_t))
}
