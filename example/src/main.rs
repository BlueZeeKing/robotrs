use robotrs::FailableDefault;

fn main() {
    robotrs::scheduler::RobotScheduler::start_robot(example::Robot::failable_default());
}
