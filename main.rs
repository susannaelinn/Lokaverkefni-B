mod equipment;
mod desk;
mod chair;
mod projector;
mod location;
mod category;

use equipment::Equipment;
use location::Location;
use category::EquipmentCategory;
use desk::Desk;

fn main() {

    let location = Location {
        building: String::from("HA"),
        floor: 2,
        room: 301,
    };

    let desk = Desk {
        seats: 4,
    };

    let equipment = Equipment {
        id: 1,
        value: 50000,
        location,
        category: EquipmentCategory::Desk(desk),
    };

    println!("Equipment created.");
}
