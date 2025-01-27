/*
Making sure there is 1 argument passed in.
Checking if the file exists and opening it.
Reading the file contents into a string.
Remove all punctuation marks from the text.
Split the text into words, based on " " (space).
Create a HashMap to store the word counts.
Iterate over the words and increment the count in the HashMap.
Find the most common word(s) and their count.
*/

use::std::env;                    // For accessing command-line arguments
use std::io::prelude::*;         // Common I/O traits
use std::fs::File;              // For file operations
use std::collections::HashMap; // For storing word counts

fn main() {
    if env::args().len() <= 1 {
        println!("Please provide a file path.");
        return;
    }

    let path = env::args().nth(1).unwrap();

    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(e) => {
            println!("Error opening file: {}", e);
            return;
        }
    };

    let mut contents = String::new();
    let punctuation_marks = [".", ",", "!", "?", ":", ";", "\"", "'"]; // all exclusion marks from the text when counting words

    file.read_to_string(&mut contents).unwrap();

    // convert the string to lowercase
    contents = contents.to_lowercase();

    for mark in punctuation_marks.iter() {
        contents = contents.replace(mark, " ");
    }

    // split the text into words and store them in a vector
    let words: Vec<&str> = contents.split_whitespace().collect();

    let mut word_counts = HashMap::new();

    for word in words.iter() {
        let count = word_counts.entry(word).or_insert(0);
        *count += 1;
    }

    let mut most_common_words = Vec::new();
    let mut highest_count = 0;

    for (word, count) in word_counts.iter() {
        if *count > highest_count {
            most_common_words.clear();
            most_common_words.push(*word);
            highest_count = *count;
        } else if *count == highest_count {
            most_common_words.push(*word);
        }
    }

    // join the most common words into a single string
    let most_common_words_str: Vec<&str> = most_common_words.iter().map(|&&s| s).collect();
    let most_common_words_string = most_common_words_str.join(", ");

    // println!("File contents: {}", contents);
    println!("File contains {} words", words.len());
    println!("The most common word is/are: {} ", most_common_words_string);
    println!("and it/they has been mentioned {} times ", highest_count);

}
