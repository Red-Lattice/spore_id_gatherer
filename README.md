# What is this?
This is a program meant to gather every valid creation ID from spore.com and put it into text files

# How does it work?
It works by sending HEAD requests to the spore servers for a given asset ID and returns an error code. If it gets a 200 OK then it adds the ID to the list. If it gets a 404 Not Found, it tosses the ID out as invalid.
Through this it is capable of testing every ID to see if it is valid, guaranteeing no creations get skipped in the process.

# How do I use it?
Download the latest release onto a 64bit Windows 10 installation, and run the executable. Provide it with a starting ID and how many ID's you want to check when prompted.
Then, it will add all ID's it finds to a series of text files in a folder called "assets" which should be in the same directory as wherever the executable is located.

# Usage terms
Feel free to use this however you like.

# Miscellaneous Notes
- Not all ID's start with 50. Maxis-made ones start with 30.
- The stuttery-ness of the progress bar is expected. ID's return to the program in chunks.
- The ids are sorted into text files based on their sub-ids, which are 3 digit chunks from the first 9 digits of the ID (reading left->right)
