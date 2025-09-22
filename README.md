# rcgen-cli

rcgen-cli is a simple command-line tool to generate self-signed certificates, ideal for prototyping and TLS validation, especially when working with QUIC or HTTP/3.

This binary is built on top of the rcgen
 crate.

## Features

- Generates a self-signed X.509 certificate.

- Supports IP addresses only (SAN = Subject Alternative Name).

- Accepts one or more IP addresses, at least one is required.

- Produces two files:

cert.pem → the generated certificate.

key.pem → the corresponding private key.

- Compatible with servers like quiche
.

## Installation

Clone the repository and build the project in release mode:
```bash 
git clone https://github.com/<user>/rcgen-cli.git
cd rcgen-cli
cargo build --release
```



## Usage

Basic command
```bash
rcgen-cli --ip 192.168.1.20 192.168.1.30
```


--ip : one or more IP addresses to include in the SAN field of the certificate.

At least one IP address is required.

## Example
```bash
./rcgen-cli --ip 192.168.1.15
```


## Output:

Create auto-signed certificate for [Args { ip: [192.168.1.15, 192.168.1.16] }]
Certified keys written successfully! => cert.pem, key.pem

 
 ## Generated files

cert.pem : the self-signed certificate containing all provided IP addresses.

key.pem : the private key associated with the certificate.

These files are created in the current working directory where the command is executed.
