# Unix Socket Example

This repos shows how to use Unix sockets to transfer data between client and daemon processes. In this project:

- `tokio` is used to provide a stable unix socket stream api for fetch stream data and credential data.
- `clap` is used to parse command arguments so I can put client and daemon in a single binary execute.

Simply build and run, you could get help of the program with `-h` argument.

```
Usage: us_example <COMMAND>

Commands:
  server     start a daemon listen on /tmp/example.socket
  send       send some message to /tmp/example.socket
  send-file  read a text file and send to /tmp/example.socket
  help       Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```
