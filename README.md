# dosu
This is NOT secure. Only use this if you're fine with the massive security risks.

## Why
Because I wanted to, and because I thought sudo was stupid.

## Installing
why?

Anyways, clone and run `make install`:

`git clone https://github.com/cursefroge/dosu.git && cd dosu && make install`

## Usage
`dosu [flags] <command>`

### Flags
`--help, -h`: Print help
`--shell, -s`: Spawn a shell (hardcoded to fish for now) instead of running a command
`--user, -u <USER>`: Run the command as the specified user
`--clear-env, -c`: Don't retain environment variables
`--login, -l`: Emulate a login shell
`--version, -V`: Print version
