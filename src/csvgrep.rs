use argh::FromArgs;
use regex::Regex;

#[derive(FromArgs)]
/// Select columns from CSV data.
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

    if (args.regex.is_none() && args.fixed_string.is_none())
        || (args.regex.is_some() && args.fixed_string.is_some()) {
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
        panic!("Unreachable.");
    }

    let input_lines = std::io::BufReader::new(std::io::stdin());
    let mut reader =
        csv::ReaderBuilder::new()
        .delimiter(args.delimiter as u8)
        .from_reader(input_lines);

    let header: Vec<String> =
        reader.headers()?.iter().map(str::to_string).collect();

    let column_index: usize =
        match header.iter().position(|value| *value == args.column) {
            Some(index) => index,
            None => {
                eprintln!("csvgrep: error: Column '{}' is not present \
                           in the input.", args.column);
                std::process::exit(3)
            }
        };

    let mut writer = csv::Writer::from_writer(std::io::stdout());

    writer.write_record(&header)?;

    for record in reader.records() {
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

        if (!args.invert && record_matches)
            || (args.invert && !record_matches) {
            writer.write_record(&input_record)?;
        }
    }

    Ok(())
}
