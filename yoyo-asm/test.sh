#!/bin/bash
# Thin wrapper → scripts/_probe/asm-inc-dec-probe.sh
# scope=primitive-probe (NOT asm peer / NOT C-ddc)
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
exec bash "$ROOT/scripts/_probe/asm-inc-dec-probe.sh"
