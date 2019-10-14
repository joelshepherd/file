# file (name wip)

Commands:

- `file init NAME [REMOTE]` Creates a file
- `file open` Opens the file
- `file shut` Closes the file

### init

- Creates a folder of name
- Creates a dotfile with name and remote (if chosen)

### open

- Pulls from remote if setup
- Decrypts the archive with gpg
- Extracts the archive to the folder

### shut

- Zips the folder to an archive
- Encrypts the archive with gpg
- Cleans the directory of any other files
- Pushes the remote if setup

## Ideas

- Should this integrate git to save history?
- Should this integrate with gpg recipients for multi-user editing?
