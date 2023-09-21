#!/bin/bash

# Get the directory of the currently executing script
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

# Navigate back to the project root directory
cd "$DIR/.."

# Ensure the 'videos' folder exists
mkdir -p videos

# Get current datetime
current_datetime=$(date +"%Y%m%d%H%M%S")

# Create the video using FFmpeg
ffmpeg -framerate 60 -pattern_type glob -i 'frames/frame_*.png' -vf "select=not(mod(n\,1))" -c:v libx264 -pix_fmt yuv420p "videos/output_${current_datetime}.mp4"
