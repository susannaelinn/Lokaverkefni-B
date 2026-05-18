pub enum ChairType {
    Office,
    School,
    Gaming,
    Other(String),
}

pub struct Chair {
    pub chair_type: ChairType,
}
