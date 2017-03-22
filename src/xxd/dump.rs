//! The dump module contains code related for outputing/dumping data.
use std::fmt::Display;
use std::convert::From;
use super::errors::*;

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

#[derive(Debug)]
pub struct OutputSettings {
    start_address: u32,
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

    pub fn start_address(mut self, address: u32) -> Self {
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
    data: &'a [u8], // TODO NiCo: Add member for data format (output format of data)
}

impl<'a> OutputLine<'a> {
    fn new(data: &[u8]) -> OutputLine {
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

    fn write_address(&self, f: &mut ::fmt::Formatter) -> Result<()> {
        write!(f, "{:08.X}: ", self.output_settings.start_address).map_err(|e| e.into())
    }

    fn write_bytes(&self, f: &mut ::fmt::Formatter) -> Result<()> {
        let mut byte_count = 0;
        for b in self.data.iter() {
            byte_count += 1;
            let is_seperator_necessary = byte_count % self.output_settings.group_size == 0;
            if is_seperator_necessary {
                self.write_formated_byte(f, b)?;
                write!(f, " ")?
            } else {
                self.write_formated_byte(f, b)?
            }
        }
        Ok(())
    }

    fn write_formated_byte(&self, f: &mut ::fmt::Formatter, byte: &u8) -> Result<()> {
        match self.output_settings.output_fmt {
            OutputFormat::Hex => write!(f, "{:02.X}", byte).map_err(|e| e.into()),
            OutputFormat::Octal => write!(f, "{:02.o}", byte).map_err(|e| e.into()),
            OutputFormat::Decimal => write!(f, "{:02}", byte).map_err(|e| e.into()),
            OutputFormat::Binary => write!(f, "{:02b}", byte).map_err(|e| e.into()),
        }
    }

    fn write_interpretation(&self, f: &mut ::fmt::Formatter) -> Result<()> {
        write!(f, "   ");
        for b in self.data.iter() {
            match *b {
                character @ 20u8...126u8 => write!(f, "{}", character as char)?,
                _ => write!(f, "{}", ".")?,
            }
        }
        Ok(())
    }
}

impl<'a> ::fmt::Display for OutputLine<'a> {
    fn fmt(&self, f: &mut ::fmt::Formatter) -> ::fmt::Result {
        if self.output_settings.show_address {
            self.write_address(f);
        }
        self.write_bytes(f);
        if self.output_settings.show_interpretation {
            self.write_interpretation(f);
        }
        Ok(())
    }
}

mod test {
    use super::*;
    use std::fmt::Write;

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
        let data = [1, 2, 3];
        let output_line = OutputLine::new(&data);
        assert!(true);
    }

    #[test]
    fn default_output_format_for_a_single_line() {
        let data = [1, 2, 3, 4, 5, 6, 7, 8];
        let expected_output = "00000000: 01 02 03 04 05 06 07 08    ........";
        let output_line = OutputLine::new(&data);
        let mut buffer = String::new();
        let result = write!(&mut buffer, "{}", output_line);
        assert_eq!(Ok(()), result);
        assert_eq!(expected_output, buffer);
    }

    #[test]
    fn octal_output_format_on_a_single_line() {
        let data = [8, 9, 10, 11, 12, 13, 14, 15];
        let expected_output = "00000000: 10 11 12 13 14 15 16 17    ........";
        let output_settings = OutputSettings::new().format(OutputFormat::Octal);
        let output_line = OutputLine::new(&data).format(output_settings);
        let mut buffer = String::new();
        let result = write!(&mut buffer, "{}", output_line);
        assert_eq!(Ok(()), result);
        assert_eq!(expected_output, buffer);
    }

    #[test]
    fn binary_output_format_on_a_single_line() {
        let data = [65, 66, 67, 68, 126, 124, 60, 46];
        let expected_output = "00000000: 1000001 1000010 1000011 1000100 1111110 1111100 111100 101110    ABCD~|<.";
        let output_settings = OutputSettings::new().format(OutputFormat::Binary);
        let output_line = OutputLine::new(&data).format(output_settings);
        let mut buffer = String::new();
        let result = write!(&mut buffer, "{}", output_line);
        assert_eq!(Ok(()), result);
        assert_eq!(expected_output, buffer);
    }

    #[test]
    fn interpretation_for_default_settings() {
        let data = [65, 66, 67, 68, 126, 124, 60, 46];
        let expected_output = "00000000: 41 42 43 44 7E 7C 3C 2E    ABCD~|<.";
        let output_settings = OutputSettings::new();
        let output_line = OutputLine::new(&data).format(output_settings);
        let mut buffer = String::new();
        let result = write!(&mut buffer, "{}", output_line);
        assert_eq!(Ok(()), result);
        assert_eq!(expected_output, buffer);
    }
}
