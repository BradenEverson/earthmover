//! Function implementation for converting raw lidar readings to a 3-space distance vector

/// Offset along p
const OFFSET_P: f32 = -0.523;
/// Offset along Nu
const OFFSET_N: f32 = 1.608;
/// Offset along r
const OFFSET_R: f32 = -0.004;

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

    let (l_x, l_y, l_z) = if theta_l != 0f32.to_radians() && theta_l != 180f32.to_radians() {
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
    let d_x = (distance * l_x) + (OFFSET_P * r_x) + (OFFSET_R * r_x) + (OFFSET_N * nu_x);
    let d_y = (distance * l_y) + (OFFSET_P * r_y) + (OFFSET_R * r_y) + (OFFSET_N * nu_y);
    let d_z = (distance * l_z) + (OFFSET_P * r_z) + (OFFSET_R * r_z) + (OFFSET_N * nu_z);

    (d_x, d_y, d_z)
}
