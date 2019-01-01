#[derive(Debug, Deserialize)]
struct Row {
    #[serde(rename = "店名")]
    name: String,
    #[serde(rename = "住所")]
    address: String,
    #[serde(rename = "台数")]
    units: String,
}

const MAX_LENGTH: usize = 2000;

pub fn bundle<'a, I>(paths: I)
where
    I: Iterator<Item = &'a str>,
{
    let mut rows = Vec::with_capacity(MAX_LENGTH);
    let mut idx = 0;

    for path in paths {
        let mut reader =
            csv::Reader::from_path(path).expect(&format!("Unable to read CSV file {}", path));
        for result in reader.deserialize() {
            let row: Row =
                result.expect(&format!("Unable to deserialize row in CSV file {}", path));
            rows.push(row);
            if rows.len() >= MAX_LENGTH {
                flush(&rows, idx);
                rows.clear();
                idx += 1;
            }
        }
    }
    if !rows.is_empty() {
        flush(&rows, idx);
    }
}

fn flush(rows: &[Row], idx: usize) {
    use std::io::Write;

    let path = format!("{}.csv", idx);
    let mut file = std::fs::File::create(&path).expect(&format!("Unable to open {}", path));
    writeln!(&mut file, "店名,住所,台数").expect("Unable to write CSV header");
    for row in rows {
        writeln!(&mut file, "{},{},{}", row.name, row.address, row.units)
            .expect("Unable to write CSV row");
    }
}
