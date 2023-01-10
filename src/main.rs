use std::fs::File;
use std::io::{stdin, Write, Read};

// Encode to furry bytes
fn encode(message: &String) -> Vec<String> {
    let binary = message.as_bytes().iter().map(|x| format!("{:08b}", x)).collect::<Vec<String>>();

    binary.iter().map(
        |character|  {
            let mut char = String::new();
            character.chars().for_each(
                |x| {
                    char.extend((match x {
                        '0' => "woof",
                        '1' => "bark",
                        _ => "ERROR!"
                    }.to_owned() + " ").chars());
                }
            );
            char
        }
    ).collect::<Vec<String>>()
}

// Decode from furry bytes
fn decode(message: Option<&String>) -> String {
    let mut input = String::new();
    let mut file = File::open(DECODE_FILE_PATH).unwrap();

    match message {
        Some(m) => input = m.clone(),
        None => { file.read_to_string(&mut input).unwrap(); },
    }

    let binary = input.split_whitespace().map(
        |word| {
            match word {
                "woof" => "0",
                "bark" => "1",
                _ => "ERROR!"
            }
        }
    ).collect::<Vec<_>>();

    let mut message = String::new();
    for i in 0..binary.len() / 8 {
        let mut byte = String::new();
        for j in 0..8 {
            byte.push_str(binary[i * 8 + j]);
        }
        message.push_str(&String::from_utf8(vec![u8::from_str_radix(&byte, 2).unwrap()]).unwrap());
    }

    message
}

const ENCODE_FILE_PATH: &str = "C:\\Users\\Frytak\\Desktop\\~\\Important\\Programming Projects\\WoofBarkTranslator\\src\\woof.txt";
const DECODE_FILE_PATH: &str = "C:\\Users\\Frytak\\Desktop\\~\\Important\\Programming Projects\\WoofBarkTranslator\\src\\bark.txt";
const PREVIEW_FILE_MAX_LEN: usize = 5;
const ENCODE_WITH_TRANSLATION: bool = true;

fn main() {
    let mut input = String::new();

    // Clear the console
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

    loop {
        println!("Choose an action:");
        println!("   1. Encode");
        println!("   2. Decode");
        println!("   3. Exit");
        stdin().read_line(&mut input).unwrap();
        print!("\n");

        match input.trim() {
            "1" => { encode_loop() },
            "2" => { decode_loop() },
            "3" => { break; },
            _ => { println!("Invalid input!"); input.clear(); continue; },
        }

        input.clear();
    }
}

fn encode_loop() {
    let mut input = String::new();
    let mut file = File::create(ENCODE_FILE_PATH).unwrap();

    // Get the message to encode
    println!("Enter a message to encode:");
    stdin().read_line(&mut input).unwrap();
    print!("\n");

    // Trim the input and encode it
    input = input.trim().to_owned();
    let chars = encode(&input);

    // Display and write to "woof.txt"
    println!("Encoded message:");
    for (i, char) in input.chars().enumerate() {
        let translation;
        if ENCODE_WITH_TRANSLATION { translation = char.to_string() + "=>" + &chars[i as usize]; }
        else { translation = chars[i as usize].clone(); }

        file.write_all((translation.clone() + "\n").as_bytes()).unwrap();
        
        if i < PREVIEW_FILE_MAX_LEN { println!("{}", translation) }
        else if i == PREVIEW_FILE_MAX_LEN { println!("..."); }
    }

    print!("\n\n\n");
}


fn decode_loop() {
    let mut input = String::new();
    let decoded;

    println!("Enter a message to decode or leave empty to read from \"bark.txt\":");
    stdin().read_line(&mut input).unwrap();
    print!("\n");

    input = input.trim().to_owned();
    if input == "" { decoded = decode(None); }
    else { decoded = decode(Some(&input)); }

    println!("Decoded message:");
    println!("'{}'", decoded);

    print!("\n\n\n");
}