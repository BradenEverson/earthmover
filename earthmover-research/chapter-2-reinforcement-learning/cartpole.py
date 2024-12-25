
# importing gymnasium
import gymnasium as gym

# creating the cartpole environment
e = gym.make("CartPole-v1")

""" problem: the stick tends to fall right or left and you need to balance it by moving the platform
to the right or left at every step """

""" observation: four floating points numbers, which contains information about:
1. stick's cetner of mass 
2. stick's speed
3. stick's angle to the platform
4. stick's angular speed """

""" the tricky part: how do we learn how to balance this system without knowing the exact meaning of
the observed numbers only by getting the reward? """

""" in this environment, the reward is 1, and it is given on every time step. The episode conitnues 
until the stick falls, so to get a better reward, we need to balance the platform to avoid the stick
from falling """

# always reset a newly made environment 
obs, info = e.reset()

# obtaining the observation
print(obs)

# checking the action space
print(e.action_space)

""" the action space is of the discrete type, so the actions will be 0 or 1, where 0 means pushing the
platform to the left and 1 means pushing to the right """

# checking the observation space
print(e.observation_space)

""" the first list printed is low bound and the second list is the high bound of parameters """

# sending a action to the environment
print(e.step(0))

""" here, we pushed the platform to the left by using 0 and received a tuple of five elements:
1. a new observation, which is a vector of 4 numbers 
2. a reward of 1
3. a done flag with False, which indicates that the episode isn't over 
4. a truncated flag with False, which means the episode wasn't truncated 
5. extra information about the environment, which is a empty dictionary """

# using the sample() method
print(e.action_space.sample())
print(e.action_space.sample())
print(e.observation_space.sample())
print(e.observation_space.sample())

""" this method returns a random sample of the underlying space, which in case of the Discrete action
space means a random number of 0 or 1 and for the observation space, means a random vector of four numbers. """

""" the random sample of the observation space isn't useful, but action space is, when we're not sure of how
to perform an action. """
