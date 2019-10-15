# setup
./setup.sh
FILES=$(find . -type f ! -name .config)

# archive
zip .file $FILES

# encrypt
gpg --symmetric .file

# push
if [[ ! -z $REMOTE ]]; then
  scp .file.gpg $REMOTE
fi

# cleanup
shred --remove .file $FILES
