use crate::location::Location;
use crate::category::EquipmentCategory;

pub struct Equipment {
    pub id: i32,
    pub value: i32,
    pub location: Location,
    pub category: EquipmentCategory,
}
