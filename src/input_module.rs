use rug::{Integer, integer::ParseIntegerError, Complete};

fn string_to_number(input: String) -> Result<Integer, ParseIntegerError> {

    let result = Integer::parse(input.as_bytes())?;
    
    Ok(result.complete())

}

fn number_to_string(n: Integer) -> String {


    "5".to_string()
}

#[test]
fn test_string_to_number() {
    let n = Integer::parse(&[1, 2, 3]);

    println!("{}", n.unwrap().complete());
}