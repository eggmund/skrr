use na::Vector2;

pub fn vec_determinant(v1: &Vector2<f32>, v2: &Vector2<f32>) -> f32 {
    // |v1, v2| = (v1.x * v2.y) - (v2.x * v1.y)
    (v1.x * v2.y) - (v2.x * v1.y)
}