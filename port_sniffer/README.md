# port_sniffer

```
➜  port_sniffer git:(master) ✗ cargo run -- -h
    Finished dev [unoptimized + debuginfo] target(s) in 0.03s
     Running `/Users/kangxiaoning/workspace/rust-exercises/target/debug/port_sniffer -h`
port_sniffer 1.0
kangxiaoning. <kangxiaoning.dba@gmail.com>
Port sniffer

USAGE:
    port_sniffer [OPTIONS] <IPADDRESS>

ARGS:
    <IPADDRESS>    Sets the IPADDRESS to be sniffed

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -b, --begin_port <BEGIN_PORT>            Sets start port to be sniffed (default 1)
    -e, --end_port <END_PORT>                Sets end port to be sniffed (default 1024)
    -n, --threads_number <THREADS_NUMBER>    Sets number of threads to execute (default 1)
➜  port_sniffer git:(master) ✗
```
