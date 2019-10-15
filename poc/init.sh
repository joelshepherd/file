# directory
mkdir $1

# dotfile
echo "NAME=$1" >> $1/.config
echo "REMOTE=$2" >> $1/.config
