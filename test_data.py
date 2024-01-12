#!/usr/bin/env python
import json

# Load the JSON file
file_path = 'pol.json'
with open(file_path, 'r') as file:
    data = json.load(file)

print("Data len: ", len(data))

# Count the number of publications between X and Y
year = (2009, 2012)

count = sum(
    1 for item in data 
    if item["gap of years"][0] <= year[1] and item["gap of years"][1] >= year[0]
)


print(count)