use clap::Parser;
use std::error::Error;
use std::ffi::OsStr;
use std::fmt::Display;
use std::process::Command;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
struct CliArgs {
    #[clap(short, long)]
    /// Port to connect to
    port: Option<Vec<String>>,
}

/* Errors */

#[derive(Debug)]
struct ParseIpError<'a>(&'a str);

impl<'a> Display for ParseIpError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} is not a valid IPv4 address", self.0)
    }
}

impl<'a> Error for ParseIpError<'a> {}

#[derive(Debug)]
struct ParsePortPairError<'a>(&'a str);

impl<'a> Display for ParsePortPairError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} is not a valid port pair", self.0)
    }
}

impl<'a> Error for ParsePortPairError<'a> {}

/* Argument types */

struct PortPairs<'a> {
    local: &'a str,
    remote: &'a str,
}

type IPv4 = [i32; 4];

/* Steps */

fn parse_ip(raw: &str) -> Result<IPv4, ParseIpError> {
    todo!()
}

fn get_port_pairs<'a>(args: &Vec<&'a str>) -> Result<Vec<PortPairs<'a>>, ParsePortPairError<'a>> {
    todo!()
}

fn get_command(ip: &IPv4, port_pairs: &Vec<PortPairs>) -> Command {
    todo!()
}

/* Tests */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_ip() {
        let ip = "192.168.73.20";
        let result = parse_ip(ip);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), [192, 168, 73, 20]);
    }

    #[test]
    fn should_fail_parse_ip() {
        let ip = "12312215";
        let result = parse_ip(ip);
        assert!(result.is_err());
    }

    #[test]
    fn should_get_port_pairs() {
        let port_pairs = vec!["3000:3000", "23:38989"];
        let result = get_port_pairs(&port_pairs);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result[0].local, "3000");
        assert_eq!(result[0].remote, "3000");
        assert_eq!(result[1].local, "23");
        assert_eq!(result[1].remote, "38989");
    }

    #[test]
    fn should_fail_get_port_pairs() {
        let port_pairs = vec!["3000:3000", "2338989"];
        let result = get_port_pairs(&port_pairs);
        assert!(result.is_err());
    }
}

fn main() {
    let args = CliArgs::parse();
    println!("{:?}", args);
}
