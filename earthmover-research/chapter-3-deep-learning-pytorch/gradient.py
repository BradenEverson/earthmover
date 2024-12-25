
# all required imports
import torch
import numpy as np


""" there are two ways to calculate the gradient: 
1. static graph: you need to define your calculations in advance and can't change them later
2. dynamic graph: you don't need to define your graph, you just need to execute operations that you want to use """



""" attributes related to gradients that every tensor has: 
1. grad: property that holds a tensor of the same shape containing computed gradients 
2. is_leaf: equals "True" if this tensor was constructed by the user and "False" if the object is a result of 
function transformation
3. requires_grad: equals "True" if the tensor requires gradients to be calculated """


# creating two tensors 
v1 = torch.tensor([1.0, 1.0], requires_grad=True)
v2 = torch.tensor([2.0, 2.0])

# adding both vectors, doubling every element, and summing them
v_sum = v1 + v2
print(v_sum)

v_res = (v_sum*2).sum()
print(v_res)


# calculating the gradients of the graph
v_res.backward()

""" when we call the backward function, we ask pytorch to calculate the numerical derivative of the v_res variable
with respect to any variable that the graph has """

""" in this example, the value of 2 is the gradients of v1 means that increasing the value of v1 by one will mean
that the value of vres will grow by two """

print(v1.grad)
print(v2.grad)
