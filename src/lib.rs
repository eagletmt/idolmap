#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;

#[derive(Debug)]
pub struct Shop {
    pub name: String,
    pub address: String,
    pub units: usize,
}

#[derive(Debug)]
pub struct CsvWriter {
    file: std::fs::File,
}

impl CsvWriter {
    pub fn open<P>(path: P) -> std::io::Result<Self>
    where
        P: AsRef<std::path::Path>,
    {
        Ok(Self {
            file: std::fs::File::create(path)?,
        })
    }

    pub fn write_header(&mut self) -> std::io::Result<()> {
        use std::io::Write;
        writeln!(&mut self.file, "店名,住所,台数")
    }

    pub fn write_shop(&mut self, shop: &Shop) -> std::io::Result<()> {
        use std::io::Write;
        writeln!(
            &mut self.file,
            "{},{},{}台",
            shop.name, shop.address, shop.units
        )
    }
}

pub mod aikatsu;
pub mod csv;
pub mod lovelive;
pub mod prichan;
