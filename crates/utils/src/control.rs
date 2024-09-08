use robotrs::{
    math::{Controller, State},
    time::get_time,
};

pub async fn run_controller<I, O, E, C, Goal, Supplier, Consumer, Check>(
    mut goal: Goal,
    mut supplier: Supplier,
    mut consumer: Consumer,
    mut checker: Check,
    mut controller: C,
) -> Result<(), E>
where
    C: Controller<State = I, Output = O>,
    Goal: FnMut() -> I,
    Supplier: FnMut() -> Result<I, E>,
    Consumer: FnMut(O) -> Result<(), E>,
    Check: FnMut(&I, &I) -> bool,
{
    loop {
        let current_state = supplier()?;
        let target = goal();

        consumer(controller.calculate(&current_state, &target))?;

        if checker(&target, &current_state) {
            return Ok(());
        }
    }
}

pub fn position_to_state<FIn, E>(mut position: FIn) -> Result<impl FnMut() -> Result<State, E>, E>
where
    FIn: FnMut() -> Result<f32, E>,
{
    let mut last = position()?;
    let mut last_time = get_time();

    Ok(move || {
        let current = position()?;
        let current_time = get_time();

        let velocity = (current - last) / (current_time - last_time).as_secs_f32();

        let state = State {
            position: current,
            velocity,
        };

        last = current;
        last_time = current_time;

        Ok(state)
    })
}
