use nom::{
    call, do_parse, error_position, flat_map, many0, map_res, named, sep, take, take_until,
    take_until_and_consume, wrap_sep, ws,
};
use std::num::ParseIntError;
use std::str;

fn hex_string_to_u8(s: &str) -> Result<u8, ParseIntError> {
    u8::from_str_radix(s, 16)
}

named!(address<&str, u64>,
    do_parse!(
        adr : map_res!(
                take_until!(":"),
                str::FromStr::from_str
            ) >>
        take!(1) >>
        (adr)
    )
);

named!(pub bytes<&str, Vec<u8> >,
    many0!(
        ws!(
            map_res!(
                take!(2),
                hex_string_to_u8
            )
        )
    )
);

#[derive(Debug, PartialEq)]
struct Line {
    address: u64,
    data: Vec<u8>,
}

named!(hexdum_line<&str, Line>,
    do_parse!(
        adr: address >>
        data: flat_map!(take_until_and_consume!("  "), bytes) >>
        take_until_and_consume!("\n") >>
        (Line { address: adr, data } )
    )
);

#[cfg(test)]
mod test {

    use super::*;
    use nom::IResult;

    #[test]
    fn address_parser() {
        {
            let result = address("00112233: some additional garbage");
            assert_eq!(IResult::Done(" some additional garbage", 112233), result);
        }
    }

    #[test]
    fn bytes_parser() {
        {
            let result = bytes("AABBCCEE");
            assert_eq!(IResult::Done("", vec![0xAA, 0xBB, 0xCC, 0xEE]), result);
        }
        {
            let result = bytes(" AA BB CC EE");
            assert_eq!(IResult::Done("", vec![0xAA, 0xBB, 0xCC, 0xEE]), result);
        }
        {
            let result = bytes(" AA BB CC EE\n BB DD");
            assert_eq!(
                IResult::Done("", vec![0xAA, 0xBB, 0xCC, 0xEE, 0xBB, 0xDD]),
                result
            );
        }
        {
            let result = bytes(" AA BB CC EE xxx  BB DD");
            assert_eq!(
                IResult::Done("xxx  BB DD", vec![0xAA, 0xBB, 0xCC, 0xEE]),
                result
            )
        }
    }

    #[test]
    fn hexdump_line_parser() {
        {
            let expected_result = IResult::Done(
                "",
                Line {
                    address: 112233,
                    data: vec![0xAA, 0xBB, 0xCC, 0xEE],
                },
            );
            let result = hexdum_line("00112233: AA BB CC EE   ....\n");
            assert_eq!(expected_result, result);
        }
    }
}
