use std::string::FromUtf8Error;
use rug::Integer;

pub fn string_to_number(input: String) -> Integer {
    let mut result = Integer::new();

    for (i, byte) in input.bytes().into_iter().enumerate() {
        let mut new_int = Integer::from(byte);
        new_int <<= i * 8;
        result += new_int;
    }

    result

}

pub fn number_to_string(n: Integer) -> Result<String, FromUtf8Error>  {
    let ptr = n.as_raw();
    let mut raw_string = String::new();

    // SAFETY: Accessing the pointer is safe, since n will be a valid integer,
    // and the pointer only accesses memory in mpz.size, which must be valid
    unsafe {
        let mpz = *ptr;
        for i in 0..mpz.size {
            let part = *mpz.d.as_ptr().add(i as usize);
            raw_string += &String::from_utf8(part.to_le_bytes().to_vec())?;
        }
    }

    // remove trailing zeroes on end of string
    Ok(raw_string.trim_end_matches(char::from(0)).to_string())
}

#[test]
fn test_string_to_number_number_to_string() {
    let string = "Sed ut perspiciatis unde omnis iste natus error sit voluptatem accusantium doloremque laudantium, totam rem aperiam, eaque ipsa quae ab illo inventore veritatis et quasi architecto beatae vitae dicta sunt explicabo. Nemo enim ipsam voluptatem quia voluptas sit aspernatur aut odit aut fugit, sed quia consequuntur magni dolores eos qui ratione voluptatem sequi nesciunt. Neque porro quisquam est, qui dolorem ipsum quia dolor sit amet, consectetur, adipisci velit, sed quia non numquam eius modi tempora incidunt ut labore et dolore magnam aliquam quaerat voluptatem. Ut enim ad minima veniam, quis nostrum exercitationem ullam corporis suscipit laboriosam, nisi ut aliquid ex ea commodi consequatur? Quis autem vel eum iure reprehenderit qui in ea voluptate velit esse quam nihil molestiae consequatur, vel illum qui dolorem eum fugiat quo voluptas nulla pariatur?".to_string();
    let n = string_to_number(string);
    let result = number_to_string(n);
    assert_eq!(result.unwrap(), "Sed ut perspiciatis unde omnis iste natus error sit voluptatem accusantium doloremque laudantium, totam rem aperiam, eaque ipsa quae ab illo inventore veritatis et quasi architecto beatae vitae dicta sunt explicabo. Nemo enim ipsam voluptatem quia voluptas sit aspernatur aut odit aut fugit, sed quia consequuntur magni dolores eos qui ratione voluptatem sequi nesciunt. Neque porro quisquam est, qui dolorem ipsum quia dolor sit amet, consectetur, adipisci velit, sed quia non numquam eius modi tempora incidunt ut labore et dolore magnam aliquam quaerat voluptatem. Ut enim ad minima veniam, quis nostrum exercitationem ullam corporis suscipit laboriosam, nisi ut aliquid ex ea commodi consequatur? Quis autem vel eum iure reprehenderit qui in ea voluptate velit esse quam nihil molestiae consequatur, vel illum qui dolorem eum fugiat quo voluptas nulla pariatur?".to_string());
}