#!/bin/bash
LOG="/tmp/cyberfeed_debug.log"
echo "========================================" >> "$LOG"
echo "--- LAUNCH ATTEMPT: $(date) ---" >> "$LOG"
echo "USER: $USER" >> "$LOG"
echo "DISPLAY: $DISPLAY" >> "$LOG"
echo "WAYLAND_DISPLAY: $WAYLAND_DISPLAY" >> "$LOG"
echo "XDG_RUNTIME_DIR: $XDG_RUNTIME_DIR" >> "$LOG"
echo "PATH: $PATH" >> "$LOG"
echo "PWD: $(pwd)" >> "$LOG"

# Test 1: Direct binary execution check
echo "Running direct binary test..." >> "$LOG"
/home/raven/Devspace/Cybercore/tooling/cyberfeed/target/release/cyberfeed --help >> "$LOG" 2>&1
echo "Direct binary exit code: $?" >> "$LOG"

# Test 2: Attempting omarchy-launch-tui execution
echo "Attempting omarchy-launch-tui..." >> "$LOG"
omarchy-launch-tui /home/raven/Devspace/Cybercore/tooling/cyberfeed/target/release/cyberfeed >> "$LOG" 2>&1
echo "omarchy-launch-tui exit code: $?" >> "$LOG"
