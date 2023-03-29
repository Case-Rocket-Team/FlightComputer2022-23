pub struct GPS_Info {
    pub latitude: f64,
    pub longitude: f64,
    pub heading: f64,
    pub velocity: f64,
}

impl GPS_Info {
    
}

pub enum message {
    BadLen,
    NotRMC,
    NoFIX,
    Fix(GPS_Info)
}

fn extract_rmc_fields(rmc_sentence: &str) -> message {
    // Split the RMC sentence into fields
    let fields: Vec<&str> = rmc_sentence.split(',').collect();

    // Check that the sentence has the correct number of fields
    if fields.len() != 13 {
        message::BadLen
    }

    // Check that the sentence is an RMC sentence
    if fields[0] != "$GPRMC" {
        message::NotRMC
    }

    // Check that the GPS receiver has a valid fix
    if fields[2] != "A" {
        message::NoFIX
    }

    // Extract the latitude and longitude fields
    // https://cdn-shop.adafruit.com/product-files/1059/CD+PA1616D+Datasheet+v.05.pdf --> go to page 20
    // latitude is written in ddmm.mmmm and longitude is written in dddmm.mmmm
    // dd and ddd indicate degrees mm.mmmm indicates minutes and fractions of a minute

    let latitude_degrees: f64 = fields[3][0..2].parse().ok()? as f64;
    let latitude_minutes: f64 = fields[3][2..].parse().ok()? as f64;
    let latitude = latitude_degrees + latitude_minutes / 60.0;
    let latitude_ns = fields[4];
    let longitude_degrees: f64 = fields[5][0..3].parse().ok()? as f64;
    let longitude_minutes: f64 = fields[5][3..].parse().ok()? as f64;
    let longitude = longitude_degrees + longitude_minutes / 60.0;
    let longitude_ew = fields[6];
    let speed_knots: f64 = fields[7].parse().ok()? as f64;
    let speed = speed_knots * 1.852; // Convert knots to km/h
    let heading_true: f64 = fields[8].parse().ok()? as f64;

    message::Fix(GPS_Info{
        latitude,
        longitude,
        speed,
        heading_true
    })
}