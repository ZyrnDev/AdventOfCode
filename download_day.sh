DAY="$1"

# Download the data
curl "https://adventofcode.com/2022/day/$DAY/input" \
    --silent \
    --compressed \
    --create-dirs \
    -o data/days/$DAY/input \
    --cookie "$(cat .cookie)" 