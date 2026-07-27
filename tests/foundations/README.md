# Foundations tests

Gate: `node scripts/check-foundations.mjs` (Part F / F-foundations).

## Checks

- [x] `fdn.battery-thresholds@1.0.0.fdn` parses required fields
- [x] Every `maps_to` resolves to a named CapabilityReport / CLI field / path
- [x] Missing field / bad k_ref / unknown maps_to ⇒ reject (fail closed)
- [x] Journal `foundation_pin=` + morph/plan `foundation_ids` required

```text
node scripts/check-foundations.mjs   # expect exit 0 when pins valid
```
