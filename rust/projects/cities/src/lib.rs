extern crate csv;
#[macro_use]
extern crate serde_derive;

use csv::Reader;

#[derive(Debug, Deserialize)]
pub struct Args {
    arg_origin: String,
    arg_destination: String,
}

#[derive(Debug)]
struct Coordinates {
    long: f64,
    lat: f64,
}

impl Coordinates {
    fn distance_from(&self, orig: &Coordinates) -> f64 {
        const EARTH_RADIUS: f64 = 6371000f64;

        let orig_lat = orig.lat.to_radians();
        let dest_lat = self.lat.to_radians();

        let delta_lat = dest_lat - orig_lat;
        let delta_long = (self.long - orig.long).to_radians();

        let a = (delta_lat / 2.0).sin().powf(2.0)
            + orig_lat.cos() * dest_lat.cos() * (delta_long / 2.0).sin().powf(2.0);

        let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

        EARTH_RADIUS * c
    }
}

#[derive(Debug)]
pub enum CoordinatesError {
    Io(::std::io::Error),
    CityNotFound(String),
}

impl ::std::fmt::Display for CoordinatesError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            CoordinatesError::Io(ref io_err) => io_err.fmt(f),
            CoordinatesError::CityNotFound(ref city) => write!(f, "City not found: {}", city),
        }
    }
}

impl ::std::error::Error for CoordinatesError {
    fn description(&self) -> &str {
        match *self {
            CoordinatesError::Io(ref io_err) => io_err.description(),
            CoordinatesError::CityNotFound(_) => "city not found",
        }
    }
}

impl From<::std::io::Error> for CoordinatesError {
    fn from(io_err: ::std::io::Error) -> CoordinatesError {
        CoordinatesError::Io(io_err)
    }
}

impl Coordinates {
    fn from_city(city: &str) -> Result<Coordinates, CoordinatesError> {
        // Err(format!("City not found: {}", city))
        Ok(Coordinates {
            long: 0.0,
            lat: 0.0,
        })
    }
}

pub fn run(args: Args) -> Result<(), CoordinatesError> {
    println!("{:?}", args);

    let orig = args.arg_origin;
    let dest = args.arg_destination;

    let coord_orig = Coordinates::from_city(&orig)?;
    let coord_dest = Coordinates::from_city(&dest)?;

    println!("Coordinates of origin ({}): {:?}", orig, coord_orig);
    println!("Coordinates of destination ({}): {:?}", dest, coord_dest);

    println!(
        "Distance between {} and {}: {}",
        orig,
        dest,
        coord_dest.distance_from(&coord_orig)
    );

    Ok(())
}
