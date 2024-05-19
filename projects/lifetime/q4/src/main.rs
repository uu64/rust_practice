// `'a` must live longer than the function.
// Here, `&String::from("foo")` would create a `String`, followed by a
// reference. Then the data is dropped upon exiting the scope, leaving
// a reference to invalid data to be returned.

/* Fix the error in three ways  */
// fn invalid_output<'a>() -> &'a String {
//     &String::from("foo")
// }

fn invalid_output() -> String {
    String::from("foo")
}

struct Test<'a> {
    data: &'a str,
}

impl<'a> Test<'a> {
    fn invalid_output_3(&self) -> &'a str {
        &self.data
    }
}

fn main() {
}
