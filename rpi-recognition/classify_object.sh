#!/bin/bash
libcamera-jpeg -o object.jpg --width 640 --height 480
./venv/bin/python3 classify.py object.jpg
