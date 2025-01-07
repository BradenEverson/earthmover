//! The Agent/Data Collection Unit. Collects metadata into a buffer and sends it away to the local
//! or cloud server, where it then receives a testable moveset and performs it accordingly

#[cfg(all(feature = "jetson", feature = "rpi"))]
compile_error! {"Jetson Nano and Raspberry Pi cannot both be targetted by the same sysetem"}

pub mod body;
pub mod brain;
pub mod communication;
pub mod goals;
pub mod protocol;
