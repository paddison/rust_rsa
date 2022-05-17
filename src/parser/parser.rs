use std::collections::HashMap;

use super::flags::{Flag, FlagType};

type Result<T> = std::result::Result<T, ParseFlagError>;

struct ParseFlagError;

pub fn get_flags(args: &[String], expected: HashMap<&str, FlagType>) -> Result<Vec<Flag>> {
    // loop through args and search for flags that are contained in expected
    // if found, try to parse it according to FlagType
    // if an invalid flag is given, return an Error
    let mut flags = vec![];
    for (i, arg) in args.iter().enumerate() {
        if Flag::is_flag(arg) {
            if let Some(f_type) = expected.get(arg.as_str()) {
                flags.push(parse_flag(arg, f_type, &args[i..])?);
            } else {
                return Err(ParseFlagError);
            }
        }
    }
    Ok(flags)
}

fn parse_flag(name: &str, f_type: &FlagType, args: &[String]) -> Result<Flag>  {
    match f_type {
        FlagType::NoArg => {
            if Flag::is_flag(&args[1]) {
                Ok(Flag { name: name.to_string(), f_type: *f_type, args: None })
            } else {
                Err(ParseFlagError)
            }
        },// assert next item is a flag
        FlagType::SingleArg => {
            if !Flag::is_flag(

            )
            Err(ParseFlagError)},
        FlagType::MultiArg => Err(ParseFlagError),
    }
}