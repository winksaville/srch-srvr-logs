# Search Server Logs

Ssh to a service and use journalctl to search services log output that match a regex

# Build
```
wink@3900x 22-09-01T23:49:29.829Z:~/prgs/rust/myrepos/srch-srvr-logs (main)
$ cargo build
   Compiling proc-macro2 v1.0.43
   Compiling version_check v0.9.4
   Compiling quote v1.0.21
   Compiling unicode-ident v1.0.3
   Compiling syn v1.0.99
   Compiling libc v0.2.132
   Compiling autocfg v1.1.0
   Compiling hashbrown v0.12.3
   Compiling os_str_bytes v6.3.0
   Compiling heck v0.4.0
   Compiling strsim v0.10.0
   Compiling once_cell v1.13.1
   Compiling textwrap v0.15.0
   Compiling bitflags v1.3.2
   Compiling termcolor v1.1.3
   Compiling clap_lex v0.2.4
   Compiling proc-macro-error-attr v1.0.4
   Compiling proc-macro-error v1.0.4
   Compiling indexmap v1.9.1
   Compiling atty v0.2.14
   Compiling clap_derive v3.2.18
   Compiling clap v3.2.19
   Compiling srch-srvr-logs v0.1.0 (/home/wink/prgs/rust/myrepos/srch-srvr-logs)
    Finished dev [unoptimized + debuginfo] target(s) in 6.46s
```

# Run

No parameters print some help
```
wink@3900x 22-09-01T23:49:38.129Z:~/prgs/rust/myrepos/srch-srvr-logs (main)
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/srch-srvr-logs`
error: The following required arguments were not provided:
    <ACCT_NAME>
    <REGEX>

USAGE:
    srch-srvr-logs [OPTIONS] <ACCT_NAME> <REGEX>

For more information try --help
```

Print help use `--help` option:
```
wink@3900x 22-09-02T00:19:02.628Z:~/prgs/rust/myrepos/srch-srvr-logs (main)
$ cargo run -- --help
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/srch-srvr-logs --help`
srch-srvr-logs 0.1.0

USAGE:
    srch-srvr-logs [OPTIONS] <ACCT_NAME> <REGEX>

ARGS:
    <ACCT_NAME>    ssh login, account@machine
    <REGEX>        rust regex string, an empty string, "", matches everything

OPTIONS:
    -h, --help                      Print help information
    -s, --services <SERVICES>...    [default: eth1 beacon-chain validator]
    -t, --since <SINCE>             journalctl --since parameter but use '=', Example: --since=-1h
                                    [default: -1d]
    -V, --version                   Print version information
```

Simple search:
```
wink@3900x 22-09-01T23:49:59.092Z:~/prgs/rust/myrepos/srch-srvr-logs (main)
$ cargo run kendall@hazel "Starting\|ERRO"
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/srch-srvr-logs 'kendall@hazel' 'Starting\|ERRO'`
Process eth1.service:
Aug 31 19:48:09 hazel geth[2138]: ERROR[08-31|19:48:09.309] Demoting invalidated transaction         hash=ea7f79..190b77
Sep 01 14:39:46 hazel geth[2138]: ERROR[09-01|14:39:46.772] Demoting invalidated transaction         hash=cff1d4..c7f337
Sep 01 14:39:46 hazel geth[2138]: ERROR[09-01|14:39:46.772] Demoting invalidated transaction         hash=e47c1b..0dbaac

Process beacon-chain.service:
Sep 01 12:41:13 hazel lighthouse[2304]: Sep 01 19:41:13.106 INFO Starting database compaction            new_finalized_epoch: 143845, old_finalized_epoch: 143844, service: beacon

Process validator.service:
 <no matches for regex: "Starting\|ERRO>"
```

You can specify a single service, such as "chrony" and if regex is "" all lines will match:
```
wink@3900x 22-09-02T00:12:29.150Z:~/prgs/rust/myrepos/srch-srvr-logs (main)
$ cargo run kendall@robert "" --since=-3d --services chrony
   Compiling srch-srvr-logs v0.1.0 (/home/wink/prgs/rust/myrepos/srch-srvr-logs)
    Finished dev [unoptimized + debuginfo] target(s) in 0.49s
     Running `target/debug/srch-srvr-logs 'kendall@robert' '' --since=-3d --services chrony`
Process chrony.service:
Aug 30 12:10:48 robert systemd[1]: Stopping chrony, an NTP client/server...
Aug 30 12:10:48 robert chronyd[898]: chronyd exiting
Aug 30 12:10:48 robert systemd[1]: chrony.service: Succeeded.
Aug 30 12:10:48 robert systemd[1]: Stopped chrony, an NTP client/server.
-- Boot dc8a145c491648dbacdcb5e79751327d --
Aug 30 12:11:40 robert systemd[1]: Starting chrony, an NTP client/server...
Aug 30 12:11:40 robert chronyd-starter.sh[888]: WARNING: libcap needs an update (cap=40 should have a name).
Aug 30 12:11:40 robert chronyd[892]: chronyd version 3.5 starting (+CMDMON +NTP +REFCLOCK +RTC +PRIVDROP +SCFILTER +SIGND +ASYNCDNS +SECHASH +IPV6 -DEBUG)
Aug 30 12:11:40 robert chronyd[892]: Frequency -28.703 +/- 0.147 ppm read from /var/lib/chrony/chrony.drift

<snip>..

Aug 30 12:36:43 robert chronyd[51777]: chronyd exiting
Aug 30 12:36:43 robert systemd[1]: Stopping chrony, an NTP client/server...
Aug 30 12:36:43 robert systemd[1]: chrony.service: Deactivated successfully.
Aug 30 12:36:43 robert systemd[1]: Stopped chrony, an NTP client/server.
-- Boot b5a4d5680bfc4f4db0556d23c0a8ef66 --
Aug 30 12:37:30 robert systemd[1]: Starting chrony, an NTP client/server...
Aug 30 12:37:30 robert chronyd[1121]: chronyd version 4.2 starting (+CMDMON +NTP +REFCLOCK +RTC +PRIVDROP +SCFILTER +SIGND +ASYNCDNS +NTS +SECHASH +IPV6 -DEBUG)
Aug 30 12:37:30 robert chronyd[1121]: Frequency -28.945 +/- 0.499 ppm read from /var/lib/chrony/chrony.drift
Aug 30 12:37:30 robert chronyd[1121]: Loaded seccomp filter (level 1)
Aug 30 12:37:30 robert systemd[1]: Started chrony, an NTP client/server.
Aug 30 12:37:39 robert chronyd[1121]: Selected source 17.253.4.125 (time.apple.com)
Aug 30 12:37:39 robert chronyd[1121]: System clock was stepped by 0.455351 seconds
Aug 30 12:37:41 robert chronyd[1121]: Selected source 192.168.1.110
```

Also list mutliple services:
```
wink@3900x 22-09-02T00:14:56.667Z:~/prgs/rust/myrepos/srch-srvr-logs (main)
$ cargo run kendall@robert "Starting" --since=-3d --services chrony eth1 validator
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/srch-srvr-logs 'kendall@robert' Starting --since=-3d --services chrony eth1 validator`
Process chrony.service:
Aug 30 12:11:40 robert systemd[1]: Starting chrony, an NTP client/server...
Aug 30 12:32:40 robert systemd[1]: Starting chrony, an NTP client/server...
Aug 30 12:37:30 robert systemd[1]: Starting chrony, an NTP client/server...

Process eth1.service:
Aug 30 12:11:46 robert systemd[1]: Starting geth eth1 service...
Aug 30 12:11:48 robert geth[1106]: INFO [08-30|12:11:48.234] Starting pprof server                    addr=http://127.0.0.1:6060/debug/pprof
Aug 30 12:11:48 robert geth[1106]: INFO [08-30|12:11:48.235] Starting Geth on Ethereum mainnet...
Aug 30 12:12:01 robert geth[1106]: INFO [08-30|12:12:01.544] Starting peer-to-peer node               instance=Geth/v1.10.23-stable-d901d853/linux-amd64/go1.18.5
Aug 30 12:37:35 robert systemd[1]: Starting geth eth1 service...
Aug 30 12:37:37 robert geth[1194]: INFO [08-30|12:37:37.158] Starting pprof server                    addr=http://127.0.0.1:6060/debug/pprof
Aug 30 12:37:37 robert geth[1194]: INFO [08-30|12:37:37.159] Starting Geth on Ethereum mainnet...
Aug 30 12:37:47 robert geth[1194]: INFO [08-30|12:37:47.974] Starting peer-to-peer node               instance=Geth/v1.10.23-stable-d901d853/linux-amd64/go1.18.5

Process validator.service:
Aug 30 12:11:50 robert systemd[1]: Starting eth2 validator service...
Aug 30 12:11:52 robert lighthouse[1135]: Aug 30 19:11:52.182 INFO Starting validator client               validator_dir: "/home/kendall/eth2-data/lighthouse/mainnet/validators", beacon_nodes: ["http://localhost:5052/"]
Aug 30 12:37:39 robert systemd[1]: Starting eth2 validator service...
Aug 30 12:37:41 robert lighthouse[1406]: Aug 30 19:37:41.625 INFO Starting validator client               validator_dir: "/home/kendall/eth2-data/lighthouse/mainnet/validators", beacon_nodes: ["http://localhost:5052/"]
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
