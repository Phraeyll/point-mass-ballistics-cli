pub use self::Adjustment::*;

use point_mass_ballistics::Length;

pub mod plain;
pub mod pretty;

pub enum Adjustment<'n> {
    Elevation(&'n Length),
    Windage(&'n Length),
}

// Show needed adjustments to correct shot
impl Adjustment<'_> {
    pub fn adjustment(&self, output_tolerance: Length) -> char {
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
