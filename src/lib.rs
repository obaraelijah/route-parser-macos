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
    v4,
    v6,
}