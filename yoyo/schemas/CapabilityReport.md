# CapabilityReport (one-pager)

> **Status**: Interface schema for Part E.6.1 PROBE. Implementations MAY stub host probes returning `unknown`. Stub reports MUST NOT green **M-posture** / Plan-derived claims.

## Purpose

Structured evidence input to PPMPR Plan / §E.19 P-PLAN. Not a trust root.

## Fields (MUST names)

| Field | Type / enum | Notes |
|-------|-------------|-------|
| `has_os` | bool / `unknown` | Process + syscall/IAT |
| `has_fs` | bool / `unknown` | Open/read/write |
| `has_alloc` | bool / `unknown` | Dynamic alloc permitted |
| `ram_bytes` | u64 / `unknown` | Usable RAM ceiling |
| `isa_family` | `x86_64` \| `aarch64` \| `riscv64` \| `other` \| `unknown` | |
| `trust_posture` | `hosted` \| `adversarial-suspected` \| `airgap` \| `rom` | |
| `power_class` | `stable` \| `intermittent` \| `unknown` | |
| `seu_class` | `none` \| `expected` \| `unknown` | ROADMAP consumers |
| `power_source` | `ac` \| `battery` \| `bus` \| `unknown` | §E.19 |
| `battery_pct` | 0–100 / `unknown` | |
| `thermal_class` | `cool` \| `warm` \| `hot` \| `critical` \| `unknown` | |
| `power_budget_mw` | number / `unknown` | |
| `latency_slo_us` | number / `unknown` | |
| `throughput_demand` | `low` \| `med` \| `high` \| `unknown` | |
| `workload_class` | `idle` \| `interactive` \| `batch` \| `realtime` \| `unknown` | |
| `e_class` | `E0`…`E10` \| `Eu` | Part E.4 |
| `policy_posture` | `energy-extreme` \| `perf-extreme` \| `blend:<w>` \| `auto` \| `unset` | |

## Rules

1. Missing fields MUST be `unknown` (never invented).
2. Physical fields with advertised physical meaning need Part Gnd cites when claimed.
3. `maps_to: CapabilityReport.<field>` in `*.fdn` MUST use names from this table.

## Toy JSON (NON-CONFORMING sample)

```json
{
  "has_os": true,
  "has_fs": true,
  "has_alloc": true,
  "ram_bytes": "unknown",
  "isa_family": "x86_64",
  "trust_posture": "hosted",
  "power_class": "stable",
  "seu_class": "none",
  "power_source": "battery",
  "battery_pct": 15,
  "thermal_class": "warm",
  "power_budget_mw": "unknown",
  "latency_slo_us": "unknown",
  "throughput_demand": "low",
  "workload_class": "interactive",
  "e_class": "E0",
  "policy_posture": "auto"
}
```
