//! The hardware controls for the agent

use std::convert::Infallible;

use inputs::Input;
use outputs::Output;
use slotmap::{new_key_type, SlotMap};
use urdf_rs::Robot;

pub mod inputs;
pub mod outputs;

#[derive(thiserror::Error, Debug)]
/// Any error that may come from a peripheral
pub enum PeripheralError {
    #[cfg(feature = "rpi")]
    #[error("SPI Error")]
    /// A raspberry pi spi error
    SpiError(#[from] rppal::spi::Error),
    #[cfg(feature = "rpi")]
    #[error("GPIO Error")]
    /// A raspberry pi gpio error
    GpioError(#[from] rppal::gpio::Error),
    #[cfg(feature = "rpi")]
    #[error("Uart Error")]
    /// A raspberry pi uart error
    UartError(#[from] rppal::uart::Error),
    #[cfg(feature = "rpi")]
    #[error("I2C Error")]
    /// A raspberry pi i2c error
    I2CError(#[from] rppal::i2c::Error),
    #[cfg(feature = "rpi")]
    #[error("PWM Error")]
    /// A raspberry pi pwm error
    PwmError(#[from] rppal::pwm::Error),
    #[error("Infallible")]
    /// An error that will never be returned
    Infallible(#[from] Infallible),
}

#[cfg(feature = "rpi")]
pub mod pi_peripherals;

/// Abstraction over the hardware of the device. This is still TODO because I don't exactly know
/// how we want to do this. My main idea is we can have an enum for Input/Output, and from there
/// some sort of method for reading or writing. The a body would only be a Vec<Peripheral> but we
/// could maybe also do a graph like structure
#[derive(Default)]
pub struct Body {
    /// The root (can be thought of as the base hardware). If a peripheral is added without being
    /// linked with another, root points to it
    pub root: Vec<PeripheralKey>,
    /// The dependency graph of all Peripherals
    pub peripheral_graph: SlotMap<PeripheralKey, PeripheralNode>,
    /// The urdf robot context
    pub urdf: Option<Robot>,
}

unsafe impl Send for Body {}
unsafe impl Sync for Body {}

impl Body {
    /// Returns all input nodes
    pub fn inputs(&self) -> Vec<&PeripheralNode> {
        self.peripheral_graph
            .iter()
            .filter(|(_, node)| node.peripheral.is_input())
            .map(|(_, node)| node)
            .collect::<Vec<_>>()
    }

    /// Returns all output nodes
    pub fn outputs(&self) -> Vec<&PeripheralNode> {
        self.peripheral_graph
            .iter()
            .filter(|(_, node)| node.peripheral.is_output())
            .map(|(_, node)| node)
            .collect::<Vec<_>>()
    }

    /// Gets a peripheral by its ID if it exists
    pub fn get_by_id(&self, id: PeripheralKey) -> Option<&PeripheralNode> {
        self.peripheral_graph.get(id)
    }

    /// Gets a mutable reference to a peripheral by its ID if it exists
    pub fn get_by_id_mut(&mut self, id: PeripheralKey) -> Option<&mut PeripheralNode> {
        self.peripheral_graph.get_mut(id)
    }
}

/// An arbitrary peripheral that is either an input or output
pub enum Peripheral {
    /// Input peripheral
    Input(Box<dyn Input<Error = PeripheralError>>),
    /// Output peripheral
    Output(Box<dyn Output<Error = PeripheralError>>),
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

impl From<Peripheral> for PeripheralNode {
    fn from(value: Peripheral) -> Self {
        PeripheralNode {
            peripheral: value,
            points_to: None,
        }
    }
}

new_key_type! {
    /// The peripheral's ID
    pub struct PeripheralKey;
}

/// Builder for a Body
pub struct Builder {
    /// The root peripheral and all peripherals with no parents
    root: Vec<PeripheralKey>,
    /// All peripherals
    graph: SlotMap<PeripheralKey, PeripheralNode>,
}

impl Builder {
    /// Adds a node to the root
    pub fn with_node(mut self, node: Peripheral) -> Self {
        let id = self.graph.insert(node.into());
        self.root.push(id);
        self
    }

    /// Adds a node that's connected to another
    pub fn add_node_to(&mut self, node: Peripheral, connected_to: PeripheralKey) {
        let id = self.graph.insert(node.into());
        if let Some(nodes) = &mut self.graph[connected_to].points_to {
            nodes.push(id)
        } else {
            self.graph[connected_to].points_to = Some(vec![id]);
        }
    }

    /// Constructs a body from a nodeset
    pub fn build(self) -> Body {
        Body {
            root: self.root,
            peripheral_graph: self.graph,
            urdf: None,
        }
    }
}
