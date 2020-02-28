use point_mass_ballistics::Length;

pub mod plain;
pub mod pretty;

#[derive(Clone, Copy)]
pub enum Adjustment {
    Elevation(Length),
    Windage(Length),
}

// Show needed adjustments to correct shot
impl Adjustment {
    pub fn adjustment(&self, tolerance: Length) -> char {
        let (n, positive, negative) = match *self {
            Self::Elevation(n) => (n, 'D', 'U'),
            Self::Windage(n) => (n, 'L', 'R'),
        };
        if n > tolerance {
            positive
        } else if n < -tolerance {
            negative
        } else {
            '*'
        }
    }
}
