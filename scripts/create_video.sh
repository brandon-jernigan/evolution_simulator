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
ffmpeg -pattern_type glob -i '/media/volume/sdb/evolution_simulator/frames_filtered/frame_*.png' \
-framerate 60 \
-c:v libx264 \
-profile:v main \
-level 4.0 \
-preset veryslow \
-crf 26 \
-pix_fmt yuv420p \
"/media/volume/sdb/evolution_simulator/videos/output_${current_datetime}.mp4"
