pub use self::Adjustment::*;

use rballistics_flat::model::point_mass::Numeric;

pub mod plain;
pub mod pretty;

pub enum Adjustment<'n> {
    Elevation(&'n Numeric),
    Windage(&'n Numeric),
}

// Show needed adjustments to correct shot
impl Adjustment<'_> {
    fn adjustment(&self, output_tolerance: Numeric) -> char {
        let tolerance = output_tolerance;
        match self {
            Elevation(&m) => {
                if m > -tolerance && m < tolerance {
                    '*'
                } else if m.is_sign_positive() {
                    'D'
                } else {
                    'U'
                }
            }
            Windage(&m) => {
                if m > -tolerance && m < tolerance {
                    '*'
                } else if m.is_sign_positive() {
                    'L'
                } else {
                    'R'
                }
            }
        }
    }
}