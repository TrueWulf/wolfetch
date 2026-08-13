#!/usr/bin/env bash
set -euo pipefail

bin="${1:?path to wolfetch binary required}"

version="$($bin --version)"
[[ "$version" == "wolfetch 0.5.4" ]]

help="$($bin --help)"
grep -F -- '--minimal' <<<"$help" >/dev/null
grep -F -- '--json' <<<"$help" >/dev/null
grep -F -- '--gpu-usage' <<<"$help" >/dev/null
gpu_usage="$($bin --gpu-usage --plain --no-logo)"
grep -E '^GPU +: .+ \([^)]*\)$' <<<"$gpu_usage" >/dev/null

minimal="$($bin --minimal --plain)"
[[ "$minimal" != *'CPU :'* ]]
[[ "$minimal" != *'GPU :'* ]]
[[ "$minimal" != *$'\e['* ]]

full="$($bin --plain --no-logo)"
grep -E '^CPU +: .+ \([^)]*\)$' <<<"$full" >/dev/null
grep -E '^GPU +: .+ \([^)]*\)$' <<<"$full" >/dev/null

json="$($bin --json)"
python -c 'import json, sys; data = json.loads(sys.stdin.read()); assert data["Distro"]; assert data["WM"]; assert "startup_ms" in data' <<<"$json"
