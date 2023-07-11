
mod gun_posn{
    use crate::fixation::fixation::{
        Grid,
        Polar,
        polar_to_grid,
        grid_to_polar,
        print_grid,
        print_polar
    };
    use crate::guns::guns::{
        Gun,
        M777,
        C3,
        LG1
    };
    struct GunPosn{
        pub battery_center: Grid,
        pub number_of_guns: i8,
        pub name: String,
        pub cs: String,
        pub c_of_a: i8,
        pub wpn_type: Gun,
        pub gun_platforms: [Gun; GunPosn::number_of_guns]
    }

    struct GunPlatform{
        gun_number: i8,
        gun_cs: String,
        gun_posn: Grid,
        gun_posn_polar: Polar,
        min_elevation: i32,
        svy_state: char
    }

    pub fn new_gun_posn(battery_center: Grid, number_of_guns: i8, name: String, cs: String, c_of_a: i8, wpn_type: Gun) -> GunPosn{
        let gun_posn: GunPosn = GunPosn{
            battery_center: battery_center,
            number_of_guns: number_of_guns,
            name: name,
            cs: cs,
            c_of_a: c_of_a,
            wpn_type: wpn_type,
            gun_platforms: gun_platforms
        };
        return gun_posn;
    }
}