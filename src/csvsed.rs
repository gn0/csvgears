use argh::FromArgs;
use regex::Regex;

use csvgears::{csv_reader_from_stdin, csv_writer_to_stdout};

#[derive(FromArgs)]
/// Replace occurrences of a pattern in a column in CSV data.
struct Args {
    #[argh(option, short = 'd', default = "','")]
    /// delimiter [default: ',']
    delimiter: char,

    #[argh(option, short = 'c')]
    /// column to apply the pattern to
    column: String,

    #[argh(option, short = 'r')]
    /// column to save the result to
    result_column: Option<String>,

    #[argh(option, short = 'p')]
    /// regular expression to match against the cells of the column
    pattern: String,

    #[argh(option, short = 't')]
    /// replacement for occurrences of the pattern
    replacement: String,
}

fn main() -> Result<(), csv::Error> {
    let args: Args = argh::from_env();

    let pattern =
        match Regex::new(&args.pattern) {
            Ok(regex) => regex,
            Err(error) => {
                eprintln!("csvsed: error: Cannot parse regular \
                           expression: {error}");
                std::process::exit(2);
            },
        };

    let mut csv_reader = csv_reader_from_stdin(args.delimiter)?;

    let column_index: usize =
        match csv_reader.header.iter().position(|value|
                                                *value == args.column) {
            Some(index) => index,
            None => {
                eprintln!("csvsed: error: Column '{}' is not present \
                           in the input.", args.column);
                std::process::exit(3)
            }
        };

    let output_header =
        match &args.result_column {
            Some(result) => {
                if csv_reader.header.contains(result) {
                    eprintln!("csvsed: error: Column '{}' already \
                               exists in the input.", result);
                    std::process::exit(4);
                }

                let mut new_header = csv_reader.header.clone();

                new_header.push(result.clone());

                new_header
            },
            None => {
                csv_reader.header.clone()
            },
        };

    let mut csv_writer =
        csv_writer_to_stdout(Some(output_header))?;

    for record in csv_reader.reader.records() {
        let input_record = record?;

        let new_cell =
            pattern.replace_all(
                &input_record[column_index],
                &args.replacement)
            .into_owned();

        let mut output_record: Vec<_> = input_record.iter().collect();

        if args.result_column.is_some() {
            output_record.push(&new_cell);
        } else {
            output_record[column_index] = &new_cell;
        }

        csv_writer.writer.write_record(&output_record)?;
    }

    Ok(())
}
