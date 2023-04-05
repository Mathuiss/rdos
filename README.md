### rdos

Rust implementation of SLdos

```bash
rdos is a tool made for DOS attacks on web servers.

Usage: rdos [OPTIONS] <target>

Arguments:
  <target>  The target host. Examples: 127.0.0.1:80, mywebsite.com:80

Options:
  -t, --threads <threads>  The size of the thread pool. [default: 64]
  -s, --size <size>        The size of the payloads. [default: 64]
  -d, --delay <delay>      The delay in miliseconds between sending the next payload. [default: 200]
  -h, --help               Print help
  -V, --version            Print version
```

Examples:

```bash
rdos 10.183.220.122:4000

rdos 10.183.220.122:4000 -t 200

rdos 10.183.220.122:4000 -t 200 -s 128

rdos 10.183.220.122:4000 -t 200 -s 128 -d 500
```