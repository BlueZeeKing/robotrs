use crate::{define_conversion, define_unit};

define_unit!(Meter);
define_unit!(Centimeter);
define_unit!(Inch);
define_unit!(Foot);
define_unit!(Millimeter);

define_conversion!(Meter, Centimeter, 100.0);
define_conversion!(Meter, Millimeter, 1000.0);
define_conversion!(Meter, Inch, 39.3701);
define_conversion!(Meter, Foot, 3.28084);

define_conversion!(Centimeter, Inch, 0.393701);
define_conversion!(Centimeter, Foot, 0.0328084);
define_conversion!(Centimeter, Millimeter, 10.0);

define_conversion!(Inch, Foot, 0.0833333);
define_conversion!(Inch, Millimeter, 25.4);

define_conversion!(Foot, Millimeter, 304.8);
