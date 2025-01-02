
# the necessary imports
import gymnasium as gym
import random

class RandomActionWrapper(gym.ActionWrapper):
    """ initializing the wrapper by calling the parent's __init__ method and saving epsilon
    epsilon - the probability of random action """
    
    def __init__(self, env: gym.Env, epsilon: float=0.1):
        super(RandomActionWrapper, self).__init__(env)
        self.epsilon = epsilon
        
    def action(self, action: gym.core.WrapperActType) -> gym.core.WrapperActType:
        """ method for tweaking a agent's actions """
        
        """ in reinforcement learning, to encourage exploration, the agent occasionally takes 
        random actions instead of its usual choice with a small probability called epsilon
        
        this helps the agent to discover new strategies and avoid getting stuck in suboptimal
        behaviors. The "wrapper" is a tool that adds this random behavior to the agent and ensures
        it works with different environments, such as Gym """
        
        if random.random() < self.epsilon:
            action = self.env.action_space.sample()
            print(f"Random action {action}")
            return action
        return action
    
if __name__ == "__main__":
    
    # here we create a cartpole environment and pass it to the wrapper constructor
    env = RandomActionWrapper(gym.make("CartPole-v1"))
    
    obs = env.reset()
    total_reward = 0.0
    
    while True:
        """ here is the same code as the random agent, except everytime, we issue the same action 0,
        so the agent is dull and does the same thing """
        obs, reward, done, _, _ = env.step(0)
        total_reward += reward
        
        if done:
            break
    
    print(f"Reward got: {total_reward:.2f}")
    

