//! Function implementation for converting raw lidar readings to a 3-space distance vector

use std::f32::consts::PI;

/// Offset along p
const OFFSET_P: f32 = -0.523;
/// Offset along Nu
const OFFSET_N: f32 = 1.608;
/// Offset along r
const OFFSET_R: f32 = -0.004;
/// The threshold for two floating points being "close enough"
#[cfg(test)]
const THRESHOLD: f32 = 0.0001;

/// Takes a matrix of theta_r, theta_p, theta_l and distances and converts it into a matrix of x,
/// y, z coordinates
pub fn lidar_angle_matrix_to_3_space_matrix(
    input: Vec<(f32, f32, f32, f32)>,
) -> Vec<(f32, f32, f32)> {
    input
        .iter()
        .map(|(theta_r, theta_p, theta_l, dist)| {
            convert_raw_lidar_to_vector_space(*theta_r, *theta_p, *theta_l, *dist)
        })
        .collect()
}

/// Given a lidar's roll, pitch and raw angles, alongside a set of fixed offsets from the
/// center of rotation to a lidar, generates the appropriate Distance vector
pub fn convert_raw_lidar_to_vector_space(
    theta_r: f32,
    theta_p: f32,
    theta_l: f32,
    distance: f32,
) -> (f32, f32, f32) {
    let theta_r = f32::to_radians(theta_r);
    let theta_p = f32::to_radians(theta_p);
    let theta_l = f32::to_radians(theta_l);

    let r_x = 0f32;
    let r_y = f32::cos(theta_r);
    let r_z = f32::sin(theta_r);

    let p_perp = f32::sin(theta_p);
    let p_x = f32::cos(theta_p);
    let p_y = p_perp * f32::sin(-theta_r);
    let p_z = p_perp * f32::cos(theta_r);

    // Cross Product
    let nu_x = (p_y * r_z) - (p_z * r_y);
    let nu_y = (p_z * r_x) - (p_x * r_z);
    let nu_z = (p_x * r_y) - (p_y * r_x);

    // N1 magnitude
    let nl = f32::sin(theta_l);

    let nl_x = nu_x * nl;
    let nl_y = nu_y * nl;
    let nl_z = nu_z * nl;

    let (l_x, l_y, l_z) = if theta_l != 0f32 && theta_l != PI {
        (
            ((p_x * f32::cos(theta_l)) - (p_y * nl_z) + (p_z * nl_y))
                / (p_x.powi(2) + p_y.powi(2) + p_z.powi(2)),
            ((p_y * f32::cos(theta_l)) - (p_z * nl_x) + (p_x * nl_z))
                / (p_x.powi(2) + p_y.powi(2) + p_z.powi(2)),
            ((p_z * f32::cos(theta_l)) - (p_x * nl_y) + (p_y * nl_x))
                / (p_x.powi(2) + p_y.powi(2) + p_z.powi(2)),
        )
    } else if theta_l == 0f32 {
        (p_x, p_y, p_z)
    } else {
        (-p_x, -p_y, -p_z)
    };

    // Solve for Vector D
    let d_x = (distance * l_x) + (OFFSET_P * p_x) + (OFFSET_R * r_x) + (OFFSET_N * nu_x);
    let d_y = (distance * l_y) + (OFFSET_P * p_y) + (OFFSET_R * r_y) + (OFFSET_N * nu_y);
    let d_z = (distance * l_z) + (OFFSET_P * p_z) + (OFFSET_R * r_z) + (OFFSET_N * nu_z);

    (d_x, d_y, d_z)
}

#[cfg(test)]
/// Test function to check if points are close to one another
pub fn check_if_points_are_close(point: f32, expected: f32) {
    let delta = (point - expected).abs();
    assert!(delta < THRESHOLD);
}

#[cfg(test)]
/// Creates a test function for expected lidar 3-space vectors
macro_rules! test_lidar_point {
    ($name:ident, $theta_r:literal, $theta_p:literal, $theta_l:literal, $d:literal, $x:literal, $y:literal, $z:literal) => {
        #[test]
        fn $name() {
            let (dx, dy, dz) = convert_raw_lidar_to_vector_space($theta_r, $theta_p, $theta_l, $d);
            check_if_points_are_close(dx, $x);
            check_if_points_are_close(dy, $y);
            check_if_points_are_close(dz, $z);
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::{check_if_points_are_close, convert_raw_lidar_to_vector_space};
    test_lidar_point!(point_0, 45.0f32, -90.0f32, 30.0f32, 1.0f32, 1.608, 0.59328, 0.10817);
    test_lidar_point!(point_1, 43.2f32, -88.2f32, 30.0f32, 1.0f32, 1.618, 0.56169, 0.12642);
    test_lidar_point!(point_2, 41.4f32, -86.4f32, 30.0f32, 1.0f32, 1.6264, 0.53168, 0.14695);
    test_lidar_point!(point_3, 39.6f32, -84.6f32, 30.0f32, 1.0f32, 1.6331, 0.5034, 0.16963);
    test_lidar_point!(point_4, 37.8f32, -82.8f32, 30.0f32, 1.0f32, 1.6383, 0.47698, 0.19434);
    test_lidar_point!(point_5, 36.0f32, -81.0f32, 30.0f32, 1.0f32, 1.6419, 0.45256, 0.22095);
    test_lidar_point!(point_6, 34.2f32, -79.2f32, 30.0f32, 1.0f32, 1.6438, 0.43026, 0.24932);
    test_lidar_point!(point_7, 32.4f32, -77.4f32, 30.0f32, 1.0f32, 1.6441, 0.41021, 0.27929);
    test_lidar_point!(point_8, 30.6f32, -75.6f32, 30.0f32, 1.0f32, 1.6428, 0.39249, 0.31071);
    test_lidar_point!(point_9, 28.8f32, -73.8f32, 30.0f32, 1.0f32, 1.6399, 0.37722, 0.34342);
    test_lidar_point!(point_10, 27.0f32, -72.0f32, 30.0f32, 1.0f32, 1.6353, 0.36446, 0.37724);
    test_lidar_point!(point_11, 25.2f32, -70.2f32, 30.0f32, 1.0f32, 1.6291, 0.35429, 0.41201);
    test_lidar_point!(point_12, 23.4f32, -68.4f32, 30.0f32, 1.0f32, 1.6214, 0.34678, 0.44754);
    test_lidar_point!(point_13, 21.6f32, -66.6f32, 30.0f32, 1.0f32, 1.612, 0.34197, 0.48365);
    test_lidar_point!(point_14, 19.8f32, -64.8f32, 30.0f32, 1.0f32, 1.601, 0.3399, 0.52016);
    test_lidar_point!(point_15, 18.0f32, -63.0f32, 30.0f32, 1.0f32, 1.5885, 0.34058, 0.55688);
    test_lidar_point!(point_16, 16.2f32, -61.2f32, 30.0f32, 1.0f32, 1.5744, 0.34405, 0.59362);
    test_lidar_point!(point_17, 14.4f32, -59.4f32, 30.0f32, 1.0f32, 1.5587, 0.35028, 0.63019);
    test_lidar_point!(point_18, 12.6f32, -57.6f32, 30.0f32, 1.0f32, 1.5415, 0.35928, 0.66641);
    test_lidar_point!(point_19, 10.8f32, -55.8f32, 30.0f32, 1.0f32, 1.5228, 0.37102, 0.70208);
    test_lidar_point!(point_20, 9.0f32, -54.0f32, 30.0f32, 1.0f32, 1.5025, 0.38545, 0.73702);
    test_lidar_point!(point_21, 7.2f32, -52.2f32, 30.0f32, 1.0f32, 1.4808, 0.40254, 0.77104);
    test_lidar_point!(point_22, 5.4f32, -50.4f32, 30.0f32, 1.0f32, 1.4576, 0.42221, 0.80397);
    test_lidar_point!(point_23, 3.6f32, -48.6f32, 30.0f32, 1.0f32, 1.433, 0.44441, 0.83564);
    test_lidar_point!(point_24, 1.8f32, -46.8f32, 30.0f32, 1.0f32, 1.407, 0.46903, 0.86586);
    test_lidar_point!(point_25, 0.0f32, -45.0f32, 30.0f32, 1.0f32, 1.3796, 0.496, 0.89447);
    test_lidar_point!(point_26, -1.8f32, -43.2f32, 30.0f32, 1.0f32, 1.3508, 0.5252, 0.92132);
    test_lidar_point!(point_27, -3.6f32, -41.4f32, 30.0f32, 1.0f32, 1.3207, 0.55651, 0.94626);
    test_lidar_point!(point_28, -5.4f32, -39.6f32, 30.0f32, 1.0f32, 1.2893, 0.58982, 0.96913);
    test_lidar_point!(point_29, -7.2f32, -37.8f32, 30.0f32, 1.0f32, 1.2566, 0.62498, 0.9898);
    test_lidar_point!(point_30, -9.0f32, -36.0f32, 30.0f32, 1.0f32, 1.2227, 0.66186, 1.0081);
    test_lidar_point!(point_31, -10.8f32, -34.2f32, 30.0f32, 1.0f32, 1.1875, 0.70029, 1.0241);
    test_lidar_point!(point_32, -12.6f32, -32.4f32, 30.0f32, 1.0f32, 1.1512, 0.74013, 1.0374);
    test_lidar_point!(point_33, -14.4f32, -30.6f32, 30.0f32, 1.0f32, 1.1138, 0.7812, 1.0481);
    test_lidar_point!(point_34, -16.2f32, -28.8f32, 30.0f32, 1.0f32, 1.0753, 0.82333, 1.0561);
    test_lidar_point!(point_35, -18.0f32, -27.0f32, 30.0f32, 1.0f32, 1.0357, 0.86634, 1.0612);
    test_lidar_point!(point_36, -19.8f32, -25.2f32, 30.0f32, 1.0f32, 0.99503, 0.91005, 1.0635);
    test_lidar_point!(point_37, -21.6f32, -23.4f32, 30.0f32, 1.0f32, 0.95343, 0.95428, 1.0629);
    test_lidar_point!(point_38, -23.4f32, -21.6f32, 30.0f32, 1.0f32, 0.91088, 0.99882, 1.0592);
    test_lidar_point!(point_39, -25.2f32, -19.8f32, 30.0f32, 1.0f32, 0.86744, 1.0435, 1.0526);
    test_lidar_point!(point_40, -27.0f32, -18.0f32, 30.0f32, 1.0f32, 0.82314, 1.0881, 1.043);
    test_lidar_point!(point_41, -28.8f32, -16.2f32, 30.0f32, 1.0f32, 0.77802, 1.1324, 1.0303);
    test_lidar_point!(point_42, -30.6f32, -14.4f32, 30.0f32, 1.0f32, 0.73214, 1.1763, 1.0147);
    test_lidar_point!(point_43, -32.4f32, -12.6f32, 30.0f32, 1.0f32, 0.68554, 1.2196, 0.99603);
    test_lidar_point!(point_44, -34.2f32, -10.8f32, 30.0f32, 1.0f32, 0.63826, 1.2619, 0.97443);
    test_lidar_point!(point_45, -36.0f32, -9.0f32, 30.0f32, 1.0f32, 0.59035, 1.3033, 0.94993);
    test_lidar_point!(point_46, -37.8f32, -7.2f32, 30.0f32, 1.0f32, 0.54186, 1.3433, 0.92258);
    test_lidar_point!(point_47, -39.6f32, -5.4f32, 30.0f32, 1.0f32, 0.49283, 1.382, 0.89245);
    test_lidar_point!(point_48, -41.4f32, -3.6f32, 30.0f32, 1.0f32, 0.44332, 1.4191, 0.85963);
    test_lidar_point!(point_49, -43.2f32, -1.8f32, 30.0f32, 1.0f32, 0.39336, 1.4544, 0.82421);
    test_lidar_point!(point_50, -45.0f32, 0.0f32, 30.0f32, 1.0f32, 0.34303, 1.4878, 0.7863);
    test_lidar_point!(point_51, 45.0f32, -90.0f32, 15.0f32, 8.0f32, 1.608, 6.55556, -3.63301);
    test_lidar_point!(point_52, 43.2f32, -88.2f32, 21.9f32, 19.0f32, 2.14452, 16.8325, -7.5782);
    test_lidar_point!(point_53, 41.4f32, -86.4f32, 28.8f32, 18.0f32, 2.56242, 16.5003, -5.60931);
    test_lidar_point!(point_54, 39.6f32, -84.6f32, 35.7f32, 12.0f32, 2.46873, 11.1482, -2.49653);
    test_lidar_point!(point_55, 37.8f32, -82.8f32, 42.6f32, 13.0f32, 2.72912, 12.327, -1.54159);
    test_lidar_point!(point_56, 36.0f32, -81.0f32, 49.5f32, 12.0f32, 2.72554, 11.4519, -0.244823);
    test_lidar_point!(point_57, 34.2f32, -79.2f32, 56.4f32, 5.0f32, 1.99999, 4.51075, 0.764754);
    test_lidar_point!(point_58, 32.4f32, -77.4f32, 63.3f32, 7.0f32, 2.1413, 6.45998, 1.48417);
    test_lidar_point!(point_59, 30.6f32, -75.6f32, 70.2f32, 10.0f32, 2.26982, 9.30383, 2.74361);
    test_lidar_point!(point_60, 28.8f32, -73.8f32, 77.1f32, 5.0f32, 1.70966, 4.32577, 2.23995);
    test_lidar_point!(point_61, 27.0f32, -72.0f32, 84.0f32, 17.0f32, 1.9168, 15.3764, 7.05386);
    test_lidar_point!(point_62, 25.2f32, -70.2f32, 90.9f32, 4.0f32, 1.31449, 3.14864, 2.69279);
    test_lidar_point!(point_63, 23.4f32, -68.4f32, 97.8f32, 5.0f32, 1.05275, 3.86386, 3.53435);
    test_lidar_point!(point_64, 21.6f32, -66.6f32, 104.7f32, 4.0f32, 0.864923, 2.83894, 3.32901);
    test_lidar_point!(point_65, 19.8f32, -64.8f32, 111.6f32, 5.0f32, 0.44858, 3.41391, 4.22981);
    test_lidar_point!(point_66, 18.0f32, -63.0f32, 118.5f32, 9.0f32, -0.754328, 5.96644, 7.21945);
    test_lidar_point!(point_67, 16.2f32, -61.2f32, 125.4f32, 7.0f32, -0.796352, 4.14014, 6.18709);
    test_lidar_point!(point_68, 14.4f32, -59.4f32, 132.3f32, 19.0f32, -5.39139, 10.5549, 15.3834);
    test_lidar_point!(point_69, 12.6f32, -57.6f32, 139.2f32, 9.0f32, -2.57312, 4.19613, 8.16762);
    test_lidar_point!(point_70, 10.8f32, -55.8f32, 146.1f32, 4.0f32, -0.830168, 1.42258, 4.42733);
    test_lidar_point!(point_71, 9.0f32, -54.0f32, 153.0f32, 19.0f32, -8.9572, 6.15911, 16.2275);
    test_lidar_point!(point_72, 7.2f32, -52.2f32, 159.9f32, 20.0f32, -10.5615, 4.77969, 16.9723);
    test_lidar_point!(point_73, 5.4f32, -50.4f32, 166.8f32, 9.0f32, -4.67963, 1.27231, 8.33609);
    test_lidar_point!(point_74, 3.6f32, -48.6f32, 173.7f32, 3.0f32, -1.11164, 0.0927109, 3.70557);
    test_lidar_point!(point_75, 1.8f32, -46.8f32, 180.6f32, 6.0f32, -3.29289, -0.250726, 5.85059);
    test_lidar_point!(point_76, 0.0f32, -45.0f32, 187.5f32, 9.0f32, -5.54231, -1.17874, 7.81636);
    test_lidar_point!(point_77, -1.8f32, -43.2f32, 194.4f32, 12.0f32, -7.7533, -2.68882, 9.57587);
    test_lidar_point!(point_78, -3.6f32, -41.4f32, 201.3f32, 6.0f32, -3.52215, -1.84962, 5.37562);
    test_lidar_point!(point_79, -5.4f32, -39.6f32, 208.2f32, 13.0f32, -8.20573, -5.28464, 9.4144);
    test_lidar_point!(point_80, -7.2f32, -37.8f32, 215.1f32, 15.0f32, -9.12467, -7.41889, 10.1225);
    test_lidar_point!(point_81, -9.0f32, -36.0f32, 222.0f32, 5.0f32, -2.48404, -2.71516, 4.26967);
    test_lidar_point!(point_82, -10.8f32, -34.2f32, 228.9f32, 3.0f32, -1.15984, -1.71257, 3.10838);
    test_lidar_point!(point_83, -12.6f32, -32.4f32, 235.8f32, 6.0f32, -2.42747, -4.09537, 4.44543);
    test_lidar_point!(point_84, -14.4f32, -30.6f32, 242.7f32, 7.0f32, -2.39508, -5.21192, 4.72934);
    test_lidar_point!(point_85, -16.2f32, -28.8f32, 249.6f32, 9.0f32, -2.43275, -7.21937, 5.40098);
    test_lidar_point!(point_86, -18.0f32, -27.0f32, 256.5f32, 11.0f32, -2.02399, -9.30001, 6.00368);
    test_lidar_point!(
        point_87, -19.8f32, -25.2f32, 263.4f32, 2.0f32, 0.00343207, -1.27162, 2.34489
    );
    test_lidar_point!(point_88, -21.6f32, -23.4f32, 270.3f32, 6.0f32, 0.18746, -4.96717, 3.76383);
    test_lidar_point!(point_89, -23.4f32, -21.6f32, 277.2f32, 17.0f32, 2.08671, -15.1237, 7.52884);
    test_lidar_point!(point_90, -25.2f32, -19.8f32, 284.1f32, 1.0f32, 0.281823, -0.196714, 1.86923);
    test_lidar_point!(point_91, -27.0f32, -18.0f32, 291.0f32, 19.0f32, 6.47523, -15.9958, 7.68657);
    test_lidar_point!(point_92, -28.8f32, -16.2f32, 297.9f32, 15.0f32, 6.68663, -11.7494, 6.1533);
    test_lidar_point!(point_93, -30.6f32, -14.4f32, 304.8f32, 10.0f32, 5.42116, -6.93487, 4.41291);
    test_lidar_point!(point_94, -32.4f32, -12.6f32, 311.7f32, 12.0f32, 7.63088, -7.59936, 4.75398);
    test_lidar_point!(point_95, -34.2f32, -10.8f32, 318.6f32, 5.0f32, 3.4717, -2.19022, 2.66699);
    test_lidar_point!(point_96, -36.0f32, -9.0f32, 325.5f32, 10.0f32, 7.87478, -4.36173, 3.63968);
    test_lidar_point!(point_97, -37.8f32, -7.2f32, 332.4f32, 20.0f32, 17.267, -7.66824, 5.23868);
    test_lidar_point!(point_98, -39.6f32, -5.4f32, 339.3f32, 11.0f32, 9.87486, -2.56447, 3.00627);
    test_lidar_point!(point_99, -41.4f32, -3.6f32, 346.2f32, 11.0f32, 10.2404, -1.33177, 2.46313);
    test_lidar_point!(point_100, -43.2f32, -1.8f32, 353.1f32, 5.0f32, 4.4891, 0.563926, 1.48386);
    test_lidar_point!(point_101, -45.0f32, 0.0f32, 360.0f32, 10.0f32, 9.477, 1.1342, 1.13986);
}
