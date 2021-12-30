use std::io;

#[derive(Debug)]
enum CustomChar {
    Digit(u8),
    Sign(String)
}

fn main() {
    loop {
        let mut buffer = String::new();
        let stdin = io::stdin();

        stdin.read_line(&mut buffer).unwrap();

        if buffer.contains("end") {
            println!("this is the end!");
            break
        }

        for v in buffer.chars() {
            let x = match analyze(v) {
                Some(CustomChar) => CustomChar,
                None => continue,
            };

            println!("value: {:?}", x);
        }
    }
}

fn analyze(v: char) -> Option<CustomChar> {
    match v {
        '0' => Some(CustomChar::Digit(0)),
        '1' => Some(CustomChar::Digit(1)),
        '2' => Some(CustomChar::Digit(2)),
        '3' => Some(CustomChar::Digit(3)),
        '4' => Some(CustomChar::Digit(4)),
        '5' => Some(CustomChar::Digit(5)),
        '6' => Some(CustomChar::Digit(6)),
        '7' => Some(CustomChar::Digit(7)),
        '8' => Some(CustomChar::Digit(8)),
        '9' => Some(CustomChar::Digit(9)),

        '+' => Some(CustomChar::Sign("+".to_string())),
        '-' => Some(CustomChar::Sign("-".to_string())),

        _ => None
    }
}

