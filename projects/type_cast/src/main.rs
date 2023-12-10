// fn multiply(x: i16, y: i16) -> i16 {
//     x * y
// }

// fn multiply2(x: i8, y: i8) -> i8 {
//     x * y
// }

fn multiply3(x: f32, y: f32) -> f32 {
    x * y
}

fn main() {
    let x: i8 = 15;
    let y: i16 = 1000;

    // println!("{x} * {y} = {}", multiply(x.into(), y));
    // println!("{x} * {y} = {}", multiply2(x, y as i8));
    println!("{x} * {y} = {}", multiply3(x.into(), y.into()));
}
