
type Celsius = f64;
type Fahrenheit = f64;

fn main() {
    let c: Celsius = 30.;
    let f: Fahrenheit = 77.;

    println!("{}C = {}F", c, to_fahrenheit(c));
    println!("{}F = {}C", f, to_celsius(f));
}

fn to_fahrenheit(c: Celsius) -> Fahrenheit {
    c as f64 * 1.8 + 32.
}

fn to_celsius(f: Fahrenheit) -> Celsius {
    (f as f64 - 32.) / 1.8
}
