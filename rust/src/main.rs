// Ctrl + Shift + F10 for first run
fn main() {
    println!("Hello, Rust world!");
    let apple_pie = vec![1., 0., 0.];
    let burger = vec![0., 1., 0.];
    let chekin = vec![0., 0., 1.];
    let food: [&Vec<f64>; 2] = [&apple_pie, &burger];

    let all_food: [&Vec<f64>; 3] = [&apple_pie, &burger, &chekin];
    let food_sequence: [&Vec<f64>; 3] = [&burger, &chekin, &apple_pie];
    let food_mix = all_food.iter().zip(&food_sequence);

    let mut x = vec![1.,3.,2.];
    let max = x.iter().fold(std::f64::NEG_INFINITY, |acc, x| f64::max(acc, *x));
    x.iter_mut().map(|v| *v = if *v < max {0.} else {1.}).count();
//    for (x, y) in &food_mix {
//        println!("x: {:?} \ny: {:?}", x, y);
//    }

//    for (i, (x, y)) in food_mix.enumerate() {
//        println!("{}: ({:?}, {:?})", i, x, y);
//     }
    //println!("{:?}", food_mix);

    let sunny = vec![1., 0.];
    let rainy = vec![0., 1.];
    let weather: [&Vec<f64>; 1] = [&rainy];

    match foo(&food, &weather) {
        Err(e) => println!("Error = {:?}", e),
        Ok(r) => {
            println!("result = {:?}", r);
            let is_burger = vec_compare(&r, &burger);
            println!("is burger: {}", is_burger);
             match all_food.iter().position(|&v| *v == r) {
                 None => println!("Not found"),
                 Some(i) => match i {
                     0 => println!("Apple Pie"),
                     1 => println!("Burger"),
                     _ => println!("Chekin"),
                 }
             }
        }
    } ;
}

type OryxResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;
fn foo(x: &[&Vec<f64>], y: &[&Vec<f64>]) -> OryxResult<Vec<f64>> {
    // n x m * m x p = n x p
    let m = x.len();   // 2 // number of vecs
    let n = x[0].len();  // 3 // vec length

    let _m = y[0].len();   // 2  // vec length
    let p = y.len();  // 1 // number of vecs
    if x.len() != y[0].len() {
        Err(format!("cross dimentional is invalid"))?
    } else {
        //let mut array: Vec<Vec<i32>> = vec![vec![Default::default(); p]; n];
        let mut array: Vec<f64> = vec![Default::default(); n];

        for i in 0..n {  // 3;
            for j in 0..m {   // 2
                for k in 0..p {   // 1
                //    array[i][k] += x[j][i] * y[k][j];
                    array[i] += x[j][i] * y[k][j];
                }
            }
        }
        Ok(array)
    }
}

fn eq_with_nan_eq(a: f64, b: f64) -> bool {
    (a.is_nan() && b.is_nan()) || (a == b)
}

fn vec_compare(va: &Vec<f64>, vb: &Vec<f64>) -> bool
{
    (va.len() == vb.len()) &&  // zip stops at the shortest
     va.iter()
       .zip(vb)
       .all(|(a,b)| eq_with_nan_eq(*a,*b))
}
