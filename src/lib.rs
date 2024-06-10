mod route_entry;
mod routing_flag;

pub enum Entity {
    Default,
    Link(String),
}

pub struct Destination {
    pub entity: Entity,
}

pub enum Protocol {
    V4,
    V6,
}
