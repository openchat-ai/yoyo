# plan.battery-15
# Toy Plan-first deduction sample (Part Deduce v0.1). Not a theorem proof.

claim_level: planned

inputs:
  foundation_ids: [fdn.battery-thresholds@1.0.0]
  probe_id: probe-stub-001
  posture_id: posture.blend:0.5
  theory_pin: posture-score-v0

steps:
  1. READ_FACT battery_low_pct -> 20
  2. READ_PROBE battery_pct -> 15
  3. COMPARE 15 < 20 -> true
  4. SELECT prefer_posture_when_low -> posture.energy-extreme
  5. EMIT target_posture=posture.energy-extreme
  6. EMIT posture_weight=0.0
  7. EMIT mutation_class=SAFE
  8. EMIT switch_path=P-DEBOUNCE then P-HOT or P-COLD per E.19.4.2

conclusions:
  target_posture: posture.energy-extreme
  posture_weight: 0.0
  mutation_class: SAFE

commands:
  - yoyo link --posture=energy-extreme

abort_if:
  - thermal_class == critical AND target would be perf-extreme

# ReplayRecord: fill hashes before Relock when claim_level becomes pinned.
# replay:
#   plan_hash: <sha256>
#   emit_payload_hash: <sha256>
#   foundation_ids: [fdn.battery-thresholds@1.0.0]
