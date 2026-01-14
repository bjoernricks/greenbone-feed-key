# greenbone-feed-key <!-- omit in toc -->

Service for Uploading a Greenbone Feed Key

- [Settings](#settings)
- [TLS](#tls)

## Settings

| CLI              | Env                           | Type   | Default                                  | Description                                                                                                               |
| ---------------- | ----------------------------- | ------ | ---------------------------------------- | ------------------------------------------------------------------------------------------------------------------------- |
| `-p, --port`     | `GREENBONE_FEED_KEY_PORT`     | int    | `3000`                                   | Port to listen on                                                                                                         |
| `-s, --server`   | `GREENBONE_FEED_KEY_SERVER`   | string | `127.0.0.1`                              | IP address or domain to listen on                                                                                         |
| `-k, --key-path` | `GREENBONE_FEED_KEY_PATH`     | string | `/etc/gvm/greenbone-enterprise-feed-key` | Path to the enterprise feed key location                                                                                  |
| `-l, --log`      | `GREENBONE_FEED_KEY_LOG`      | string | `greenbone_feed_key=info`                | [Logging directive](https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html#directives) |
| `--tls-cert`     | `GREENBONE_FEED_KEY_TLS_CERT` | file   |                                          | Path to a TLS certificate file                                                                                            |
| `--tls-key`      | `GREENBONE_FEED_KEY_TLS_KEY`  | file   |                                          | Path to a TLS private key file                                                                                            |

## TLS

See [TLS documentation](./certs/README.md) for more details
