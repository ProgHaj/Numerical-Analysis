mod numan;

use numan::nested_multiplication;
use numan::bisection_method;

fn main () {
    nested();
    bisection();
}


fn nested() {
    let c: Vec<f32>  = vec![1., 0.5, 0.5, -0.5];
    let x: f32 = 1.;
    let b: Vec<f32> = vec![0., 2., 3.];
    let y  = match nested_multiplication(&c, x, &b) {
        Ok(result)  =>  result,
        Err(err)    =>  panic!("Error: {}", err),
    };
    println!("y equals to: {}", y);

}

fn bisection() {
    let closure = |x: f32| -> f32 {x.powf(3.) + x - 1.};
    let y = bisection_method(closure, (0., 1.), 0.00005);
    println!("bisec y equals to: {:?}", y);
}
