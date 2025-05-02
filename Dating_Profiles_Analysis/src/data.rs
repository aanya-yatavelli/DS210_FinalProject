// handle reading CSV into vector
// randomly sample some number of those records

use csv::Reader;
use rand::{seq::SliceRandom, thread_rng}; 
use std::collections::HashMap;
use std::error::Error;

// load each row as a map of column to value
pub fn load_records(path: &str) -> Result<Vec<HashMap<String, String>>, Box<dyn Error>> {
    let mut rdr = Reader::from_path(path)?;
    let headers = rdr.headers()?.clone();
    let mut out = Vec::new();
    for result in rdr.records() {
        let rec = result?;
        let mut map: HashMap<String, String> = HashMap::new();
        for (h, v) in headers.iter().zip(rec.iter()) {
            map.insert(h.to_string(), v.to_string()); 
        }
        out.push(map);
    }
    Ok(out)
}

// randomly sample 'n' records from 'records'
pub fn sample_records(
    records: &[HashMap<String, String>], n: usize,
) -> Vec<HashMap<String, String>> {
    let mut sampled = records.to_vec();
    let mut rng = thread_rng();
    sampled.shuffle(&mut rng); 
    sampled.truncate(n);
    sampled
}