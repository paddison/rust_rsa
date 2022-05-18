use std::collections::HashMap;

use super::flags::{ParsedOpt, FlagType, OptDescriptor};

type Result<T> = std::result::Result<T, ParseFlagError>;

struct ParseFlagError;

pub(crate) struct OptParser<'args> {
    args: &'args[String],
    args_index: usize,
    expected: Vec<OptDescriptor>
}

impl<'args> OptParser<'args> {
    pub fn new(args: &'args[String], expected: Vec<OptDescriptor>) -> OptParser {
        OptParser { args, args_index: 0, expected }
    }

    fn get_opt(&self) -> Option<ParsedOpt> {
        // loop through args and search for flags that are contained in expected
        // if found, try to parse it according to FlagType
        // if an invalid flag is given, return an Error
        // let mut flags = vec![];
        for (i, arg) in self.args.iter().enumerate() {
            if ParsedOpt::is_flag(arg) {
                    ParsedOpt { name: "a".to_string(), args: None };
                } 
        }
        
        Some(ParsedOpt { name: "a".to_string(), args: None })
    }
}

impl<'args> Iterator for OptParser<'args> {
    type Item = ParsedOpt;

    fn next(&mut self) -> Option<Self::Item> {
        self.get_opt()
    }
}