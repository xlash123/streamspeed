# StreamSpeed

A simple Rust program that prints the transfer speed of stdin, then prints the total throughput data and average speed when stdin closes or on Ctrl-C.

All status output is written to stderr to allow for forwarding stdin to stdout (not forwarded buy default).

# Build
So simple, the build is a one-liner!

```bash
cargo build --release
```

# Install
To install, throw it in your `/usr/local/bin`. Run with `streamspeed` command, or whatever you want to name it to.

# Usage
Pipe whatever stream you want into it.
```bash
# Use right out of the box to benchmark storage speed
streamspeed < some_large_file

# Set custom update period (seconds)
streamspeed -t 0.5

# Print help menu
streamspeed --help

# Pipe program output into it
any-command | streamspeed

# Forward stdin to stdout so you can benchmark a command's output while writing data
gzip -cd some_file.tar.gz | streamspeed -f > some_file.tar


# Benchmark randomness generation
streamspeed < /dev/urandom

# Go for the high score!
streamspeed < /dev/zero

# Only print final results
streamspeed -q
```
