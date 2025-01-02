
import gymnasium as gym
import numpy as np
import matplotlib.pyplot as plt
import pickle

# the run function
def run(episodes, is_training=True, render=False):

    # creating the frozen lake environment
    env = gym.make('FrozenLake-v1', map_name="8x8", is_slippery=True, render_mode='human' if render else None)

    if(is_training):
        # a 64 x 4 array with all zeros
        q = np.zeros((env.observation_space.n, env.action_space.n))
    else:
        file = open('earthmover-research/frozen-lake-environment/frozen_lake8x8.pkl', 'rb')
        q = pickle.load(file)
        file.close()

    #initializing learning rate
    learning_rate_a = 0.9
    # initializing gamma
    discount_factor_g = 0.9

    # 1 = 100% random actions
    epsilon = 1 
    # the epsilon decay rate
    epsilon_decay_rate = 0.0001
    # random number generator
    rng = np.random.default_rng()

    
    rewards_per_episode = np.zeros(episodes)



    # simulating over multiple episodes
    for i in range(episodes):            

        # states from 0 to 63 , where 0 is top left corner and 63 is bottom right corner
        state = env.reset()[0]

        # true when the bot falls into the hole or has reached the goal
        terminated = False

        # true when the actions > 200, which basically limits to 200 steps
        truncated = False


        while(not terminated and not truncated):

            if is_training and rng.random() < epsilon:

                # used to select a random action
                # 0 is left, 1 is down, 2 is right, 3 is up
                action = env.action_space.sample()

            else:

                # following the q-table
                action = np.argmax(q[state,:])

            # updates the environment after an action has been performed
            new_state, reward, terminated, truncated, info = env.step(action)

            if(is_training):
                 # q learning formula
                q[state, action] = q[state, action] + learning_rate_a * (reward + discount_factor_g * np.max(q[new_state,:]) - q[state, action])

            # updating the state
            state = new_state


        epsilon = max(epsilon - epsilon_decay_rate, 0)

        if(epsilon == 0):
            learning_rate_a = 0.0001

        if(reward == 1):
            rewards_per_episode[i] = 1
    
    # closing the environment
    env.close()

    # creating a plot
    sum_rewards = np.zeros(episodes)
    for t in range(episodes):
        sum_rewards[t] = np.sum(rewards_per_episode[max(0, t-100): (t+1)])
    plt.plot(sum_rewards)
    plt.savefig('frozen_lake8x8.png')


    if(is_training):
        file = open("frozen_lake8x8.pkl", "wb")
        pickle.dump(q, file)
        file.close()

# calling the run method
if __name__ == '__main__':
    run(15000)
    # run(1, is_training=False, render=True)
    
