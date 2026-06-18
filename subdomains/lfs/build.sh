#!/usr/bin/env bash
set -euo pipefail

# Configuration options
THEME=sunset

status=0
for book in lfs slfs glfs blfs; do
    rm -rf "$book/target"

    (
        set -eu
        cd "$book"

        echo "Building $book"
        THEME="$THEME" ./build.sh
    ) || status=1
done

case $status in
    0) echo "Built all books" ;;
    *) echo "Failed to build one or more books" >&2 ;;
esac

exit $status
