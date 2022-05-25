
use super::opt::{ParsedOpt, FlagType, OptDescriptor, self};

type Result<T> = std::result::Result<T, ParseFlagError>;

#[derive(Debug)]
pub enum ParseFlagError {
    ArgRequired(String),
    InvalidOpt(String),
}

pub(crate) struct OptParser<'args> {
    args: &'args[String],
    args_index: usize,
    expected: Vec<OptDescriptor>
}

impl<'args> OptParser<'args> {
    pub fn new(args: &'args[String], expected: Vec<OptDescriptor>) -> OptParser {
        OptParser { args, args_index: 0, expected }
    }

    /// Get's the next opt. 
    /// If opt is not in expected, return Some(Err(ParseFlagError)),
    /// if args_index >= args.len() return None, 
    /// else return Some(Ok(ParsedOpt))
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
        else { return Some(Err(ParseFlagError::InvalidOpt(arg.to_string()))); };

        // match by FlagType, to determine additional options
        let args = match found_opt.get_f_type() {
            FlagType::NoArg => {
                None
            },
            FlagType::SingleArg(is_optional) => {
                match self.args.get(self.args_index + 1) {
                    Some(arg) => {
                        // if additional option is required, but not given, return Error
                        if !is_optional && opt::is_opt(arg) {
                            return Some(Err(ParseFlagError::ArgRequired(arg.to_string())));
                        } 
                        if opt::is_opt(arg) {
                            None
                        } else {
                            Some(vec![arg.to_string()])
                        }
                    },
                    None => if is_optional { None } else { return Some(Err(ParseFlagError::ArgRequired(arg.to_string()))); }
                }
            },
            FlagType::MultiArg(is_optional) => {
                let mut found_args = vec![];
                let mut c = 1;
                // collect additional options
                while let Some(arg) = self.args.get(self.args_index + c) {
                    if !opt::is_opt(arg) {
                        found_args.push(arg.to_string());
                        c += 1;
                    } else {
                        break;
                    }
                }
                // if additional options are required, but not found, return Err
                if found_args.len() == 0 && !is_optional {
                    return Some(Err(ParseFlagError::ArgRequired(arg.to_string())));
                } else if found_args.len() == 0 {
                    None
                } else {
                    Some(found_args)
                }
            },
        };
        let result = Some(Ok(ParsedOpt::new(found_opt.get_name(), found_opt.get_f_type(), args )));
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

    // reposition the parser to point to the next flag
    fn reposition(&mut self) {
        let mut i = 1;
        while let Some(arg) = self.args.get(self.args_index + i) {
            if opt::is_opt(arg) {
                self.args_index += i;
                return;
            }
            i += 1;
        }
        self.args_index += i;
    }
}

impl<'args> Iterator for OptParser<'args> {
    type Item = Result<ParsedOpt>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.args_index >= self.args.len() {
            None
        } else {
            let opt = self.get_opt();
            self.reposition();
            opt
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::opt;
    use crate::parser::opt::OptDescriptor;
    use crate::parser::opt::FlagType;
    use super::OptParser;

    // parser tests
    #[test]
    fn test_reposition() {
        let args: Vec<String> = (vec!["-a", "blub", "-b", "-c", "1", "2", "3", "-d", "bla", "bla"])
            .into_iter().map(|s| s.to_string())
            .collect();

        let mut parser = OptParser::new(&args, vec![]);
        assert_eq!(parser.args_index, 0);
        parser.reposition();
        assert_eq!(parser.args_index, 2);
        parser.reposition();
        assert_eq!(parser.args_index, 3);
        parser.reposition();
        assert_eq!(parser.args_index, 7);
        parser.reposition();
        assert!(parser.next().is_none())
    }

    #[test]
    fn test_no_args() {
        let expected = vec![
            opt!("a", "aaa", FlagType::NoArg),
            opt!("b", "bbb", FlagType::NoArg)];
        let args = vec!["-a".to_string(), "--bbb".to_string(), "-c".to_string()];
        let mut parser = OptParser::new(&args, expected);
        let opt1 = parser.next();
        assert!(opt1.is_some());
        let opt2 = parser.next();
        assert!(opt2.is_some());
        let opt3 = parser.next();
        assert!(opt3.is_some());
        assert!(opt3.unwrap().is_err());
        let opt4 = parser.next();
        assert!(opt4.is_none());
    }

    #[test]
    fn test_one_arg_valid() {
        let expected = vec![
            opt!("a", "aaa", FlagType::SingleArg(true)),
            opt!("b", "bbb", FlagType::SingleArg(false)),
        ];
        let args = (vec!["--aaa", "-b", "im_an_opt"]).into_iter().map(|s| s.to_string()).collect::<Vec<String>>();
        let mut parser = OptParser::new(&args, expected);

        let opt = parser.next();
        assert!(opt.is_some());
        let opt = opt.unwrap();
        assert!(opt.is_ok());
        assert!(opt.unwrap().verify("aaa", None));

        let opt = parser.next();
        assert!(opt.is_some());
        let opt = opt.unwrap();
        assert!(opt.is_ok());
        assert!(opt.unwrap().verify("bbb", Some(vec!["im_an_opt".to_string()])));

        let opt = parser.next();
        assert!(opt.is_none());
    }

    #[test]
    fn test_one_arg_invalid() {
        let expected = vec![
            opt!("a", "aaa", FlagType::SingleArg(false)),
            opt!("b", "bbb", FlagType::SingleArg(false)),
            opt!("c", "ccc", FlagType::SingleArg(true)),
        ];
        let args = vec!["-a".to_string(), "opt_a".to_string(), "-b".to_string(), "--ccc".to_string()];
        let mut parser = OptParser::new(&args, expected);

        let opt = parser.next();
        assert!(opt.is_some());
        let opt = opt.unwrap();
        assert!(opt.is_ok());
        assert!(opt.unwrap().verify("aaa", Some(vec!["opt_a".to_string()])));

        let opt = parser.next();
        assert!(opt.is_some());
        let opt = opt.unwrap();
        assert!(opt.is_err());

        let opt = parser.next();
        assert!(opt.is_some());
        let opt = opt.unwrap();
        assert!(opt.is_ok());
        assert!(opt.unwrap().verify("ccc", None));

        let opt = parser.next();
        assert!(opt.is_none());
    }

    #[test]
    fn test_multi_arg_valid() {
        let expected = vec![
            opt!("a", "aaa", FlagType::MultiArg(true)),
            opt!("b", "bbb", FlagType::MultiArg(true)),
            opt!("c", "ccc", FlagType::MultiArg(false)),
        ];
        let args = (vec!["-a", "--bbb", "one", "two", "-c", "one", "two", "three"])
            .into_iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let mut parser = OptParser::new(&args, expected);

        let opt = parser.next();
        assert!(opt.is_some());
        let opt = opt.unwrap();
        assert!(opt.is_ok());
        assert!(opt.unwrap().verify("aaa", None));

        let opt = parser.next();
        assert!(opt.is_some());
        let opt = opt.unwrap();
        assert!(opt.is_ok());
        assert!(opt.unwrap().verify("bbb", Some(vec!["one".to_string(), "two".to_string()])));

        let opt = parser.next();
        assert!(opt.is_some());
        let opt = opt.unwrap();
        assert!(opt.is_ok());
        assert!(opt.unwrap().verify("ccc", Some(vec!["one".to_string(), "two".to_string(), "three".to_string()])));

        let opt = parser.next();
        assert!(opt.is_none());
    }

    #[test]
    fn test_multi_arg_invalid() {
        let expected = vec![
            opt!("a", "aaa", FlagType::MultiArg(true)),
            opt!("b", "bbb", FlagType::MultiArg(false)),
            opt!("c", "ccc", FlagType::MultiArg(false)),
        ];
        let args = (vec!["-a", "one", "two", "--bbb",  "-c", "one", "two", "three"])
            .into_iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        let mut parser = OptParser::new(&args, expected);

        let opt = parser.next();
        assert!(opt.is_some());
        let opt = opt.unwrap();
        assert!(opt.is_ok());
        assert!(opt.unwrap().verify("aaa", Some(vec!["one".to_string(), "two".to_string()])));

        let opt = parser.next();
        assert!(opt.is_some());
        let opt = opt.unwrap();
        assert!(opt.is_err());

        let opt = parser.next();
        assert!(opt.is_some());
        let opt = opt.unwrap();
        assert!(opt.is_ok());
        assert!(opt.unwrap().verify("ccc", Some(vec!["one".to_string(), "two".to_string(), "three".to_string()])));

        let opt = parser.next();
        assert!(opt.is_none());
    }
}