
/*
 * Evaluates polynomial from nested form using Horner's Method
 * d = degree of polynomial
 * array of d+1 coefficients c (constant term first),
 * x-coordinate x at which to evaluate,
 * array of d base points b, if needed (optional)
 */
pub fn nested_multiplication(d: usize, c: &Vec<f32>, x: f32, b: &Vec<f32>) -> Result<f32, &'static str>
{
    let mut y = match c.get(d) {
        Some(c_last)    => *c_last,
        None            => return Err("d must be one less than the size of c"),
    };


    for i in (0..d).rev() {
        y *= (x - b[i]) + c[i];
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
}
