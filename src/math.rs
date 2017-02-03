// Precise method, which guarantees v = v1 when t = 1.
pub fn lerp(v0: i32, v1: i32, t: f32) -> i32 {
    return ((1.0 - t) * v0 as f32 + t * v1 as f32) as i32;
}


pub fn lerp_pos(pos0: (i32, i32), pos1: (i32, i32), t: f32) -> (i32, i32) {
    (lerp(pos0.0, pos1.0, t), lerp(pos0.1, pos1.1, t))
}
