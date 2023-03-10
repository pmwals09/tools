#!/usr/bin/env bash

# Ideally would have a place to read the checklist from
# prompt each item
# exit on first failed item

declare -a CheckList=(
  "check PDF output"
  "check all report types (single facility, multi-facility, asc, office"
  "check against all test cases"
)

for val in "${CheckList[@]}"; do
  prompt="Did you $val? [y/N]: "
  should_continue= true
  while $should_continue; do
    read -r -p "$prompt " response
    case "$response" in
      [Yy][Ee][Ss]|[Yy]) # Yes or Y (case-insensitive).
        should_continue= true
        break
        ;;
      [Nn][Oo]|[Nn])  # No or N.
        exit 1
        ;;
      *) # Anything else (including a blank) is invalid.
        should_continue= false
        ;;
    esac
  done
done

exit 0
