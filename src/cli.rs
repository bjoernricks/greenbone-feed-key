// SPDX-FileCopyrightText: 2026 Greenbone AG
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Port to listen on
    #[arg(short, long, env = "GREENBONE_FEED_KEY_PORT", default_value_t = 3000)]
    pub port: u16,

    /// Server IP address to bind to
    #[arg(short, long, env = "GREENBONE_FEED_KEY_SERVER", default_value_t = String::from("127.0.0.1"))]
    pub server: String,

    /// Path to the feed key file
    #[arg(short = 'k', long, env = "GREENBONE_FEED_KEY_PATH", default_value_t = String::from("/etc/gvm/greenbone-enterprise-feed-key"))]
    pub feed_key_path: String,

    /// Tracing log level directive
    #[arg(short, long, env = "GREENBONE_FEED_KEY_LOG", default_value_t = format!("{}=info", env!("CARGO_CRATE_NAME")))]
    pub log: String,

    /// Path to TLS server certificate (.pem) file (enables HTTPS)
    #[arg(
        long,
        env = "GREENBONE_FEED_KEY_TLS_SERVER_CERT",
        requires = "tls_server_key"
    )]
    pub tls_server_cert: Option<String>,

    /// Path to TLS server key file (enables HTTPS)
    #[arg(
        long,
        env = "GREENBONE_FEED_KEY_TLS_SERVER_KEY",
        requires = "tls_server_cert"
    )]
    pub tls_server_key: Option<String>,

    /// Path to TLS client certificates (.pem) file (enables mTLS)
    #[arg(
        long,
        env = "GREENBONE_FEED_KEY_TLS_CLIENT_CERTS",
        requires = "tls_server_cert",
        requires = "tls_server_key"
    )]
    pub tls_client_certs: Option<String>,

    /// Maximum upload size in bytes for feed key uploads
    #[arg(long, env = "GREENBONE_FEED_KEY_UPLOAD_LIMIT")]
    pub upload_limit: Option<usize>,
}

impl Default for Cli {
    fn default() -> Cli {
        Cli::parse()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn try_parse_from(args: Vec<&str>) -> Cli {
        Cli::try_parse_from(["test"].into_iter().chain(args.into_iter()))
            .expect("unable to parse args")
    }

    #[test]
    fn test_default_cli() {
        let cli = try_parse_from(vec![]);

        assert_eq!(cli.port, 3000);
        assert_eq!(cli.server, "127.0.0.1");
        assert_eq!(cli.upload_limit, None);
        assert_eq!(cli.feed_key_path, "/etc/gvm/greenbone-enterprise-feed-key");
        assert_eq!(cli.log, format!("{}=info", env!("CARGO_CRATE_NAME")));
        assert_eq!(cli.tls_server_cert, None);
        assert_eq!(cli.tls_server_key, None);
        assert_eq!(cli.tls_client_certs, None);
        assert_eq!(cli.upload_limit, None);
    }
}
