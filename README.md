special-git-commit
===

A command line tool for generating commit hashes with specified prefixes.

- Simple and readable
- Using libgit2 API instead of calling `git xxx` commands
- Using multiple threads

## Installation

### Cargo install [![Crates.io Version](https://img.shields.io/crates/v/special-git-commit.svg)](https://crates.io/crates/special-git-commit)

```
cargo install special-git-commit
```

### Build from source

```
git clone https://github.com/OmmyZhang/special-git-commit-hash.git
cd special-git-commit-hash
cargo build --release
```

### Download from releases [![Release](https://github.com/OmmyZhang/special-git-commit-hash/actions/workflows/release.yml/badge.svg)](https://github.com/OmmyZhang/special-git-commit-hash/actions/workflows/release.yml)

Executable files for Linux, Mac and Windows.

[Releases](https://github.com/OmmyZhang/special-git-commit-hash/releases)

## Usage

```
special-git-commit <target-prefix>
```

or

```
special-git-commit
```

It will amend the HEAD commit with the target prefix. If no prefix is provided, the default prefix `0000000` will be used.

## How it works

Let the result of `git cat-file commit HEAD` be `contents`.

```
$ git cat-file commit HEAD                                                                                                                                                    master
tree 66b5178f6feba85812818aeaca5a86b07cdd106f
parent 0000004a2eddb98c4b005362dd4fe37636633b3f
author Tdxdxoz <tdxdxoz@gmail.com> 1711436406 +0800
committer Tdxdxoz <tdxdxoz@gmail.com> 1711436919 +0800

update README
```

The hash of this commit (HEAD) is `sha128("commit {contents.len()}\0{content}")`.

```
commit 216\0tree 66b5178f6feba85812818aeaca5a86b07cdd106f
parent 0000004a2eddb98c4b005362dd4fe37636633b3f
author Tdxdxoz <tdxdxoz@gmail.com> 1711436406 +0800
committer Tdxdxoz <tdxdxoz@gmail.com> 1711436919 +0800

update README
```

A prefix will be added to the committer's name, and checked if the new sha128 result matches.


## You can ...

### use it to
- have fun
- help find potential vulnerabilities of tools that use git commit hashes
    - only use short hashes
    - incorrectly interpret commit hashes as integers instead of strings (for example, in YAML)
    - ...

### and not use it
- in production


## Similar projects

- https://github.com/prasmussen/git-vanity-hash
- https://github.com/mattbaker/git-vanity-sha
- https://github.com/tochev/git-vanity
- https://github.com/recolic/git-commit-hash-vanitygen
