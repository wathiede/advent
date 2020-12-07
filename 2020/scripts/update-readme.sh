#!/usr/bin/env bash
MAX_DAY=$(ls src/day* | tr -d 'a-z/. ' | sort -n | tail -1)
(
echo "# Results"
echo
echo "\`\`\`"
cargo run --release
echo "\`\`\`"
echo
) > README.md
