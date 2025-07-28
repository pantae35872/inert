#!/bin/bash

if [ -z "$1" ]; then
  echo "Usage: $0 <image_file>"
  exit 1
fi

./venv/bin/python3 classify.py "$1" 
