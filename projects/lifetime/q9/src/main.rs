/* Make it work by adding proper lifetime annotations */
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&'a self) -> i32 {
        3
    }
}

fn main() {}
