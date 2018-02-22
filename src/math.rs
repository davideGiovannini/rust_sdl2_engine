// Precise method, which guarantees v = v1 when t = 1.
pub fn lerp(v0: i32, v1: i32, t: f32) -> i32 {
    ((1.0 - t) * v0 as f32 + t * v1 as f32) as i32
}

pub fn lerp_pos(pos0: (i32, i32), pos1: (i32, i32), t: f32) -> (i32, i32) {
    (lerp(pos0.0, pos1.0, t), lerp(pos0.1, pos1.1, t))
}

pub fn lerp_color(color0: (u8, u8, u8, u8), color1: (u8, u8, u8, u8), t: f32) -> (u8, u8, u8, u8) {
    (
        lerp(color0.0 as i32, color1.0 as i32, t) as u8,
        lerp(color0.1 as i32, color1.1 as i32, t) as u8,
        lerp(color0.2 as i32, color1.2 as i32, t) as u8,
        lerp(color0.3 as i32, color1.3 as i32, t) as u8,
    )
}

#[cfg(test)]
mod test {
    use super::*;

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
