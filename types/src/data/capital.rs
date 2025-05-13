use geo::Coord;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Capital {
    pub name: String,
    pub coordinates: Coord<f32>,
}
