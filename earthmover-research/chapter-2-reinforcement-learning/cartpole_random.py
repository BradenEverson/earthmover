
# gymnasium import
import gymnasium as gym


if __name__ == "__main__":
    """ creating a environment and initializing the counter of steps and the reward, and at 
    the last line, we reset the environment to obtain the first observation """
    env = gym.make("CartPole-v1")
    total_reward = 0.0
    total_steps = 0
    obs, _ = env.reset()
    
    """ in the loop below, after sampling a random action, we ask the environment to execute 
    it, and return to us the next observation, which are the reward, is_done flag and is_trunc
    flag. """
    
    """ if the episode is over, we stop the loop and show how many steps have been taken and 
    how much reward has been accumulated. """
    
    while True:
        action = env.action_space.sample()
        obs, reward, is_done, is_trunc, _ = env.step(action)
        total_reward += reward
        total_steps += 1
        if is_done:
            break
    
    print("Episode done in %d steps, total reward %.2f" % (total_steps, total_reward))
    
    """ depending on the results of the code, of which the latest result was about 12 to 15 steps,
    which is far below the required average of 195 steps needed to "solve" the environment """
    
    """ this simply means the agent is performing poorly, but that's because the agent hasn't started
    learning yet """
    
    """ most of the environments in gym have a "reward boundary", which is the average reward that the 
    agent should gain during 100 consecutive episodes to "solve" the environment """
    
    """ for "cartpole", the boundary is 195, which means the agent must hold the stick for 195 time steps
    or longer """