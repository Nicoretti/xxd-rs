//! The dump module contains code related for outputing/dumping data.
use std::fmt;
use std::io::Write;
use std::iter::Iterator;

/// Enum which provides all possible output value formats supported by the dump module.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Format {
    HexUpperCase,
    Hex,
    Decimal,
    Octal,
    Binary,
}

impl From<String> for Format {
    fn from(format_string: String) -> Self {
        match format_string.as_ref() {
            "Hex" => Format::HexUpperCase,
            "hex" => Format::Hex,
            "dec" => Format::Decimal,
            "oct" => Format::Octal,
            "bin" => Format::Binary,
            _ => panic!("Invalid output format"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Config {
    start_address: usize,
    show_address: bool,
    group_size: usize,
    columns: usize,
    show_interpretation: bool,
    use_separator: bool,
    output_fmt: Format,
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

impl Config {
    pub fn new() -> Config {
        Config {
            start_address: 0,
            show_address: true,
            group_size: 1,
            columns: 8,
            show_interpretation: true,
            use_separator: true,
            output_fmt: Format::HexUpperCase,
        }
    }

    pub fn bytes_per_line(&self) -> usize {
        self.columns * self.group_size
    }

    pub fn start_address(&self) -> usize {
        self.start_address
    }

    pub fn set_address(mut self, address: usize) -> Self {
        self.start_address = address;
        self
    }

    pub fn show_address(mut self, show: bool) -> Self {
        self.show_address = show;
        self
    }

    pub fn group_size(mut self, size: usize) -> Self {
        self.group_size = size;
        self
    }

    pub fn columns(mut self, columns: usize) -> Self {
        self.columns = columns;
        self
    }

    pub fn show_interpretation(mut self, show: bool) -> Self {
        self.show_interpretation = show;
        self
    }

    pub fn separator(mut self, use_separator: bool) -> Self {
        self.use_separator = use_separator;
        self
    }

    pub fn format(mut self, fmt: Format) -> Self {
        self.output_fmt = fmt;
        self
    }
}

/// The `OutputLine` struct contains all  information needed to dump/output a single line of data.
#[derive(Debug)]
pub struct OutputLine<'a> {
    output_settings: Config,
    data: &'a [u8],
}

impl<'a> OutputLine<'a> {
    pub fn new(data: &[u8]) -> OutputLine {
        OutputLine {
            output_settings: Config::new(),
            data,
        }
    }

    pub fn format(self, settings: Config) -> Self {
        OutputLine {
            output_settings: settings,
            data: self.data,
        }
    }

    fn write_address(&self, f: &mut fmt::Formatter) -> Result<usize, anyhow::Error> {
        write!(f, "{:08.X}: ", self.output_settings.start_address)?;
        Ok(10)
    }

    fn write_bytes(&self, f: &mut fmt::Formatter) -> Result<usize, anyhow::Error> {
        let mut bytes_written = 0;
        for (byte_offset, b) in self.data.iter().enumerate() {
            let is_seperator_necessary = (byte_offset + 1) % self.output_settings.group_size == 0;
            if is_seperator_necessary && self.output_settings.use_separator {
                bytes_written += self.write_formated_byte(f, *b)?;
                write!(f, " ")?;
                bytes_written += 1;
            } else {
                bytes_written += self.write_formated_byte(f, *b)?;
            }
        }
        Ok(bytes_written)
    }

    fn write_formated_byte(
        &self,
        f: &mut fmt::Formatter,
        byte: u8,
    ) -> Result<usize, anyhow::Error> {
        match self.output_settings.output_fmt {
            Format::HexUpperCase => {
                write!(f, "{:02.X}", byte)?;
                Ok(2)
            }
            Format::Hex => {
                write!(f, "{:02.x}", byte)?;
                Ok(2)
            }
            Format::Octal => {
                write!(f, "{:03.o}", byte)?;
                Ok(3)
            }
            Format::Decimal => {
                write!(f, "{:03}", byte)?;
                Ok(3)
            }
            Format::Binary => {
                write!(f, "{:08b}", byte)?;
                Ok(8)
            }
        }
    }

    fn write_interpretation(&self, f: &mut fmt::Formatter) -> Result<usize, anyhow::Error> {
        write!(f, " ")?;
        for b in self.data.iter() {
            match *b {
                character @ 20u8..=126u8 => write!(f, "{}", character as char)?,
                _ => write!(f, ".")?,
            }
        }
        Ok(self.data.len())
    }
}

impl<'a> fmt::Display for OutputLine<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let format_size = |fmt: Format| match fmt {
            Format::HexUpperCase => 2,
            Format::Hex => 2,
            Format::Octal => 3,
            Format::Decimal => 3,
            Format::Binary => 8,
        };
        if self.output_settings.show_address {
            match self.write_address(f) {
                Ok(_) => {}
                Err(_) => return Err(std::fmt::Error),
            }
        }
        let bytes_written = self.write_bytes(f).map_err(|_| ::std::fmt::Error)?;
        let expected_length = self.output_settings.columns
            * self.output_settings.group_size
            * format_size(self.output_settings.output_fmt)
            + (self.output_settings.columns);
        let padding = expected_length - bytes_written;
        for _ in 0..padding {
            write!(f, " ")?;
        }
        if self.output_settings.show_interpretation {
            match self.write_interpretation(f) {
                Ok(_) => {}
                Err(_) => return Err(std::fmt::Error),
            }
        }
        Ok(())
    }
}

// try static dispatch by changing params -> accept gernic with trait bounds e.g. into_iter
pub fn dump_iterator<I>(
    sequence: I,
    writer: &mut dyn Write,
    output_settings: Config,
) -> Result<(), anyhow::Error>
where
    I: Iterator<Item = u8>,
{
    let mut data: Vec<u8> = Vec::new();
    let mut offset: usize = 0;
    for byte in sequence {
        data.push(byte);
        if data.len() == output_settings.bytes_per_line() {
            dump_line(
                data.as_slice(),
                writer,
                output_settings.set_address(output_settings.start_address() + offset),
            );
            offset += data.len();
            data.clear();
        }
    }
    if !data.is_empty() {
        dump_line(
            data.as_slice(),
            writer,
            output_settings.set_address(output_settings.start_address() + offset),
        );
        data.clear();
    }
    Ok(())
}

fn dump_line(data: &[u8], writer: &mut dyn Write, output_settings: Config) {
    // TODO: handle error properly in stead of using `unwrap()`. The
    // error should propagate with `?` and the calling function should
    // handle it.
    let output_line = OutputLine::new(data).format(output_settings);
    writer.write_fmt(format_args!("{}\n", output_line)).unwrap();
}

#[cfg(test)]
mod test {

    use super::*;
    use std::fmt::Write;

    struct TestFixture {
        data: [u8; 8],
        small_data: [u8; 5],
    }

    impl TestFixture {
        fn new() -> Self {
            TestFixture {
                data: [0, 255, 127, 128, 56, 65, 1, 33],
                small_data: [0, 255, 80, 44, 7],
            }
        }

        fn data(&self) -> &[u8] {
            &self.data
        }

        fn small_data(&self) -> &[u8] {
            &self.small_data
        }
    }

    #[test]
    fn output_settings_can_be_constructed() {
        let _output_settings = Config::new();
        assert!(true);
    }

    #[test]
    fn output_settings_builder() {
        let format = Format::Binary;
        let start_address = 0xFF00AABB;
        let group_size = 2;
        let show_address = false;
        let show_interpretation = false;

        let output_settings = Config::new()
            .format(format)
            .set_address(start_address)
            .group_size(group_size)
            .show_address(show_address)
            .show_interpretation(show_interpretation);

        assert_eq!(output_settings.output_fmt, format);
        assert_eq!(output_settings.start_address, start_address);
        assert_eq!(output_settings.group_size, group_size);
        assert_eq!(output_settings.show_address, show_address);
        assert_eq!(output_settings.show_interpretation, show_interpretation);
    }

    #[test]
    fn output_settings_get_bytes_per_line() {
        {
            let group_size = 8;
            let columns = 2;
            let output_settings = Config::new().columns(columns).group_size(group_size);
            assert_eq!(group_size * columns, output_settings.bytes_per_line())
        }
        {
            let group_size = 5;
            let columns = 4;
            let output_settings = Config::new().columns(columns).group_size(group_size);
            assert_eq!(group_size * columns, output_settings.bytes_per_line())
        }
        {
            let group_size = 9;
            let columns = 4;
            let output_settings = Config::new().columns(columns).group_size(group_size);
            assert_eq!(group_size * columns, output_settings.bytes_per_line())
        }
    }

    #[test]
    fn output_settings_from_string() {
        assert_eq!(Format::Binary, Format::from("bin".to_string()));
        assert_eq!(Format::HexUpperCase, Format::from("Hex".to_string()));
        assert_eq!(Format::Hex, Format::from("hex".to_string()));
        assert_eq!(Format::Octal, Format::from("oct".to_string()));
        assert_eq!(Format::Decimal, Format::from("dec".to_string()));
    }

    #[test]
    #[should_panic]
    fn output_settings_from_panics_on_uppercase() {
        assert_eq!(Format::Decimal, Format::from("DEC".to_string()));
    }

    #[test]
    #[should_panic]
    fn output_settings_from_panics_on_unknown_string() {
        assert_eq!(
            Format::Decimal,
            Format::from("SomeRandomString".to_string())
        );
    }

    #[test]
    fn outputline_can_be_constructed() {
        let fixture = TestFixture::new();
        let _output_line = OutputLine::new(fixture.data());
        assert!(true);
    }

    #[test]
    fn default_output_format_for_a_single_line() {
        let fixture = TestFixture::new();
        let output_line = OutputLine::new(fixture.data());
        let expected_output = "00000000: 00 FF 7F 80 38 41 01 21  ....8A.!";
        let mut buffer = String::new();
        let result = write!(&mut buffer, "{}", output_line);
        assert_eq!(Ok(()), result);
        assert_eq!(expected_output, buffer);
    }

    #[test]
    fn default_output_format_for_a_single_line_with_padding() {
        let fixture = TestFixture::new();
        let output_line = OutputLine::new(fixture.small_data());
        let expected_output = "00000000: 00 FF 50 2C 07           ..P,.";
        let mut buffer = String::new();
        let result = write!(&mut buffer, "{}", output_line);
        assert_eq!(Ok(()), result);
        assert_eq!(expected_output, buffer);
    }

    #[test]
    fn octal_output_format_on_a_single_line() {
        let fixture = TestFixture::new();
        let output_settings = Config::new().format(Format::Octal);
        let output_line = OutputLine::new(fixture.data()).format(output_settings);
        let expected_output = "00000000: 000 377 177 200 070 101 001 041  ....8A.!";
        let mut buffer = String::new();
        let result = write!(&mut buffer, "{}", output_line);
        assert_eq!(Ok(()), result);
        assert_eq!(expected_output, buffer);
    }

    #[test]
    fn octal_output_format_for_a_single_line_with_padding() {
        let fixture = TestFixture::new();
        let output_settings = Config::new().format(Format::Octal);
        let output_line = OutputLine::new(fixture.small_data()).format(output_settings);
        let expected_output = "00000000: 000 377 120 054 007              ..P,.";
        let mut buffer = String::new();
        let result = write!(&mut buffer, "{}", output_line);
        assert_eq!(Ok(()), result);
        assert_eq!(expected_output, buffer);
    }

    #[test]
    fn binary_output_format_on_a_single_line() {
        let fixture = TestFixture::new();
        let output_settings = Config::new().format(Format::Binary);
        let output_line = OutputLine::new(fixture.data()).format(output_settings);
        let expected_output = "00000000: 00000000 11111111 01111111 10000000 00111000 01000001 00000001 00100001  ....8A.!";
        let mut buffer = String::new();
        let result = write!(&mut buffer, "{}", output_line);
        assert_eq!(Ok(()), result);
        assert_eq!(expected_output, buffer);
    }

    #[test]
    fn binary_output_format_for_a_single_line_with_padding() {
        let fixture = TestFixture::new();
        let output_settings = Config::new().format(Format::Binary);
        let output_line = OutputLine::new(fixture.small_data()).format(output_settings);
        let expected_output = "00000000: 00000000 11111111 01010000 00101100 00000111                             ..P,.";
        let mut buffer = String::new();
        let result = write!(&mut buffer, "{}", output_line);
        assert_eq!(Ok(()), result);
        assert_eq!(expected_output, buffer);
    }

    #[test]
    fn dump_line() {
        // set up
        let fixture = TestFixture::new();
        let expected_output = "00000000: 00 FF 50 2C 07           ..P,.\n";
        let output_settings = Config::new().format(Format::HexUpperCase);
        let mut buffer: Vec<u8> = Vec::new();

        // run test scenario
        super::dump_line(&fixture.small_data(), &mut buffer, output_settings);

        // assert expectations
        assert_eq!(expected_output.as_bytes(), buffer.as_slice());
    }

    #[test]
    fn dump_iterator() {
        // set up
        let v: Vec<u8> = vec![0, 255, 80, 44, 7];
        let small_data = v.into_iter();
        let expected_output = "00000000: 00 FF 50 2C 07           ..P,.\n";
        let output_settings = Config::new().format(Format::HexUpperCase);
        let mut buffer: Vec<u8> = Vec::new();

        // run test scenario
        super::dump_iterator(small_data, &mut buffer, output_settings).unwrap();

        // assert expectations
        assert_eq!(expected_output.as_bytes(), buffer.as_slice());
    }

    #[test]
    fn dump_iterator_with_offset() {
        // set up
        let v: Vec<u8> = vec![0, 255, 80, 44, 7];
        let small_data = v.into_iter();
        let expected_output = "0000000A: 00 FF 50 2C 07           ..P,.\n";
        let output_settings = Config::new().format(Format::HexUpperCase).set_address(10);
        let mut buffer: Vec<u8> = Vec::new();

        // run test scenario
        super::dump_iterator(small_data, &mut buffer, output_settings).unwrap();

        // assert expectations
        assert_eq!(expected_output.as_bytes(), buffer.as_slice());
    }
}
