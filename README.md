Messing around with hyper
=============================================

This is me messing around with code from *Hands-On Microservices With Rust* to explore Rust further, and the [hyper](http://hyper.rs) crate.

You'd probably be better off looking at *Hands-On Microservices With Rust* unless you really like half-baked comments mixed in with your code.

However, this builds and runs in a Docker container which is a neat change from the book.

Run

`docker build -t hyper_server_test .`

then

`docker run -it --rm -p 8000:8000 --name hyper-server-test-running hyper_server_test`

to run it. Going to localhost:8000 should show you a white page saying "Rust Microservice FTW!!!!!!!!!!!!!!1111", which is nice.

This will probably be replaced by something better, or used as a base for future projects.
