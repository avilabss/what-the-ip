# What The... IP?

A tiny CLI tool to fetch IP address information from [ipinfo.io](https://ipinfo.io/).

## Usage

```bash
Usage: what-the-ip [OPTIONS]

Options:
  -i, --ip <IP>  
  -h, --help     Print help
```

## Example

## Fetch information about an IP address

```bash
what-the-ip -i 1.1.1.1

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
