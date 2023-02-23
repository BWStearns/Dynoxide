use dynoxide::*;

fn main() {
    let mut data: Vec<BlackBoxTick> = Vec::new();
    let mut rdr = csv::Reader::from_path("flight_data/first_solo_black_box.csv").unwrap();
    let mut err_count = 0;
    for result in rdr.deserialize() {
        match result {
            Ok(record) => data.push(record),
            Err(e) => {
                println!("Error: {}", e);
                err_count += 1;
            }
        }
    }
    println!("data: {:?}", data);
    println!("err_count: {}", err_count);
}
