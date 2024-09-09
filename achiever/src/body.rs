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
#[derive(Default)]
pub struct Body {
    /// The dependency graph of all Peripherals
    pub peripheral_graph: SlotMap<PeripheralKey, PeripheralNode>,
}

impl Body {
    /// Returns all input nodes
    pub fn inputs(&self) -> Vec<&PeripheralNode> {
        self.peripheral_graph.iter().filter(|(_, node)| node.peripheral.is_input()).map(|(_, node)| node).collect::<Vec<_>>()
    }

    /// Returns all output nodes
    pub fn outputs(&self) -> Vec<&PeripheralNode> {
        self.peripheral_graph.iter().filter(|(_, node)| node.peripheral.is_output()).map(|(_, node)| node).collect::<Vec<_>>()
    }
}

/// An arbitrary peripheral that is either an input or output
pub enum Peripheral {
    /// Input peripheral
    Input(Box<dyn Input>),
    /// Output peripheral
    Output(Box<dyn Output>),
}

impl Peripheral {
    /// Returns true if the peripheral is an input
    pub fn is_input(&self) -> bool {
        matches!(self, Self::Input(_))
    }

    /// Returns true if the peripheral is an output
    pub fn is_output(&self) -> bool {
        matches!(self, Self::Output(_))
    }
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
