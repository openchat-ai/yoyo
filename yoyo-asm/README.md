# yoyo-asm — 3rd DDC peer (ground truth)

Pure x64 assembly implementation (~500 lines target). See PROMPT-v3 Appendix B.

```bash
# Linux
make
./yoyo-asm

# Windows (Phase 4d): nasm -f win64 + lld-link
```

Must not reuse `yoyo-js/src/platform/*` — independently re-implemented.
