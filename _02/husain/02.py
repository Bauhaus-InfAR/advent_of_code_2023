import pandas as pd
import numpy as np
import matplotlib.pyplot as plt
import re

Lines = [line.strip() for line in open('02Dec/input.txt', 'r')]

games={}
for j in range(len(Lines)):
    color_dict = {"green":[], "red":[], "blue":[]}
    green = []
    red = []
    blue = []
    for i in range(len(Lines[j].split(": ")[1:][0].split("; "))):    
        if re.findall(r"(\d+) green", Lines[j].split(": ")[1:][0].split("; ")[i]) != []:
            green.append(int(re.findall(r"(\d+) green", Lines[j].split(": ")[1:][0].split("; ")[i])[0]))
        if re.findall(r"(\d+) red", Lines[j].split(": ")[1:][0].split("; ")[i]) != []:
            red.append(int(re.findall(r"(\d+) red", Lines[j].split(": ")[1:][0].split("; ")[i])[0]))
        if re.findall(r"(\d+) blue", Lines[j].split(": ")[1:][0].split("; ")[i]) != []:
            blue.append(int(re.findall(r"(\d+) blue", Lines[j].split(": ")[1:][0].split("; ")[i])[0]))
    color_dict.update({"green":green, "red":red, "blue":blue})
    games.update({j+1:color_dict})

power = []  

for n in games.keys():
    g=max(games[n]["green"])
    r=max(games[n]["red"])
    b=max(games[n]["blue"])
    power.append(g*r*b)


print("total power:", sum(power))

