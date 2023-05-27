use argh::FromArgs;

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

    if (args.include.is_none() && args.exclude.is_none())
        || (args.include.is_some() && args.exclude.is_some()) {
            eprintln!("csvcut: error: Must specify either -c or -C \
                       but not both.");
            std::process::exit(1);
        }

    let input_lines = std::io::BufReader::new(std::io::stdin());
    let mut reader =
        csv::ReaderBuilder::new()
        .delimiter(args.delimiter as u8)
        .from_reader(input_lines);

    let input_header: Vec<String> =
        reader.headers()?.iter().map(str::to_string).collect();
    let column_spec =
        args.include.clone()
            .unwrap_or_else(
                || args.exclude.clone().unwrap());

    let column_indices: Vec<usize> =
        column_spec
        .split(',')
        .map(|name| {
            match input_header.iter().position(|value| value == name) {
                Some(index) => index,
                None => {
                    eprintln!("csvcut: error: Column '{name}' is not \
                               present in the input.");
                    std::process::exit(2)
                }
            }
        })
        .collect();

    let mut writer = csv::Writer::from_writer(std::io::stdout());

    let output_header_record: Vec<_> =
        if args.include.is_some() {
            column_indices.iter()
                .map(|index| &input_header[*index])
                .collect()
        } else {
            input_header.iter()
                .enumerate()
                .filter(|(index, _)| !column_indices.contains(index))
                .map(|(_, value)| value)
                .collect()
        };

    writer.write_record(&output_header_record)?;

    for record in reader.records() {
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

        writer.write_record(&output_record)?;
    }

    Ok(())
}
