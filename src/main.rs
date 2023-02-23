use dynoxide::*;

fn main() {
    let mut data: Vec<BlackBoxTick> = Vec::new();
    let mut rdr = csv::Reader::from_path("flight_data/first_solo_black_box.csv").unwrap();
    let mut err_count = 0;
    let mut point_count = 0;
    for result in rdr.deserialize() {
        match result {
            Ok(record) => {
                data.push(record);
                point_count += 1;
            }
            Err(e) => {
                println!("Error: {e}");
                err_count += 1;
            }
        }
    }
    println!("point_count: {point_count}");
    println!("err_count: {err_count}");
    let output_file_name = "sample_out.kml".to_string();
    generate_kml_of_flight(&output_file_name, &data);
}
