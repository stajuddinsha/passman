#!/bin/bash

echo "Testing auto-quit functionality..."
echo "The terminal version will now start and auto-quit after copying a password."
echo "Press Enter to start the test..."

# Start the terminal version
./target/release/keytui-tui

echo "Application exited (auto-quit after copying password)"
