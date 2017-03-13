//! The dump module contains code related for outputing/dumping data.
use std::fmt::Display;

/// Enum which provides all possible output value formats supported by the dump module.
#[derive(Debug)]
pub enum OutputFormat {
    Hex,
    Decimal,
    Octal,
}

/// The OutputLine struct contains all  information needed to dump/output a single line of data.
#[derive(Debug)]
pub struct OutputLine<'a> {
    start_address: u32,
    data: &'a [u8], // TODO NiCo: Add member for data format (output format of data)
    group_size: usize,
    columns: usize,
    output_fmt: OutputFormat,
}

impl<'a> OutputLine<'a> {
    fn new(data: &[u8]) -> OutputLine {
        OutputLine {
            start_address: 0,
            data: data,
            group_size: 1,
            columns: 8,
            output_fmt: OutputFormat::Hex,
        }
    }

    pub fn format(self, fmt: OutputFormat) -> Self {
        OutputLine {
            start_address: self.start_address,
            data: self.data,
            group_size: self.group_size,
            columns: self.columns,
            output_fmt: fmt,
        }
    }

    fn write_formated_byte(&self, f: &mut ::fmt::Formatter, byte: &u8) -> ::fmt::Result {
        match self.output_fmt {
            OutputFormat::Hex => write!(f, "{:02.X}", byte),
            OutputFormat::Octal => write!(f, "{:02.o}", byte),
            OutputFormat::Decimal => write!(f, "{:02}", byte),
        }
    }
}

impl<'a> ::fmt::Display for OutputLine<'a> {
    fn fmt(&self, f: &mut ::fmt::Formatter) -> ::fmt::Result {
        write!(f, "{:08.X}: ", self.start_address);
        let mut byte_count = 0;
        for b in self.data.iter() {
            byte_count += 1;
            let is_seperator_necessary = byte_count % self.group_size == 0;
            if is_seperator_necessary {
                self.write_formated_byte(f, b);
                write!(f, " ");
            } else {
                self.write_formated_byte(f, b);
            }
        }
        write!(f, "   ........");
        Ok(())
    }
}

mod test {
    use super::*;
    use std::fmt::Write;

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
        let output_line = OutputLine::new(&data).format(OutputFormat::Octal);
        let mut buffer = String::new();
        let result = write!(&mut buffer, "{}", output_line);
        assert_eq!(Ok(()), result);
        assert_eq!(expected_output, buffer);
    }
}
