#!/usr/bin/env bash
export LC_ALL=C
set -u

iterations="${1:-15}"
root="$(CDPATH= cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"
wolfetch_bin="${WOLFETCH_BIN:-$root/target/release/wolfetch}"

if [[ ! -x "$wolfetch_bin" ]]; then
  wolfetch_bin="$(command -v wolfetch || true)"
fi

tmp="$(mktemp)"
trap 'rm -f "$tmp"' EXIT

now_ns() {
  local seconds="${EPOCHREALTIME%%.*}"
  local fraction="${EPOCHREALTIME#*.}"
  fraction="${fraction}000000"
  now_ns_value=$((seconds * 1000000 + 10#${fraction:0:6}))
}

measure() {
  local name="$1"
  shift
  local values=()
  local start end elapsed i median min max

  if [[ "$name" == "wolfetch" && ! -x "$1" ]] ||
     [[ "$name" != "wolfetch" && ! -x "$(command -v "$1" 2>/dev/null || true)" ]]; then
    printf '%-10s unavailable\n' "$name"
    return 0
  fi

  "$@" >/dev/null 2>&1 || true
  for ((i = 0; i < iterations; i++)); do
    now_ns
    start="$now_ns_value"
    "$@" >/dev/null 2>&1 || true
    now_ns
    end="$now_ns_value"
    elapsed=$((end - start))
    values+=("$elapsed")
  done

  mapfile -t values < <(printf '%s\n' "${values[@]}" | sort -n)
  min="${values[0]}"
  max="${values[${#values[@]}-1]}"
  median="${values[$(((${#values[@]} - 1) / 2))]}"
  printf '%-10s median %d.%03d ms | min %d.%03d ms | max %d.%03d ms\n' \
    "$name" \
    "$((median / 1000))" "$((median % 1000))" \
    "$((min / 1000))" "$((min % 1000))" \
    "$((max / 1000))" "$((max % 1000))"
}

printf 'startup benchmark: %s iterations, output redirected\n' "$iterations"
measure wolfetch "$wolfetch_bin"
measure fastfetch fastfetch
measure macchina macchina
