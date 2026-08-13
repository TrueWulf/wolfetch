#!/usr/bin/env bash
set -euo pipefail
export LC_ALL=C

runs="${1:-30}"
warmup="${2:-4}"
root="$(CDPATH= cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"
wolfetch_bin="${WOLFETCH_BIN:-$root/target/release/wolfetch}"

if [[ ! -x "$wolfetch_bin" ]]; then
  printf 'Build wolfetch first or set WOLFETCH_BIN.\n' >&2
  exit 1
fi
if ! command -v hyperfine >/dev/null 2>&1; then
  printf 'Install hyperfine before running this benchmark.\n' >&2
  exit 1
fi

commands=(
  wolfetch "$wolfetch_bin"
  fastfetch fastfetch
  macchina macchina
  pfetch pfetch
  screenfetch screenfetch
)

if command -v hyfetch >/dev/null 2>&1 && [[ -f "$HOME/.config/hyfetch.json" ]]; then
  commands+=(hyfetch hyfetch)
fi

args=(--warmup "$warmup" --runs "$runs" --shell=none)
for ((i = 0; i < ${#commands[@]}; i += 2)); do
  args+=(-n "${commands[i]}" "${commands[i + 1]}")
done

printf 'Hyperfine startup benchmark: %s runs, %s warmups, shell disabled\n' \
  "$runs" "$warmup"
hyperfine "${args[@]}"
