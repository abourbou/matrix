
pub mod projection_matrix;
pub mod view_matrix;
pub mod translation;
pub mod rotation;
pub mod scaling;

pub use projection_matrix::projection_matrix;
pub use view_matrix::view_matrix;
pub use translation::{translation, translation_v};
<<<<<<< HEAD
pub use scaling::scaling;
=======
pub use scaling::{scaling, scaling_v};
pub use rotation::{quat_to_rotation, axis_angle_to_quaternion};
pub use slerp::slerp;
>>>>>>> cb02c6c... fetch transfo
