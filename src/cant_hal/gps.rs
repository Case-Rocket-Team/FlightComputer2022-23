use cortex_m::prelude::{_embedded_hal_blocking_i2c_WriteRead, _embedded_hal_blocking_i2c_Read, _embedded_hal_serial_Write, _embedded_hal_serial_Read};
use nb::block;
use ndarray::arr2;

use crate::{i2c::i2c::I2CHAL, logging, uart::{uart::UARTHAL, self}};

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

pub struct GPS {
    info: GPS_Info,
}

struct CartesianPoint {
    x: f64,
    y: f64,
    z: f64
}

struct SphericalPoint {
    r: f64,        // rho
    latitude: f64, // theta
    longitude: f64 // phi
}

impl GPS {
    pub fn write(uart: &mut UARTHAL, str: &[u8]) {
        for char in str {
            block!(uart.write(*char));
        }
    }

    pub fn read(uart: &mut UARTHAL, out: &mut [u8]) {
        for byte in out.iter_mut() {
            loop {
                match uart.read() {
                    Ok(b) => *byte = b,
                    Err(nb::Error::WouldBlock) => continue,
                    Err(nb::Error::Other(err)) => {
                        log::info!("Read error! {:?}", err);
                    }
                };

                break
            }
        }
    }
}

struct GPS_Math {

}

impl GPS_Math {
    fn findRadius(height: f64, latitude: f64, longitude: f64) -> f64 {
        // information from source below
        const EQUATORIAL_RADIUS: f64 = 20925646.3255; // a ft
        const POLAR_RADIUS: f64 = 20855486.5945; // b ft

        // equation from: https://www.summitpost.org/distance-to-the-center-of-the-earth/849764
        let topFirst = EQUATORIAL_RADIUS.powi(2) * longitude.cos().powi(2) + POLAR_RADIUS.powi(4) * longitude.sin().powi(2); 
        let bottomFirst = EQUATORIAL_RADIUS.powi(2) * longitude.cos().powi(2) + POLAR_RADIUS.powi(2) * longitude.sin().powi(2);
        let first = topFirst / bottomFirst;

        let secondInner = EQUATORIAL_RADIUS.powi(2) * longitude.cos().powi(2) + POLAR_RADIUS.powi(2) * longitude.sin().powi(2);
        let second = (2 as f64) * height * secondInner.sqrt();

        let third = height;

        (first + second + third).sqrt()
    }

    fn addCoreHeight(height: f64, latitude: f64, longitude: f64) -> f64 {
        height + GPS_Math::findRadius(height, latitude, longitude)
    } 

    fn toCartesian(position: SphericalPoint) -> CartesianPoint {
        const PI: f64 = std::f64::consts::PI;
        let longPrime = position.longitude + PI;
        let latPrime = -position.latitude + PI / 2.0;

        let x = position.r * latPrime.sin() * longPrime.cos();
        let y = position.r * latPrime.sin() * longPrime.sin();
        let z = position.r * latPrime.cos();

        CartesianPoint { x, y, z }
    }

    // toCartesianCoords replacement
    fn scaleRadiusToXYZ(radius: f64, coefficients: (f64, f64, f64)) -> CartesianPoint {
        let (xCoeff, yCoeff, zCoeff) = coefficients;
        let x = radius * xCoeff;
        let y = radius * yCoeff;
        let z = radius * zCoeff;

        CartesianPoint { x, y, z }
    }

    fn getCartesianCoefficients(point: SphericalPoint) -> (f64, f64, f64) {
        const PI: f64 = std::f64::consts::PI;

        let longPrime = point.longitude + PI;
        let latPrime = -point.latitude + PI / 2.0;

        let xCoeff = latPrime.sin() * longPrime.cos();
        let yCoeff = latPrime.sin() * longPrime.sin();
        let zCoeff = latPrime.cos();

        (xCoeff, yCoeff, zCoeff)
    }

    fn doRotationMatrices(point: CartesianPoint, longitude: f64, latitude: f64) -> [[f64; 3]; 3] {
        // x = arr[0]
        // y = arr[1]
        // z = arr[2]

        // #these need to be inverted because we are undoing the latitude and longitude
        let longitude = -longitude;
        let latitude = -latitude;

        // #rotate around z by long
        // RotMatrix1 = [[np.cos(long), -np.sin(long), 0],
        //             [np.sin(long),np.cos(long)  , 0],
        //             [0           ,0             , 1]]
        let rotMatrix1 = arr2(&[[longitude.cos(), -longitude.sin(), 0.0],
                                [longitude.sin(), longitude.cos(),  0.0],
                                [0.0,             0.0,              1.0]]);
        // let longRotated = rotMatrix1.dot(&point);
        
        // longRotated = np.matmul(RotMatrix1,arr)
        // #print("step 3.1")
        // #print(longRotated)

        // #rotate around y by lat
        // #RotMatrix2 = [[np.cos(lat) , 0, np.sin(lat)],
        // #              [0           , 1, 0          ],
        // #              [-np.sin(lat), 0, np.cos(lat)]]

        // #rotate around x by lat
        // RotMatrix2 = [[1,           0,            0],
        //             [0, np.cos(lat), -np.sin(lat)],
        //             [0, np.sin(lat), np.cos(lat)]]

        // fullRotated = np.matmul(RotMatrix2,longRotated)
        // #print("step 3.2")
        // #print(fullRotated)

        // #print("test")
        // #print(np.matmul(RotMatrix2,(np.matmul(RotMatrix1,arr))))
        // return fullRotated
    }

    fn approxCos(inPadAngle: f64, padSin: f64, padCos: f64, inAngle: f64) -> f64 {
        // padAngle = inPadAngle
        // #print("padAngle: ")
        // #print(padAngle)
        // angle = inAngle
        // #print("Angle: ")
        // #print(angle)
        // step1 = padCos
        // #print("step 1")
        // #print(step1)
        // step2 = -padSin * (angle - padAngle)/1
        // #print("step 2")
        // #print(step2)
        // step3 = -padCos * (np.float_power((angle - padAngle),2))/2
        // #print("step 3")
        // #print(step3)
        // step4 = padSin * (np.float_power((angle - padAngle),3))/6
        // #print("step 4")
        // #print(step4)
        // step5 = padCos * (np.float_power((angle - padAngle),4))/24
        // #print("step 5")
        // #print(step5)
        // sum = step1+step2+step3+step4+step5
        // #print("sum")
        // #print(sum)
        // return sum
    }
}

/*
fn extract_rmc_fields(rmc_sentence: &str) -> message {
    // Split the RMC sentence into fields

    // We don't have Vec!!!!
    let fields: Vec<&str> = rmc_sentence.split(',').collect();

    // Check that the sentence has the correct number of fields
    if fields.len() != 13 {
        return message::BadLen
    }

    // Check that the sentence is an RMC sentence
    if fields[0] != "$GPRMC" {
        return message::NotRMC
    }

    // Check that the GPS receiver has a valid fix
    if fields[2] != "A" {
        return message::NoFIX
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
}*/