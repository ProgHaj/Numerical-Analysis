mod fundamentals;

use fundamentals::nested_multiplication;

fn main () {
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
