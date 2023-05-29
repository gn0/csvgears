use argh::FromArgs;

use csvgears::{csv_reader_from_stdin, csv_writer_to_stdout};

#[derive(FromArgs)]
/// Select columns from CSV data.
struct Args {
    #[argh(option, short = 'd', default = "','")]
    /// delimiter [default: ',']
    delimiter: char,

    #[argh(option, short = 'c')]
    /// columns to include in the output
    include: Option<String>,

    #[argh(option, short = 'C')]
    /// columns to exclude from the output
    exclude: Option<String>,
}

fn main() -> Result<(), csv::Error> {
    let args: Args = argh::from_env();

    if !(args.include.is_none() ^ args.exclude.is_none()) {
        eprintln!("csvcut: error: Must specify either -c or -C \
                   but not both.");
        std::process::exit(1);
    }

    let mut csv_reader = csv_reader_from_stdin(args.delimiter)?;
    let column_spec =
        args.include.clone()
            .unwrap_or_else(
                || args.exclude.clone().unwrap());

    let column_indices: Vec<usize> =
        column_spec
        .split(',')
        .map(|name| {
            match csv_reader.header.iter().position(|value|
                                                    value == name) {
                Some(index) => index,
                None => {
                    eprintln!("csvcut: error: Column '{name}' is not \
                               present in the input.");
                    std::process::exit(2)
                }
            }
        })
        .collect();

    let output_header_record: Vec<String> =
        if args.include.is_some() {
            column_indices.iter()
                .map(|index| csv_reader.header[*index].clone())
                .collect()
        } else {
            csv_reader.header.iter()
                .enumerate()
                .filter(|(index, _)| !column_indices.contains(index))
                .map(|(_, value)| value.to_string())
                .collect()
        };

    let mut csv_writer =
        csv_writer_to_stdout(Some(output_header_record))?;

    for record in csv_reader.reader.records() {
        let input_record = record?;

        let output_record: Vec<_> =
            if args.include.is_some() {
                column_indices.iter()
                    .map(|index| &input_record[*index])
                    .collect()
            } else {
                input_record.iter()
                    .enumerate()
                    .filter(|(index, _)|
                            !column_indices.contains(index))
                    .map(|(_, value)| value)
                    .collect()
            };

        csv_writer.writer.write_record(&output_record)?;
    }

    Ok(())
}
