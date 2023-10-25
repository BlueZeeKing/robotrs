use units::{length::Meter, time::Second, Unit};

fn main() {
    let meters = Meter::new(5.0);
    let seconds = Second::new(1.0);

    let mps = meters / seconds;

    let _accel = mps / Second::new(1.0);
}
