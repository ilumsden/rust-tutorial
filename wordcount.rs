use itertools::Itertools;
use std::fs;

fn load_text(fname: &str) -> Vec<String> {
    let contents = fs::read_to_string(fname).expect("Something went wrong while reading the file");
    let mut asciionly = contents.replace(|c: char| !c.is_ascii(), "");
    asciionly.make_ascii_uppercase();
    asciionly.split_whitespace().take_while(|x| !x.is_empty()).map(|x| String::from(x)).collect()
}

fn main() {
    let words = load_text("book_CountOfMonteCristo.txt");

    let word_lens = words.iter().map(|x| x.len());
    let word_len_counts = word_lens.sorted()
                                   .group_by(|x| *x)
                                   .into_iter()
                                   .map(|(k, v)| (k, v.count()))
                                   .collect::<Vec<(usize, usize)>>();
    println!("Expected Output");
    println!("Word Length : Count");
    for (wordlen, count) in vec![(3, 109798), (2, 84021), (4, 81777), (5, 49101), (6, 39015), (7, 30701)].iter() {
        println!("{:<11} : {:>6}", wordlen, count);
    };
    println!("\nActual Output");
    println!("Word Length : Count");
    for (wordlen, count) in word_len_counts.iter().sorted_by(|a, b| Ord::cmp(&b.1, &a.1)).take(6) {
        println!("{:<11} : {:>6}", wordlen, count);
    };

    let chars = words.iter().flat_map(|s| s.chars());
    let char_len_counts = chars.sorted()
                               .group_by(|x| *x)
                               .into_iter()
                               .map(|(k, v)| (k, v.count()))
                               .collect::<Vec<(char, usize)>>();
    println!("Expected Output");
    println!("Character : Count");
    for (wordlen, count) in vec![('E', 258693), ('T', 180211), ('A', 165306), ('O', 156817), ('I', 142095), ('N', 137343)].iter() {
        println!("{:<9} : {:>6}", wordlen, count);
    };
    println!("\nActual Output");
    println!("Character : Count");
    for (charac, count) in char_len_counts.iter().sorted_by(|a, b| Ord::cmp(&b.1, &a.1)).take(6) {
        println!("{:<9} : {:>6}", charac, count);
    };
}
