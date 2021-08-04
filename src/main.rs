use clap::{App, Arg, ArgMatches};
use itertools::Itertools;
use regex::Regex;
use std::{collections::HashSet, env, fs};

const CHARS_MAP: [&str; 19] = [
    "АA", "аa", "ВB", "СC", "ЕE", "еe", "КK", "кk", "З3", "з3", "МM", "НH", "ОO", "оo", "РP", "рp",
    "ТT", "ХX", "хx",
];
// const CHARS_MAP: [&str; 3] = ["аx", "Мx", "Тx"];

fn main() {
    let executable = env::current_exe().unwrap();
    println!("{:?}", executable);

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
    //permutation(word, &mut words, 0);
    permutation_test(word, &mut words, 0);

    words.iter().for_each(|w| {
        if !word.eq(w) {
            println!("{},", &w)
        }
    });
}

fn permutation_file(input_filename: &str, output_filename: &str) {
    let contents: String =
        fs::read_to_string(input_filename).expect("File could not be opened for reading");

    let re = Regex::new(r"[*()]|[~\d+]").unwrap();
    let contents = re.replace_all(&contents, "");

    let re = Regex::new(r"[ ]|[\r\n]|[\r]").unwrap();
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
            if !word.eq(w) {
                println!("{},", &w)
            }
        });
    }
}

fn permutation(word: &str, words: &mut HashSet<String>, index: usize) {
    let ch = match word.chars().nth(index) {
        Some(x) => x,
        None => return,
    };
    for chars in CHARS_MAP {
        if !chars.contains(ch) {
            permutation(&word, words, index + 1);
            continue;
        }

        for replace_char in chars.chars() {
            let new_word = replace_nth_char(word, index, replace_char);
            words.insert(new_word.clone());
            permutation(&new_word, words, index + 1);
        }
        break;
    }
}

fn permutation_test(word: &str, words: &mut HashSet<String>, index: usize) {
    for (i, ch) in word.chars().skip(index).enumerate() {
        for chars in CHARS_MAP {
            if !chars.contains(ch) {
                continue;
            }

            for replace_char in chars.chars() {
                let new_word = replace_nth_char(word, index + i, replace_char);

                words.insert(new_word.clone());
                permutation_test(&new_word, words, index + i + 1);
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
