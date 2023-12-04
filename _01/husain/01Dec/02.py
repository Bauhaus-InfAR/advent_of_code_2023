import pandas as pd
import numpy as np
import matplotlib.pyplot as plt
import re


# Using readlines()
file = open('01Dec/temp.txt', 'r')
Lines = file.readlines()

num_dict={"one":"1", "two":"2", "three":"3", "four":"4", "five":"5", "six":"6","seven":"7","eight":"8","nine":"9"}

number_dict = {"one":"1", "two":"2", "three":"3", "four":"4", "five":"5", "six":"6","seven":"7","eight":"8","nine":"9","ten":"10", 
        "eleven":"11", "twelve":"12", "thirteen":"13", "fourteen":"14", "fifteen":"15", "sixteen":"16", "seventeen":"17", "eighteen":"18", "nineteen":"19", 
        "twenty":"20", "thirty":"30", "forty":"40", "fifty":"50", "sixty":"60", "seventy":"70", "eighty":"80", "ninety":"90", 
        "hundred":"00", "thousand":"000", "million":"000000", "billion":"000000000", "trillion":"000000000000"}

for i in range(len(Lines)):
    matched={}
    
    for k in num_dict.keys():
        x=re.search(k,Lines[i])
        if x:
            matched.update({x.start():x.group()})

    z=dict(sorted(matched.items()))
    
    if z:
        a=list(z.values())[0]
        a_=list(z.keys())[0]
        b=list(z.values())[-1]
        b_=list(z.keys())[-1]

        #Lines[i]=Lines[i].insert(Lines[i][:a_], num_dict[a])
        #Lines[i]=Lines[i].insert(Lines[i][b_:], num_dict[b])
        Lines[i]=Lines[i][:a_]+num_dict[a]+Lines[i][a_:]
        Lines[i]=Lines[i][:b_]+num_dict[b]+Lines[i][b_:]
    """
    for v in z.values():
        Lines[i] = Lines[i].replace(v, num_dict[v])
    """
print(Lines)
len(Lines)

integers = []
count = 0
for line in Lines:
    x = "".join(re.findall("\d+", line))[0]
    y = "".join(re.findall("\d+", line))[-1]
    str = x+y

    integers.append(int(str))
    count += 1

print(integers)
print("total integers:", len(integers))
print("sum of integers is : ", sum(integers))




"""    
for i in range(len(Lines)):
    if "teen" in Lines[i]:
        for k , v in teen_dict.items():
            Lines[i] = Lines[i].replace(k, v)   
    else:
        for k , v in num_dict.items():
            Lines[i] = Lines[i].replace(k, v)
"""
