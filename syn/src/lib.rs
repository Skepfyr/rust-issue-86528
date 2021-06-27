use std::str::FromStr;

pub fn foo() {
    parse("").map(|_f: f64| ());
}

pub fn parse<F: FromStr>(this: &str) -> Result<F, F::Err> {
    FromStr::from_str(this)
}
