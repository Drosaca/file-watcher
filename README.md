# File Watcher

A lightweight command-line tool that monitors filesystem changes in specified paths and reports meaningful events.

This tool listens for operations on files or directories.  
It is built in pure Rust and uses platform-native file notification backends for maximum performance and low overhead.

---

## Features

-  Monitor any number of files or directories
-  Detect changes instantly
-  Reports these events:
    - **CREATE**
    - **REMOVE**
    - **MODIFY**
-  Reports internal states:
    - **WATCHING** – when a path begins being watched
    - **ERROR** – when something goes wrong

---

## Usage

```sh
Monitors given entries events

POSSIBLE EVENTS:
    CREATE, REMOVE, MODIFY
OTHER:
    WATCHING, Error


Usage: file-watcher [OPTIONS]

Options:
  -f, --files <FILES>
          

  -h, --help
          Print help (see a summary with '-h')
```