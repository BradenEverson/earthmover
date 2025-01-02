
# the necessary imports
import random
from typing import List




class Environment:
    
    """ represents a simple environment for the agent to interact with """
    
    def __init__(self):
        """ initializes the environment's internal state, where the state is just a counter that limits
        the number of time steps the agent is allowed to take to interact with the environment """
        
        self.steps_left = 10
        
        
    def get_observation(self) -> List[float]:
        """ this method returns the current environment's observation to the agent, the observation vector
        is always zero, as the environment has no internal state """
        
        return [0.0, 0.0, 0.0]
    
    def get_actions(self) -> List[int]:
        """ allows the agent to query the set of actions it can execute. in this simplistic game, there are
        only two actions that can be carried out by the agent, which are encoded in 0 and 1 """
        
        return [0,1]
    
    def is_done(self) -> bool:
        """ this method signals the end of the episode to the agent """
        
        return self.steps_left == 0
    
    def action(self, action: int) -> float:
        """this method handles a agent's action and returns the reward for this action. in this example, the
        reward is random and its action is discarded. we also update the count of steps and don't continue the 
        episodes that are over """
        
        if self.is_done():
            raise Exception("Game is over")
        self.steps_left -= 1
        return random.random()
    

class Agent:
    
    """ the agents part is much simpler, and had only two methods: the constructor and the method that performs 
    one step in the environment """
    
    def __init__(self):
        """in the constructor, we initialize the counter that will keep the total reward accumulated by the agent 
        during the episode """
        
        self.total_reward = 0.0
        
    def step(self, env: Environment):
        """ allows for the following: 
        - observe the environment
        - make a decision about action to take based on observation
        - submit the action to the environment
        - get the reward for the current step """
        
        
        """here, the agent is dull and ingores the observations made during the decision making process about which
        action to take. instead, the action is selected randomly """
        
        current_obs = env.get_observation()
        actions = env.get_actions()
        reward = env.action(random.choice(actions))
        self.total_reward += reward
    

if __name__ == "__main__":
    """ here, we create both the classes and run one episode """
    
    env = Environment()
    agent = Agent()
    while not env.is_done():
        agent.step(env)
    print("Total reward got: %.4f" % agent.total_reward)    
    
    
    