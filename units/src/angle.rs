use crate::{define_conversion, define_unit};

define_unit!(Radian);
define_unit!(Degree);
define_unit!(Rotation);

define_conversion!(Rotation, Degree, 360.0);
define_conversion!(Rotation, Radian, 6.283185307);

define_conversion!(Radian, Degree, 57.29577951);
