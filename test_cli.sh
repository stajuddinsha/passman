#!/bin/bash

echo "Testing CLI version..."

# Test help command
echo "Testing help command:"
./target/release/passman help

echo ""
echo "Testing list command:"
./target/release/passman list

echo ""
echo "Testing add command:"
echo "This will prompt for a password"
./target/release/passman add testentry

echo ""
echo "Testing list after add:"
./target/release/passman list

echo ""
echo "Testing delete command:"
./target/release/passman delete testentry

echo ""
echo "Testing list after delete:"
./target/release/passman list

echo "CLI test completed!"
