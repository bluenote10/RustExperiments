#!/bin/bash
while true; do
  change=$(inotifywait -r -e close_write,moved_to,create,modify . 2> /dev/null) 
  sleep 0.1
  clear
  echo "changed: $change"
  ./cargo_cmd
done
