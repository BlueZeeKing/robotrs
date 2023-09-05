# Robotrs

A port of WPILib and REVLib to Rust for use in the FIRST Robotics Competition.

## Security

The `hal-sys`, `ctre`, and `revlib` packages all generate bindings to C
libraries at compile time. The headers to these libraries and the binaries
themselves are automatically downloaded at compile time. All code related to
the downloading of these libs can be found in the `build-utils` crate and the
`build.rs` of the 3 crates mentioned earlier.

## Not affilited with or supported by WPILib, REV Robotics, or FIRST
