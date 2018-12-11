pub mod detail;

use self::detail::Detail;

pub enum Component {
    Uniform(Detail),
    Composite(Device),
}

pub struct Device {
    name: String,
    units: Vec<Component>,
}

pub struct Machine(pub Component);