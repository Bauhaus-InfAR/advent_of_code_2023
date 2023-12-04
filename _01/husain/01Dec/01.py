import pandas as pd
import numpy as np
import matplotlib.pyplot as plt
import re


#read the lines of the file
file = open('01Dec/input.txt', 'r')
Lines = file.readlines()

integers = [] 

count = 0
for line in Lines:
    x = "".join(re.findall("\d+", line))[0]
    y = "".join(re.findall("\d+", line))[-1]
    str = x+y

    integers.append(int(str))
    count += 1
#print(integers)
print(sum(integers))