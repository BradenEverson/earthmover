//! The hardware controls for the agent

use inputs::Input;
use outputs::Output;
use slotmap::{new_key_type, SlotMap};

pub mod inputs;
pub mod outputs;

/// Abstraction over the hardware of the device. This is still TODO because I don't exactly know
/// how we want to do this. My main idea is we can have an enum for Input/Output, and from there
/// some sort of method for reading or writing. The a body would only be a Vec<Peripheral> but we
/// could maybe also do a graph like structure
pub struct Body {
    /// The dependency graph of all Peripherals
    pub peripheral_graph: SlotMap<PeripheralKey, PeripheralNode>,
}

/// An arbitrary peripheral that is either an input or output
pub enum Peripheral {
    /// Input peripheral
    Input(Box<dyn Input>),
    /// Output peripheral
    Output(Box<dyn Output>),
}

/// A node in a hardware graph that describes what peripheral it is and what it's connected in some
/// way to
pub struct PeripheralNode {
    /// The peripheral at this node
    pub peripheral: Peripheral,
    /// All peripherals this peripheral connects to
    pub points_to: Option<Vec<PeripheralKey>>,
}

new_key_type! {
    /// The peripheral's ID
    pub struct PeripheralKey;
}
