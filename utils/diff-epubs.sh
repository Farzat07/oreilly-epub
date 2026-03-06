#!/usr/bin/env bash

# Usage:
# diff-epubs.sh path/to/old.epub path/to/new.epub

tmp_dir="$(mktemp -d --suffix=-diff-epubs)"

unzip "$1" -d "$tmp_dir/old"
unzip "$2" -d "$tmp_dir/new"
diff --strip-trailing-cr -r "$tmp_dir"/{old,new}
exit_status=$?
rm -rf "$tmp_dir"
exit $exit_status
