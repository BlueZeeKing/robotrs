# Robotrs

A port of WPILib and REVLib to Rust for use in the FIRST Robotics Competition.

## Security

The `hal-sys`, `ctre`, and `revlib` packages all generate bindings to C
libraries at compile time. The headers to these libraries and the binaries
themselves are automatically downloaded at compile time. All code related to
the downloading of these libs can be found in the `build-utils` crate and the
`build.rs` of the 3 crates mentioned earlier.

## Not affilited with or supported by WPILib, REV Robotics, CTRE, or FIRST

## Quick Start

0. Install the compiler toolchain by running the `installRoboRioToolchain` task
   in allwpilib or downloading from
   (Github)[https://github.com/wpilibsuite/opensdk/releases) and putting the
   `bin` directory on the path. 
1. Create a new binary Rust crate with `cargo new <NAME>`
2. Add the following to the ./.cargo/config.toml file
```toml
[target.arm-unknown-linux-gnueabi]
linker = "arm-frc2023-linux-gnueabi-gcc"
rustflags = ["-C", "target-cpu=cortex-a9"]

[build]
target = "arm-unknown-linux-gnueabi"

[env]
LIBS_OUT_DIR = { value = "target/lib", relative = true }
```

3. Add the required dependencies to the `Cargo.toml` file. For example:
```toml
[dependencies]
robotrs = { git = "https://github.com/BlueZeeKing/robotrs.git", tag="v0.1.0" }
revlib = { git = "https://github.com/BlueZeeKing/robotrs.git", tag="v0.1.0" }
ctre = { git = "https://github.com/BlueZeeKing/robotrs.git", tag="v0.1.0" }
anyhow = "1.0.75" # This is an error handling library that is used extensively
```

4. Add a `rust-toolchain.toml` file to the root of the project with the following contents:
```toml
[toolchain]
channel = "nightly"
targets = ["arm-unknown-linux-gnueabi"]
```

5. In the `src/main.rs` file run the scheduler with closure that produces a struct that implements
   the `AsyncRobot` trait. For example:
```rust
fn main() {
    robotrs::scheduler::RobotScheduler::start_robot(|| example::Robot::new());
}
```

6. Deploy your code by first installing the deployment tool:
```
cargo install cargo-deploy --git https://github.com/BlueZeeKing/robotrs.git
```
Then running it:
```
cargo deploy [TEAM NUMBER]
```
