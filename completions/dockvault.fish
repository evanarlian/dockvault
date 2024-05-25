complete -c dockvault -f

set -l __commands list delete use shell help
set -l __shells fish

complete -c dockvault -n "not __fish_seen_subcommand_from $__commands" -a "$__commands"
complete -c dockvault -n "__fish_seen_subcommand_from shell" -n "not __fish_seen_subcommand_from $__shells" -a "$__shells"
complete -c dockvault -n "__fish_seen_subcommand_from use" -n "not __fish_seen_subcommand_from (dockvault completion)" -a "(dockvault completion)"

set -e __commands
set -e __shells
