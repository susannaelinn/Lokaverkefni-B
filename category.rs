use crate::desk::Desk;
use crate::chair::Chair;
use crate::projector::Projector;

pub enum EquipmentCategory {
    Desk(Desk),
    Chair(Chair),
    Projector(Projector),
}
