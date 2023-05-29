
# `csvgears`: command-line utilities to work with CSV files

This Rust crate provides the following command-line utilities:

- [`csvcut`](#csvcut)
- [`csvgrep`](#csvgrep)
- [`csvsed`](#csvsed)

These implement the basic functionalities of the homonymous utilities in the [`csvkit`](https://pypi.org/project/csvkit/) and [`sacsv`](https://github.com/gn0/sacsv) Python packages but with 6-to-12-times faster execution and with lower memory use.

## Installation

You need `cargo` to install `csvgears`.
If you are running Ubuntu, Debian, or another Linux distribution that uses `apt`, then you can install it by running `sudo apt install cargo`.
After that, run:

```
$ cargo install --git https://codeberg.org/gnyeki/csvgears.git
```

If `$HOME/.cargo/bin` is not in your `PATH` environment variable yet, then you also need to run:

```
$ export PATH=$HOME/.cargo/bin:$PATH
```

To make this setting permanent:

```
$ echo 'export PATH=$HOME/.cargo/bin:$PATH' >> $HOME/.bashrc  # If using bash.
$ echo 'export PATH=$HOME/.cargo/bin:$PATH' >> $HOME/.zshrc   # If using zsh.
```

## Examples

### `csvcut`

This utility selects columns from the input data.
To select specific columns:

```
$ printf "foo,bar,baz\n1,2,3\n4,5,6\n" | csvcut -c foo,baz
foo,baz
1,3
4,6
$
```

To exclude specific columns:

```
$ printf "foo,bar,baz\n1,2,3\n4,5,6\n" | csvcut -C foo,baz
bar
2
5
$
```

### `csvgrep`

This utility selects rows from the input data by checking if cell values match a regular expression or contain a fixed string.
With regular expressions:

```
$ printf "foo,bar,baz\nlorem,ipsum,dolor\nsit,amet,\n" | csvgrep -c bar -r 'm$'
foo,bar,baz
lorem,ipsum,dolor
$
```

With fixed strings:

```
$ printf "foo,bar,baz\nlorem,ipsum,dolor\nsit,amet,\n" | csvgrep -c bar -m m
foo,bar,baz
lorem,ipsum,dolor
sit,amet,
$
```

Matches can be inverted with the `-i` option:

```
$ printf "foo,bar,baz\nlorem,ipsum,dolor\nsit,amet,\n" | csvgrep -i -c bar -r 'm$'
foo,bar,baz
sit,amet,
$
```

### `csvsed`

This utility modifies cell values by replacing occurrences of a pattern with the replacement given.

```
$ printf "foo,bar,baz\nlorem,ipsum,dolor\nsit,amet,\n" \
    | csvsed -c bar -p '.m' -t 'x'
foo,bar,baz
lorem,ipsx,dolor
sit,xet,
$
```

Since `csvgears` uses the [`regex`](https://crates.io/crates/regex) crate as its regular expression engine, named capture groups can also be used in the pattern and the replacement:

```
$ printf "foo,bar\n2023-05-29,ipsum\n1970-01-01,dolor\n" \
    | csvsed \
        -c foo \
        -p '^(?P<y>\d{4})-(?P<m>\d{2})-(?P<d>\d{2})$' \
        -t '$m/$d/$y'
foo,bar
05/29/2023,ipsum
01/01/1970,dolor
$
```

### Alternative delimiters

`csvgears` supports delimiters other than comma in the input.
It always uses comma as the delimiter in the output, consistently with the behavior of [`csvkit`](https://pypi.org/project/csvkit/):

```
$ printf "foo:bar:baz\n1:2:3\n4:5:6\n" | csvcut -d : -c foo,baz
foo,baz
1,3
4,6
$
```

For tab-separated input, the delimiter has to be specified as a `<TAB>` character.
A portable way to do this is to use `printf`:

```
$ printf "foo\tbar\tbaz\n1\t2\t3\n4\t5\t6\n" | csvcut -d "$(printf "\t")" -c foo,baz
foo,baz
1,3
4,6
$
```

