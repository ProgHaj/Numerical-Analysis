extern crate num;

use self::num::Num;
use std::ops::Mul;

/// Calculates nested form polynomial using Horner's Method
pub fn nested_multiplication<T> (coeff: &Vec<T>, x: T, base: &Vec<T>) -> Result<T, &'static str>
    where T: Num + Mul<Output = T> + Copy,
{
    let degree = coeff.len() - 1;
    let mut y = match coeff.get(degree) {
        Some(c_last)    => *c_last,
        None            => return Err("degree must be one less than the size of c"),
    };


    for i in (0..degree).rev() {
        y = y * (x - base[i]) + coeff[i];
    }
    Ok(y)
}


/// Bisection method
/// Computes approximate solution of f(x)=0
pub fn bisection_method(f: fn(f32) -> f32, pair: (f32, f32), stopping_criteria: f32) -> Result<(f32, f32), &'static str>
{
    let (mut a, mut b) = pair;
    if !(f(a)*f(b) < 0.) {
        return Err("a and b must have seperate sign");
    }

    while (b - a) / 2. > stopping_criteria {
        let c: f32 = (a + b) / 2.;

        if f(c).abs() < stopping_criteria {
            return Ok((a,b));
        }

        if f(a) * f(c) < 0. {
            b = c;
        } else {
            a = c;
        }
    }

    Ok((a, b))
}


#[cfg(test)]
mod test_nested_multiplication {
    use super::*;

    #[test]
    fn it_works() {
        let c: Vec<f32>  = vec![1., 2., 3., 4.];
        let x: f32  = 0.5;
        let b: Vec<f32> = vec![0., 0., 0., 0.];
        let y  = match nested_multiplication(&c, x, &b) {
            Ok(result)  =>  result,
            Err(err)    =>  panic!("Error: {}", err),
        };
        println!("y equals to: {}", y);
    }

    #[test]
    fn interpolating_polynomial() {
        let c: Vec<f32>  = vec![1., 0.5, 0.5, -0.5];
        let x: f32 = 1.;
        let b: Vec<f32> = vec![0., 2., 3.];
        let y  = match nested_multiplication(&c, x, &b) {
            Ok(result)  =>  result,
            Err(err)    =>  panic!("Error: {}", err),
        };
        assert_eq!(y, 0.);
        println!("y equals to: {}", y);
    }
}
