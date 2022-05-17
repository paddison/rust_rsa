pub struct Flag {
    name: String,
    f_type: FlagType,
    args: Option<Vec<String>>,
}

impl Flag {
    // checks if the argument is a flag or not (flags will always have the form of "-[a-zA-Z]")
    pub fn is_flag(flag: &str) -> bool {
        let match_alphabetic = |s: char| -> bool {
            s.is_ascii_alphabetic()
        };
        flag.len() == 2 && flag.starts_with("-") && flag.ends_with(match_alphabetic)
    }
}

pub(crate) enum FlagType {
    NoArg,
    SingleArg,
    MultiArg,
}