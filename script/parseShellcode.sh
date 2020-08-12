#!/bin/sh

# Parse the shellcode produced by -f c from a file
# EXAMPLE: msfvenom -p linux/x64/exec CMD=id -f c -b "\x00" > shellcode
# ./parseShellcode shellcode

file=$1
cat $1 | sed 's/"//g' | sed 's/;//g' | tr -d '\n' 
