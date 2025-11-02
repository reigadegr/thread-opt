use taplo::formatter;

pub fn format_toml(input: &str) -> String {
    let options = formatter::Options {
        indent_string: "    ".to_string(),
        ..Default::default()
    };
    formatter::format(input, options)
}
