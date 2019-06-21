use std::iter::{once, repeat};

fn main() {
    let fizzes = repeat("").take(2).chain(once("Fizz")).cycle();
    let buzzes = repeat("").take(4).chain(once("Buzz")).cycle();

    let fizzes_buzzes = fizzes.zip(buzzes);

    let fizz_buzz = (1..100).zip(fizzes_buzzes).map(|tuple| match tuple {
        (i, ("", "")) => i.to_string(),
        (_, (fizz, buzz)) => format!("{}{}", fizz, buzz),
    });

    for line in fizz_buzz {
        println!("{}", line);
    }
}
