# See https://github.com/mitsuhiko/console/issues/36


This is an attempt to create a minimum repro for a potential bug with console crate.
The problem is that new lines do not start column 0.
Here is the example of the output I see on Mac:
```
$ ./run.sh 
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/repro_tui_bug`
Hello, world!
[2019-10-27T03:52:26Z DEBUG repro_tui_bug] Hello from main thread 1
                                                                   [2019-10-27T03:52:26Z DEBUG repro_tui_bug] Hello from main thread 2
                                                      [2019-10-27T03:52:33Z DEBUG repro_tui_bug] Please press any button
[src/main.rs:12] key = Ok(
    Char(
        'f',
    ),
)
[2019-10-27T03:52:33Z DEBUG repro_tui_bug] Hello from spawned thread 1
[2019-10-27T03:52:33Z DEBUG repro_tui_bug] Hello from spawned thread 2
$ 
```
