//! Movement instructions for an Agent's Body

use serde::{Deserialize, Serialize};

use crate::body::PeripheralKey;

/// A single instruction of movement for an Agent
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Instruction {
    /// The node affected by the instruction
    pub node: PeripheralKey,
    /// The timespan this action lasts for
    pub lasts_for_ms: u32,
    /// The arbitrary data to send to the agent
    pub instructions: [u8; 4],
}

#[cfg(test)]
mod tests {
    use super::Instruction;

    #[test]
    fn instruction_has_no_padding() {
        let size = std::mem::size_of::<Instruction>();
        let align = std::mem::align_of::<Instruction>();

        assert_eq!(size, 16);
        assert_eq!(align, 4);
    }
}
