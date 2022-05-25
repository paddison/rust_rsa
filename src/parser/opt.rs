#[derive(Debug)]
pub(crate) struct ParsedOpt {
    name: String,
    f_type: FlagType,
    args: Option<Vec<String>>,
}

impl ParsedOpt {
    pub fn new(name: String, f_type: FlagType, args: Option<Vec<String>>) -> Self {
        ParsedOpt{ name, f_type, args }
    }

    // checks if the argument is a flag or not (flags will always have the form of "-[a-zA-Z]")
    pub fn is_flag(flag: &str) -> bool {
        let match_alphabetic = |s: char| -> bool {
            s.is_ascii_alphabetic()
        };
        flag.len() == 2 && flag.starts_with("-") && flag.ends_with(match_alphabetic)
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum FlagType {
    NoArg,
    SingleArg(bool), // boolean indicating if arg is optional
    MultiArg(bool),
}

pub(crate) struct OptDescriptor {
    short: String,
    long: String,
    f_type: FlagType,
}

impl OptDescriptor {

    pub fn new(short: String, long: String, f_type: FlagType) -> Self {
        Self {short, long, f_type }
    }

    #[inline(always)]
    pub fn contains_short(&self, other: &str) -> bool {
        &self.short == other
    }

    #[inline(always)]
    pub fn contains_long(&self, other: &str) -> bool {
        &self.long == other
    }

    #[inline(always)]
    pub fn get_f_type(&self) -> FlagType {
        self.f_type
    }

    #[inline(always)]
    pub fn get_name(&self) -> String {
        String::from(&self.short)
    }
}

#[inline(always)]
pub fn is_opt(arg: &str) -> bool {
    arg.starts_with("--") || arg.starts_with("-")
}