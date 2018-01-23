extern crate num;

use self::num::Num;
use std::ops::Mul;

/*
 * Evaluates polynomial from nested form using Horner's Method
 * d = degree of polynomial
 * array of d+1 coefficients c (constant term first),
 * x-coordinate x at which to evaluate,
 * array of d base points b, if needed (optional)
 */
pub fn nested_multiplication<T> (d: usize, c: &Vec<T>, x: T, b: &Vec<T>) -> Result<T, &'static str>
    where T: Num + Mul<Output = T> + Copy,
{
    let mut y = match c.get(d) {
        Some(c_last)    => *c_last,
        None            => return Err("d must be one less than the size of c"),
    };


    for i in (0..d).rev() {
        y = y * (x - b[i]) + c[i];
    }
    Ok(y)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let d: usize  = 3;
        let c: Vec<f32>  = vec![1., 2., 3., 4.];
        let x: f32  = 0.5;
        let b: Vec<f32> = vec![0., 0., 0., 0.];
        let y  = match nested_multiplication(d, &c, x, &b) {
            Ok(result)  =>  result,
            Err(err)    =>  panic!("Error: {}", err),
        };
        println!("y equals to: {}", y);
    }

    #[test]
    fn interpolating_polynomial() {
        let d: usize  = 3;
        let c: Vec<f32>  = vec![1., 0.5, 0.5, -0.5];
        let x: f32 = 1.;
        let b: Vec<f32> = vec![0., 2., 3.];
        let y  = match nested_multiplication(d, &c, x, &b) {
            Ok(result)  =>  result,
            Err(err)    =>  panic!("Error: {}", err),
        };
        assert_eq!(y, 0.);
        println!("y equals to: {}", y);
    }
}
