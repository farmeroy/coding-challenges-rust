use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::{env, io};

struct FileData {
    bytes: usize,
    lines: usize,
    words: usize,
    chars: usize,
}

struct Args {
    bytes: bool,
    lines: bool,
    words: bool,
    chars: bool,
    file_name: String,
}

impl Args {}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_name = args.last().expect("No filename give").to_string();

    let mut args = Args {
        bytes: args.iter().any(|arg| arg == "-c"),
        lines: args.iter().any(|arg| arg == "-l"),
        words: args.iter().any(|arg| arg == "-w"),
        chars: args.iter().any(|arg| arg == "-m"),
        file_name: file_name.clone(),
    };
    let file_data = get_text_data(&args.file_name).unwrap();

    if !args.bytes && !args.lines && !args.words && !args.chars {
        args = Args {
            bytes: true,
            lines: true,
            words: true,
            chars: false,
            file_name,
        };
    }

    println!("{}", format_output(file_data, args))
}

fn format_output(data: FileData, args: Args) -> String {
    let lines_str = match args.lines {
        true => format!("{} ", data.lines),
        false => "".to_owned(),
    };
    let words_str = match args.words {
        true => format!("{} ", data.words),
        false => "".to_owned(),
    };
    let bytes_str = match args.bytes {
        true => format!("{} ", data.bytes),
        false => "".to_owned(),
    };
    let chars_str = match args.chars {
        true => format!("{} ", data.chars),
        false => "".to_owned(),
    };
    //
    // @TODO: format the string so that there is an indentation on the default view (step 5)
    // @TODO: pipe stdin (step 6)
    format!(
        "{}{}{}{}{}",
        lines_str, words_str, bytes_str, chars_str, args.file_name
    )
}

fn get_text_data(file: &str) -> Result<FileData, Error> {
    match File::open(file) {
        Err(e) => {
            eprintln!("Error opening file: {e}");
            Err(e)
        }
        Ok(file) => {
            let mut reader = BufReader::new(file);
            let mut lines_count = 0;
            let mut bytes_count = 0;
            let mut words_count = 0;
            let mut chars_count = 0;

            loop {
                let mut line = String::new();
                let line_bytes = reader.read_line(&mut line).expect("couldn't read line");
                if line_bytes == 0 {
                    break;
                };

                // we use split_whitespace to account for any amount/kind of whitespace
                let words = line.trim().split_whitespace().count();
                let chars = line.chars().count();

                bytes_count += line_bytes;
                lines_count += 1;
                words_count += words;
                chars_count += chars;
            }
            Ok(FileData {
                lines: lines_count,
                bytes: bytes_count,
                words: words_count,
                chars: chars_count,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::Read};

    #[test]
    fn test_bytes() {
        let f = File::open("test.txt").expect("Cannot find test.txt");
        let bytes = f.bytes().count();
        assert_eq!(bytes, 342190);
    }
}
