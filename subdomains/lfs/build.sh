#!/usr/bin/env bash
set -euo pipefail

# Configuration options
THEME=sunset

status=0
for book in lfs slfs glfs blfs; do
    (
        set -eu
        cd "$book"

        if cmp sha target/sha; then
            echo "Skipping build for $book"
            exit 0
        fi

        rm -rf target
        echo "Building $book"
        THEME="$THEME" ./build.sh && cp -vf sha target/sha
    ) || status=1
done

case $status in
    0) echo "Built all books" ;;
    *) echo "Failed to build one or more books" >&2 ;;
esac

exit $status
