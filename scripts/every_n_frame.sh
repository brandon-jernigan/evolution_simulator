#!/bin/bash

# Check if an argument is provided, otherwise exit with an error message.
if [ -z "$1" ]; then
  echo "Usage: $0 <step>"
  exit 1
fi

# Assign the first argument to the variable "step"
step=$1

src_folder="/media/volume/sdb/evolution_simulator/frames"
dest_folder="/media/volume/sdb/evolution_simulator/frames_filtered"

# Clear existing files in the destination folder
rm -f "$dest_folder"/*.png

# Create the destination folder if it doesn't exist
mkdir -p "$dest_folder"

count=0
new_count=0
for f in $(ls "$src_folder"/frame_*.png | sort -V); do
  if [ $((count % step)) -eq 0 ]; then
    new_filename=$(printf "frame_%04d.png" $new_count)
    cp "$f" "$dest_folder/$new_filename"
    ((new_count++))
  fi
  ((count++))
done

