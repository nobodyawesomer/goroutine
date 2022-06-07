# Goroutine

An attempt at goroutines in rust. One thing Golang is better at is making it easy to write reasonable concurrent code. It does this using the `go` keyword and encouraging the use of `chan`nels rather than shared state.

Of course, Go's scheduler is partially preemptive, and with Rust being fully cooperatively scheduled, there are some things the developer must take into consideration.

This library intends on making use of Rust's existing `async/.await` syntax.


Edit: wth am I even doing. I don't want to make my own executor/runtime and I want the user to be able to choose their executor.

Is this even at all necessary? I mean, `tokio::sync` has mpsc which is basically channels, and `tokio::spawn` is basically just `go`.

I mean I feel sorta silly