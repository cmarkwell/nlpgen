use rand::Rng;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);

    let (adjectives, num_adjs) = parse_bytes(include_bytes!("resources/adjectives.txt"));
    let (nouns, num_nouns) = parse_bytes(include_bytes!("resources/nouns.txt"));

    let mut rng = rand::thread_rng();

    for _ in 0..config.count {
        let mut nlp = String::new();
        for _ in 0..config.length {
            let dice_adj = rng.gen_range(0..num_adjs) as usize;
            let dice_noun = rng.gen_range(0..num_nouns) as usize;
            nlp.push_str(&format!("{} {} ", adjectives[dice_adj], nouns[dice_noun]));
        }
        println!("{}", nlp);
    }
}

struct Config {
    length: i32,
    count: i32,
}

impl Config {
    fn new(args: &[String]) -> Config {
        let length = args[1].clone().parse().unwrap();
        let count = args[2].clone().parse().unwrap();
        Config { length, count }
    }
}

/// Parse newline-delineated strings from an array of bytes.
fn parse_bytes(bytes: &[u8]) -> (Vec<String>, u16) {
    let byte_str = String::from_utf8_lossy(bytes);
    let lines: Vec<String> = byte_str.lines().map(String::from).collect();
    let num_lines: u16 = lines.len() as u16;
    (lines, num_lines)
}
