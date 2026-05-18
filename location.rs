use std::fmt;
use std::convert::TryFrom;

pub struct Location {
    pub building: String,
    pub floor: u8,
    pub room: u16,
}

impl fmt::Display for Location {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        write!(
            f,
            "{}-{}{}",
            self.building,
            self.floor,
            self.room
        )
    }
}

impl TryFrom<&str> for Location {

    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {

        let parts: Vec<&str> = value.split('-').collect();

        if parts.len() != 2 {
            return Err(String::from("Invalid format"));
        }

        let building = parts[0].to_string();

        let numbers = parts[1];

        if numbers.len() < 3 {
            return Err(String::from("Invalid room format"));
        }

        let floor = numbers[0..1]
            .parse::<u8>()
            .map_err(|_| String::from("Invalid floor"))?;

        let room = numbers[1..]
            .parse::<u16>()
            .map_err(|_| String::from("Invalid room"))?;

        Ok(Location {
            building,
            floor,
            room,
        })
    }
}
