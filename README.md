# Rust concurrency patterns

This is the Rust re-implementation of the Go code presented in the talk "Go
Concurrency Patterns" by Rob Pike.

https://talks.golang.org/2012/concurrency.slide

## Quickstart

Install [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) then

    $ cargo build

You can run the examples:

    $ cargo run --bin search10
    $ cargo run --bin search20
    $ cargo run --bin search21
    $ cargo run --bin search30

Run the `bench` program to compare the different implementations:

    $ cargo run --bin bench
    .search10 in 230.463518ms
    .search20 in 69.660359ms
    ....search21 in 305.107455ms
    .search30 in 36.067319ms

The output shows the time and the number of executions to deliver a non-timeout
search result to the user.  The number of dots represents how many times the
search function was executed: this models how many times the user refreshes the
search page to obtain a search result.
