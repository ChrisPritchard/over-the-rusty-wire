# Over the Rusty Wire

This started as a Rust-built solver for behemoth, a challenge class at Over The Wire: https://overthewire.org/wargames/behemoth/.

But has now expanded to solvers for challenges from there and up, with a little re-organisation and extra documentation.

When run, it will connect over SSH using the password for the first user (behemoth0:behemoth0), and exploit the first binary to get the second password. It will then repeat the process until all of the behemoth challenges have been completed.

Why? Mainly an experiment in using Rust both for remote SSH orchestration and binary exploitation. The code was written to be a bit like 'expect', with write lines and read untils functions.