use clap::{App, Arg, ArgMatches};
use itertools::Itertools;
use regex::Regex;
use std::{
    collections::HashSet,
    fs::{self, OpenOptions},
    io::{Read, Write},
};

use encoding_rs::WINDOWS_1251;
use encoding_rs_io::DecodeReaderBytesBuilder;

// Array of characters visually similar to Cyrillic characters
// In the first place should be a Cyrillic character, then all characters that may be similar
const CHARS_MAP: [&str; 21] = [
    "АA", "аa", "ВB", "СC", "сc", "ЕE", "еe", "КK", "кk", "З3", "з3", "МM", "НH", "ОO", "оo", "РP",
    "рp", "ТT", "ХX", "хx", "уy",
];

fn main() {
    let arg_matches = get_args_matches();

    if let Some(word) = arg_matches.value_of("word") {
        permutation_single_word(word.trim());
        return;
    }

    let input_file = arg_matches
        .value_of("input")
        .expect("No input file specified");

    let output_file = arg_matches
        .value_of("output")
        .expect("No output file specified");

    permutation_file(input_file.trim(), output_file.trim());
}

fn get_args_matches<'a>() -> ArgMatches<'a> {
    App::new("Permutation chars in word")
        .version("1.0")
        .about("")
        .arg(
            Arg::with_name("input")
                .short("i")
                .long("input")
                .value_name("FILE")
                .help("Input .txt file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .value_name("FILE")
                .help("Output .txt file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("word")
                .short("-w")
                .long("word")
                .value_name("STRING")
                .help("Single word to process")
                .takes_value(true),
        )
        .get_matches()
}

fn permutation_single_word(word: &str) {
    let mut words = HashSet::<String>::new();
    permutation(word, &mut words, 0);

    words.iter().for_each(|w| println!("{},", &w));
}

fn permutation_file(input_filename: &str, output_filename: &str) {
    let contents: String =
        fs::read_to_string(input_filename).expect("File could not be opened for reading");

    // TODO for
    // let input_file = OpenOptions::new().read(true).open(input_filename).unwrap();
    // let mut input_transcoded = DecodeReaderBytesBuilder::new()
    //     .encoding(Some(WINDOWS_1251))
    //     .build(input_file);

    // let mut contents: String = String::new();
    // if let Err(e) = input_transcoded.read_to_string(&mut contents) {
    //     eprintln!("Couldn't read from file: {}", e);
    // }

    let mut output_file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(false)
        .open(output_filename)
        .unwrap();

    // let re = Regex::new(r"[*()]|[~\d+]").unwrap();
    // let contents = re.replace_all(&contents, "");

    // let re = Regex::new(r"[ ]|[\r\n]|[\r]").unwrap();
    let re = Regex::new(r"[\r\n]|[\r]").unwrap();
    let fields: Vec<&str> = re
        .split(&contents)
        .unique()
        .filter(|&x| !x.is_empty())
        .collect();

    // println!("{}", contents);
    // println!("{:?}", fields);

    for word in fields {
        let mut words = HashSet::<String>::new();
        permutation(word, &mut words, 0);

        words.iter().for_each(|w| {
            if let Err(e) = writeln!(output_file, "{},", w) {
                eprintln!("Couldn't write to file: {}", e);
                return;
            }
        });
    }
}

// Рекурсивный метод подстановки/перестановки символов. Строится дерево слов, каждый следующий узел строится на основе родительского
fn permutation(word: &str, words: &mut HashSet<String>, index: usize) {
    for (i, ch) in word.chars().skip(index).enumerate() {
        for chars in CHARS_MAP {
            if !chars.contains(ch) {
                continue;
            }

            for replace_char in chars.chars() {
                let new_word = replace_nth_char(word, index + i, replace_char);
                if new_word.eq(word) {
                    continue;
                }

                words.insert(new_word.clone());
                permutation(&new_word, words, index + i + 1);
            }
            break;
        }
    }
}

fn replace_nth_char(s: &str, idx: usize, newchar: char) -> String {
    s.chars()
        .enumerate()
        .map(|(i, c)| if i == idx { newchar } else { c })
        .collect()
}
