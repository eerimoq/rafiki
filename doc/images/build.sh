#!/usr/bin/env bash

set -e

# export slides to pdf and then convert them to jpg
sudo soffice --headless --convert-to pdf Rafiki.odp
convert -density 150 Rafiki.pdf -quality 95 Rafiki.jpg

# crop exported slides
convert Rafiki.jpg -crop 690x320+500+460 logo.jpg

rm -f Rafiki.pdf
rm Rafiki.jpg
