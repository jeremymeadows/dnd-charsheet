#!/bin/bash

convert -density 1200 -background none -resize 256x256 icon.svg icon.bmp
convert icon.bmp icon.rgba
