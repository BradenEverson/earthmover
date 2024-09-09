# The Agent

`Achiever` aims to do exactly as the name suggests, achieve some arbitrary task. It will complete this by collecting data about it's enviornment and sending it to a remote `hivemind` server. This server could be local or cloud-based. The server will simulate the environment and the agent with respect to the collected data, and create an instruction set of what it deems the most rewarding actions. After these actions are performed, the agent will evaluate and collect more data until the reward function has reached an acceptable area
