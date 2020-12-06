#!/usr/bin/env bash
MAX_DAY=$(ls src/day* | tr -d 'a-z/. ' | sort -n | tail -1)
(
echo "# Results"
echo
for day in $(seq 1 ${MAX_DAY:?});
do
  echo "## Day ${day:?}"
  echo "\`\`\`"
  cargo aoc -d ${day:?} 2> /dev/null
  echo "\`\`\`"
  echo
done) > README.md
