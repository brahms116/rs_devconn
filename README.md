# Utility to connect to dev server

This command line utility helps me to ssh into my ec2 instance by providing the public ip. I can also specify which ports I would like to tunnel with the `-p` option. This was built with practising Rust in mind; used the "clap" framework to parse the cli arguments

### Example

```bash

devcon 192.168.0.0 -p 3000:3000 -p 5000:8000

```
