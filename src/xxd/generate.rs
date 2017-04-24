use super::errors::*;

static C_PRE: &'static str = "const char data[] = {";
static C_SEPERATOR: &'static str = ",";
static C_POST: &'static str = "};";

static CPP_PRE: &'static str = "const char data[] = {";
static CPP_SEPERATOR: &'static str = ",";
static CPP_POST: &'static str = "};";

static PYTHON_PRE: &'static str = "data = [";
static PYTHON_SEPERATOR: &'static str = ",";
static PYTHON_POST: &'static str = "]";

static RUST_PRE: &'static str = "let data = [";
static RUST_SEPERATOR: &'static str = ",";
static RUST_POST: &'static str = "];";

trait Render {
    fn render(&self, data: &[u8]) -> String;
}

pub enum Language {
    C,
    Cpp,
    Rust,
    Python,
}

struct Template {
    prefix: String,
    separator: String,
    suffix: String,
}

impl Template {
    fn new(lang: Language) -> Template {
        match lang {
            Language::C => {
                Template {
                    prefix: C_PRE.to_string(),
                    separator: C_SEPERATOR.to_string(),
                    suffix: C_POST.to_string(),
                }
            }
            Language::Cpp => {
                Template {
                    prefix: CPP_PRE.to_string(),
                    separator: CPP_SEPERATOR.to_string(),
                    suffix: CPP_POST.to_string(),
                }
            }
            Language::Python => {
                Template {
                    prefix: PYTHON_PRE.to_string(),
                    separator: PYTHON_SEPERATOR.to_string(),
                    suffix: PYTHON_POST.to_string(),
                }
            }
            Language::Rust => {
                Template {
                    prefix: RUST_PRE.to_string(),
                    separator: RUST_SEPERATOR.to_string(),
                    suffix: RUST_POST.to_string(),
                }
            }
        }
    }
}

impl Render for Template {
    fn render(&self, data: &[u8]) -> String {
        let mut output = String::new();
        output = output + &self.prefix;
        for element in data.iter().enumerate() {
            let (index, byte) = element;
            if data.len() - 1 == index {
                output = output + &format!("{}", byte);
            } else {
                output = output + &format!("{}{} ", byte, self.separator);
            }
        }
        output = output + &self.suffix;
        output
    }
}

mod test {
    use super::*;

    #[test]
    fn render_basic_c_template() {
        let data = [0, 1, 2, 3, 4, 5];
        let template = Template::new(Language::C);
        let expected_result = "const char data[] = {0, 1, 2, 3, 4, 5};";
        assert_eq!(template.render(&data), expected_result);
    }

    #[test]
    fn render_basic_cplusplus_template() {
        let data = [0, 1, 2, 3, 4, 5];
        let template = Template::new(Language::Cpp);
        let expected_result = "const char data[] = {0, 1, 2, 3, 4, 5};";
        assert_eq!(template.render(&data), expected_result);
    }

    #[test]
    fn render_basic_python_template() {
        let data = [0, 1, 2, 3, 4, 5];
        let template = Template::new(Language::Python);
        let expected_result = "data = [0, 1, 2, 3, 4, 5]";
        assert_eq!(template.render(&data), expected_result);
    }

    #[test]
    fn render_basic_rust_template() {
        let data = [0, 1, 2, 3, 4, 5];
        let template = Template::new(Language::Rust);
        let expected_result = "let data = [0, 1, 2, 3, 4, 5];";
        assert_eq!(template.render(&data), expected_result);
    }
}
