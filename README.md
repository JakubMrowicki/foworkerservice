# foworkerservice

A Rustlang app for deleting files in a folder periodically.

# Functionality

This software is designed to run in the background and to run at startup. It will check the contents of C:\data every 10 seconds and check each file & folder. If a file or folder's modified date is more than 60 seconds in the past, the software will attempt to delete it.

If the file cannot be deleted for any reason, the program will attempt it again 10 seconds later.

For example:

![Example](https://i.imgur.com/mNoy8DI.png)

Lightweight:
![light](https://i.imgur.com/gd9RjIC.png)

## Logging

You can output logs easily like so in the console.

`foworker_service.exe > C:\Utils\logs.log`

Download: https://github.com/JakubMrowicki/foworkerservice/releases/download/release/foworker_service.zip
