_wolfetch_complete() {
  local options='--plain --json --no-logo --fast --theme --config --version --help'
  COMPREPLY=( $(compgen -W "$options" -- "${COMP_WORDS[COMP_CWORD]}") )
}
complete -F _wolfetch_complete wolfetch wfetch
