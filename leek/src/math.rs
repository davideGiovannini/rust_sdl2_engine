// Precise method, which guarantees v = v1 when t = 1.
pub fn lerp(v0: f32, v1: f32, t: f32) -> f32 {
    ((1.0 - t) * v0 + t * v1)
}

// Precise method, which guarantees v = v1 when t = 1.
pub fn lerp_i32(v0: i32, v1: i32, t: f32) -> i32 {
    ((1.0 - t) * v0 as f32 + t * v1 as f32) as i32
}

pub fn lerp_pos(pos0: (i32, i32), pos1: (i32, i32), t: f32) -> (i32, i32) {
    (lerp_i32(pos0.0, pos1.0, t), lerp_i32(pos0.1, pos1.1, t))
}

pub fn lerp_color(color0: (u8, u8, u8, u8), color1: (u8, u8, u8, u8), t: f32) -> (u8, u8, u8, u8) {
    (
        lerp_i32(i32::from(color0.0), i32::from(color1.0), t) as u8,
        lerp_i32(i32::from(color0.1), i32::from(color1.1), t) as u8,
        lerp_i32(i32::from(color0.2), i32::from(color1.2), t) as u8,
        lerp_i32(i32::from(color0.3), i32::from(color1.3), t) as u8,
    )
}

pub fn format_bytes(num: f64) -> String {
    if num == 0_f64 {
        return "0 bytes".to_string();
    }

    use std::cmp;
    let units = ["bytes", "kB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"];
    let delimiter = 1000_f64;
    let exponent = cmp::min(
        (num.ln() / delimiter.ln()).floor() as i32,
        (units.len() - 1) as i32,
    );
    let pretty_bytes = format!("{:.2}", num / delimiter.powi(exponent))
        .parse::<f64>()
        .unwrap() * 1_f64;
    let unit = units[exponent as usize];
    format!("{} {}", pretty_bytes, unit)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_f_bytes() {
        assert_eq!(&format_bytes(0_f64), "0 bytes");
        assert_eq!(&format_bytes(8_f64), "8 bytes");
        assert_eq!(&format_bytes(80_f64), "80 bytes");
        assert_eq!(&format_bytes(800_f64), "800 bytes");
        assert_eq!(&format_bytes(8000_f64), "8 kB");
        assert_eq!(&format_bytes(80000_f64), "80 kB");
        assert_eq!(&format_bytes(800000_f64), "800 kB");
        assert_eq!(&format_bytes(8000000_f64), "8 MB");
        assert_eq!(&format_bytes(80000000_f64), "80 MB");
        assert_eq!(&format_bytes(800000000_f64), "800 MB");
        assert_eq!(&format_bytes(8000000000_f64), "8 GB");
    }

    #[test]
    fn test_lerp() {
        assert_eq!(lerp(0, 100, 1.0), 100);
        assert_eq!(lerp(0, 100, 0.0), 0);
        assert_eq!(lerp(0, 100, 0.5), 50);
    }

    #[test]
    fn test_lerp_color() {
        let starting_color = (255, 0, 255, 0);
        let ending_color = (0, 255, 0, 255);

        let middle_color = (127, 127, 127, 127);
        assert_eq!(lerp_color(starting_color, ending_color, 1.0), ending_color);
        assert_eq!(
            lerp_color(starting_color, ending_color, 0.0),
            starting_color
        );
        assert_eq!(lerp_color(starting_color, ending_color, 0.5), middle_color);
    }
}
