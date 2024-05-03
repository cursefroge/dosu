use clap::{Parser};
use std::os::unix::fs::PermissionsExt;
use libc;

/// dosu
#[derive(Parser, Debug)]
#[clap(version = "1.0", author = "cursefroge <cursefroge@curseforged.dev>", trailing_var_arg = true)]
struct Opts {
    /// The command to run
    #[clap(required_unless_present = "shell", default_value="")]
    command: Vec<String>,

    // shell (-s) option
    #[clap(short, long)]
    shell: bool,

    // user (-u) option
    #[clap(short, long, default_value="root")]
    user: Option<String>,

    #[clap(short, long)]
    clear_env: bool,

    #[clap(short, long, requires="shell")]
    login: bool,

}

fn main() {
    let opts: Opts = Opts::parse();

    let (arg_command, args) = opts.command.split_at(1);
    // command will be everything after opts until end of command
    let shell = opts.shell;
    let user = opts.user;
    let clear_env = opts.clear_env;
    let login = opts.login;
    // clear the env until the end of the process
    // make list of env vars
    // if shell, then command should be /bin/fish
    let command = if shell {
        if login {
            vec!["/bin/fish".to_string(), "-l".to_string()]
        } else {
            vec!["/bin/fish".to_string()]
        }
    } else {
        arg_command.to_vec()
    };
    // user: change username to uid
    let user = user.unwrap();
    let c_user = std::ffi::CString::new(user).unwrap();
    let passwd = unsafe { libc::getpwnam(c_user.as_ptr()) };
    if passwd.is_null() {
        panic!("User does not exist.");
    }
    let uid = unsafe { (*passwd).pw_uid };
    // check setuid bit on current process
    let path = std::env::current_exe().unwrap();
    let metadata = std::fs::metadata(&path).unwrap();
    let setuid = metadata.permissions().mode() & 0o4000 != 0;
    if setuid {
        let env_vars = std::env::vars().collect::<Vec<(String, String)>>();
        // clear the env
        if clear_env {
            for (key, _) in &env_vars {
                std::env::remove_var(&key);
            }
            // set some env vars like TERM, etc
            std::env::set_var("TERM", "xterm-256color");
            std::env::set_var("SHELL", "/bin/fish");
        }
        // setuid to root
        unsafe {
            libc::setuid(uid);
        };
        // run the command
        std::process::Command::new(&command[0])
            .args(args)
            .status()
            .unwrap();
        // restore env
        if clear_env {
            for (key, value) in env_vars {
                std::env::set_var(&key, &value);
            }
        }
    } else {
        panic!("setuid bit is not set on {}", path.display());
    }
}