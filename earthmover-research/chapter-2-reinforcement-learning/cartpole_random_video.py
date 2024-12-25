
# necessary imports
import gymnasium as gym

""" rendering the environment using two wrappers: Human Rendering & RecordVideo 
1. Human Rendering: opens a separate graphical window in which the image from the environment
is shown interactively

2. RecordVideo: captures pixels from the environment and produces a video file of your agent in
action. Used the same as Human Rendering but requires an extra argument specifying the folder to store """

if __name__ == "__main__":
    env = gym.make("CartPole-v1", render_mode="rgb_array")
    env = gym.wrappers.RecordVideo(env, video_folder="chapter-2-reinforcement-learning")
    
    total_reward = 0.0
    total_steps = 0
    obs = env.reset()
    
    while True:
        action = env.action_space.sample()
        obs, reward, done, _, _ = env.step(action)
        total_reward += reward
        total_steps += 1
        if done:
            break
        
    print(f"Episode done in {total_steps} steps, total reward {total_reward:.2f}")
    env.close()