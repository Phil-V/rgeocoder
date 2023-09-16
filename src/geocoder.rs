// Originally based on
// https://github.com/llambda/rust-reverse-geocoder/blob/master/src/geocoder.rs

use kdtree::KdTree;
use kdtree::distance::squared_euclidean;
use thiserror::Error;
use std;
use csv;
use serde::Deserialize;


#[derive(Error, Debug)]
pub enum ErrorKind {
    #[error("Cannot open the locations csv file.")]
    CsvReadError(#[source] std::io::Error),
    #[error("Cannot parse the locations csv file.")]
    CsvParseError(#[source] csv::Error),
    #[error("Cannot initialize the KdTree.")]
    InitializationError(#[source] kdtree::ErrorKind),
}

type Result<T> = std::result::Result<T, ErrorKind>;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Record {
    pub lat: f64,
    pub lon: f64,
    pub name: String,
    pub admin1: String,
    pub admin2: String,
    pub cc: String,
}

pub struct Locations {
    records: Vec<([f64; 2], Record)>,
}

impl Locations {
    pub fn from_file(path: &str) -> Result<Locations> {
        let mut records = Vec::new();

        let input = std::fs::File::open(path).map_err(ErrorKind::CsvReadError)?;
        let buffered = std::io::BufReader::new(input);
        let mut reader = csv::Reader::from_reader(buffered);

        for line in reader.deserialize() {
            let record: Record = line.map_err(ErrorKind::CsvParseError)?;
            records.push(([record.lat, record.lon], record));
        }

        Ok(Locations { records })
    }
}

pub struct ReverseGeocoder {
    tree: KdTree<f64, Record, [f64; 2]>,
}

impl ReverseGeocoder {
    pub fn new(path: &str) -> Result<ReverseGeocoder> {
        let locations = Locations::from_file(path)?;
        let mut reverse_geocoder = ReverseGeocoder {
            tree: KdTree::with_capacity(2, locations.records.len()),
        };
        reverse_geocoder.initialize(locations)?;
        Ok(reverse_geocoder)
    }

    fn initialize(&mut self, locations: Locations) -> Result<()> {
        for record in locations.records {
            self.tree
                .add(record.0, record.1)
                .map_err(ErrorKind::InitializationError)?;
        }
        Ok(())
    }

    pub fn search(&self, loc: &[f64; 2]) -> Option<&Record> {
        match self.tree.nearest(loc, 1, &squared_euclidean) {
            Ok(nearest) => if nearest.is_empty() {
                None
            } else {
                Some(&nearest[0].1)
            },
            Err(_) => None,
        }
    }
}

#[allow(dead_code)]
pub fn print_record(record: &Record) {
    println!(
        "({}, {}): {} {} {} {}",
        record.lat, record.lon, record.name, record.admin1, record.admin2, record.cc
    );
}

mod tests {

    #[test]
    fn it_works() {
        use super::*;
        use std::path::PathBuf;

        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("rgeocoder/cities.csv");

        let geocoder = ReverseGeocoder::new(path.to_str().unwrap()).unwrap();
        let y = geocoder.search(&[44.962786, -93.344722]);
        assert_eq!(y.is_some(), true);
        let slp = y.unwrap();

        assert_eq!(slp.name, "Saint Louis Park");

        // [44.894519, -93.308702] is 60 St W @ Penn Ave S, Minneapolis, Minnesota; however, this is physically closer to Richfield
        let mpls = geocoder.search(&[44.894519, -93.308702]).unwrap();
        assert_eq!(mpls.name, "Richfield");

        // [44.887055, -93.334204] is HWY 62 and Valley View Road, whish is in Edina
        let edina = geocoder.search(&[44.887055, -93.334204]).unwrap();
        assert_eq!(edina.name, "Edina");
    }
}
