# Note

Take quick notes

## Features

* Simple and light
* Can save local as well as global notes
  * If current folder contains a `.notes.json` file, then all notes related to current directory or project will be displayed, as well as all notes will be stored in current directory.
  * If it is not found in current dir, then notes will be saved to a global location which can be set using env variables.
* Search and list notes

## Configuration

Set `NOTES_FILE` env variable e.g.
`export NOTES_FILE=~/some/path/.notes.json` #default value is `~/.notes.json`
