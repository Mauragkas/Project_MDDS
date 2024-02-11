import json
import random
import requests 
from bs4 import BeautifulSoup 
 
# Define the URL to scrape 
url = 'https://dblp.org/pid/236/9099.html' 
 
# Send a GET request to the URL and get the response 
response = requests.get(url) 
 
# Create a BeautifulSoup object by passing in the response content and 'html.parser' as the parser 
soup = BeautifulSoup(response.content, 'html.parser') 

# Extract specific metadata
author_tags = soup.find_all('span', class_='this-person')
coauthor_tags = soup.find_all('span', class_='author', itemprop='name')
title_tags = soup.find_all('span', class_='title')
year_tags = soup.find_all('span', itemprop='datePublished')
gapofyears_tags = soup.find_all('header', class_="hide-head h2",)
dblp_tags = soup.find_all('li', class_='select-on-click')

#Get author names
authors = [author.text.strip() for author in author_tags]

#Get coauthor names
coauthors = [coauthor.text.strip() for coauthor in coauthor_tags]

#Get publication title
titles = [title.text.strip() for title in title_tags]

#Get year of publication
dblp_records = [dblp.text.strip() for dblp in dblp_tags]
#Remove first dblp because is the general one
dblp_records = dblp_records[1:]

#Get year of publication
years = [year.text.strip() for year in year_tags]

#Get year of publication
gapofyears = [gapyear.find('h2').text.strip() for gapyear in gapofyears_tags]
#Remove first and last tag because are irrelevant
gapofyears = gapofyears[1:]
gapofyears = gapofyears[:-1]

gapofyearsCorrect = []
#for gap in gapofyears:
     # Split, strip, and convert to integers 
 #    splt= gap.strip().split(' - ')
  #   print(splt)
     #start_year, end_year = splt
     # Replace the string with a tuple
     #gapofyearsCorrect.append((start_year, end_year))


for i in range(len(authors)):
    print("Authors:", authors[i])
    #print ("Coauthors:", coauthors[i])
    print("Title:", titles[i])
    print("DBLP_Record:", dblp_records[i])
    print("Gap of Years:", gapofyears)
    print("Year:", years[i])

#json list
records = []
# Iterate over the data and create dictionaries
for i in range(len(authors)):
    record = {
        "author's name": authors[i],
        "title": titles[i],
        "gap of years": [],
        "year of release": years[i],
        "DBLP_Record": dblp_records[i],
        "Awards": random.randint(0,2),
        "kind": "none",
        "co-author": "none",
        "surname": authors[i].strip().split(' ')[-1].strip(),
    }
    records.append(record)

# Define the filename for the JSON file
filename = "records.json"

# Write the data to a JSON file
with open(filename, 'w') as json_file:
    json.dump(records, json_file, indent=4)

print("Data has been successfully written to", filename)
