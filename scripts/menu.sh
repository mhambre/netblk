#!/usr/bin/env bash
set -euo pipefail

project_dir=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)
justfile="$project_dir/Justfile"
listing=$(just --justfile "$justfile" --color never --list --unsorted --list-heading '' --list-prefix '')
recipes=() descriptions=()
width=15

while read -r recipe details; do
    [[ -n "$recipe" ]] || continue
    recipes+=("$recipe")
    if [[ "$details" == *'# '* ]]; then
        descriptions+=("${details#*# }")
    else
        descriptions+=('')
    fi
    if (( ${#recipe} > width )); then
        width=${#recipe}
    fi
done <<< "$listing"

bold='' cyan='' green='' dim='' reset=''
if [[ -t 1 && -z "${NO_COLOR+x}" && "${TERM:-dumb}" != dumb ]]; then
    bold=$'\033[1m' cyan=$'\033[36m' green=$'\033[32m'
    dim=$'\033[2m' reset=$'\033[0m'
fi

printf '\n%s%s  n e t b l k%s\n' "$bold" "$cyan" "$reset"
printf '%s  Rust development commands%s\n\n' "$dim" "$reset"
for i in "${!recipes[@]}"; do
    printf '  %s%2d%s  %s%-*s%s  %s\n' \
        "$cyan" "$((i + 1))" "$reset" \
        "$green" "$width" "${recipes[$i]}" "$reset" "${descriptions[$i]}"
done
printf '\n%s  Run directly: just <command>%s\n\n' "$dim" "$reset"

[[ -t 0 && -t 1 ]] || exit 0
while true; do
    printf '%s  Choose a number or command (q to quit): %s' "$bold" "$reset"
    IFS= read -r choice || exit 0
    case "$choice" in
        q|quit|exit|'') exit 0 ;;
    esac
    for i in "${!recipes[@]}"; do
        if [[ "$choice" == "$((i + 1))" || "$choice" == "${recipes[$i]}" ]]; then
            printf '\n%s  Running just %s%s\n\n' "$cyan" "${recipes[$i]}" "$reset"
            exec just --justfile "$justfile" "${recipes[$i]}"
        fi
    done
    printf '%s  Unknown choice. Use 1-%d or a listed command.%s\n' "$dim" "${#recipes[@]}" "$reset"
done
