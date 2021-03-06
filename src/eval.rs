use crate::{Matrix, FONT_SIZE};

pub fn max(arr: &[f64]) -> f64 {
    let mut max = arr[0].abs();
    for value in arr {
        let value = value.abs();
        if value > max {
            max = value;
        }
    }
    max
}

pub fn min(arr: &[f64]) -> f64 {
    let mut min = arr[0];
    for value in arr {
        if value < &min {
            min = *value;
        }
    }
    min
}

pub fn count_tens(mut num: f64) -> u128 {
    let mut count = 1;
    while num > 10. {
        num /= 10.;
        count *= 10;
    }
    //10u128.pow(count)
    count
}

pub fn count_inv_tens(mut num: f64) -> u128 {
    let mut count = 1;
    while num.round() == 0. {
        num *= 10.;
        count += 1;
    }
    10u128.pow(count)
}

pub fn divs(lhs: &[f64], rhs: f64) -> Vec<f64> {
    lhs.iter().map(|v| v / rhs).collect()
}

pub fn sub(lhs: &[f64], rhs: f64) -> Vec<f64> {
    let mut out = Vec::with_capacity(lhs.len());
    for val in lhs.iter() {
        out.push(val - rhs);
    }
    out
}

pub fn get_font_size_x(max: f64) -> f32 {
    let a = if max >= 3. {
        count_tens(max) * 10
    } else {
        count_inv_tens(max) * 12
    };

    let a = (a as f64).log10() as f32;
    FONT_SIZE - (2.9 * a)
}

pub fn get_font_size_y(max: f64) -> f32 {
    let a = if max >= 3. {
        count_tens(max) * 10
    } else {
        count_inv_tens(max) * 1000
    };

    let a = (a as f64).log10() as f32;
    FONT_SIZE - (1. * a)
}

const MAX_MIN_STEPS: f64 = 14.;
const MIN_MIN_STEPS: f64 = 3.;

pub fn get_steps(max: f64, mut min_steps: f64) -> f64 {
    //let mut steps = 4.;
    if max >= min_steps {
        while max % min_steps != 0. {
            min_steps += 1.;

            if min_steps == MAX_MIN_STEPS {
                return MAX_MIN_STEPS;
            }
        }
        return min_steps;
    }
    /* 
    let max = count_inv_tens(max) as f64 * max * 10.;
    while max % min_steps != 0. {
        min_steps -= 1.;
        println!("min_steps: {min_steps}");
    }
    */
    if min_steps < MIN_MIN_STEPS {
        return MIN_MIN_STEPS;
    }
    min_steps
}

pub fn max_display(max: f64, other_scaling: bool) -> f64 {
    if max == 0. {
        return 1.;
    }
    if max >= 2. { 
        
        /*let tens = count_tens(max) / 10;
        if tens == 0 {
            return max;
        }
        if tens == 1 {
            return (max / 10.).round() * 10.;
        }

        ((max / tens as f64 / 2.).round() * tens as f64) * 2.*/
            
        if other_scaling {
            let tens = count_tens(max) / 10;
            if tens == 0 {
                return max;
            }
            if tens == 1 {
                return (max / 10.).round() * 10.;
            }
            ((max / tens as f64 / 2.).round() * tens as f64) * 2.
        } else {
            let tens = count_tens(max);
            ((max / tens as f64 / 2.).round() * tens as f64) * 2.
        }
        
        

        /*
        let new_max = (max / 10f64).round() * 10.;
        if new_max == 0. {
            return max;
        }
        new_max
        */
        
    } else {
        let tens = count_inv_tens(max);
        ((max * tens as f64 / 2.).round() / tens as f64) * 2.
    }
}

#[test]
fn test_max() {
    let num = 26.;
    let x = (num / 10f64).round() * 10.;
    println!("x: {x}")
}


pub fn max_matrix(mat: &Matrix) -> f64 {
    let mut max = mat[0][0].abs();
    for x in mat {
        for x in x {
            let x = x.abs();
            if x > max {
                max = x;
            }
        }
    }
    max
}

pub fn min_matrix(mat: &Matrix) -> f64 {
    let mut min = mat[0][0].abs();
    for x in mat {
        for x in x {
            let x = x.abs();
            if x < min {
                min = x;
            }
        }
    }
    min
}


pub fn negative(arr: &[f64]) -> bool {
    let negative = true;
    for value in arr {
        if value.is_sign_positive() {
            return false;
        }
    }
    negative
}

#[test]
fn test_negative() {
    let a = [-1., -4., -2., -5.];
    let b = [-1., -4., -2., 5.];

    assert_eq!(negative(&a), true);
    assert_eq!(negative(&b), false);
}