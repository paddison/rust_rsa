use crate::commands::util::*;

type Result<T> = std::result::Result<T, ParseCommandError>;

// Generate a key pair
// generate [OPTIONS]
// flags:
// -b [n]
// specify length n of key in bits, only powers of 2 permitted
// if empty, generate 2k key
// -f [file_name]
// save results to file,
// if [file_name] is empty, a default name with the date and time is created
// -h
// show help for this command
pub struct GenerateConfig {
    length: u16,
    file: Option<String>,
}

impl GenerateConfig {
    pub fn new(args: &[String]) -> Result<Self> {
        let mut length = 2048;
        let mut file = None;

        for (i, arg) in args.iter().enumerate() {
            if is_flag(arg) {
                match arg.as_str() {
                    "-b" => length = Self::parse_key_length(&args[i..])?,
                    "-f" => file = Some(Self::parse_file_name(&args[i..])),
                    "-h" => {
                        let err_type = ErrorType::HelpFlag(Self::get_help_message());
                        return Err(ParseCommandError::from(err_type));
                    }
                    invalid_flag => { 
                        let err_type = ErrorType::InvalidFlag(Self::get_error_message(invalid_flag, "generate"));
                        return Err(ParseCommandError::from(err_type));
                    }
                }
            }
        } 
        Ok(GenerateConfig { length, file })
    }

    #[inline(always)]
    fn parse_key_length(args: &[String]) -> Result<u16> {
        if let None = args.get(1) {
            return Err(ParseCommandError::from(ErrorType::InvalidBitSize("No key length specified.".to_string())));
        }
        if let Ok(n) = args[1].parse::<u16>() {
            if Self::is_valid_bit_size(n) {
                return Ok(n);
            }
        }
        println!("Not a number or invalid key length.\nNeeds to be power of two and in range 512 to 8192.");
        Err(ParseCommandError::from(ErrorType::InvalidBitSize("Not a number or invalid key length.\nNeeds to be power of two and in range 512 to 8192.".to_string())))
    }
}

impl Configuration for GenerateConfig {
    fn get_help_message() -> String {
        todo!()
    }
}