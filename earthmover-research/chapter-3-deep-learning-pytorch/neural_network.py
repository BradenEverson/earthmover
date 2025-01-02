
# all required imports
import torch
import numpy as np
from torch import nn


""" we created a randomly initialized feed-forward layer, with two inputs and five outputs, and applied it to 
the float tensor. """

# creating a feed forward layer with optional bias
l = nn.Linear(2, 5)
v = torch.FloatTensor([1, 2])
print(l(v))


# using the sequential class to combine other layers into the pipe 

""" here, we defined a three layer neural network with softmax on output, applied along dimension 1 (dimension 0 
is batch samples), rectified linear unit (ReLU) nonlinearities and dropout"""

s = nn.Sequential(
    nn.Linear(2, 5),
    nn.ReLU(),
    nn.Linear(5, 20),
    nn.ReLU(),
    nn.Linear(20, 10),
    nn.Dropout(p=0.3),
    nn.Softmax(dim=1)
)

print(s)

# pushing data through the layers
print(s(torch.FloatTensor([[1, 2]])))
