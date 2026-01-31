#!/bin/bash

# 1. Check if ffmpeg is installed
if ! command -v ffmpeg &> /dev/null; then
    echo "Error: ffmpeg is not installed or not in your PATH."
    exit 1
fi

# 2. Check if there are actually frames to convert
if ! ls frame_*.png 1> /dev/null 2>&1; then
    echo "Error: No 'frame_XXXXX.png' files found in this directory."
    echo "Make sure you run this script inside the 'out/rec_TIMESTAMP' folder."
    exit 1
fi

# 3. Prompt for Framerate (default to 30)
read -p "Enter output framerate [default: 30]: " fps
fps=${fps:-30} # Use 30 if input is empty

# 4. Prompt for Deletion
read -p "Delete source PNG images after success? (y/N): " delete_choice

output_file="output.mp4"

echo "------------------------------------------------"
echo "Starting conversion at $fps FPS..."
echo "------------------------------------------------"

# 5. Run FFmpeg
# -y overwrites output.mp4 if it already exists without asking
ffmpeg -y -framerate "$fps" -i frame_%05d.png -c:v libx264 -pix_fmt yuv420p "$output_file"

# 6. Check if FFmpeg succeeded (Exit code 0)
if [ $? -eq 0 ]; then
    echo "------------------------------------------------"
    echo "✅ Success! Video saved as: $output_file"
    
    # Check if user wanted to delete files (Case insensitive match for Y or y)
    if [[ "$delete_choice" =~ ^[Yy]$ ]]; then
        echo "🗑️  Deleting source PNGs..."
        rm frame_*.png
        echo "Done."
    else
        echo "📄 Source PNGs preserved."
    fi
else
    echo "------------------------------------------------"
    echo "❌ Error: Video conversion failed. Images were NOT deleted."
    exit 1
fi
