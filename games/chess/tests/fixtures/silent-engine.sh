#!/bin/sh
# Keep both pipes open without replying or creating a descendant process.
# The engine client must terminate this process on timeout or cancellation.
while IFS= read -r line; do
    :
done
