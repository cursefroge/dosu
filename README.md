# dosu
This is NOT secure. Only use this if you're fine with the massive security risks.

## Why
Because I wanted to, and because I thought sudo was stupid.

## Installing
why?

### From Source
Clone and run `make install`:

`git clone https://github.com/cursefroge/dosu.git && cd dosu && make install`

### Binary

With `curl`:

`sudo curl -o /usr/local/bin/dosu https://github.com/cursefroge/dosu/releases/latest/download/dosu && chown root:root /usr/local/bin/dosu && chmod u+s /usr/local/bin/dosu`

## Usage
`dosu [flags] <command>`

### Flags
`--help, -h`: Print help

`--shell, -s`: Spawn a shell (hardcoded to fish for now) instead of running a command

`--user, -u <USER>`: Run the command as the specified user

`--clear-env, -c`: Don't retain environment variables

`--login, -l`: Emulate a login shell

`--version, -V`: Print version
