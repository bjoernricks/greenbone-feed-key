# TLS and Certificates <!-- omit in toc -->

This document explains how to handle TLS certificates for the feed key service.

The service uses [rustls] internally to handle the TLS connections.

- [Certificate Generation](#certificate-generation)
  - [Server Certificates Quick Start](#server-certificates-quick-start)
  - [Server Certificates with CA](#server-certificates-with-ca)
- [View Content of Certificates](#view-content-of-certificates)
  - [Server Private Key](#server-private-key)
  - [Server Certificate](#server-certificate)
  - [Server Certificate Signing Request (CSR)](#server-certificate-signing-request-csr)

## Certificate Generation

For a local setup it is possible to setup a self-signed certificate chain which
requires the `openssl` command to be installed.

### Server Certificates Quick Start

A self-signed TLS server private key and server certificate for testing purposes
can be generated with the following command easily

```sh
openssl req -newkey rsa:4096 -noenc -keyout server-key.pem -x509 -days 365 -out server-cert.pem -subj "/CN=ACME" -batch
```

### Server Certificates with CA

```sh
./server-certificates.sh
```

## View Content of Certificates

### Server Private Key

```sh
openssl rsa -noout -text -in ./server.key.pem
```

### Server Certificate

```sh
openssl x509 -noout -text -in ./server.cert.pem
```

### Server Certificate Signing Request (CSR)

```sh
openssl req -noout -text -in ./server.csr.pem
```

[rustls]: https://rustls.dev/
