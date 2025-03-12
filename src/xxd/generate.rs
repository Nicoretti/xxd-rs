use std::convert::From;

static C_PRE: &str = "const char data[] = {";
static C_SEPERATOR: &str = ",";
static C_POST: &str = "};";

static CPP_PRE: &str = "const char data[] = {";
static CPP_SEPERATOR: &str = ",";
static CPP_POST: &str = "};";

static PYTHON_PRE: &str = "data = [";
static PYTHON_SEPERATOR: &str = ",";
static PYTHON_POST: &str = "]";

static RUST_PRE: &str = "pub const DATA: &'static[u8] = &[";
static RUST_SEPERATOR: &str = ",";
static RUST_POST: &str = "];";

pub trait Render {
    fn render(&self, data: &[u8]) -> String;
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Language {
    C,
    Cpp,
    Rust,
    Python,
    Unknown,
}

impl<'a> From<&'a str> for Language {
    fn from(other: &str) -> Language {
        match other.to_lowercase().as_ref() {
            "c" => Language::C,
            "cpp" => Language::Cpp,
            "rust" => Language::Rust,
            "python" => Language::Python,
            _ => Language::Unknown,
        }
    }
}

pub struct Template {
    prefix: String,
    separator: String,
    suffix: String,
    bytes_per_line: usize,
}

impl Template {
    pub fn new(lang: Language) -> Template {
        match lang {
            Language::C => Template {
                prefix: C_PRE.to_string(),
                separator: C_SEPERATOR.to_string(),
                suffix: C_POST.to_string(),
                bytes_per_line: 16,
            },
            Language::Cpp => Template {
                prefix: CPP_PRE.to_string(),
                separator: CPP_SEPERATOR.to_string(),
                suffix: CPP_POST.to_string(),
                bytes_per_line: 16,
            },
            Language::Python => Template {
                prefix: PYTHON_PRE.to_string(),
                separator: PYTHON_SEPERATOR.to_string(),
                suffix: PYTHON_POST.to_string(),
                bytes_per_line: 16,
            },
            Language::Rust => Template {
                prefix: RUST_PRE.to_string(),
                separator: RUST_SEPERATOR.to_string(),
                suffix: RUST_POST.to_string(),
                bytes_per_line: 16,
            },
            Language::Unknown => Template {
                prefix: "".to_string(),
                separator: ",".to_string(),
                suffix: "".to_string(),
                bytes_per_line: 16,
            },
        }
    }
}

impl Render for Template {
    fn render(&self, data: &[u8]) -> String {
        let mut output = String::new();
        if data.len() <= 16 {
            output = output + &self.prefix + " ";
            for element in data.iter().enumerate() {
                let (index, byte) = element;
                if data.len() - 1 == index {
                    output = output + &format!("0x{:02X}", byte);
                } else {
                    output = output + &format!("0x{:02X}{} ", byte, self.separator);
                    if (index + 1) % self.bytes_per_line == 0 {
                        output += "\n";
                    }
                }
            }
            output = output + " " + &self.suffix;
        } else {
            output = output + &self.prefix + "\n";
            for element in data.iter().enumerate() {
                let (index, byte) = element;
                if (index + 1) % self.bytes_per_line == 1 {
                    output += "    ";
                }
                if data.len() - 1 == index {
                    output = output + &format!("0x{:02X}", byte);
                } else if (index + 1) % self.bytes_per_line == 0 {
                    output = output + &format!("0x{:02X}{}", byte, self.separator);
                    output += "\n";
                } else {
                    output = output + &format!("0x{:02X}{} ", byte, self.separator);
                }
            }
            output = output + "\n" + &self.suffix;
        }
        output
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn from_conversion_of_language_enum() {
        assert_eq!(Language::from("c"), Language::C);
        assert_eq!(Language::from("C"), Language::C);
        assert_eq!(Language::from("cpp"), Language::Cpp);
        assert_eq!(Language::from("Cpp"), Language::Cpp);
        assert_eq!(Language::from("Rust"), Language::Rust);
        assert_eq!(Language::from("rust"), Language::Rust);
        assert_eq!(Language::from("Python"), Language::Python);
        assert_eq!(Language::from("python"), Language::Python);
        assert_eq!(Language::from("smth"), Language::Unknown);
        assert_eq!(Language::from("Smth"), Language::Unknown);
        assert_eq!(Language::from("-*@"), Language::Unknown);
    }

    #[test]
    fn render_basic_c_template() {
        let data = [0, 1, 2, 3, 4, 5];
        let template = Template::new(Language::C);
        let expected_result = "const char data[] = { 0x00, 0x01, 0x02, 0x03, 0x04, 0x05 };";
        assert_eq!(template.render(&data), expected_result);
    }

    #[test]
    fn render_c_template_with_large_data() {
        let data = [
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
            24, 25, 26,
        ];
        let template = Template::new(Language::C);
        let expected_result = r#"const char data[] = {
    0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F,
    0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A
};"#;
        assert_eq!(template.render(&data), expected_result);
    }

    #[test]
    fn render_basic_cplusplus_template() {
        let data = [0, 1, 2, 3, 4, 5];
        let template = Template::new(Language::Cpp);
        let expected_result = "const char data[] = { 0x00, 0x01, 0x02, 0x03, 0x04, 0x05 };";
        assert_eq!(template.render(&data), expected_result);
    }

    #[test]
    fn render_cplusplus_template_with_large_data() {
        let data = [
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
            24, 25, 26,
        ];
        let template = Template::new(Language::Cpp);
        let expected_result = r#"const char data[] = {
    0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F,
    0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A
};"#;
        assert_eq!(template.render(&data), expected_result);
    }

    #[test]
    fn render_basic_python_template() {
        let data = [0, 1, 2, 3, 4, 5];
        let template = Template::new(Language::Python);
        let expected_result = r#"data = [ 0x00, 0x01, 0x02, 0x03, 0x04, 0x05 ]"#;
        assert_eq!(template.render(&data), expected_result);
    }

    #[test]
    fn render_python_template_with_large_data() {
        let data = [
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
            24, 25, 26,
        ];
        let template = Template::new(Language::Python);
        let expected_result = r#"data = [
    0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F,
    0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A
]"#;
        assert_eq!(template.render(&data), expected_result);
    }

    #[test]
    fn render_basic_rust_template() {
        let data = [0, 1, 2, 3, 4, 5];
        let template = Template::new(Language::Rust);
        let expected_result = r#"let data = [ 0x00, 0x01, 0x02, 0x03, 0x04, 0x05 ];"#;
        assert_eq!(template.render(&data), expected_result);
    }

    #[test]
    fn render_rust_template_with_large_data() {
        let data = [
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
            24, 25, 26,
        ];
        let template = Template::new(Language::Rust);
        let expected_result = r#"let data = [
    0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F,
    0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A
];"#;
        assert_eq!(template.render(&data), expected_result);
    }
}
