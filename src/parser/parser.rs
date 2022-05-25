
use super::opt::{ParsedOpt, FlagType, OptDescriptor, self};

type Result<T> = std::result::Result<T, ParseFlagError>;

pub struct ParseFlagError;

pub(crate) struct OptParser<'args> {
    args: &'args[String],
    args_index: usize,
    expected: Vec<OptDescriptor>
}

impl<'args> OptParser<'args> {
    pub fn new(args: &'args[String], expected: Vec<OptDescriptor>) -> OptParser {
        OptParser { args, args_index: 0, expected }
    }

    fn get_opt(&mut self) -> Option<Result<ParsedOpt>> {
        // loop through args and search for flags that are contained in expected
        // if found, try to parse it according to FlagType
        // if an invalid flag is given, return an Error
        // let mut flags = vec![];
        if let None = self.args.get(self.args_index) {
            return None;
        }
        let arg = self.args.get(self.args_index).unwrap();
        // check if arg is a flag contained in expected
        // if not, throw exception
        // if it is, check type and try to collect arguments

        // arg is no option, so return Ok with None
        if !opt::is_opt(arg) {
            return None;
        }

        // if arg is a long option
        let found_opt = if let Some(opt) = self.is_expected_long(&arg) { opt }
        else if let Some(opt) = self.is_expected_short(&arg) { opt }
        else { return Some(Err(ParseFlagError)); };

        let mut c = 1;
        let args = match found_opt.get_f_type() {
            FlagType::NoArg => {
                None
            },
            FlagType::SingleArg(is_optional) => {
                match self.args.get(self.args_index + c) {
                    Some(arg) => {
                        if !is_optional && opt::is_opt(arg) {
                            return Some(Err(ParseFlagError));
                        } 
                        if opt::is_opt(arg) {
                            None
                        } else {
                            c += 1;
                            Some(vec![arg.to_string()])
                        }
                    },
                    None => if is_optional { None } else { return Some(Err(ParseFlagError)); }
                }
            },
            FlagType::MultiArg(is_optional) => {
                let mut found_args = vec![];
                // let c = 0;
                while let Some(arg) = self.args.get(self.args_index + c) {
                    if !opt::is_opt(arg) {
                        found_args.push(arg.to_string());
                        c += 1;
                    } else {
                        break;
                    }
                }
                if found_args.len() == 0 && !is_optional {
                    return Some(Err(ParseFlagError));
                } else if found_args.len() == 0 {
                    None
                } else {
                    Some(found_args)
                }
            },
        };
        let result = Some(Ok(ParsedOpt::new(found_opt.get_name(), found_opt.get_f_type(), args )));
        self.args_index += c;
        result
    }

    fn is_expected_long(&self, arg: &str) -> Option<&OptDescriptor> {
        if !arg.starts_with("--") {
            return None;
        }
        for opt in &self.expected {
            if opt.contains_long(&arg[2..]) {
                return Some(opt);
            } 
        }
        None
    }

    fn is_expected_short(&self, arg: &str) -> Option<&OptDescriptor> {
        if !arg.starts_with("-") {
            return None;
        }
        for opt in &self.expected {
            if opt.contains_short(&arg[1..]) {
                return Some(opt);
            }
        }
        None
    }
}

impl<'args> Iterator for OptParser<'args> {
    type Item = Result<ParsedOpt>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.args_index >= self.args.len() {
            None
        } else {
            self.get_opt()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::opt::OptDescriptor;
    use crate::parser::opt::FlagType;
    use super::OptParser;

    // parser tests
    #[test]
    fn test_no_args() {
        let expected = vec![
            OptDescriptor::new( "a".to_string(), "aaa".to_string(), FlagType::NoArg),
            OptDescriptor::new("b".to_string(), "bbb".to_string(), FlagType::NoArg)];
        let args = vec!["-a".to_string(), "--bbb".to_string(), "-c".to_string()];
        let mut parser = OptParser::new(&args, expected);
        let opt1 = parser.next();
        assert!(opt1.is_some());
        let opt2 = parser.next();
        assert!(opt2.is_some());
        let opt3 = parser.next();
        assert!(opt3.is_some());
        assert!(opt3.unwrap().is_err());
    }
}