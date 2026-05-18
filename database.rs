use rusqlite::{Connection, Result};

use crate::equipment::Equipment;
use crate::category::EquipmentCategory;

pub fn create_database() -> Result<Connection> {

    let conn = Connection::open("school_equipment.db")?;

    conn.execute(
        "
        CREATE TABLE IF NOT EXISTS equipment (

            id INTEGER PRIMARY KEY AUTOINCREMENT,

            value INTEGER NOT NULL,

            building TEXT NOT NULL,

            floor INTEGER NOT NULL,

            room INTEGER NOT NULL,

            category TEXT NOT NULL
        )
        ",
        [],
    )?;

    Ok(conn)
}

pub fn insert_equipment(
    conn: &Connection,
    equipment: &Equipment,
) -> Result<()> {

    let category_name = match &equipment.category {

        EquipmentCategory::Desk(_) => "Desk",

        EquipmentCategory::Chair(_) => "Chair",

        EquipmentCategory::Projector(_) => "Projector",
    };

    conn.execute(
        "
        INSERT INTO equipment (

            value,
            building,
            floor,
            room,
            category

        )
        VALUES (?1, ?2, ?3, ?4, ?5)
        ",
        (
            equipment.value,
            &equipment.location.building,
            equipment.location.floor,
            equipment.location.room,
            category_name,
        ),
    )?;

    Ok(())
}

pub fn list_equipment(conn: &Connection) -> Result<()> {

    let mut stmt = conn.prepare(
        "
        SELECT
            id,
            value,
            building,
            floor,
            room,
            category
        FROM equipment
        "
    )?;

    let equipment_iter = stmt.query_map([], |row| {

        Ok((
            row.get::<_, i32>(0)?,
            row.get::<_, i32>(1)?,
            row.get::<_, String>(2)?,
            row.get::<_, u8>(3)?,
            row.get::<_, u16>(4)?,
            row.get::<_, String>(5)?,
        ))
    })?;

    for item in equipment_iter {

        let (
            id,
            value,
            building,
            floor,
            room,
            category
        ) = item?;

        println!(
            "ID: {} | Value: {} | Location: {}-{}{} | Category: {}",
            id,
            value,
            building,
            floor,
            room,
            category
        );
    }

    Ok(())
}

pub fn delete_equipment(
    conn: &Connection,
    id: i32,
) -> Result<()> {

    conn.execute(
        "
        DELETE FROM equipment
        WHERE id = ?1
        ",
        [id],
    )?;

    Ok(())
}

pub fn update_location(
    conn: &Connection,
    id: i32,
    new_location: &crate::location::Location,
) -> Result<()> {

    conn.execute(
        "
        UPDATE equipment

        SET
            building = ?1,
            floor = ?2,
            room = ?3

        WHERE id = ?4
        ",
        (
            &new_location.building,
            new_location.floor,
            new_location.room,
            id,
        ),
    )?;

    Ok(())
}

pub fn filter_by_building(
    conn: &Connection,
    building_name: &str,
) -> Result<()> {

    let mut stmt = conn.prepare(
        "
        SELECT
            id,
            value,
            building,
            floor,
            room,
            category

        FROM equipment

        WHERE building = ?1
        "
    )?;

    let equipment_iter = stmt.query_map(
        [building_name],
        |row| {

            Ok((
                row.get::<_, i32>(0)?,
                row.get::<_, i32>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, u8>(3)?,
                row.get::<_, u16>(4)?,
                row.get::<_, String>(5)?,
            ))
        },
    )?;

    for item in equipment_iter {

        let (
            id,
            value,
            building,
            floor,
            room,
            category
        ) = item?;

        println!(
            "ID: {} | Value: {} | Location: {}-{}{} | Category: {}",
            id,
            value,
            building,
            floor,
            room,
            category
        );
    }

    Ok(())
}

pub fn sorted_equipment(
    conn: &Connection,
) -> Result<()> {

    let mut stmt = conn.prepare(
        "
        SELECT
            id,
            value,
            building,
            floor,
            room,
            category

        FROM equipment

        ORDER BY
            building,
            floor,
            room
        "
    )?;

    let equipment_iter = stmt.query_map([], |row| {

        Ok((
            row.get::<_, i32>(0)?,
            row.get::<_, i32>(1)?,
            row.get::<_, String>(2)?,
            row.get::<_, u8>(3)?,
            row.get::<_, u16>(4)?,
            row.get::<_, String>(5)?,
        ))
    })?;

    for item in equipment_iter {

        let (
            id,
            value,
            building,
            floor,
            room,
            category
        ) = item?;

        println!(
            "ID: {} | Value: {} | Location: {}-{}{} | Category: {}",
            id,
            value,
            building,
            floor,
            room,
            category
        );
    }

    Ok(())
}

