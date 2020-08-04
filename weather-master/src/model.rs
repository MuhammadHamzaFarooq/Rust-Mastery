// We deactivate the 'unused' lint on this crate because that seems less destructive than
// removing a lot of model properties that are not currently used by may eventually be used.
#![allow(unused)]

use serde::{Deserialize, Deserializer};
use std::error;
use std::fmt;

#[derive(Deserialize)]
pub struct Weather {
    name: String,

    #[serde(rename = "coord")]
    coords: Coords,

    #[serde(rename = "main")]
    conditions: Conditions,

    wind: Wind,
}

#[derive(Deserialize)]
pub struct Coords {
    lon: f64,
    lat: f64,
}

#[derive(Deserialize)]
pub struct Conditions {
    temp: f32,
    humidity: i32,
    pressure: f32,
    temp_min: f32,
    temp_max: f32,
}

#[derive(Deserialize)]
pub struct Wind {
    speed: f32,
    deg: f32,
}

pub enum WindDisplay {
    N(f32, f32),
    NE(f32, f32),
    E(f32, f32),
    SE(f32, f32),
    S(f32, f32),
    SW(f32, f32),
    W(f32, f32),
    NW(f32, f32),
}

impl Wind {
    pub fn display(&self) -> WindDisplay {
        match self.deg % 360.0 {
            // North is split in two, because half is on the left side of 0.
            deg if deg >= 337.5 && deg < 360.0 => WindDisplay::N(deg, self.speed), 
            deg if deg >= 0.0 && deg < 22.5 => WindDisplay::N(deg, self.speed),
            
            deg if deg >= 22.5 && deg < 67.5 => WindDisplay::NE(deg, self.speed),
            deg if deg >= 67.5 && deg < 112.5 => WindDisplay::E(deg, self.speed),
            deg if deg >= 112.5 && deg < 157.5 => WindDisplay::SE(deg, self.speed),
            deg if deg >= 157.5 && deg < 202.5 => WindDisplay::S(deg, self.speed),
            deg if deg >= 202.5 && deg < 247.5 => WindDisplay::SW(deg, self.speed),
            deg if deg >= 247.5 && deg < 292.5 => WindDisplay::W(deg, self.speed),
            deg if deg >= 292.5 && deg < 337.5 => WindDisplay::NW(deg, self.speed),

            _ => panic!("What a world! What a world...."),
        }
    }
}

impl fmt::Display for WindDisplay {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            WindDisplay::N(deg, speed) => write!(f, "Wind {:.0} mph N", speed * (11.0 / 25.0)),
            WindDisplay::NE(deg, speed) => write!(f, "Wind {:.0} mph NE", speed * (11.0 / 25.0)),
            WindDisplay::E(deg, speed) => write!(f, "Wind {:.0} mph E", speed * (11.0 / 25.0)),
            WindDisplay::SE(deg, speed) => write!(f, "Wind {:.0} mph SE", speed * (11.0 / 25.0)),
            WindDisplay::S(deg, speed) => write!(f, "Wind {:.0} mph S", speed * (11.0 / 25.0)),
            WindDisplay::SW(deg, speed) => write!(f, "Wind {:.0} mph SW", speed * (11.0 / 25.0)),
            WindDisplay::W(deg, speed) => write!(f, "Wind {:.0} mph W", speed * (11.0 / 25.0)),
            WindDisplay::NW(deg, speed) => write!(f, "Wind {:.0} mph NW", speed * (11.0 / 25.0)),
        }
    }
}

impl Weather {
    /// The name of the nearest city.
    pub fn city(&self) -> &str {
        &self.name
    }

    /// Temperature in Fahrenheit.
    pub fn temperature(&self) -> f32 {
        self.conditions.temp * (9.0 / 5.0) - 459.67
    }

    /// Wind speed in miles per hour.
    pub fn wind(&self) -> WindDisplay {
        self.wind.display()
    }
}

#[derive(Debug)]
pub struct ApiError {
    code: i32,
    message: String,
}

impl<'a> Deserialize<'a> for ApiError {
    fn deserialize<D: Deserializer<'a>>(d: D) -> Result<Self, D::Error> {
        use serde::de::Error;

        // This is practically the canonical implementation of my favorite deserialization
        // pattern: start with a template to transform text into something structured, then
        // derive your real data based on that template.
        //
        // In this case, I also make use of a fairly easy mechanism for converting numeric
        // parsing errors into deserialization errors.

        #[derive(Deserialize)]
        struct Template {
            #[serde(rename = "cod")]
            code: String,
            message: String,
        }

        let Template { code, message } = Template::deserialize(d)?;
        Ok(Self {
            code: code.parse().map_err(|e| D::Error::custom(e))?,
            message,
        })
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.code, self.message)
    }
}

impl error::Error for ApiError {
    fn description(&self) -> &str {
        "An API error occurred"
    }
}

#[cfg(test)]
mod tests {
    use serde_json as json;

    static DATA: &'static str = include_str!("../response.json");

    #[test]
    fn deserialize() {
        json::from_str::<super::Weather>(DATA).unwrap();
    }
}
