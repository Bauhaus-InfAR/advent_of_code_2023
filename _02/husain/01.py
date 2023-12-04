import pandas as pd
import numpy as np
import matplotlib.pyplot as plt
import re


#file = open('02Dec/input.txt', 'r')
#Lines = file.readlines()

Lines = [line.strip() for line in open('02Dec/input.txt', 'r')]

"""
Lines[0].split(": ")[0]                  #Game number
Lines[0].split(": ")[1:][0]              #Game scores all together
Lines[0].split(": ")[1:][0].split("; ")  #Game scores split
Lines[0].split(": ")[1:][0].split("; ")[0] #game 1
"""
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

NewDict= {x: games[x] for x in games.keys() if max(games[x]["green"])<14 and max(games[x]["red"])<13 and max(games[x]["blue"])<15}
print("Sum of keys:", sum(list(NewDict.keys())))
