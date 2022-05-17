use clap::Parser;
use regex::Regex;
use std::error::Error;
use std::fmt::Display;
use std::process::Command;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
/// A little utility to connect to my development ec2 instance
struct CliArgs {
    /// Public ip for the ec2 instance
    ip: String,
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

#[derive(Debug)]
struct GetEc2IpError<'a>(&'a str);

impl<'a> Display for GetEc2IpError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Could not get ec2 ip: {}", self.0)
    }
}

impl<'a> Error for GetEc2IpError<'a> {}

/* Argument types */

struct PortPairs<'a> {
    local: &'a str,
    remote: &'a str,
}

type IPv4 = [i32; 4];

/* Steps */

fn parse_ip(raw: &str) -> Result<IPv4, ParseIpError> {
    let re = Regex::new(r"^(\d+)\.(\d+)\.(\d+)\.(\d+)$").unwrap();

    if !re.is_match(raw) {
        return Err(ParseIpError(raw));
    }

    let caps = re.captures(raw).unwrap();

    Ok([
        caps[1].parse::<i32>().unwrap(),
        caps[2].parse::<i32>().unwrap(),
        caps[3].parse::<i32>().unwrap(),
        caps[4].parse::<i32>().unwrap(),
    ])
}

fn get_port_pairs<'a>(args: &Vec<&'a str>) -> Result<Vec<PortPairs<'a>>, ParsePortPairError<'a>> {
    let re = Regex::new(r"^(\d+):(\d+)$").unwrap();
    let mut result = Vec::<PortPairs>::new();
    result.reserve(4);

    for port in args.iter() {
        if !re.is_match(port) {
            return Err(ParsePortPairError(port));
        }
        let caps = re.captures(port).unwrap();
        result.push(PortPairs {
            local: caps.get(1).unwrap().as_str(),
            remote: caps.get(2).unwrap().as_str(),
        })
    }

    Ok(result)
}

fn get_command(ip: &IPv4, port_pairs: &Vec<PortPairs>) -> Command {
    let ec2_ip = get_ec2_ip(ip).unwrap();
    let mut command = Command::new("ssh");
    for pair in port_pairs.iter() {
        command.arg("-L");
        command.arg(format!("{}:{}:{}", pair.local, ec2_ip, pair.remote));
    }
    command.arg(format!("ubuntu@{}", ec2_ip));
    command
}

/* Utilities */

fn get_ec2_ip(ip: &IPv4) -> Result<String, GetEc2IpError> {
    let one = ip
        .get(0)
        .ok_or(GetEc2IpError("could not get index 0"))?
        .to_string();
    let two = ip
        .get(1)
        .ok_or(GetEc2IpError("could not get index 1"))?
        .to_string();
    let three = ip
        .get(2)
        .ok_or(GetEc2IpError("could not get index 2"))?
        .to_string();
    let four = ip
        .get(3)
        .ok_or(GetEc2IpError("could not get index 4"))?
        .to_string();

    Ok(format!(
        "ec2-{}-{}-{}-{}.ap-southeast-2.compute.amazonaws.com",
        one, two, three, four
    ))
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
        let result = result.unwrap();
        assert_eq!(result[0], 192);
        assert_eq!(result[1], 168);
        assert_eq!(result[2], 73);
        assert_eq!(result[3], 20);
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

    #[test]
    fn should_get_correct_command() {
        let port1 = PortPairs {
            local: "3000",
            remote: "5000",
        };

        let port2 = PortPairs {
            local: "5000",
            remote: "8000",
        };
        let ip: IPv4 = [192, 168, 0, 1];
        let command = get_command(&ip, &vec![port1, port2]);
        let mut arguments = command.get_args();
        assert_eq!("-L", arguments.next().unwrap().to_str().unwrap());
        assert_eq!(
            format!("3000:{}:5000", get_ec2_ip(&ip).unwrap()),
            arguments.next().unwrap().to_str().unwrap()
        );
        assert_eq!("-L", arguments.next().unwrap().to_str().unwrap());
        assert_eq!(
            format!("5000:{}:8000", get_ec2_ip(&ip).unwrap()),
            arguments.next().unwrap().to_str().unwrap()
        );

        assert_eq!(
            format!("ubuntu@{}", get_ec2_ip(&ip).unwrap()),
            arguments.next().unwrap().to_str().unwrap()
        );
    }
}

fn main() {
    let args = CliArgs::parse();

    /* Try to parse the ip */
    let ip = parse_ip(&args.ip);
    if let Err(err) = ip {
        eprintln!("{}", err);
        return;
    }
    let ip = ip.unwrap();
    let mut port_pairs = Vec::<PortPairs>::new();
    if let Some(ports) = args.port.as_ref() {
        let formated_pairs: Vec<&str> = ports.iter().map(|e| e.as_str()).collect();
        let result = get_port_pairs(&formated_pairs);
        if let Err(err) = result {
            eprintln!("{}", err);
            return;
        }
        port_pairs = result.unwrap();
    }
    let result = get_command(&ip, &port_pairs).spawn();
    if let Ok(mut child) = result {
        child.wait().unwrap();
    } else {
        eprintln!("{}", result.unwrap_err());
    }
}
