//! The arguments to be passed through to a simulation and outputs that can be returned

/// Any agruments that a simulation may take in
pub struct SimArgs;

impl AsRef<SimArgs> for &str {
    fn as_ref(&self) -> &SimArgs {
        todo!()
    }
}

/// The output from a simulation's runtime
pub struct SimRes;
