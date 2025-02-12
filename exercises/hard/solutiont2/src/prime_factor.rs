use std::time::{SystemTime, UNIX_EPOCH};

/// ```python
/// def factor(n):
///    """因数分解"""
///    factors = []
///    stack = [n]
///    while stack:
///        m = stack.pop()
///        if is_prime(m):
///            factors.append(m)
///            continue
///        d = pollards_rho(m)
///        stack.append(d)
///        stack.append(m // d)
///    return sorted(factors)
/// ```
pub fn find_max_prime_factor(number: u128) -> u128 {
    let mut factors = Vec::new();
    let mut number = number;
    while number != 1 {
        let d = pollards_rho(number);
        if !is_prime(d) {
            continue;
        }
        factors.push(d);
        number /= d;
        if is_prime(number) {
            factors.push(number);
            break;
        }
    }
    factors.sort();
    factors.pop().unwrap()
}

/// ```python
/// def pollards_rho(n):
///     """Pollard's Rho算法分解因数"""
///     if n % 2 == 0:
///         return 2
///     if n % 3 == 0:
///         return 3
///     if n % 5 == 0:
///         return 5
///     while True:
///         c = random.randint(1, n - 1)
///         f = lambda x: (pow(x, 2, n) + c) % n
///         x, y = 2, 2
///         d = 1
///         while d == 1:
///             x = f(x)
///             y = f(f(y))
///             d = math.gcd(abs(x - y), n)
///         if d != n:
///             return d
/// ```
fn pollards_rho(n: u128) -> u128 {
    if n % 2 == 0 {
        return 2;
    }
    if n % 3 == 0 {
        return 3;
    }
    if n % 5 == 0 {
        return 5;
    }
    loop {
        let c = simple_random(n);
        let f = |x: u128| (pow(x, 2, n) + c) % n;
        let mut x = 2;
        let mut y = 2;
        let mut d = 1;
        while d == 1 {
            x = f(x);
            y = f(f(y));
            d = gcd(abs(x, y), n);
        }
        if d != n {
            return d;
        }
    }
}

fn simple_random(n: u128) -> u128 {
    let seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as u128;
    let mut seed = seed;
    seed ^= seed << 13;
    seed ^= seed >> 7;
    seed ^= seed << 17;
    seed % n
}

static TIP_PRIMES: [u128; 11] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31];

fn pow(base: u128, exponent: u128, modulus: u128) -> u128 {
    if modulus == 1 {
        return 0; // 任何数对1取模都是0
    }

    let mut result = 1;
    let mut base = base % modulus; // 先对base取模，避免溢出
    let mut exponent = exponent;

    while exponent > 0 {
        // 如果当前最低位是1，累乘到结果
        if exponent % 2 == 1 {
            result = mul_mod(result, base, modulus);
        }
        // 将base平方并取模
        base = mul_mod(base, base, modulus);
        // 右移一位，相当于除以2
        exponent >>= 1;
    }

    result
}
fn mul_mod(a: u128, b: u128, m: u128) -> u128 {
    if let Some(val) = a.checked_mul(b) {
        // 先尝试直接计算
        val % m
    } else {
        // 溢出时走安全路径
        let mut result = 0;
        let (mut a, mut b) = (a % m, b % m);
        while b > 0 {
            if b % 2 == 1 {
                result = (result + a) % m;
            }
            a = (a * 2) % m;
            b /= 2;
        }
        result
    }
}
fn abs(x: u128, y: u128) -> u128 {
    if x < y {
        y - x
    } else {
        x - y
    }
}
fn gcd(mut a: u128, mut b: u128) -> u128 {
    if a == 0 {
        return b;
    }
    if b == 0 {
        return a;
    }

    let shift = (a | b).trailing_zeros();
    a >>= a.trailing_zeros();
    b >>= b.trailing_zeros();

    while a != b {
        if a > b {
            a -= b;
            a >>= a.trailing_zeros();
        } else {
            b -= a;
            b >>= b.trailing_zeros();
        }
    }

    a << shift
}

fn is_prime(number: u128) -> bool {
    // Miller-Rabin素数检测
    if number < 2 {
        return false;
    }
    if TIP_PRIMES.iter().any(|&p| number % p == 0) {
        return TIP_PRIMES.contains(&number);
    }
    let mut d = number - 1;
    let mut s = 0;
    while d % 2 == 0 {
        d /= 2;
        s += 1;
    }
    for prime in TIP_PRIMES {
        if prime >= number {
            continue;
        }
        let mut x = pow(prime, d, number);
        if x == 1 || x == number - 1 {
            continue;
        }
        for _ in 0..s - 1 {
            x = pow(x, 2, number);
            if x == number - 1 {
                return true;
            }
        }
        return false;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pow() {
        assert_eq!(pow(10, 2, 9), 1);
        assert_eq!(pow(2, 10, 13), 10);
        assert_eq!(pow(3, 5, 7), 5);
        assert_eq!(pow(5, 10, 17), 9);
    }

    #[test]
    fn test_is_prime() {
        assert!(is_prime(2));
        assert!(is_prime(3));
        assert!(!is_prime(4));
        assert!(is_prime(5));
        assert!(is_prime(29));
        assert!(!is_prime(33));
        assert!(is_prime(9523809523809521497));
        assert!(!is_prime(1234169));
    }

    #[test]
    fn test_find_max_prime_factor1() {
        assert_eq!(find_max_prime_factor(10), 5);
    }

    #[test]
    fn test_find_max_prime_factor2() {
        assert_eq!(find_max_prime_factor(13195), 29);
    }

    #[test]
    fn test_find_max_prime_factor3() {
        assert_eq!(find_max_prime_factor(600851475143), 6857);
    }

    #[test]
    fn test_find_max_prime_factor4() {
        assert_eq!(
            find_max_prime_factor(97993999919999958437),
            203729729563409477
        );
    }
}
