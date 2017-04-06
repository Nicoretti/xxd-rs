//! The dump module contains code related for outputing/dumping data.
use std::fmt::Display;
use std::fmt::Error;
use std::convert::From;
use super::errors::*;
use std::convert::Into;

/// Enum which provides all possible output value formats supported by the dump module.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum OutputFormat {
    Hex,
    Decimal,
    Octal,
    Binary,
}

impl From<String> for OutputFormat {
    fn from(format_string: String) -> Self {
        match format_string.to_lowercase().as_ref() {
            "hex" => OutputFormat::Hex,
            "dec" => OutputFormat::Decimal,
            "oct" => OutputFormat::Octal,
            "bin" => OutputFormat::Binary,
            _ => panic!("Invalid output format"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct OutputSettings {
    start_address: usize,
    show_address: bool,
    group_size: usize,
    columns: usize,
    show_interpretation: bool,
    output_fmt: OutputFormat,
}

impl OutputSettings {
    pub fn new() -> OutputSettings {
        OutputSettings {
            start_address: 0,
            show_address: true,
            group_size: 1,
            columns: 8,
            show_interpretation: true,
            output_fmt: OutputFormat::Hex,
        }
    }

    pub fn bytes_per_line(&self) -> usize {
        self.columns * self.group_size
    }

    pub fn start_address(mut self, address: usize) -> Self {
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

    pub fn format(mut self, fmt: OutputFormat) -> Self {
        self.output_fmt = fmt;
        self
    }
}

/// The `OutputLine` struct contains all  information needed to dump/output a single line of data.
#[derive(Debug)]
pub struct OutputLine<'a> {
    output_settings: OutputSettings,
    data: &'a [u8],
}

impl<'a> OutputLine<'a> {
    pub fn new(data: &[u8]) -> OutputLine {
        OutputLine {
            output_settings: OutputSettings::new(),
            data: data,
        }
    }

    pub fn format(self, settings: OutputSettings) -> Self {
        OutputLine {
            output_settings: settings,
            data: self.data,
        }
    }

    fn write_address(&self, f: &mut ::fmt::Formatter) -> Result<usize> {
        write!(f, "{:08.X}: ", self.output_settings.start_address)?;
        Ok(10)
    }

    fn write_bytes(&self, f: &mut ::fmt::Formatter) -> Result<usize> {
        let mut byte_count = 0;
        let mut bytes_written = 0;
        for b in self.data.iter() {
            byte_count += 1;
            let is_seperator_necessary = byte_count % self.output_settings.group_size == 0;
            if is_seperator_necessary {
                bytes_written += self.write_formated_byte(f, b)?;
                write!(f, " ")?;
                bytes_written += 1;
            } else {
                bytes_written += self.write_formated_byte(f, b)?;
            }
        }
        Ok(bytes_written)
    }

    fn write_formated_byte(&self, f: &mut ::fmt::Formatter, byte: &u8) -> Result<usize> {
        match self.output_settings.output_fmt {
            OutputFormat::Hex => {
                write!(f, "{:02.X}", byte)?;
                Ok(2)
            }
            OutputFormat::Octal => {
                write!(f, "{:03.o}", byte)?;
                Ok(3)
            }
            OutputFormat::Decimal => {
                write!(f, "{:03}", byte)?;
                Ok(3)
            }
            OutputFormat::Binary => {
                write!(f, "{:08b}", byte)?;
                Ok(8)
            }
        }
    }

    fn write_interpretation(&self, f: &mut ::fmt::Formatter) -> Result<usize> {
        write!(f, "   ");
        for b in self.data.iter() {
            match *b {
                character @ 20u8...126u8 => write!(f, "{}", character as char)?,
                _ => write!(f, "{}", ".")?,
            }
        }
        Ok(self.data.len())
    }
}

impl<'a> ::fmt::Display for OutputLine<'a> {
    fn fmt(&self, f: &mut ::fmt::Formatter) -> ::std::fmt::Result {
        let format_size = |fmt: OutputFormat| match fmt {
            OutputFormat::Hex => 2,
            OutputFormat::Octal => 3,
            OutputFormat::Decimal => 3,
            OutputFormat::Binary => 8,
        };
        if self.output_settings.show_address {
            self.write_address(f);
        }
        let bytes_written = self.write_bytes(f).map_err(|e| ::std::fmt::Error)?;
        let expected_length = self.output_settings.columns * self.output_settings.group_size *
                              format_size(self.output_settings.output_fmt) +
                              (self.output_settings.columns);
        let padding = expected_length - bytes_written;
        for i in 0..padding {
            write!(f, " ")?;
        }
        if self.output_settings.show_interpretation {
            self.write_interpretation(f);
        }
        Ok(())
    }
}

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
        let output_settings = OutputSettings::new();
        assert!(true);
    }

    #[test]
    fn output_settings_builder() {
        let format = OutputFormat::Binary;
        let start_address = 0xFF00AABB;
        let group_size = 2;
        let show_address = false;
        let show_interpretation = false;

        let mut output_settings = OutputSettings::new()
            .format(format)
            .start_address(start_address)
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
            let mut output_settings = OutputSettings::new().columns(columns).group_size(group_size);
            assert_eq!(group_size * columns, output_settings.bytes_per_line())
        }
        {
            let group_size = 5;
            let columns = 4;
            let mut output_settings = OutputSettings::new().columns(columns).group_size(group_size);
            assert_eq!(group_size * columns, output_settings.bytes_per_line())
        }
        {
            let group_size = 9;
            let columns = 4;
            let mut output_settings = OutputSettings::new().columns(columns).group_size(group_size);
            assert_eq!(group_size * columns, output_settings.bytes_per_line())
        }
    }

    #[test]
    fn output_settings_from_string() {
        assert_eq!(OutputFormat::Binary, OutputFormat::from("bin".to_string()));
        assert_eq!(OutputFormat::Hex, OutputFormat::from("hex".to_string()));
        assert_eq!(OutputFormat::Octal, OutputFormat::from("oct".to_string()));
        assert_eq!(OutputFormat::Decimal, OutputFormat::from("dec".to_string()));

        assert_eq!(OutputFormat::Binary, OutputFormat::from("Bin".to_string()));
        assert_eq!(OutputFormat::Hex, OutputFormat::from("hEx".to_string()));
        assert_eq!(OutputFormat::Octal, OutputFormat::from("ocT".to_string()));
        assert_eq!(OutputFormat::Decimal, OutputFormat::from("DEC".to_string()));
    }

    #[test]
    fn outputline_can_be_constructed() {
        let fixture = TestFixture::new();
        let output_line = OutputLine::new(fixture.data());
        assert!(true);
    }

    #[test]
    fn default_output_format_for_a_single_line() {
        let fixture = TestFixture::new();
        let output_line = OutputLine::new(fixture.data());
        let expected_output = "00000000: 00 FF 7F 80 38 41 01 21    ....8A.!";
        let mut buffer = String::new();
        let result = write!(&mut buffer, "{}", output_line);
        assert_eq!(Ok(()), result);
        assert_eq!(expected_output, buffer);
    }

    #[test]
    fn default_output_format_for_a_single_line_with_padding() {
        let fixture = TestFixture::new();
        let output_line = OutputLine::new(fixture.small_data());
        let expected_output = "00000000: 00 FF 50 2C 07             ..P,.";
        let mut buffer = String::new();
        let result = write!(&mut buffer, "{}", output_line);
        assert_eq!(Ok(()), result);
        assert_eq!(expected_output, buffer);
    }

    #[test]
    fn octal_output_format_on_a_single_line() {
        let fixture = TestFixture::new();
        let output_settings = OutputSettings::new().format(OutputFormat::Octal);
        let output_line = OutputLine::new(fixture.data()).format(output_settings);
        let expected_output = "00000000: 000 377 177 200 070 101 001 041    ....8A.!";
        let mut buffer = String::new();
        let result = write!(&mut buffer, "{}", output_line);
        assert_eq!(Ok(()), result);
        assert_eq!(expected_output, buffer);
    }

    #[test]
    fn octal_output_format_for_a_single_line_with_padding() {
        let fixture = TestFixture::new();
        let output_settings = OutputSettings::new().format(OutputFormat::Octal);
        let output_line = OutputLine::new(fixture.small_data()).format(output_settings);
        let expected_output = "00000000: 000 377 120 054 007                ..P,.";
        let mut buffer = String::new();
        let result = write!(&mut buffer, "{}", output_line);
        assert_eq!(Ok(()), result);
        assert_eq!(expected_output, buffer);
    }

    #[test]
    fn binary_output_format_on_a_single_line() {
        let fixture = TestFixture::new();
        let output_settings = OutputSettings::new().format(OutputFormat::Binary);
        let output_line = OutputLine::new(fixture.data()).format(output_settings);
        let expected_output = "00000000: 00000000 11111111 01111111 10000000 00111000 01000001 00000001 00100001    ....8A.!";
        let mut buffer = String::new();
        let result = write!(&mut buffer, "{}", output_line);
        assert_eq!(Ok(()), result);
        assert_eq!(expected_output, buffer);
    }

    #[test]
    fn binary_output_format_for_a_single_line_with_padding() {
        let fixture = TestFixture::new();
        let output_settings = OutputSettings::new().format(OutputFormat::Binary);
        let output_line = OutputLine::new(fixture.small_data()).format(output_settings);
        let expected_output = "00000000: 00000000 11111111 01010000 00101100 00000111                               ..P,.";
        let mut buffer = String::new();
        let result = write!(&mut buffer, "{}", output_line);
        assert_eq!(Ok(()), result);
        assert_eq!(expected_output, buffer);
    }
}
