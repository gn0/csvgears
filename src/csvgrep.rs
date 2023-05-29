use argh::FromArgs;
use regex::Regex;

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
    /// fixed string to search for in the cells of the column
    fixed_string: Option<String>,
}

enum Pattern {
    Regex(Regex),
    FixedString(String),
}

fn main() -> Result<(), csv::Error> {
    let args: Args = argh::from_env();

    if !(args.regex.is_none() ^ args.fixed_string.is_none()) {
        eprintln!("csvgrep: error: Must specify either -r or -m \
                   but not both.");
        std::process::exit(1);
    }

    let pattern: Pattern;

    if let Some(regex_string) = args.regex {
        match Regex::new(&regex_string) {
            Err(error) => {
                eprintln!("csvgrep: error: Cannot parse regular \
                           expression: {error}");
                std::process::exit(2);
            },
            Ok(regex) => {
                pattern = Pattern::Regex(regex);
            },
        }
    } else if let Some(fixed_string) = args.fixed_string {
        pattern = Pattern::FixedString(fixed_string);
    } else {
        unreachable!();
    }

    let mut csv_reader = csv_reader_from_stdin(args.delimiter)?;

    let column_index: usize =
        match csv_reader.header.iter().position(|value|
                                                *value == args.column) {
            Some(index) => index,
            None => {
                eprintln!("csvgrep: error: Column '{}' is not present \
                           in the input.", args.column);
                std::process::exit(3)
            }
        };

    let mut csv_writer =
        csv_writer_to_stdout(Some(csv_reader.header.clone()))?;

    for record in csv_reader.reader.records() {
        let input_record = record?;
        let cell = input_record[column_index].to_string();

        let record_matches: bool =
            match &pattern {
                Pattern::Regex(regex) => {
                    regex.find(&cell).is_some()
                },
                Pattern::FixedString(fixed_string) => {
                    cell.contains(fixed_string)
                },
            };

        if args.invert ^ record_matches {
            csv_writer.writer.write_record(&input_record)?;
        }
    }

    Ok(())
}
