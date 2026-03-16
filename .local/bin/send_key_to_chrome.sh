#!/bin/bash

# Search for chrome windows
window_ids=($(xdotool search --name 'chrome'))

# Define the key combination to send
key_combination='alt+o'

# Loop through the window IDs and activate each window before sending the keypress
for window_id in "${window_ids[@]}"; do
    xdotool windowactivate "$window_id" key --clearmodifiers "$key_combination"
done

