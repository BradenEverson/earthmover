
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
        if self.is_done():
            raise Exception("Game is over")
        self.steps_left -= 1
        return random.random()
    
    
    
    
    