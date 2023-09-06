#!/bin/bash
#ffmpeg -y -r 1/5 -f concat -safe 0 -i images.txt -c:v libx264 -vf fps=25 -pix_fmt yuv420p out.mp4
ffmpeg -y -f concat -i input.txt -c:v libx264 -pix_fmt yuv420p video.mp4
