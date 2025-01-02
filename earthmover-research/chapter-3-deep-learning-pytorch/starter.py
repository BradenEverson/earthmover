
# all required imports
import torch
import numpy as np

""" three ways to create a tensor in pytorch: 
1. by calling a constructor of the required type 
2. by asking pytroch to create a tensor with specific data 
3. by converting a numpy array or python list into a tensor """

# creating a float tensor of size 3 x 2
a = torch.FloatTensor(3, 2)
print(a)

# creating a zero tensor of size 3 x 4
print(torch.zeros(3, 4))

# alternative way to create a zero tensor
print(a.zero_())



""" two types of operations for tensors: inplace and functional 
1. inplace operation: have a underscore appended to their name and operate on the tensor's content 
2. functional operation: creates a copy of the tensor with the modification, leaving the original tensor """


# using a python iterable (list or tuple) to create a tensor
print(torch.FloatTensor([[1, 2, 3], [3, 2, 1]]))

# using a numpy array to create a tensor
n = np.zeros(shape=(3, 2), dtype=np.float32)
print(n)

# torch.tensor() accepts a numpy array as a argument
b = torch.tensor(n)
print(b)



""" pytorch has support for zero-dimensional tensors which are just scalar values. With zero dimensional 
tensors, pytorch returns a single number, which makes it more simpler to work with """
a = torch.tensor([1, 2, 3])
print(a)

s = a.sum()
print(s)

# .item() is used to access the actual python value
print(s.item())
print(torch.tensor(1))



"""pytorch supports CUDA GPU's, which means all operations have two versions: CPU and GPU. To convert from
CPU  to GPU, there is a method, to(device) which creates a copy of the tensor to a device """

# creating a 3 x 2 tensor
a = torch.FloatTensor([2, 3])
print(a)

# converting to CUDA
# ca = a.to('cuda') too old to run
# print(ca)