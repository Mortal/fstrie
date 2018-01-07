Key-value store in a trie
=========================

A filesystem-backed trie dictionary stores `key:value` pairs
in sorted text files on a disk such that the file at path
`a/b/e` stores all keys starting with `abe`, `ABE`, `Abe`, etc.
To search for a specific key, e.g. `AbEkAt`, in a database named `db`, the paths
`db`, `db/a`, `db/a/b`, `db/a/b/e`, `db/a/b/e/k`, `db/a/b/e/k/a` and `db/a/b/e/k/a/t`
are tried in succession until a file is found.
In the simplest case, `db` itself is just a sorted text file.

The file is then opened and searched line-by-line for matching keys.

This project implements efficient searching in such a key-value store
as a Python CFFI extension module written in Rust.

Thanks to
[Armin Ronacher](https://github.com/mitsuhiko)
for his excellent PyCon talk on
[A Python and Rust love story](https://www.youtube.com/watch?v=zmtHaZG7pPc).

Prerequisites
-------------

* Python
* Python cffi module
* Python wheel module
* libffi
* Rust and Cargo

Ubuntu's Rust 1.17 is too old; use [rustup](https://www.rustup.rs/)
to install the most recent stable Rust.

Example
-------

```
$ pip install --user https://github.com/Mortal/fstrie/releases/download/v0.1.0/fstrie-0.1.0-py2.py3-none-manylinux1_x86_64.whl
$ sort /etc/passwd > passwd.txt
$ python3 -m fstrie passwd.txt nobody
x:99:99:nobody:/:/usr/bin/nologin
```

Building with Docker
--------------------

Remove `target` (containing Cargo's output) and `py/build`, `py/dist`,
`py/fstrie.egg-info`, `py/.eggs` (containing setup.py's output) before running
`make wheel-manylinux`.
