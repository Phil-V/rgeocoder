// from https://github.com/llambda/rust-reverse-geocoder/blob/master/src/geocoder.rs

extern crate quick_csv;
extern crate kdtree;

use self::kdtree::KdTree;
use self::kdtree::distance::squared_euclidean;

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
    pub fn from_file() -> Locations {
        let mut records = Vec::new();

        let reader = quick_csv::Csv::from_file("cities.csv").unwrap().has_header(true);

        for read_record in reader {
            let record: Record = read_record.unwrap().decode().unwrap();
            records.push(([record.lat, record.lon], record));
        }

        Locations { records: records }
    }
}

pub struct ReverseGeocoder<'a> {
    tree: KdTree<&'a Record, &'a [f64; 2]>,
}

impl<'a> ReverseGeocoder<'a> {
    pub fn new(loc: &'a Locations) -> ReverseGeocoder<'a> {
        let mut reverse_geocoder =
            ReverseGeocoder::<'a> { tree: KdTree::new_with_capacity(2, loc.records.len()) };
        reverse_geocoder.initialize(loc);
        reverse_geocoder
    }

    fn initialize(&mut self, loc: &'a Locations) {
        for record in &loc.records {
            self.tree.add(&record.0, &record.1).unwrap();
        }
    }

    pub fn search(&'a self, loc: &[f64; 2]) -> Option<&'a Record> {
        let nearest = self.tree.nearest(loc, 1, &squared_euclidean).unwrap();
        if nearest.is_empty() {
            None
        } else {
            Some(&nearest[0].1)
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
        let loc = Locations::from_file();
        let geocoder = ReverseGeocoder::new(&loc);
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
