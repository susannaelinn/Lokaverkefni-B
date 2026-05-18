mod equipment;
mod desk;
mod chair;
mod projector;
mod location;
mod category;
mod database;
mod json;

use std::convert::TryFrom;

use equipment::Equipment;
use location::Location;
use category::EquipmentCategory;

use desk::Desk;
use chair::{Chair, ChairType};
use projector::Projector;

use database::create_database;
use database::insert_equipment;
use database::list_equipment;
use database::update_location;
use database::delete_equipment;
use database::filter_by_building;
use database::sorted_equipment;

use json::save_to_json;
use json::load_json;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let conn = create_database()?;

    println!("Database connected.");

    // Desk equipment
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

    println!("\nDesk inserted.");

    // Chair equipment
    let chair_location = Location::try_from("H-1201")?;

    let chair = Chair {
        chair_type: ChairType::Office,
    };

    let chair_equipment = Equipment {

        value: 25000,

        location: chair_location,

        category: EquipmentCategory::Chair(chair),
    };

    insert_equipment(
        &conn,
        &chair_equipment,
    )?;

    println!("\nChair inserted.");

    // Projector equipment
    let projector_location = Location::try_from("S-3302")?;

    let projector = Projector {
        lumens: 5000,
    };

    let projector_equipment = Equipment {

        value: 120000,

        location: projector_location,

        category: EquipmentCategory::Projector(projector),
    };

    insert_equipment(
        &conn,
        &projector_equipment,
    )?;

    println!("\nProjector inserted.");

    // Save JSON
    save_to_json(&equipment)?;

    println!("\nEquipment saved to JSON.");

    // Load JSON
    let json_content = load_json()?;

    println!("\nLoaded JSON:");

    println!("{}", json_content);

    // Show all equipment
    println!("\nAll Equipment:");

    list_equipment(&conn)?;

    // Filter equipment
    println!("\nFiltered by building HA:");

    filter_by_building(
        &conn,
        "HA",
    )?;

    // Sorted equipment
    println!("\nSorted Equipment:");

    sorted_equipment(&conn)?;

    // Update location
    let new_location = Location::try_from("H-1105")?;

    update_location(
        &conn,
        1,
        &new_location,
    )?;

    println!("\nLocation updated.");

    // Updated list
    println!("\nUpdated Equipment List:");

    list_equipment(&conn)?;

    // Delete equipment
    println!("\nDeleting equipment with ID 1...");

    delete_equipment(&conn, 1)?;

    println!("Deleted.");

    // Final list
    println!("\nFinal Equipment List:");

    list_equipment(&conn)?;

    Ok(())
}
