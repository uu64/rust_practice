fn main() {
    let month = [
        "first",
        "second",
        "third",
        "fourth",
        "fifth",
        "sixth",
        "seventh",
        "eighth",
        "ninth",
        "tenth",
        "eleventh",
        "twelfth",
    ];
    let presents = [
        "a Partridge in a Pear Tree",
        "2 Turtle Doves",
        "3 French Hens",
        "4 Calling Birds",
        "5 Golden Rings",
        "6 Geese a Laying",
        "7 Swans a Swimming",
        "8 Maids a Milking",
        "9 Ladies Dancing",
        "10 Loads a Leaping",
        "11 Pipers Piping",
        "12 Drummers Drumming",
    ];


    for (i, m) in month.iter().enumerate() {
        println!("On the {} day of Christmas", m);
        println!("my true love sent to me:");
        if i == 0 {
            println!("{}", presents[i][0..1].to_uppercase() + &presents[i][1..]);
        } else {
            for p in presents[1..i+1].iter().rev() {
                println!("{p}");
            }
            println!("and {}", presents[0])
        }
        println!("");
    }
}
