use cargo_snippet::snippet;

#[snippet]
fn gcd(a: &i64, b: &i64) -> i64 {
    let mut a = *a;
    let mut b = *b;
    if a < b {
        std::mem::swap(&mut a, &mut b);
    }
    if a % b == 0 {
        return b;
    }
    return gcd(&b, &(a % b));
}
