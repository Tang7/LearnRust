pub trait Formatter {
    fn format(&self, input: &mut String) -> bool;
}

struct RustFormatter;
impl Formatter for RustFormatter {
    fn format(&self, input: &mut String) -> bool {
        input.push_str("\nRust formatter");
        true
    }
}

struct HtmlFormatter;
impl Formatter for HtmlFormatter {
    fn format(&self, input: &mut String) -> bool {
        input.push_str("\nHtml formatter");
        true
    }
}

pub fn format(input: &mut String, formatters: Vec<&dyn Formatter>) {
    for formatter in formatters {
        formatter.format(input);
    }
}

fn main() {
    let mut text = "Hello world!".to_string();
    let rust: &dyn Formatter = &RustFormatter;
    let html: &dyn Formatter = &HtmlFormatter;
    let formatters = vec![rust, html];
    format(&mut text, formatters);

    println!("text: {}", text);
}
