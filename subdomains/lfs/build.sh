#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"

# Configuration options
THEME=sunset

status=0
for book in lfs slfs glfs blfs; do
    (
        set -eu
        cd "$book"

        if cmp sha target/sha; then
            echo "Skipping build for $book"
        else
            rm -rf target
            echo "Building $book"
            THEME="$THEME" ./build.sh && cp -vf sha target/sha
        fi

        # TODO: Probably rsync would be better
        echo "Copying files into place for $book"
        mkdir -pv "$SCRIPT_DIR/../../target/subdomains/lfs/$book"
        cp -af target/book -T "$SCRIPT_DIR/../../target/subdomains/lfs/$book"
    ) || status=1
done

case $status in
    0) echo "Built all books" ;;
    *) echo "Failed to build one or more books" >&2 ;;
esac

exit $status
