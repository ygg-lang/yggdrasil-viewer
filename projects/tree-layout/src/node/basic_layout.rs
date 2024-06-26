use crate::Coordinate;

/// <img src="https://i.ibb.co/BLCfz0g/image.png" width="300" alt="Relative position"/>
///
/// Relative position illustration
#[derive(Debug, Clone)]
pub struct BoundingBox {
    pub total_width: Coordinate,
    pub total_height: Coordinate,
}

impl Default for BoundingBox {
    fn default() -> Self {
        Self { total_height: 0., total_width: 0. }
    }
}
