# `samp` (sampler)

`samp` is a simple command-line program that randomly samples lines from standard input. This can
be used to trim down on large newline-delimited files for testing or other purposes.

## Installation

To get started, install `samp` from source:

```shell
# Directly from Github
cargo install --git https://github.com/jerluc/samp.git

# Or from local source
git clone https://github.com/jerluc/samp.git && cd samp/ && cargo install --path .
```

## Usage

To use `samp`:

```
Usage: samp [-r <ratio>] [-s <seed>]

Sample stdin

Options:
  -r, --ratio       sample ratio
  -s, --seed        seed string
  --help            display usage information
```

For example, here's how you can randomly sample ~10% of your computer's dictionary file:

```shell
cat /usr/share/dict/words | samp -r 0.1
```

And here's how you can randomly sample ~5% of "War and Peace" using a reproducible text seed:

```shell
# Save sample to file
curl -s https://www.gutenberg.org/cache/epub/2600/pg2600.txt | samp -r 0.05 -s tolstoy > wp.txt

# Save second sample to another file
curl -s https://www.gutenberg.org/cache/epub/2600/pg2600.txt | samp -r 0.05 -s tolstoy > wp2.txt

diff wp.txt wp2.txt
# No differences!
```

## Motivations

I basically had two motivations in creating this software:

1. I often find myself working with very large, newline-delimited data; I use `samp` to randomly
   down-sample this data for running various tests
2. I wanted an excuse to practice some more Rust :)

## Contributing

When contributing to this repository, please follow the steps below:

1. Fork the repository
2. Submit your patch in one commit, or a series of well-defined commits
3. Submit your pull request and make sure you reference the issue you are addressing

## License

See [LICENSE](LICENSE)
