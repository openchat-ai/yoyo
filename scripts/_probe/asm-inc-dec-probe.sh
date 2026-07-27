#!/usr/bin/env bash
# Thin wrapper — prior INC/DEC probe path kept working.
# Real work: asm-primitives-probe.sh (INC/DEC/SET+GET/ADDV).
set -euo pipefail
exec "$(cd "$(dirname "$0")" && pwd)/asm-primitives-probe.sh" "$@"
