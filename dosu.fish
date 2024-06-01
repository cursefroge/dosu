# Completion for dosu https://gitlab.com/cursefroge/dosu
# based on the doas completions
#

function __fish_dosu_print_remaining_args
    set -l tokens (commandline -opc) (commandline -ct)
    set -e tokens[1]
    set -l opts u= C= h V L s
    argparse -s $opts -- $tokens 2>/dev/null
    # The remaining argv is the subcommand with all its options, which is what
    # we want.
    if test -n "$argv"
        and not string match -qr '^-' $argv[1]
        string join0 -- $argv
        return 0
    else
        return 1
    end
end

function __fish_complete_dosu_subcommand
    set -l args (__fish_dosu_print_remaining_args | string split0)
    set -lx -a PATH /usr/local/sbin /sbin /usr/sbin
    __fish_complete_subcommand --commandline $args
end

complete -c dosu -n "not __fish_dosu_print_remaining_args" -s h -d "Print help"
complete -c dosu -n "not __fish_dosu_print_remaining_args" -s V -d "Print version"
complete -c dosu -n "not __fish_dosu_print_remaining_args" -s l -d "Imitate login shell"
complete -c dosu -n "not __fish_dosu_print_remaining_args" -s c -d "Clear the environment variables of the elevated session."
complete -c dosu -n "not __fish_dosu_print_remaining_args" -s s -d "Execute the shell from SHELL or /etc/passwd"
complete -c dosu -n "not __fish_dosu_print_remaining_args" -s u -a "(__fish_complete_users)" -x -d "Execute the command as user. The default is root."

# Complete the command we are executing under dosu
complete -c dosu -x -a "(__fish_complete_dosu_subcommand)"
