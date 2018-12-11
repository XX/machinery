use strum_macros::EnumDiscriminants;
use crate::unit::{Meter, Watt};

#[derive(EnumDiscriminants)]
#[strum_discriminants(derive(Hash))]
pub enum DetailKind {
    Roll(Roll),
    Tube(Tube),
    Bar(Bar),
    Join(Join),
    Engine(Engine),
    Wheel(Wheel),
    Gear(Gear),
    Belt(Belt),
}

pub struct Roll {
    radius: Meter,
    length: Meter,
}

pub struct Tube {
    radius: Meter,
    length: Meter,
    width: Meter,
}

pub struct Bar {
    length: Meter,
    width: Meter,
    height: Meter,
}

pub struct Join {
    radius: Meter,
}

pub struct Engine {
    power: Watt,
}

pub struct Wheel {
    radius: Meter,
}

pub struct Gear {
    radius: Meter,
}

pub struct Belt {
}

pub struct Detail {
    kind: DetailKind,
    name: String,
}
