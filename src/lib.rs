use std::{error::Error, fs::File, io::Write, collections::HashMap};

use kml::{Kml, types::{Placemark, LineString, Geometry, Coord, AltitudeMode}};
use serde::{Deserialize, Serialize, de::Unexpected};

fn bool_from_int<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let i = u8::deserialize(deserializer)?;
    match i {
        0 => Ok(false),
        1 => Ok(true),
        _ => Err(serde::de::Error::invalid_value(Unexpected::Unsigned(i as u64), &"0 or 1")),
    }
}

fn maybe_bool_from_maybe_int<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let i = Option::<u8>::deserialize(deserializer)?;
    match i {
        Some(0) => Ok(Some(false)),
        Some(1) => Ok(Some(true)),
        Some(_) => Err(serde::de::Error::invalid_value(Unexpected::Unsigned(i.unwrap() as u64), &"0 or 1")),
        None => Ok(None),
    }
}

// Fields: Session Time,GPS Fix Quality,Number of Satellites,GPS Date & Time,Latitude (deg),Longitude (deg),GPS Altitude (feet),Ground Speed (knots),Ground Track (deg),Mag Var (deg),Cross Track Error (NM),Destination Waypoint ID,Range to Destination (NM),Bearing to Destination (deg),System Time,Pitch (deg),Roll (deg),Magnetic Heading (deg),Indicated Airspeed (knots),Pressure Altitude (ft),Turn Rate (deg/s),Lateral Accel (g),Vertical Accel(g),Angle of Attack (%),Vertical Speed (ft/min),OAT (deg C),True Airspeed (knots),Barometer Setting (inHg),Density Altitude (ft),Wind Direction (deg),Wind Speed (knots),Heading Bug (deg),Altitude Bug (ft),Airspeed Bug (knots),Vertical Speed Bug (ft/min),Course (deg),CDI Source Type,CDI Source Port,CDI Scale (NM),CDI Deflection (%),Glideslope (%),AP Engaged,AP Roll Mode,AP Roll Force,AP Roll Position (steps),AP Roll Slip (bool),AP Pitch Force,AP Pitch Position (steps),AP Pitch Slip (bool),AP Yaw Force,AP Yaw Position,AP Yaw Slip (bool),Transponder Status,Transponder Reply (bool),Transponder Identing (bool),Transponder Code (octal),Oil Pressure (PSI),Oil Temp (deg C),RPM L,RPM R,Manifold Pressure (inHg),Fuel Flow 1 (gal/hr),Fuel Flow 2 (gal/hr),Fuel Pressure (PSI),Fuel Level L (gal),Fuel Level R (gal),Fuel Remaining (gal),Volts 1,Volts 2,Amps,Hobbs Time,Tach Time,Thermocouple 1 (deg C),Thermocouple 2 (deg C),Thermocouple 3 (deg C),Thermocouple 4 (deg C),CHT 4 (deg C),EGT 4 (deg C),CHT 3 (deg C),EGT 3 (deg C),CHT 2 (deg C),EGT 2 (deg C),CHT 1 (deg C),EGT 1 (deg C),Thermocouple 13 (deg C),Thermocouple 14 (deg C),GP Input 1,GP Input 2,CARB TEMPERATURE (deg C),FUEL PRESSURE (PSI),PHEAT CONTACT (V),HDRTNK CONTACT (V),BATT CONTACT (V),GP Input 8,LEFT LEVEL (gal),RIGHT LEVEL (gal),OIL PRESSURE (PSI),OIL TEMPERATURE (deg C),GP Input 13,Contacts,Percent Power,EGT Leaning State,
#[derive(Debug, Serialize, Deserialize)]
pub struct BlackBoxTick {
    #[serde(alias="Session Time")]
    session_time: f64, // Session Time
    #[serde(alias="GPS Fix Quality")]
    gps_fix_quality: u8, // GPS Fix Quality
    #[serde(alias="Number of Satellites")]
    number_of_satellites: u8, // Number of Satellites
    #[serde(alias="GPS Date & Time")]
    gps_date_time: String, // GPS Date & Time
    #[serde(alias="Latitude (deg)")]
    latitude_deg: f64, // Latitude (deg)
    #[serde(alias="Longitude (deg)")]
    longitude_deg: f64, // Longitude (deg)
    #[serde(alias="GPS Altitude (feet)")]
    gps_altitude_feet: f64, // GPS Altitude (feet)
    #[serde(alias="Ground Speed (knots)")]
    ground_speed_knots: f64, // Ground Speed (knots)
    #[serde(alias="Ground Track (deg)")]
    ground_track_deg: f64, // Ground Track (deg)
    #[serde(alias="Mag Var (deg)")]
    mag_var_deg: f64, // Mag Var (deg)
    #[serde(alias="Cross Track Error (NM)")]
    cross_track_error_nm: Option<f64>, // Cross Track Error (NM)
    #[serde(alias="Destination Waypoint ID")]
    destination_waypoint_id: Option<String>, // Destination Waypoint ID
    #[serde(alias="Range to Destination (NM)")]
    range_to_destination_nm: Option<f64>, // Range to Destination (NM)
    #[serde(alias="Bearing to Destination (deg)")]
    bearing_to_destination_deg: Option<f64>, // Bearing to Destination (deg)
    #[serde(alias="System Time")]
    system_time: String, // System Time
    #[serde(alias="Pitch (deg)")]
    pitch_deg: f64, // Pitch (deg)
    #[serde(alias="Roll (deg)")]
    roll_deg: f64, // Roll (deg)
    #[serde(alias="Magnetic Heading (deg)")]
    magnetic_heading_deg: f64, // Magnetic Heading (deg)
    #[serde(alias="Indicated Airspeed (knots)")]
    indicated_airspeed_knots: f64, // Indicated Airspeed (knots)
    #[serde(alias="Pressure Altitude (ft)")]
    pressure_altitude_ft: f64, // Pressure Altitude (ft)
    #[serde(alias="Turn Rate (deg/s)")]
    turn_rate_deg_s: f64, // Turn Rate (deg/s)
    #[serde(alias="Lateral Accel (g)")]
    lateral_accel_g: f64, // Lateral Accel (g)
    #[serde(alias="Vertical Accel(g)")]
    vertical_accel_g: f64, // Vertical Accel(g)
    #[serde(alias="Angle of Attack (%)")]
    angle_of_attack_percent: f64, // Angle of Attack (%)
    #[serde(alias="Vertical Speed (ft/min)")]
    vertical_speed_ft_min: f64, // Vertical Speed (ft/min)
    #[serde(alias="OAT (deg C)")]
    oat_deg_c: f64, // OAT (deg C)
    #[serde(alias="True Airspeed (knots)")]
    true_airspeed_knots: f64, // True Airspeed (knots)
    #[serde(alias="Barometer Setting (inHg)")]
    barometer_setting_inhg: f64, // Barometer Setting (inHg)
    #[serde(alias="Density Altitude (ft)")]
    density_altitude_ft: f64, // Density Altitude (ft)
    #[serde(alias="Wind Direction (deg)")]
    wind_direction_deg: Option<f64>, // Wind Direction (deg)
    #[serde(alias="Wind Speed (knots)")]
    wind_speed_knots: Option<f64>, // Wind Speed (knots)
    #[serde(alias="Heading Bug (deg)")]
    heading_bug_deg: Option<f64>, // Heading Bug (deg)
    #[serde(alias="Altitude Bug (ft)")]
    altitude_bug_ft: Option<f64>, // Altitude Bug (ft)
    #[serde(alias="Airspeed Bug (knots)")]
    airspeed_bug_knots: Option<f64>, // Airspeed Bug (knots)
    #[serde(alias="Vertical Speed Bug (ft/min)")]
    vertical_speed_bug_ft_min: Option<f64>, // Vertical Speed Bug (ft/min)
    #[serde(alias="Course (deg)")]
    course_deg: Option<f64>, // Course (deg)
    #[serde(alias="CDI Source Type")]
    cdi_source_type: String, // CDI Source Type
    #[serde(alias="CDI Source Port")]
    cdi_source_port: String, // CDI Source Port
    #[serde(alias="CDI Scale (NM)")]
    cdi_scale_nm: Option<f64>, // CDI Scale (NM)
    #[serde(alias="CDI Deflection (%)")]
    cdi_deflection_percent: Option<f64>, // CDI Deflection (%)
    #[serde(alias="Glideslope (%)")]
    glideslope_percent: Option<f64>, // Glideslope (%)
    #[serde(alias="AP Engaged")]
    #[serde(deserialize_with="maybe_bool_from_maybe_int")]
    ap_engaged: Option<bool>, // AP Engaged
    #[serde(alias="AP Roll Mode")]
    ap_roll_mode: Option<String>, // AP Roll Mode
    #[serde(alias="AP Roll Force")]
    ap_roll_force: Option<f64>, // AP Roll Force
    #[serde(alias="AP Roll Position (steps)")]
    ap_roll_position_steps: Option<f64>, // AP Roll Position (steps)
    #[serde(alias="AP Roll Slip (bool)")]
    #[serde(deserialize_with="maybe_bool_from_maybe_int")]
    ap_roll_slip_bool: Option<bool>, // AP Roll Slip (bool)
    #[serde(alias="AP Pitch Force")]
    ap_pitch_force: Option<f64>, // AP Pitch Force
    #[serde(alias="AP Pitch Position (steps)")]
    ap_pitch_position_steps: Option<f64>, // AP Pitch Position (steps)
    #[serde(alias="AP Pitch Slip (bool)")]
    #[serde(deserialize_with="maybe_bool_from_maybe_int")]
    ap_pitch_slip_bool: Option<bool>, // AP Pitch Slip (bool)
    #[serde(alias="AP Yaw Force")]
    ap_yaw_force: Option<f64>, // AP Yaw Force
    #[serde(alias="AP Yaw Position")]
    ap_yaw_position: Option<f64>, // AP Yaw Position
    #[serde(alias="AP Yaw Slip (bool)")]
    #[serde(deserialize_with="maybe_bool_from_maybe_int")]
    ap_yaw_slip_bool: Option<bool>, // AP Yaw Slip (bool)
    #[serde(alias="Transponder Status")]
    transponder_status: String, // Transponder Status
    #[serde(alias="Transponder Reply (bool)")]
    #[serde(deserialize_with="bool_from_int")]
    transponder_reply_bool: bool, // Transponder Reply (bool)
    #[serde(alias="Transponder Identing (bool)")]
    #[serde(deserialize_with="bool_from_int")]
    transponder_identing_bool: bool, // Transponder Identing (bool)
    #[serde(alias="Transponder Code (octal)")]
    transponder_code_octal: String, // Transponder Code (octal)
    #[serde(alias="Oil Pressure (PSI)")]
    oil_pressure_psi: f64, // Oil Pressure (PSI)
    #[serde(alias="Oil Temp (deg C)")]
    oil_temp_deg_c: f64, // Oil Temp (deg C)
    #[serde(alias="RPM L")]
    rpm_l: f64, // RPM L
    #[serde(alias="RPM R")]
    rpm_r: f64, // RPM R
    #[serde(alias="Manifold Pressure (inHg)")]
    manifold_pressure_inhg: f64, // Manifold Pressure (inHg)
    #[serde(alias="Fuel Flow 1 (gal/hr)")]
    fuel_flow_1_gal_hr: f64, // Fuel Flow 1 (gal/hr)
    #[serde(alias="Fuel Flow 2 (gal/hr)")]
    fuel_flow_2_gal_hr: f64, // Fuel Flow 2 (gal/hr)
    #[serde(alias="Fuel Pressure (PSI)")]
    fuel_pressure_psi: f64, // Fuel Pressure (PSI)
    #[serde(alias="Fuel Level L (gal)")]
    fuel_level_l_gal: f64, // Fuel Level L (gal)
    #[serde(alias="Fuel Level R (gal)")]
    fuel_level_r_gal: f64, // Fuel Level R (gal)
    #[serde(alias="Fuel Remaining (gal)")]
    fuel_remaining_gal: f64, // Fuel Remaining (gal)
    #[serde(alias="Volts 1")]
    volts_1: Option<f64>, // Volts 1
    #[serde(alias="Volts 2")]
    volts_2: Option<f64>, // Volts 2
    #[serde(alias="Amps")]
    amps: Option<f64>, // Amps
    #[serde(alias="Hobbs Time")]
    hobbs_time: f64, // Hobbs Time
    #[serde(alias="Tach Time")]
    tach_time: f64, // Tach Time
    #[serde(alias="Thermocouple 1 (deg C)")]
    thermocouple_1_deg_c: Option<f64>, // Thermocouple 1 (deg C)
    #[serde(alias="Thermocouple 2 (deg C)")]
    thermocouple_2_deg_c: Option<f64>, // Thermocouple 2 (deg C)
    #[serde(alias="Thermocouple 3 (deg C)")]
    thermocouple_3_deg_c: Option<f64>, // Thermocouple 3 (deg C)
    #[serde(alias="Thermocouple 4 (deg C)")]
    thermocouple_4_deg_c: Option<f64>, // Thermocouple 4 (deg C)
    #[serde(alias="CHT 4 (deg C)")]
    cht_4_deg_c: f64, // CHT 4 (deg C)
    #[serde(alias="EGT 4 (deg C)")]
    egt_4_deg_c: f64, // EGT 4 (deg C)
    #[serde(alias="CHT 3 (deg C)")]
    cht_3_deg_c: f64, // CHT 3 (deg C)
    #[serde(alias="EGT 3 (deg C)")]
    egt_3_deg_c: f64, // EGT 3 (deg C)
    #[serde(alias="CHT 2 (deg C)")]
    cht_2_deg_c: f64, // CHT 2 (deg C)
    #[serde(alias="EGT 2 (deg C)")]
    egt_2_deg_c: f64, // EGT 2 (deg C)
    #[serde(alias="CHT 1 (deg C)")]
    cht_1_deg_c: f64, // CHT 1 (deg C)
    #[serde(alias="EGT 1 (deg C)")]
    egt_1_deg_c: f64, // EGT 1 (deg C)
    #[serde(alias="Thermocouple 13 (deg C)")]
    thermocouple_13_deg_c: Option<f64>, // Thermocouple 13 (deg C)
    #[serde(alias="Thermocouple 14 (deg C)")]
    thermocouple_14_deg_c: Option<f64>, // Thermocouple 14 (deg C)
    #[serde(alias="GP Input 1")]
    gp_input_1: Option<String>, // GP Input 1
    #[serde(alias="GP Input 2")]
    gp_input_2: Option<String>, // GP Input 2
    #[serde(alias="CARB TEMPERATURE (deg C)")]
    carb_temperature_deg_c: Option<f64>, // CARB TEMPERATURE (deg C)
    #[serde(alias="FUEL PRESSURE (PSI)")]
    fuel_pressure_psi_2: f64, // FUEL PRESSURE (PSI)
    #[serde(alias="PHEAT CONTACT (V)")]
    pheat_contact_v: f64, // PHEAT CONTACT (V)
    #[serde(alias="HDRTNK CONTACT (V)")]
    hdrtnk_contact_v: f64, // HDRTNK CONTACT (V)
    #[serde(alias="BATT CONTACT (V)")]
    batt_contact_v: f64, // BATT CONTACT (V)
    #[serde(alias="GP Input 8")]
    gp_input_8: Option<String>, // GP Input 8
    #[serde(alias="LEFT LEVEL (gal)")]
    left_level_gal: f64, // LEFT LEVEL (gal)
    #[serde(alias="RIGHT LEVEL (gal)")]
    right_level_gal: f64, // RIGHT LEVEL (gal)
    #[serde(alias="OIL PRESSURE (PSI)")]
    oil_pressure_psi_2: f64, // OIL PRESSURE (PSI)
    #[serde(alias="OIL TEMPERATURE (deg C)")]
    oil_temperature_deg_c: f64, // OIL TEMPERATURE (deg C)
    #[serde(alias="GP Input 13")]
    gp_input_13: Option<String>, // GP Input 13
    #[serde(alias="Contacts")]
    contacts: Option<String>, // Contacts
    #[serde(alias="Percent Power")]
    percent_power: Option<f64>, // Percent Power
    #[serde(alias="EGT Leaning State")]
    egt_leaning_state: Option<String>, // EGT Leaning State

}

pub fn generate_kml_of_flight(output_file: &String, data: &Vec<BlackBoxTick>) {
    let points: Vec<Coord> = data.iter().map(|tick| {
        Coord::new(tick.longitude_deg, tick.latitude_deg, Some(tick.gps_altitude_feet))
    }).collect();
    // Create a KML document of the flight from the vector of points    
    let linestring = LineString{
        coords: points,
        attrs: HashMap::new(),
        extrude: true,
        altitude_mode: AltitudeMode::Absolute,
        tessellate: true,
    };
    let document = Kml::Document{
        attrs: HashMap::new(),
        elements: vec!(Kml::Placemark (
            Placemark {
                description: None,
                geometry: Some(Geometry::LineString(linestring)),
                ..Default::default()
            }
        )),
    };
    // Write the KML document to a file
    let mut file = File::create(output_file).unwrap();
    file.write_all(document.to_string().as_bytes()).unwrap();
}

// read csv data from file
pub fn read_csv_data(filename: &str) -> Result<Vec<BlackBoxTick>, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path(filename)?;
    let mut data = Vec::new();
    for result in rdr.deserialize() {
        let record: BlackBoxTick = result?;
        data.push(record);
    }
    Ok(data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_csv_data() {
        let data = read_csv_data("flight_data/first_solo_black_box.csv").unwrap();
        assert_eq!(data.len(), 25485);
        assert_eq!(data[0].latitude_deg, 27.14215);
        assert_eq!(data[0].longitude_deg, -82.47485);
        assert_eq!(data[0].gps_altitude_feet, 2563.0);
        assert_eq!(data[0].ground_speed_knots, 91.6);
    }
}
