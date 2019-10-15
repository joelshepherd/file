# setup
./setup.sh

# pull
if [[ ! -z $REMOTE ]]; then
  scp $REMOTE .file.gpg
fi

# decrypt
gpg --output .file --decrypt .file.gpg

# extract
unzip .file

# cleanup
shred --remove .file .file.gpg
