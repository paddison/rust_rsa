use std::string::FromUtf8Error;
use rug::{Integer, Complete};

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
    let string = "Sed ut perspiciatis unde omnis iste natus error sit voluptatem accusantium doloremque laudantium, totam rem aperiam, eaque ipsa quae ab illo inventore veritatis et quasi architecto beatae vitae dicta sunt explicabo. Nemo enim ipsam voluptatem quia voluptas sit aspernatur aut odit aut fugit, sed quia consequuntur magni dolores eos qui ratione voluptatem sequi nesciunt. Neque porro quisquam est, qui dolorem ipsum quia dolor sit amet, consectetur, adipisci velit, sed quia non numquam eius modi tempora incidunt ut labore et dolore magnam aliquam quaerat voluptatem. Ut enim ad minima veniam, quis nostrum exercitationem ullam corporis suscipit laboriosam, nisi ut aliquid ex ea commodi consequatur? Quis autem vel eum iure reprehenderit qui in ea voluptate velit esse quam nihil molestiae consequatur, vel illum qui dolorem eum fugiat quo voluptas nulla pariatur?\r\n".to_string();
    let n = string_to_number(string);
    let result = number_to_string(n);
    assert_eq!(result.unwrap(), "Sed ut perspiciatis unde omnis iste natus error sit voluptatem accusantium doloremque laudantium, totam rem aperiam, eaque ipsa quae ab illo inventore veritatis et quasi architecto beatae vitae dicta sunt explicabo. Nemo enim ipsam voluptatem quia voluptas sit aspernatur aut odit aut fugit, sed quia consequuntur magni dolores eos qui ratione voluptatem sequi nesciunt. Neque porro quisquam est, qui dolorem ipsum quia dolor sit amet, consectetur, adipisci velit, sed quia non numquam eius modi tempora incidunt ut labore et dolore magnam aliquam quaerat voluptatem. Ut enim ad minima veniam, quis nostrum exercitationem ullam corporis suscipit laboriosam, nisi ut aliquid ex ea commodi consequatur? Quis autem vel eum iure reprehenderit qui in ea voluptate velit esse quam nihil molestiae consequatur, vel illum qui dolorem eum fugiat quo voluptas nulla pariatur?\r\n".to_string());
}

#[test]
fn test_number_to_string() {
    let n = Integer::parse("559596567373260415231431918234957998553125852963231188497621056000865455009738048161554988395927597815452214700613997393991051478379170771083163606778095596823558945098806732031726375335294661881617589673589132555051328177652126079299712161538706163910755777070219435411431067885618379065390730923337123153705639708387550833092274827126038398104004850630955368137076949249874815952632953873885977346909813255690711300968329636864606150077697623163616627385829428491793589220976501555822363868535595916211967774307347919772431756717543436626465638326233078396135919574814990397950007989612783407718995467707994445187869916244248666126769238657743195387000849253563320179631559836525137637201807709341767571605508489754593346136294900125735400681826657382486875737944243680102317425873302295323835337417894205117657386576240924469875117116308434566368433184309013224745323823079459339641492947903318188641980213782450025634712848405747042000839946379088442093215128760189159575930410170698359815415185185290546753813878638687887558711992620248363655350839224807396344914707274513691721734325680315537376956868750207776691238733560913542495405253778387434004112907982177087285357604733822269430764752518716596829949010303479068297181761");
    let n = n.unwrap().complete();
    println!("{:?}", n); 
    println!("{:?}", number_to_string(n));
}