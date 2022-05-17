struct Flag {
    name: String,
    f_type: FlagType,
    args: Vec<String>,
}

impl Flag {
    fn is_type(&self, other: FlagType) -> bool {
        self.f_type == other
    }
}

enum FlagType {
    NoArg,
    SingleArg,
    MultiArg,
}

impl PartialEq for FlagType {
    fn eq(&self, other: &Self) -> bool {
        core::mem::discriminant(self) == core::mem::discriminant(other)
    }
}