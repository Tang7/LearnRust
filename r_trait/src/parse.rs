use std::str::FromStr;
use regex::Regex;

pub trait Parse {
    fn parse(s: &str) -> Self;
}

// restrict T has trait bound FromStr and Default
impl<T> Parse for T
where
    T: FromStr + Default,
{
    fn parse(s: &str) -> Self {
        let re: Regex = Regex::new(r"^[0-9]+(\.[0-9]+)?").unwrap();
        let d = || Default::default();
        
        if let Some(captures) = re.captures(s) {
            captures
                .get(0)
                .map_or(d(), |s| s.as_str().parse().unwrap_or(d()))
        } else {
            d()
        }
    }
}

#[test]
fn parse_test() {
    assert_eq!(u32::parse("123abc"), 123);
    assert_eq!(u32::parse("123.45abc"), 0);
    assert_eq!(f64::parse("123.45abc"), 123.45);
    assert_eq!(f64::parse("abc"), 0f64);
    assert_eq!(u32::parse("abc123"), 0);
}

fn main() {
    println!("result: {}", u8::parse("255 hello world!"));
}
