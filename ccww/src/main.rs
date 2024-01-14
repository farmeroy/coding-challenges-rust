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

enum WCTarget {
    UseStdin,
    TryFile,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let wc_target = match args.len() == 1 {
        true => WCTarget::UseStdin,
        false => WCTarget::TryFile,
    };

    let file_name = match wc_target {
        WCTarget::UseStdin => "".to_owned(),
        WCTarget::TryFile => args.last().expect("No filename give").to_string(),
    };

    let mut args = Args {
        bytes: args.iter().any(|arg| arg == "-c"),
        lines: args.iter().any(|arg| arg == "-l"),
        words: args.iter().any(|arg| arg == "-w"),
        chars: args.iter().any(|arg| arg == "-m"),
        file_name: file_name.clone(),
    };
    let file_data = get_text_data(&args.file_name, wc_target).unwrap();

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

fn get_text_data(file_name: &str, wc_target: WCTarget) -> Result<FileData, Error> {
    let reader: Result<Box<dyn BufRead>, Error> = match wc_target {
        WCTarget::TryFile => match File::open(file_name) {
            Err(e) => Err(e),
            Ok(file) => Ok(Box::new(BufReader::new(file))),
        },
        WCTarget::UseStdin => Ok(Box::new(BufReader::new(io::stdin()))),
    };

    let mut reader = reader
        .map_err(|_| {
            eprintln!("No such file");
            std::process::exit(1);
        })
        .unwrap();

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

#[cfg(test)]
mod tests {
    use crate::{get_text_data, WCTarget};

    #[test]
    fn test_get_text_data() {
        let wc_target = WCTarget::TryFile;
        let data = get_text_data("test.txt", wc_target).unwrap();
        assert_eq!(data.bytes, 342190);
        assert_eq!(data.lines, 7145);
        assert_eq!(data.words, 58164);
    }
}
