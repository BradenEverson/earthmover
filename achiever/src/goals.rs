
/// Goals will be a modular abstraction over anything that we want the agent to do. It will be
/// modular as this REWARD can be anything from a boolean to a dynamic reward type. It could be the
/// reading from one or many peripherals. I think we should have some sort of exposed breadboard
/// parts on the machine to include these dynamic peripherals. 
///
///
/// For example, we could hook the agent up with a fire sensor, the Goal in this case would be Goal::Maximum and
/// Reward would be an f32 to represent the reading from the flame sensor. During data collection
/// the simulation would become aware of areas of higher flame concentration and infer to go close
/// to these sources.
pub enum Goal<REWARD: Rewardable> {
    Maximize(REWARD),
    Minimize(REWARD),
    None,
}

/// Basic implementations of reward function for primitive types that make sense
///
pub trait Rewardable {
    fn to_reward(&self) -> f64;
}

impl Rewardable for f32 {
    fn to_reward(&self) -> f64 {
        *self as f64
    }
}

impl Rewardable for f64 {
    fn to_reward(&self) -> f64 {
        *self
    }
}

impl Rewardable for usize {
    fn to_reward(&self) -> f64 {
        *self as f64
    }
}

impl Rewardable for u8 {
    fn to_reward(&self) -> f64 {
        *self as f64
    }
}
