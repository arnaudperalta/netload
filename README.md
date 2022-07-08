# netload

![example workflow](https://github.com/arnaudperalta/netload/actions/workflows/ci.yml/badge.svg)
[![HitCount](https://hits.dwyl.com/arnaudperalta/netload.svg?style=flat-square)](http://hits.dwyl.com/arnaudperalta/netload)

Simple load tester written in Rust. All requests are started concurrently to simulate an high traffic from the real world.

## Arguments
```
-b, --body <BODY>          JSON Body
-c, --count <COUNT>        Number of total requests to perform on the API [default: 1000]
-h, --headers <HEADERS>    Headers (key=value)
    --help                 Print help information
-m, --method <METHOD>      HTTP Method [default: get]
-s, --speed <SPEED>        Number of query per second [default: 100]
-u, --url <URL>            Target URL
-V, --version              Print version information
```

## Live demo
<p align="center">
    <img src="https://i.imgur.com/g8rEcCQ.gif" alt="gif" />
</p>
