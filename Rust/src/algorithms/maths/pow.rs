pub fn pow_iterative(base: f64, exp: f64) -> f64 {
    let is_exp_negative = exp < 0.0;
    let mut exp = if is_exp_negative { -exp } else { exp };

    let mut res = 1.0;

    while exp > 0.0 {
        res *= base;
        exp -= 1.0;
    }

    if is_exp_negative {
        1.0 / res
    } else {
        res
    }
}

pub fn pow_recursive(base: f64, exp: f64) -> f64 {
    fn aux(acc: f64, base: f64, exp: f64) -> f64 {
        if exp == 0.0 {
            return acc;
        }

        aux(base * acc, base, exp - 1.0)
    }

    if exp < 0.0 {
        1.0 / aux(1.0, base, -exp)
    } else {
        aux(1.0, base, exp)
    }
}

pub fn pow(base: f64, exp: f64) -> f64 {
    fn aux(base: f64, exp: f64) -> f64 {
        if exp < 2.0 {
            return if exp == 0.0 { 1.0 } else { base };
        }

        let half = aux(base, f64::floor(exp / 2.0));
        let res = if exp % 2.0 == 0.0 {
            half * half
        } else {
            base * half * half
        };

        res
    }

    if exp < 0.0 {
        1.0 / aux(base, -exp)
    } else {
        aux(base, exp)
    }
}
