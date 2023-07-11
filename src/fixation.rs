// use std::f64;

pub mod fixation{

    const MIL_TO_DEG: f64 = 0.05625;
    const RAD_TO_DEG: f64 = 57.295779513082320876798154814105;
    const DEG_TO_RAD: f64 = 0.017453292519943295769236907684886;
    pub struct Grid{
        pub mgrs_zone: i8,
        pub mgrssub_zone: char,
        pub mgrs_square: [char; 2],
        pub northing: f64,
        pub easting: f64,
        pub altitude: f64,
    }

    pub struct Polar{
        pub bearing: f64,
        pub distance: f64,
        pub angle_of_site: f64
    }


    pub fn grid_to_polar(start: Grid, end :Grid) -> Polar{
        let mut bearing: f64;

        let diff_easting = end.easting - start.easting;
        let diff_northing = end.northing - start.northing;

        if diff_easting.is_sign_negative() && diff_northing.is_sign_negative(){
            let rad_val = f64::atan(diff_northing/diff_easting);
            rad_val.to_degrees(); 
            bearing = 4800.0 - (rad_val / MIL_TO_DEG);
        }
        else if diff_easting.is_sign_negative() && diff_northing.is_sign_positive(){
            let rad_val = f64::atan(diff_northing/diff_easting);
            rad_val.to_degrees(); 
            bearing = 4800.0 + (rad_val / MIL_TO_DEG);
        }
        else if diff_easting.is_sign_positive() && diff_northing.is_sign_negative(){
            let rad_val = f64::atan(diff_northing/diff_easting);
            rad_val.to_degrees(); 
            bearing = 1600.0 - (rad_val / MIL_TO_DEG);
        }
        else if diff_easting.is_sign_positive() && diff_northing.is_sign_positive(){
            let rad_val = f64::atan(diff_northing/diff_easting);
            rad_val.to_degrees(); 
            bearing = 1600.0 + (rad_val / MIL_TO_DEG);
        } else{
            bearing = 0.0;
        }
        
        let distance = ((diff_easting.powi(2)) + (diff_northing.powi(2))).sqrt();
        let mut angle_of_site = f64::asin((end.altitude - start.altitude)/distance);
        angle_of_site.to_degrees();
        angle_of_site = angle_of_site / MIL_TO_DEG;
        let polar: Polar = Polar{
            bearing: bearing,
            distance: distance,
            angle_of_site: angle_of_site
        };

        return polar;

    }

    pub fn polar_to_grid(start: Grid, polar: Polar) -> Grid{

        let mgrs_zone = start.mgrs_zone;
        let mgrssub_zone = start.mgrssub_zone;
        let mgrs_square = start.mgrs_square;
        let mut northing = start.northing;
        let mut easting = start.easting;
        let mut altitude = start.altitude;

        easting = easting + (polar.distance * f64::sin((polar.bearing * MIL_TO_DEG).to_radians()));
        northing = northing + (polar.distance * f64::cos((polar.bearing * MIL_TO_DEG).to_radians()));
        altitude = altitude + polar.angle_of_site;

        let grid: Grid = Grid{
            mgrs_zone: mgrs_zone,
            mgrssub_zone: mgrssub_zone,
            mgrs_square: mgrs_square,
            northing: northing,
            easting: easting,
            altitude: altitude
        };

        return grid;
    }

    pub fn print_grid(grid: Grid){
        println!("MGRS Zone: {}", grid.mgrs_zone);
        println!("MGRS Sub Zone: {}", grid.mgrssub_zone);
        println!("MGRS Square: {}{}", grid.mgrs_square[0], grid.mgrs_square[1]);
        println!("Easting: {}", grid.easting.round());
        println!("Northing: {}", grid.northing.round());
        println!("Altitude: {}", grid.altitude.round());
    }

    pub fn print_polar(polar: Polar){
        println!("Bearing: {}", polar.bearing);
        println!("Distance: {}", polar.distance);
        println!("Angle of Site: {}", polar.angle_of_site);
    }

}