# Crawler, Change Data, Get Hash Size

## Overview
Ο φάκελος αυτός έχει δημιουργηθεί για την μεταφορά και μετατροπή των δεδομένων για επιστήμονες όπου αυτή κριθεί απαραίτητη σε json μορφή, όπως επίσης για την εύρεση και μεταφόρα μηκών δεδομένων που θα χρησιμοποιήσουμε στην συνέχεια σε ένα environmental αρχείο.

## Table of Contents
- [Crawler, Change Data, Get Hash Size](#crawler-change-data-get-hash-size)
  - [Overview](#overview)
  - [Table of Contents](#table-of-contents)
  - [Installation & Requirements](#installation--requirements)
  - [Usage](#usage)
  - [Code Documentation](#code-documentation)
    - [Modules](#modules)
    - [Crawler](#crawler)

## Table of Contents

## Installation & Requirements
Τα requirements για την εκτελσεση του κώδικα είναι τα εξής:
- Εγκατάσταση της Python
  - Για την εγκατάσταση της Python, ακολουθήστε τις οδηγίες που βρίσκονται στον παρακάτω σύνδεσμο: [Python Installation](https://www.python.org/downloads/)
  - Εγκατάσταση του pip
    - Μετά την εγκατάσταση της Python, εγκαταστήστε το pip ακολουθώντας τις οδηγίες που βρίσκονται στον παρακάτω σύνδεσμο: [Pip Installation](https://pip.pypa.io/en/stable/installation/)
  - Εγκατάσταση του lib `random`
    - Μετά την εγκατάσταση του pip, εγκαταστήστε το lib `random` μέσω του pip με την εντολή: `pip install random2`
- Εγκατάσταση του lib `requests`
    - Μετά την εγκατάσταση του pip, εγκαταστήστε το lib `BeautifulSoup` μέσω του pip με την εντολή: `pip install beautifulsoup4`

## Usage

Για την εκτέλεση του κώδικα ακολουθήστε τα παρακάτω βήματα:
- Για το αρχείο change_data τρεξτε τον κώδικα με την εντολή `./change_data.py`
- Για το αρχείο crawler τρεξτε τον κώδικα με την εντολή `./crawler.py`
- Για το αρχείο `get_hash_size` τρεξτε τον κώδικα με την εντολή `./get_hash_size.py`

## Code Documentation

### Modules
- `change_data.py` : Περιέχει τον κώδικα για την μετατροπή δεδομένων όπου αυτό κριθεί απαραίτητο.
- `crawler` : Περιέχει τον κώδικα για την μεταφορά των δεδομένων που μας ενδιαφέρουν για computer scientists από την ιστοσελίδα [DBLP_Records for computer_scientists](https://dblp.org.pers/) και την δημιουργία ενός json αρχείου `records.json` όπου αποθηκεύονται τα δεδομένα για κάθε επιστήμονα ξεχωριστά.
- `get_hash_size.py` : Περιέχει τον κώδικα για την δημιουργία ενός `.env`file στο οποίο περιέχονται τα μήκη των DBLP_Records και Surnames για να μπορούμε όπου χρειαστεί να μετατρέψουμε τα strings DBLP_Records και Surnames σε integers (hash).

### Crawler

Εδώ θα αναλύσουμε λίγο μόνο τον κώδικα του Crawler εφόσον ο κώδικας του change_data.py και get_hash_size.py είναι αρκετά απλός και δεν υπάρχει ανάγκη για περεταίρω ανάλυση.

- Βιβλιοθήκη `requests`: χρησιμοποιείτε για αποστολή αιτήματος σε μια ιστοσελίδα για άδεια χρήσης των δεδομένων της.

- Με την βοήθεια της βιβλιοθήκης `beautifulsoup4` αναλύουμε HTML και XML εγγραφές. 

- Δημιουργούμε αίτηση προς την ιστοσελίδα [DBLP_Records for computer scientists](https://dblp.org.pers/) και αποθηκεύουμε τα δεδομένα της σε μια μεταβλητή `response = requests.get(url)` και στην συνέχεια χρησιμοποιώντας την βιβλιοθήκη `BeautifulSoup` βρίσκουμε τα tags και έπειτα τα URL που αντιστοιχούν στις ιστοσελίδες για τον κάθε επιστήμονα ξεχωριστά. Στην συνέχεια βρίσκουμε τα tags για τα δεδομένα που μας ενδιαφέρουν και αποθηκεύουμε τα δεδομένα που αντιστοιχούν στα tags σε ένα json αρχείο `records.json` όπου γίνεται έλεγχος εάν οι εγγραφές έχουν μεταφερθεί σωστά και τα μεταφέρουμε στο αρχείο `poll.son` όπου υπάρχουν πλέον τα δεδομένα των computer scientists για να τα χρησιμοποιήσουμε στα επόμενα ερωτήματα.

- Αρχείο `records.json`: Περιέχει τα δεδομένα των computer scientists.

- Τα δεδομένα αυτά είναι:

    1.`author's name` 2.`gap of year` 3.`year of release` 4.`DBLP_Record` 5.`Awards` 6.`kind` 7.`co-author` 8.`surname`
