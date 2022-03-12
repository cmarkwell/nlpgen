use std::fs;

use clap::{ AppSettings, Parser };
use rand::Rng;

/// Generate natural language passwords
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(global_setting(AppSettings::DeriveDisplayOrder))]
struct Args {
    /// Number of natural language passwords to generate
    #[clap(short, long, default_value_t = 1)]
    count: u32,

    /// Number of adj-n pairs per generated password
    #[clap(short, long, default_value_t = 1)]
    length: u32,

    /// Path to a text file containing newline-delimited adjectives
    #[clap(short, long)]
    adjectives: Option<String>,

    /// Path to a text file containing newline-delimited nouns
    #[clap(short, long)]
    nouns: Option<String>,
}

fn main() {
    // Parse command line args, include default resources
    let args = Args::parse();
    let mut adj_bytes: &[u8] = include_bytes!("resources/adjectives.txt");
    let mut noun_bytes: &[u8] = include_bytes!("resources/nouns.txt");
    let adj_vec: Vec<u8>;
    let noun_vec: Vec<u8>;
    
    // Read bytes from optional adjectives file
    if args.adjectives.is_some() {
        adj_vec = fs::read(args.adjectives.unwrap()).expect("Failed to read adjectives :(");
        adj_bytes = adj_vec.as_slice();
    }

    // Read bytes from optional nouns file
    if args.nouns.is_some() {
        noun_vec = fs::read(args.nouns.unwrap()).expect("Failed to read nouns :(");
        noun_bytes = noun_vec.as_slice();
    }

    // Parse data from default or read corpora
    let (adjectives, num_adjs) = parse_bytes(adj_bytes);
    let (nouns, num_nouns) = parse_bytes(noun_bytes);

    // Randomly generate adjective-noun pairs
    let mut rng = rand::thread_rng();
    for _ in 0..args.count {
        let mut nlp = String::new();
        for _ in 0..args.length {
            let dice_adj = rng.gen_range(0..num_adjs) as usize;
            let dice_noun = rng.gen_range(0..num_nouns) as usize;
            nlp.push_str(&format!("{} {} ", adjectives[dice_adj], nouns[dice_noun]));
        }
        println!("{}", nlp);
    }
}

/// Parse newline-delimited strings from an array of bytes.
fn parse_bytes(bytes: &[u8]) -> (Vec<String>, u16) {
    let byte_str = String::from_utf8_lossy(bytes);
    let lines: Vec<String> = byte_str.lines().map(String::from).collect();
    let num_lines: u16 = lines.len() as u16;
    (lines, num_lines)
}
