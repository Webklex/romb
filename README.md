# ROMB - Rust bomb
Sound more dramatic than it is...

# How I tried to learn rust in 6 days - and failed?

## Prolog
I'm usually creating web apps with php, js, go or python. I've played around with
C, C++ and C# but never used them in a bigger project.

So why rust? I've heard a lot of people talk about it. It's supposed to be pretty fast
and provides some additional features which, regarding app security, are great as well.
At the end [d0nutptr](https://github.com/d0nutptr) motivated me to participate in his
[giveaway/challenge](https://twitter.com/d0nutptr/status/1487259366524817410).

## Scope & Description
I recently thought about what would happen if you try to race against a firewall / IDS; Are they 
fast enough, or could you "squeeze" in a bunch of request before the client gets blocked? 
Basically a one-shot port scan.

This application will scan a port range of a given target simultaneously, in an attempt to race 
against a firewall IDS. The result can be saved or printed as csv, json or xml.

### The application should:
- Spawn a new thread for each port to be scanned
- Thread is halted until all threads are ready
- All threads are released as soon as the last thread is ready
- A "Scan thread" should do nothing besides connecting to a given port after it has been released
- Accept command line arguments to configure the target and port range

### Nice to have:
- Export open ports as json, xml or txt file
- Possibility to use UDP instead of TCP
- If a port is considered open, check if any response is returned

# Result & Conclusion
Well the time has come to an end. I had a lot of plans and even though I failed to realize all of them,
I'm still a bit proud of having realized at least the core feature (port check). This might not sound
like much but for my first steps at rust, I'm pretty happy with it :)

The exports and udp / response filtering support is still missing, the code isn't cleaned up and the
`Options` are never used. Oh and you can't spawn more than ~16337 threads - I still haven't figured out
why that's the case.

I spend the most time trying to understand how `Response` works and how to handle errors. I'm still
not fully sure, but I feel comfortable to play around with them some more.
Rust definitely isn't like any other language I've played around with.
It is extremely strict, has a unique logic and app "life cycle". 

Have I failed my quest? Yes and no, the application isn't finished but my second goal, to learn some 
rust was a success!

I tried to document my journey and took some notes along the way. Those are all listed below under 
**Timeline**.

## Timeline
### 1. Development / IDE setup
- Searched for "rust install"
    - https://www.rust-lang.org/tools/install
- Searching for an IDE
    - https://www.rust-lang.org/tools
    - Decided to use "CLion" (30-day trial version)
        - https://www.jetbrains.com/clion/
        - Installed the Rust plugin
            - https://plugins.jetbrains.com/plugin/8182-rust
- Like always, lets google for a "Hello World" example
    - https://doc.rust-lang.org/rust-by-example/hello.html
- Tested "Hello World" and made sure everything works as expected

### 2. Created a new rust project inside the IDE
- This created a folder in my projects directory:
    - Cargo.lock
    - Cargo.toml
    - src/main.rs
- running `cargo run` looks good:
```
   Compiling rust_test v0.1.0 (/mnt/raid1/projects/rust_test)
    Finished dev [unoptimized + debuginfo] target(s) in 0.52s
     Running `target/debug/rust_test`
Hello, world!
```

### 3. Build the application
- Developing the actual application and gather some information on how this can be accomplished.
- I'll like to use something like objects or structs
    - Searching for "rust structs" brought up some interesting results:
        - https://doc.rust-lang.org/book/ch05-01-defining-structs.html
        - Nice, also an example project! https://doc.rust-lang.org/book/ch05-02-example-structs.html
- I'll need to thread all potential port checks
    - Searching for "rust threading"
        - https://doc.rust-lang.org/book/ch16-01-threads.html
- I want to "hold" the thread until all threads are ready. I know of "WaitGroups" from golang. So perhaps they exist in rust as well?
    - Searching for "rust waitgroup"
        - This seems to be like what I'm looking for... https://docs.rs/crossbeam/0.5.0/crossbeam/sync/struct.WaitGroup.html
            - References https://doc.rust-lang.org/std/sync/struct.Barrier.html
            - "Barrier" looks like what I want. I'll know the number of potential threads so this should be fine
- I used the provided examples and created a first simple program
    - Got hooked on a Question; Using the "Barrier" example and going over the documentation I couldn't figure out at which point the `c.wait();` lock would release
        - I've decided to put a sleep command before and after the `c.wait();` lock and watched the behavior
            - https://doc.rust-lang.org/std/thread/fn.sleep.html
            - The `c.wait();` lock is released as soon as the "last" thread got pushed into the "Barrier Arc"
    - Great it works. I can spawn a dynamic number of threads, hold them until they are all "initialized", continue them and wait until they've all run.
- Next I'll need to make some structs
    - Where do I place them? In their own file, inside a new directory?
        - Decided to use a new file called "scanner.rs"
            - Created a struct called `Scanner`
                - What's the correct type for and unsigned integer? The port is always >= 0
                    - Searching "rust unsigned int"
                        - https://doc.rust-lang.org/book/ch03-02-data-types.html
                        - `u16` seems to be the type I'm looking for
```rust
use std::sync::{Arc, Barrier};
use std::thread::JoinHandle;
use std::time::Duration;

struct Scanner {
   handles: Vec<JoinHandle<()>>,
   barrier: Arc<Barrier>,

   target: String,
   start_port: u16,
   max_port: u16,
   timeout: Duration
}
```
- The Scanner struct now need to have some methods
    - Searching for "rust struct methods"
        - https://doc.rust-lang.org/book/ch05-03-method-syntax.html
        - `impl StructName` is the magic syntax
    - Turns out you have to use `&mut self` instead of `self` if the method "mutates?" / updates an attribute
    - `pub(crate)` in front of a function makes it accessible outside the file
- The scanner might need some options, so I added a new struct called `Options` and added it to the Scanner
```rust
struct Options {
    pub response: bool, // Check if the connection returns any bytes
    pub udp: bool,      // use udp
    pub tcp: bool       // use tcp
}
```
- How are getters and setters handled?
    - Searching for "rust getter setter convention"
        - https://users.rust-lang.org/t/idiomatic-naming-for-getters-setters/3581
            - `set_attribute_name(attr: Type)` and `attribute_name() -> Type` for the getter
        - https://www.reddit.com/r/rust/comments/65ud89/how_to_name_setters_and_getters_when_a_struct_can/
            - There is an "auto builder"?
                - https://docs.rs/derive_builder/0.4.4/derive_builder/
                - Looks easy to implement.
```rust
#[derive(Builder)]
pub(crate) struct Options {
    pub response: bool, // Check if the connection returns any bytes
    pub udp: bool,      // use udp
    pub tcp: bool       // use tcp
}
```
- Putting everything together:
```rust
fn main() {
    let target = String::from("somedomain.tld");
    let opt = scanner::Options::default().udp(false).tcp(true).response(false).build().unwrap();

    let mut s = scanner::build_scanner(opt);
    s.set_target(target);
    s.set_port_range(0, 65535);
    s.set_timeout(Duration::from_secs(10));
}
```
- The next steps will require some error and "response" handling
- How are errors handled?
    - Searching for "rust error handling"
        - https://doc.rust-lang.org/book/ch09-00-error-handling.html
            - Well, that was underwhelming
        - https://www.sheshbabu.com/posts/rust-error-handling/
            - Error / response handling works differently than expected
            - Custom error structs not required in an application project but a nice to have
            - Response is always an enum (OK, Error)
            - Error can be an enum of many error types
            - There is a "?" operator which is a bit magic but seems to return the error if it occurred
            - Where do you handle the errors? Are all errors handled within main or along the way?
                - Depends on the use case - if the error is recoverable, then handle the error in between
            - Using `Box<dyn std::error::Error>` many errors can be returned / handled
                - Searching for "rust Box<dyn"
                    - https://doc.rust-lang.org/rust-by-example/trait/dyn.html
                        - A "Box" is just a reference to some function
            - Custom errors can be implemented via enums
- How is `match` correctly used?
    - Searching for "rust match"
        - https://doc.rust-lang.org/rust-by-example/flow_control/match.html
            - It's basically a switch statement with magic
            - Returns the assigned value
- Time to run the code and see what's getting thrown
- Getting error "error: cannot find derive macro `Builder` in this scope"
    - Searching "error: cannot find derive macro `Builder` in this scope"
    - Searching for "rust derive(Builder)"
        - https://docs.rs/derive_builder/latest/derive_builder/
        - Dependency and macro wasn't set
- Getting error "error[E0468]: an `extern crate` loading macros must be at the crate root"
    - https://stackoverflow.com/questions/39175953/how-do-you-import-macros-in-submodules-in-rust
        - `extern crate derive_builder;` has to be placed within the main.rs
        - `derive_builder` dependency was missing inside the Cargo.toml file
- Searching "rust getters and setters struct"
    - https://stackoverflow.com/questions/35390615/writing-getter-setter-properties-in-rust
        - Looks like setters work a bit different - update the attribute pointer instead of updating the attribute itself
- Get error "cannot move out of `*self` which is behind a mutable reference"
    - Searching for "cannot move out of `*self` which is behind a mutable reference"
        - Happens if you move literal inside an implementation and switch between mut and simple methods
    - CLI help `rustc --explain E0507`
    - Combining the questionable method with the one using it. Problem solved :)
- Refactoring the Options struct and the main method to implement the recent changes
```rust
#[derive(Default)]
pub(crate) struct Options {
  pub response: bool,
  // Check if the connection returns any bytes
  pub udp: bool,
  // use udp
  pub tcp: bool, // use tcp
}
```
```rust
fn main() {
    let target = String::from("somedomain.tld");

    let mut opt = scanner::build_options();
    *opt.udp_mut() = false;
    *opt.tcp_mut() = false;
    *opt.response_mut() = false;

    let mut s = scanner::build_scanner(opt);
    s.set_target(target);
    s.set_port_range(0, 65535);
    s.set_timeout(Duration::from_secs(10));

    match s.start() {
        Ok(()) => println!("Completed"),
        Err(e) => {
            match e {
                ScannerError::InvalidPortRange => println!("Scanner error: {}", e),
            }
        }
    };
}
```
- Getting a stack overflow error
    - Using `RUST_BACKTRACE=1 cargo run` can be used to get additional information
        - `RUST_BACKTRACE=full` provides even more detail
- Searching for "thread '<unnamed>' panicked at 'failed to allocate an alternative stack"
    - https://github.com/rust-lang/rust/issues/78497
        - Looks like I'm not the first who wants to spawn up to 65530 threads
            - It's not going to happen?
- Searching for "rust spawn thousands of threads"
    - https://stackoverflow.com/questions/49573335/what-is-the-maximum-number-of-threads-a-rust-program-can-spawn
        - https://stackoverflow.com/questions/344203/maximum-number-of-threads-per-process-in-linux
            - Max number of threads can be determined under linux with this command: `cat /proc/sys/kernel/threads-max`
                - Returns `255609` - So this isn't the problem
- Keeps crashing at ~16337 spawned threads
    - Checking my available ram `free -h`
```
              total        used        free      shared  buff/cache   available
Mem:           31Gi        22Gi       1,4Gi       312Mi       7,5Gi       8,2Gi
Swap:          11Gi        46Mi        11Gi
```
- Killed Firefox:
```
              total        used        free      shared  buff/cache   available
Mem:           31Gi        14Gi       9,2Gi       266Mi       7,5Gi        15Gi
Swap:          11Gi        46Mi        11Gi
```
- Still can't spawn more threads
- How many threads are currently being executed by the system?
    - Looking at htop revels ~2400
- Searching for `rust failed to spawn thread "Resource temporarily unavailable"`
    - https://github.com/rust-lang/rust/issues/46345
        - Introducing a delay doesn't work `thread::sleep(Duration::from_millis(1))`
    - https://github.com/rust-lang/rust/issues/77894
        - There might be something like a process limit
            - Searching for "rust process limit"
                - Nothing interesting comes up
    - https://unix.stackexchange.com/questions/253903/creating-threads-fails-with-resource-temporarily-unavailable-with-4-3-kernel
        - https://www.freedesktop.org/software/systemd/man/systemd.resource-control.html#TasksMax=N
            - There might be some os limitations in place
            - Is `TasksMax` set in my environment?
                - `cat /etc/systemd/system.conf | grep "TasksMax"` returns a disabled config parameter "#DefaultTasksMax="
                - `cat /etc/systemd/user.conf | grep "TasksMax"` returns nothing
                - What's the default value if it isn't set?
                    - "freedesktop" links to https://www.kernel.org/doc/html/latest/admin-guide/cgroup-v1/pids.html which doesn't reveal anything
                    - https://www.freedesktop.org/software/systemd/man/systemd-system.conf.html#
                        - 4915 is the default - which seems odd, since I was able to spawn ~16337 threads
                            - Ahh the default isn't always 4915 - only sometimes..
- Searching for "rust spawn threads"
    - https://doc.rust-lang.org/std/thread/fn.spawn.html
        - A Channel might help?
            - No it can't help, since I can't keep it alive and monitor its incoming messages
        - https://doc.rust-lang.org/std/thread/struct.Builder.html#method.spawn
            - Perhaps a custom builder helps..
            - ..no it doesn't. The problem persists.
            - > Limiting myself to fewer threads inorder to be able to continue
- Searching for "rust send tcp packet"
    - https://doc.rust-lang.org/std/net/struct.TcpStream.html
        - Looks simple enough but perhaps unnecessary overhead?
        - I suspect this would limit the app to only do ACK checks
- How can you implement `std::io::Error` inside the `ScannerError`?
    - Searching for "rust implement std::io::Error"
        - https://stevedonovan.github.io/rust-gentle-intro/6-error-handling.html
- Searching for "rust resolve host to ip"
    - https://doc.rust-lang.org/std/net/trait.ToSocketAddrs.html
- Getting error "expected struct `Vec`, found struct `std::io::Error`"
    - Searching for "expected struct `Vec`, found struct `std::io::Error`"
        - https://stackoverflow.com/questions/62417235/expected-enum-stdresultresult-found-struct-stdvecvec
- Getting error "could not find `time` in `tokio`"
    - Searching for "could not find `time` in `tokio`"
        - https://users.rust-lang.org/t/could-not-find-time-in-tokio-0-2-10/37533
            - "time" is a feature that has to be loaded / included within the "Cargo.toml" file
- Searching for "rust timeout tcp stream"
    - https://stackoverflow.com/questions/30022084/how-do-i-set-connect-timeout-on-tcpstream
        - `tokio::time::timeout` provides this feature
- Getting the error "`Result<std::net::TcpStream, std::io::Error>` is not a future" after implementing the timeout
    - Searching for "`Result<std::net::TcpStream, std::io::Error>` is not a future"
    - Getting the hint "help: the trait `Future` is not implemented for `Result<std::net::TcpStream, std::io::Error>`"
        - What is "trait `Future`"?
            - Searching for "rust trait `Future`"
                - https://doc.rust-lang.org/std/future/trait.Future.html
                    - Something with async function results / await
                    - Result polling instead of... ?
    - Searching for "the trait `Future` is not implemented for `Result<std::net::TcpStream, std::io::Error>`"
    - `match tokio::time::timeout(self.timeout, TcpStream::connect(&target)).await` inside a thread isn't working because some `Future` is missing
        - required because of the requirements on the impl of `Future` for `tokio::time::Timeout<Result<std::net::TcpStream, std::io::Error>>` what ever that's supposed to mean
    - Searching for "rust required by this bound in `timeout`"
        - https://www.reddit.com/r/rust/comments/ejkf3w/asyncstd_timeout_with_custom_error_types_question/
            - block_on doesn't seem to work in this combination either
    - CLI help `rustc --explain E0277`
        - Not really helpful - types mismatch, but why does it want a "Future"?
- Searching for "rust tokio timeout tcpstream future"
    - https://fasterthanli.me/articles/understanding-rust-futures-by-going-way-too-deep
        - Dependencies can be added by calling `cargo add dependency_name@version`
- Searching for "rust tokio::time::timeout with TcpStream::connect"
    - https://docs.rs/tokio-postgres/0.5.0-alpha.1/i686-apple-darwin/src/tokio_postgres/connect_socket.rs.html
- Checked the TcpStream::connect source code and found the method `connect_timeout`
    - Removed tokio and switched to this one
        - Getting error " `self` has an anonymous lifetime `'_` but it needs to satisfy a `'static` lifetime requirement"
            - The timeout has been moved into a new variable `let timeout = self.timeout;` this fixes the issue
- Searching for "rust cli arguments"
    - https://doc.rust-lang.org/book/ch12-01-accepting-command-line-arguments.html
        - Basic argument parsing
    - https://github.com/clap-rs/clap
        - Advanced parsing, perhaps a bit overpowered
        - Searching for "rust clap cli example"
            - https://rustrepo.com/repo/clap-rs-clap-rust-command-line
                - The Builder seems about perfect
                - To run `cargo run` with arguments, you have to it with appended "--" like this: "cargo run -- --max_port 25 --start_port 20 target.tld"