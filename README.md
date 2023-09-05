# Robotrs

A port of WPILib and REVLib to Rust for use in the FIRST Robotics Competition.

## Security

The `hal-sys`, `ctre`, and `revlib` packages all generate bindings to C
libraries at compile time. The headers to these libraries and the binaries
themselves are automatically downloaded at compile time. All code related to
the downloading of these libs can be found in the `build-utils` crate and the
`build.rs` of the 3 crates mentioned earlier.

## Not affilited with or supported by WPILib, REV Robotics, or FIRST

## Getting Started

0. Install the compiler toolchain by running the `installRoboRioToolchain` task
   in allwpilib and placing the generated `bin` to the path.
   `echo 'export PATH="$HOME/.gradle/toolchains/frc/2023/roborio/bin:$PATH"' >> ~/.zshrc`
   will add the the path automatically on Mac.
1. Create a new binary Rust crate with `cargo new <NAME>`
2. Add the following to the ./.cargo/config.toml file
```toml
[target.arm-unknown-linux-gnueabi]
linker = "arm-frc2023-linux-gnueabi-gcc"
rustflags = ["-C", "target-cpu=cortex-a9"]

[build]
target = "arm-unknown-linux-gnueabi"
```

3. Add the required dependencies to the `Cargo.toml` file. For example:
```toml
[dependencies]
robotrs = { git = "https://github.com/BlueZeeKing/robotrs.git", tag="v0.1.0" }
revlib = { git = "https://github.com/BlueZeeKing/robotrs.git", tag="v0.1.0" }
ctre = { git = "https://github.com/BlueZeeKing/robotrs.git", tag="v0.1.0" }
anyhow = "1.0.75" # This is an error handling library that is used extensively
```

4. In the `src/main.rs` file run the scheduler with a struct that implements
   the `AsyncRobot` trait. For example:
```rust
fn main() {
    robotrs::scheduler::RobotScheduler::start_robot(example::Robot::new());
}
```

5. Deployment is a WIP
