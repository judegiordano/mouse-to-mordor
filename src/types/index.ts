export type DistanceTraveled = {
	total_pixels_traveled: number
	total_inches_traveled: number
	total_feet_traveled: number
	total_miles_traveled: number
}

export type Landmarks = {
	the_shire_to_bree: number
	bree_to_rivendell: number
	rivendell_to_lothlorien: number
	lothlorien_to_parth_galen: number
	parth_galen_to_the_black_gates: number
	the_black_gates_to_minas_morgul: number
	minas_morgul_to_mount_doom: number
	total_walking_distance: number
}

export type Landmark = 'THE_SHIRE_TO_BREE' |
	'BREE_TO_RIVENDELL' |
	'RIVENDELL_TO_LOTHLORIEN' |
	'LOTHLORIEN_TO_PARTH_GALEN' |
	'PARTH_GALEN_TO_THE_BLACK_GATES' |
	'THE_BLACK_GATES_TO_MINAS_MORGUL' |
	'MINAS_MORGUL_TO_MOUNT_DOOM' |
	'TOTAL_WALKING_DISTANCE'

export type Progress = {
	distance_traveled: DistanceTraveled
	landmarks: [Landmark, number][]
}
