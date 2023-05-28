use std::io::{BufReader, Read, Stdin, Stdout, Write};

pub struct CsvReader<R: Read> {
    pub reader: csv::Reader<R>,
    pub header: Vec<String>,
}

pub struct CsvWriter<W: Write> {
    pub writer: csv::Writer<W>,
    pub header: Option<Vec<String>>,
}

pub fn csv_reader_from_stdin(delimiter: char)
                             -> Result<CsvReader<BufReader<Stdin>>,
                                       csv::Error> {
    let input_lines = std::io::BufReader::new(std::io::stdin());

    let mut reader =
        csv::ReaderBuilder::new()
        .delimiter(delimiter as u8)
        .from_reader(input_lines);

    let header: Vec<String> =
        reader.headers()?.iter().map(str::to_string).collect();

    Ok(CsvReader {
        reader,
        header,
    })
}

pub fn csv_writer_to_stdout(header: Option<Vec<String>>)
                            -> Result<CsvWriter<Stdout>, csv::Error> {
    let mut writer = csv::Writer::from_writer(std::io::stdout());

    if let Some(cells) = header.clone() {
        writer.write_record(cells)?;
    }

    Ok(CsvWriter {
        writer,
        header,
    })
}
