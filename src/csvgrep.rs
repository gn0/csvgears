use argh::FromArgs;
use regex::Regex;
use std::vec::Vec;

use csvgears::{csv_reader_from_stdin, csv_writer_to_stdout};

#[derive(FromArgs)]
/// Select rows from CSV data based on cell contents in a column.
struct Args {
    #[argh(option, short = 'd', default = "','")]
    /// delimiter [default: ',']
    delimiter: char,

    #[argh(switch, short = 'i')]
    /// invert the match
    invert: bool,

    #[argh(option, short = 'c')]
    /// column to apply the pattern to
    column: String,

    #[argh(option, short = 'r')]
    /// regular expression to match against the cells of the column
    regex: Option<String>,

    #[argh(option, short = 'm')]
    /// fixed string to search for in the cells of the column (need not
    /// be an exact match)
    fixed_string: Option<String>,

    #[argh(option, short = 'f', long = "file")]
    /// path to a file, each line of which gets matched against the
    /// cells of a column (must be an exact match)
    file_path: Option<String>,
}

enum Pattern {
    Regex(Regex),
    FixedString(String),
    File(Vec<String>),
}

fn load_file(file_path: &str) -> Option<Vec<String>> {
    Some(
        std::fs::read_to_string(file_path)
            .ok()?
            .lines()
            .map(ToString::to_string)
            .collect::<_>(),
    )
}

fn exactly_one<T>(values: &[T], predicate: fn(&T) -> bool) -> bool {
    values.iter().filter(|x| predicate(x)).count() == 1
}

fn main() -> Result<(), csv::Error> {
    let args: Args = argh::from_env();

    if !exactly_one(&[&args.regex, &args.fixed_string, &args.file_path], |x| {
        x.is_some()
    }) {
        eprintln!("csvgrep: error: Must specify exactly one of -r, -m, and -f.");
        std::process::exit(1);
    }

    let pattern: Pattern;

    if let Some(regex_string) = args.regex {
        match Regex::new(&regex_string) {
            Err(error) => {
                eprintln!(
                    "csvgrep: error: Cannot parse regular \
                           expression: {error}"
                );
                std::process::exit(2);
            }
            Ok(regex) => {
                pattern = Pattern::Regex(regex);
            }
        }
    } else if let Some(fixed_string) = args.fixed_string {
        pattern = Pattern::FixedString(fixed_string);
    } else if let Some(file_path) = args.file_path {
        let Some(exact_strings) = load_file(&file_path) else {
            eprintln!("csvgrep: error: Cannot load contents of '{file_path}'.");
            std::process::exit(3);
        };

        pattern = Pattern::File(exact_strings);
    } else {
        unreachable!();
    }

    let mut csv_reader = csv_reader_from_stdin(args.delimiter)?;

    let column_index: usize = match csv_reader
        .header
        .iter()
        .position(|value| *value == args.column)
    {
        Some(index) => index,
        None => {
            eprintln!(
                "csvgrep: error: Column '{}' is not present in the input.",
                args.column
            );
            std::process::exit(4)
        }
    };

    let mut csv_writer = csv_writer_to_stdout(Some(csv_reader.header.clone()))?;

    for record in csv_reader.reader.records() {
        let input_record = record?;
        let cell = &input_record[column_index];

        let record_matches: bool = match &pattern {
            Pattern::Regex(regex) => regex.find(cell).is_some(),
            Pattern::FixedString(fixed_string) => cell.contains(fixed_string),
            Pattern::File(exact_strings) => exact_strings.iter().any(|x| cell == x),
        };

        if args.invert ^ record_matches {
            csv_writer.writer.write_record(&input_record)?;
        }
    }

    Ok(())
}
