# StreamSpeed

A simple C program that prints the throughput speed of stdin in Mebibytes (1024 * 1024 bytes).

# Build
So simple, the build is a one-liner!

```bash
gcc src/streamspeed.c -o streamspeed -pthread
```

# Install
To install, throw it in your `/usr/local/bin`. Run with `streamspeed` command, or whatever you want to name it to.

# Usage
Pipe whatever stream you want into it.
```bash
# Set custom update period (milliseconds) and block size (bytes)
streamspeed [period] [block size]

# Print help menu
streamspeed --help

# Test file read speeds
streamspeed < some_file.txt

# Benchmark randomness generation
streamspeed < /dev/urandom

# Go for the high score
streamspeed < /dev/zero

# Pipe program output into it
any-command | streamspeed
```
