use rand::Rng;

fn gcd(a: u128, b: u128) -> u128 {
    if b == 0 {
        return a;
    }
    return gcd(b, a % b);
}

// 安全的模乘运算
fn mod_mul(a: u128, b: u128, modu: u128) -> u128 {
    let mut res = 0;
    let mut a = a;
    let mut b = b;
    while b > 0 {
        if b & 1 == 1 {
            res = (res + a) % modu;
        }
        a = (a + a) % modu;
        b >>= 1;
    }
    return res;
}

// 快速幂取模
fn mod_pow(mut base: u128, mut exp: u128, modu: u128) -> u128 {
    let mut res = 1;
    base %= modu;
    while exp > 0 {
        if exp % 2 == 1 {
            res = mod_mul(res, base, modu);
        }
        base = mod_mul(base, base, modu);
        exp >>= 1;
    }
    res
}

// Miller-Rabin素数检测
pub fn is_prime(n: u128) -> bool {
    match n {
        0 | 1 => return false,
        2 | 3 => return true,
        _ if n % 2 == 0 => return false,
        _ => {}
    }

    let mut d = n - 1;
    let mut s = 0;
    while d % 2 == 0 {
        d >>= 1;
        s += 1;
    }

    let base = match () {
        _ if n < 2_152_302_898_747 => vec![2, 3, 5, 7, 11, 13, 17],
        _ if n < 3_323_393_158_797 => vec![2, 3, 5, 7, 11, 13, 17, 19, 23],
        _ => vec![
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 325, 9375, 28178, 450775, 9780504,
            1795265022,
        ],
    };

    for &a in base.iter() {
        let a = a % n;

        let mut x = mod_pow(a, d, n);
        if x == 1 || x == n - 1 {
            continue;
        }

        let mut flag = false;
        for _ in 0..s {
            x = mod_mul(x, x, n);
            if x == n - 1 {
                flag = true;
                break;
            }
        }
        if !flag {
            return false;
        }
    }
    true
}

fn mod_add(a: u128, b: u128, modu: u128) -> u128 {
    (a % modu + b % modu) % modu
}

fn help(x: u128, c: u128, n: u128) -> u128 {
    let x_square = mod_mul(x, x, n);
    let c_mod = c % n;
    mod_add(x_square, c_mod, n)
}

fn pollard_rho(n: u128) -> u128 {
    if n == 4 {
        return 2;
    }

    if is_prime(n) {
        return n;
    }

    let mut rng = rand::thread_rng();

    loop {
        let c = rng.gen::<u128>() % n + 1;
        let mut t = help(0, c, n);
        let mut r = help(help(0, c, n), c, n);

        while t != r {
            let abs_res = std::cmp::max(t, r) - std::cmp::min(t, r);
            let d = gcd(abs_res, n);
            if d != 1 {
                return d;
            }
            t = help(t, c, n);
            r = help(help(r, c, n), c, n);
        }
    }
}

pub fn find_max_prime_factor(number: u128) -> u128 {
    if is_prime(number) {
        return number;
    }

    let fac:u128 = pollard_rho(number);

    let res = std::cmp::max(fac, find_max_prime_factor(number / fac));
    res
}