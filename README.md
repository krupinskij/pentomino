# [Pentomino](https://en.wikipedia.org/wiki/Pentomino)

## Usage

```bash
$ cargo run -- [options]
$ cargo run --release -- [options]
```

### Options
- `-b` | `--blocks` - character list of type of blocks according to [first convention](https://en.wikipedia.org/wiki/Pentomino#/media/File:Pentomino_Naming_Conventions.svg) (default `FILNPTUVWXYZ`)
- `-h` | `--height` - height of board (default `6`)
- `-w` | `--width` - width of board (default `10`)

### Example
```bash
$ cargo run --release
$ cargo run --release -- -b LLNP -w 5 -h 4
$ cargo run --release -- --blocks LLNP -h 4 -w 5
```


## Output
![output](output.png)