
use super::opt::{ParsedOpt, FlagType, OptDescriptor, self};

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

    fn get_opt(&self) -> Result<Option<ParsedOpt>> {
        // loop through args and search for flags that are contained in expected
        // if found, try to parse it according to FlagType
        // if an invalid flag is given, return an Error
        // let mut flags = vec![];
        if let Some(arg) = self.args.get(self.args_index) {
            // check if arg is a flag contained in expected
            // if not, throw exception
            // if it is, check type and try to collect arguments

            // if arg is a long option
            let found_opt = if arg.starts_with("--") { 
                self.is_expected(&arg, true)? 
            } else if arg.starts_with("-") {
                self.is_expected(&arg, false)?
            // arg is no option, so return Ok with None
            } else {
                return Ok(None);
            };

            match found_opt.get_f_type() {
                FlagType::NoArg => {
                    self.args_index += 1;
                    Ok(Some(ParsedOpt {
                        name: found_opt.get_name(),
                        f_type: *found_opt.get_f_type(),
                        args: None,
                    }))
                },
                FlagType::SingleArg(is_optional) => {
                    let mut args = None;
                    self.args_index += 1;
                    if let Some(arg) = self.args.get(self.args_index) {
                        if !opt::is_opt(arg) {
                            args = Some(vec![arg.to_string()]);
                            self.args_index += 1;
                        } 
                    }
                    Ok(Some(ParsedOpt {
                        name: found_opt.get_name(),
                        f_type: *found_opt.get_f_type(),
                        args,
                    }))
                },
                FlagType::MultiArg(is_optional) => {
                    let mut found_args = vec![];
                    self.args_index += 1;
                    let c = 0;
                    while let Some(arg) = self.args.get(self.args_index) {
                        if !opt::is_opt(arg) {
                            found_args.push(arg.to_string());
                            self.args_index += 1;
                        } else {
                            break;
                        }
                    }
                    Ok(Some(ParsedOpt {
                        name: found_opt.get_name(),
                        f_type: *found_opt.get_f_type(),
                        args: if found_args.len() == 0 { None } else { Some(found_args) }
                    }))
                },
            }
        } else {
            Ok(None)
        } 
    }

    fn is_expected(&self, arg: &str, is_long: bool) -> Result<&OptDescriptor> {
        for opt in &self.expected {
            if is_long && opt.contains_long(&arg[2..]) {
                return Ok(opt);
            } else if opt.contains_short(&arg[1..]) {
                return Ok(opt);
            }
        }
        Err(ParseFlagError)
    }

}

// impl<'args> Iterator for OptParser<'args> {
//     type Item = ParsedOpt;

//     fn next(&mut self) -> Option<Self::Item> {
//         self.get_opt()
//     }
// }