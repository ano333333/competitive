use cargo_snippet::snippet;

#[snippet("factorize")]
#[derive(Debug, PartialEq, Eq)]
struct FactorizeError;

#[snippet("factorize")]
fn factorize(n: LongLong) -> Result<HashMap<LongLong, LongLong>, FactorizeError> {
    if n <= 0 {
        return Err(FactorizeError {});
    }
    let mut map = std::collections::HashMap::<LongLong, LongLong>::new();
    let mut n = n;
    let mut p = 2;
    while n > 1 {
        if p * p > n {
            let old = map.get(&n);
            match old {
                Some(c) => {
                    map.insert(n, c + 1);
                }
                None => {
                    map.insert(n, 1);
                }
            }
            break;
        }
        if n % p != 0 {
            p += 1;
        } else {
            n /= p;
            let old = map.get(&p);
            match old {
                Some(c) => {
                    map.insert(p, c + 1);
                }
                None => {
                    map.insert(p, 1);
                }
            }
        }
    }
    return Ok(map);
}
