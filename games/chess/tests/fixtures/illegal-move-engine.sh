#!/bin/sh
while IFS= read -r line; do
    case "$line" in
        uci) echo uciok ;;
        isready) echo readyok ;;
        go*) echo 'bestmove e2e5' ;;
    esac
done
