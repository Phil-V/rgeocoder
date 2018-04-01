// based on https://github.com/llambda/rust-reverse-geocoder/blob/master/src/geocoder.rs
extern crate quick_csv;
extern crate kdtree;

use self::kdtree::KdTree;
use self::kdtree::distance::squared_euclidean;
use self::kdtree::kdtree::ErrorKind;

use self::quick_csv::error::Error;


#[derive(Debug, Clone, RustcDecodable, RustcEncodable)]
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
    pub fn from_file() -> Result<Locations, Error> {
        let mut records = Vec::new();

        let reader = quick_csv::Csv::from_file("cities.csv")?.has_header(true);

        for read_record in reader {
            let record: Record = read_record?.decode()?;
            records.push(([record.lat, record.lon], record));
        }

        Ok(Locations { records: records })
    }
}

pub struct ReverseGeocoder {
    tree: KdTree<Record, [f64; 2]>,
}

impl ReverseGeocoder {
    pub fn new() -> Result<ReverseGeocoder, ErrorKind> {
        let locations = Locations::from_file().unwrap();
        let mut reverse_geocoder =
            ReverseGeocoder {
                tree: KdTree::new_with_capacity(2, locations.records.len()),
            };
        reverse_geocoder.initialize(locations)?;
        Ok(reverse_geocoder)
    }

    fn initialize(&mut self, locations: Locations) -> Result<(), ErrorKind> {
        for record in locations.records {
            self.tree.add(record.0, record.1).unwrap();
        }
        Ok(())
    }

    pub fn search(&self, loc: &[f64; 2]) -> Option<&Record> {
        match self.tree.nearest(loc, 1, &squared_euclidean) {
            Ok(nearest) => if nearest.is_empty() { None } else { Some(&nearest[0].1) },
            Err(_) => None
        }
    }
}


#[allow(dead_code)]
pub fn print_record(record: &Record) {
    println!("({}, {}): {} {} {} {}",
             record.lat,
             record.lon,
             record.name,
             record.admin1,
             record.admin2,
             record.cc);
}

mod tests {

    #[test]
    fn it_works() {
        use super::*;
        // let loc = Locations::from_file().unwrap();
        let geocoder = ReverseGeocoder::new().unwrap();
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
