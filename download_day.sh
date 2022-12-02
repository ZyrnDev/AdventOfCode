DAY="$1"

# Download the data
curl 'https://adventofcode.com/2022/day/1/input' \
    --silent \
    --compressed \
    --create-dirs \
    -o data/days/$DAY/sections/1/input \
    --cookie "$(cat .cookie)" 