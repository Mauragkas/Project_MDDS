#!/usr/bin/env python 
import json

# Path to your JSON file
filename = '../pol.json'

try:
    # Read JSON data
    with open(filename, 'r') as file:
        data = json.load(file)

    # Iterate through each record
    for record in data:
        # Check if 'gap of years' exists and is a string (unconverted)
        if 'gap of years' in record and isinstance(record['gap of years'], str):
            # Split, strip, and convert to integers
            start_year, end_year = map(lambda x: int(x.strip()), record['gap of years'].split('-'))
            # Replace the string with a tuple
            record['gap of years'] = (start_year, end_year)

    # Write modified data back to JSON
    with open(filename, 'w') as file:
        json.dump(data, file, indent=4)

    print(f"Updated data saved in {filename}")

except FileNotFoundError:
    print("File not found. Please check the file path.")
except json.JSONDecodeError:
    print("Invalid JSON file. Please check the file's structure.")
except Exception as e:
    print(f"An error occurred: {e}")
