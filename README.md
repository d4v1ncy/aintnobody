# aintnobody

Plays snippet of "ain't nobody" for 1284 milliseconds or so and die


## USAGE

### Library

#### Install

```bash
$ cargo add aintnobody
```

#### Code

```rust
use aintnobody::{Error, playintnobody};

fn main() -> Result<(), Error>{
    playintnobody(std::time::Duration::from_millis(1284))?;
    Ok(())
}

```

### Executable

#### Install

```bash
$ cargo install aintnobody
```

#### Run

```bash
$ aintnobody
```
