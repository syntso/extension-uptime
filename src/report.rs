use serde::Serialize;

use crate::load::Load;
use crate::up::Up;

#[derive(Debug, Serialize)]
pub struct Report {
    pub up: Up,
    pub load: Load,
}
