use rand::Rng;

fn main() {
    let (adjectives, num_adjs) = parse_bytes(include_bytes!("resources/adjectives.txt"));
    let (nouns, num_nouns) = parse_bytes(include_bytes!("resources/nouns.txt"));

    let mut rng = rand::thread_rng();
    let dice_adj: usize = rng.gen_range(0..num_adjs) as usize;
    let dice_noun: usize = rng.gen_range(0..num_nouns) as usize;

    print!("{} {}", adjectives[dice_adj], nouns[dice_noun]);
}

/// Parse newline-delineated strings from an array of bytes.
fn parse_bytes(bytes: &[u8]) -> (Vec<String>, u16) {
    let byte_str = String::from_utf8_lossy(bytes);
    let lines: Vec<String> = byte_str.lines().map(String::from).collect();
    let num_lines: u16 = lines.len() as u16;
    (lines, num_lines)
}