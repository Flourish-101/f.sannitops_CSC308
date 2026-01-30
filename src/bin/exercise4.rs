use std::io;

fn main() {
    let mut sentence = String::new();

    println!("Enter a sentence:");

    io::stdin()
        .read_line(&mut sentence)
        .expect("Failed to read input");

    let mut longest = "";
    let mut shortest = "";

    for word in sentence.split_whitespace() {
        if longest.is_empty() || word.len() > longest.len() {
            longest = word;
        }

        if shortest.is_empty() || word.len() < shortest.len() {
            shortest = word;
        }
    }

    if longest.is_empty() {
        println!("No words found.");
    } else {
        println!("Longest word: {}", longest);
        println!("Shortest word: {}", shortest);
    }
}
