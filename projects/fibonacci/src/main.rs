use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{}", fibonacci(args[1].parse().expect("error: not a number")))
}

fn fibonacci(index: i64) -> i64 {
    if index < 3 {
        return 1;
    }

    let (mut f1, mut f2)  = (1, 1);
    let mut n = 3;

    while n <= index {
        let tmp = f1 + f2;
        f1 = f2;
        f2 = tmp;
        n += 1;
    }
    return f2;
}
