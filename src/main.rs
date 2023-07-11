use std::io;

mod fixation;

use crate::fixation::fixation as fix;
pub mod gun_posn;

fn main() {
    let start: fix::Grid = fix::Grid{
        mgrs_zone: 19,
        mgrssub_zone: 'T',
        mgrs_square: ['G', 'L'],
        northing: 69000.0,
        easting: 13000.0,
        altitude: 85.0,
    };
    
    let correction = fix::Polar{
        bearing: 1650.0,
        distance: 1000.0,
        angle_of_site: -3.1
    };

    let end: fix::Grid = fix::polar_to_grid(start, correction);

    fix::print_grid(end);
}
