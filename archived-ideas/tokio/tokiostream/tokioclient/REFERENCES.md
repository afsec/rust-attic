# Tokio TCP streams

- https://tcpbin.com/
- [Getting started with tokio by creating an async tcp echo server | rust tokio tutorial | rustlang](https://www.youtube.com/watch?v=DJzgUmH30h8)
- [Tutorial github code](https://github.com/chrishayuk/chrishayuk-monorepo/tree/main/apps)

## Echo client/server
### netcat client: `nc 127.0.0.1 1234`-> Replace by Rust using Tokio TCP
###  socat server: s`socat -v tcp-l:1234,fork exec:'/bin/cat'` -> -> Replace by Rust using Tokio TCP