use std::fmt;

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
