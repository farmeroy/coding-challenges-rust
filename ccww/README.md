# My solution to the WC challenge

You can find the challenge description [here](https://codingchallenges.fyi/challenges/challenge-wc/)

The challenge is to recreate a simplified version of the unix tool `wc` (word count).

  In the spirit of minimilism I opted *not* to use an external crate, such as `clap`, to parse my commands and arguments. 
  Instead, I store the args, including the target filename, in a struct:
  
```rust
struct Args {
    bytes: bool,
    lines: bool,
    words: bool,
    chars: bool,
    file_name: String,
}
```

One problem I encountered is deciding when attempt to open a file and when to turn to `sdtin`. 
I decided to use an enum:

```rust
enum WCTarget {
    UseStdin,
    TryFile,
}
```

and I use this enum when reading to the buffer:

```rust
    let reader: Result<Box<dyn BufRead>, Error> = match wc_target {
        WCTarget::TryFile => match File::open(file_name) {
            Err(e) => Err(e),
            Ok(file) => Ok(Box::new(BufReader::new(file))),
        },
        WCTarget::UseStdin => Ok(Box::new(BufReader::new(io::stdin()))),
    };
```

I can then exit the process if there is an Error result using `map_err`:

```rust
    let mut reader = reader
        .map_err(|_| {
            eprintln!("No such file");
            std::process::exit(1);
        })
        .unwrap();
```

This lead to the problem that, since `env::args` will likely contain the actual process as the first argument, my program was trying to parse the process as a file (in this case my `target/debug/ccwc`). 

To fix that, I set the `WCTarget` according to the length of the args:

```rust
    let wc_target = match args.len() == 1 {
        true => WCTarget::UseStdin,
        false => WCTarget::TryFile,
    };
```

This feels pretty hacky, and I would definitely not do this in production.
In fact, for any real CLI, I would 100% prefer to use `clap`. 
