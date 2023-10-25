use crate::{define_conversion, define_unit};

define_unit!(Hour);
define_unit!(Minute);
define_unit!(Second);
define_unit!(Millisecond);

define_conversion!(Hour, Minute, 60.0);
define_conversion!(Hour, Second, 3_600.0);
define_conversion!(Hour, Millisecond, 3_600_000.0);

define_conversion!(Minute, Second, 60.0);
define_conversion!(Minute, Millisecond, 60_000.0);

define_conversion!(Second, Millisecond, 1_000.0);
