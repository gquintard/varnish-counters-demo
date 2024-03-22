# Building a worse varnishstat for fun and teaching

# Build

You will need `rust`, `cargo` as well as `varnish` 7.5 and its dev libraries.

``` shell
cargo build
```

# Run

``` shell
./target/debug/varnish-counters-demo -g '*LCK.*' | head
+---------+------------------------+--------+
| section | name                   | value  |
+---------+------------------------+--------+
| LCK     | pipestat.destroy       | 0      |
+---------+------------------------+--------+
| LCK     | perpool.locks          | 996    |
+---------+------------------------+--------+
| LCK     | busyobj.destroy        | 1      |
...
```

Use `-h` or `--help` to get the usage message.

# Learning more

Check out the [docs](https://docs.rs/varnish/latest/varnish/vsc/index.html), and [src/main.rs.simple](src/main.rs.simple)
