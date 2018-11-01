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

#[derive(Debug)]
enum CoordinatesError {
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

pub fn run(args: Args) -> Result <(), String> {
    println!("{:?}", args);

    let orig = args.arg_origin;
    let dest = args.arg_destination;

    let coord_orig = Coordinates::from_city(&orig)?;
    let coord_dest = Coordinates::from_city(&dest)?;

    println!("Coordinates of origin ({}): {:?}", orig, coord_orig);
    println!("Coordinates of destination ({}): {:?}", dest, coord_dest);

    Ok(())
}
