#!/bin/bash

# Requires ffmpeg
for i in ./out/*.ppm;
  do name=${i%.ppm}
    echo "$name"
    ffmpeg -y -i "$i" "${name}.png"
done