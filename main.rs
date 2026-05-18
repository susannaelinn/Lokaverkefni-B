mod equipment;
mod desk;
mod chair;
mod projector;
mod location;
mod category;
mod database;

use std::convert::TryFrom;

use equipment::Equipment;
use location::Location;
use category::EquipmentCategory;
use desk::Desk;

use database::create_database;
use database::insert_equipment;
use database::list_equipment;
use database::delete_equipment;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let conn = create_database()?;

    println!("Database connected.");

    let location = Location::try_from("HA-2301")?;

    let desk = Desk {
        seats: 4,
    };

    let equipment = Equipment {

        value: 50000,

        location,

        category: EquipmentCategory::Desk(desk),
    };

    insert_equipment(&conn, &equipment)?;

    println!("Equipment inserted.");

    println!("\nEquipment List:");

    list_equipment(&conn)?;

    println!("\nDeleting equipment with ID 1...");

    delete_equipment(&conn, 1)?;

    println!("Deleted.");

    println!("\nUpdated Equipment List:");

    list_equipment(&conn)?;

    Ok(())
}
