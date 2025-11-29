// conversions
pub const INCHES_IN_PIXELS: f64 = 0.010417_f64;
pub const INCHES_IN_FEET: f64 = 12.0_f64;
pub const FEET_IN_MILES: f64 = 5_280.0_f64;

// landmarks
pub const THE_SHIRE_TO_BREE: f64 = 120.0_f64;
pub const BREE_TO_RIVENDELL: f64 = 300.0_f64;
pub const RIVENDELL_TO_LOTHLORIEN: f64 = 175.0_f64;
pub const LOTHLORIEN_TO_PARTH_GALEN: f64 = 300.0_f64;
pub const PARTH_GALEN_TO_THE_BLACK_GATES: f64 = 160.0_f64;
pub const THE_BLACK_GATES_TO_MINAS_MORGUL: f64 = 110.0_f64;
pub const MINAS_MORGUL_TO_MOUNT_DOOM: f64 = 70.0_f64;

pub const TOTAL_WALKING_DISTANCE: f64 = THE_SHIRE_TO_BREE
    + BREE_TO_RIVENDELL
    + RIVENDELL_TO_LOTHLORIEN
    + LOTHLORIEN_TO_PARTH_GALEN
    + PARTH_GALEN_TO_THE_BLACK_GATES
    + THE_BLACK_GATES_TO_MINAS_MORGUL
    + MINAS_MORGUL_TO_MOUNT_DOOM;

// pub const TOTAL_WALKING_DISTANCE: f64 = 1_235.0_f64;

// maybe these ar are more correct?

// landmarks
// pub const THE_SHIRE_TO_BREE_V2: f64 = 120.0_f64;
// pub const BREE_TO_RIVENDELL_V2: f64 = 300.0_f64;

// pub const RIVENDELL_TO_LOTHLORIEN_V2: f64 = 462.0_f64;
// pub const LOTHLORIEN_TO_PARTH_GALEN_V2: f64 = 300.0_f64;
// pub const PARTH_GALEN_TO_THE_BLACK_GATES_V2: f64 = 160.0_f64;
// pub const THE_BLACK_GATES_TO_MINAS_MORGUL_V2: f64 = 110.0_f64;
// pub const MINAS_MORGUL_TO_MOUNT_DOOM_V2: f64 = 70.0_f64;

// pub const TOTAL_WALKING_DISTANCE_V2: f64 =
//     MINAS_MORGUL_TO_MOUNT_DOOM_V2 + THE_BLACK_GATES_TO_MINAS_MORGUL_V2;

// 458 miles from Hobbiton to Rivendell (not following the road)

// 462 miles from Rivendell to Lothlorien

// 389 miles from Lothlorien to the falls of Rauros (not sure if this counts as walking?)

// 470 miles from the falls of Rauros to Mt. Doom

// 50 miles from Cormallen to Minas Tirith

// 535 miles from Minas Tirith to Isengard

// 693 miles from Isengard to Rivendell

// 397 miles from Rivendell to Bag End (following the road)

// 260 miles from Hobbiton to the Grey Havens

// So I've got about 1800 miles from Hobbiton to Mount Doom, but that's forgetting the return journey which is about as long. You could get to 1350 from Hobbiton to Mount Doom if you don't count the boat trip down the Anduin.
