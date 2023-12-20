#!/usr/bin/env python3
import requests
from bs4 import BeautifulSoup
import json

# Function to parse DBLP page
def parse_dblp(url):
    page = requests.get(url)
    soup = BeautifulSoup(page.content, 'html.parser')
    # TODO: Add code to extract years and create intervals here

    return intervals

# Main execution
dblp_url = "https://dblp.org/pid/12/1509.html"
intervals = parse_dblp(dblp_url)
# Convert intervals to JSON and store

# Print for verification
print(json.dumps(intervals, indent=4))
