# Dot


## Running Tests

To run the unit tests, run:

``` sh
$ cargo test -- --test-threads=1
```

To run the integration test, run:

``` sh
$ ./integration_test.sh
```

## Building

``` sh
$ cargo build
$ cargo build --release # to build optimized binary
```

## Installing

``` sh
$ cargo install --path .
```

## Usage

The integration tests provide an example on how to use the application. After you have a repo with your dotfiles already saved, just run

``` sh
$ dot init <repo>
```

and Dot will take care of the rest!

## Todo

- [ ] `dot status` should show the git status of the dotfiles dir
- [ ] `dot show` should show the path of the dotfiles dir
- [ ] `dot add` should add a move a file to the dotfiles dir and symlink the file
- [ ] Run clippy to lint the project and fix issues
