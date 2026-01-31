# What The... IP?

A tiny CLI tool to fetch IP address information.

> Because sometimes you just want to know: *what the… IP?*

## Usage

```bash
Usage: what-the-ip [OPTIONS]

Options:
  -a, --ip <IP>            IP address to look up (defaults to your own IP)
  -j, --json               Output in JSON format (default = false)
  -t, --timeout <TIMEOUT>  Timeout in seconds (default = 10) [default: 10]
  -p, --proxy <PROXY>      Proxy server to use for the request
  -e, --extra-metadata     Include extra metadata in the output (default = false)
  -h, --help               Print help
  -V, --version            Print version
```

## Example

## Fetch information about an IP address

```bash
what-the-ip -a 1.1.1.1

ip: 1.1.1.1
hostname: one.one.one.one
city: Brisbane
region: Queensland
country: AU
loc: -27.4679,153.0281
org: AS13335 Cloudflare, Inc.
postal: 9010
timezone: Australia/Brisbane
readme: https://ipinfo.io/missingauth
```

## Fetch information about your own IP address

```bash
what-the-ip

...your ip info
```
