# YOYO ISA — 38 core instructions

Canonical table (also mirrored in `yoyo-rust/verifier/src/isa_table.txt`):

| Op | Mnemonic | Args |
|----|----------|------|
| 00 | NOP | — |
| 10 | DATA | str/raw |
| 12 | STR | string |
| 13 | RAW | bytes |
| 20 | ALLOC | slot size |
| 30 | SET | slot imm |
| 40 | HANDLER | hh |
| 41 | CALL | hh |
| 50 | LOAD_FILE | slot str_idx |
| 51 | WRITE_FILE | slot str_idx sz |
| 60–6A | GET/SUB/ADD/IMUL/MOVRR/CMP/INC/DEC/ADDV/ORV(bitwise OR)/SUBV | … |
| 70–7A | JMP/JE/JNE/JL/JGE/JLE/JG/JB/JAE/JBE/JA | hh |
| 80 | LDB | dd ss oo |
| 84–85 | MEMCPY_DATA / MEMCPY_STATE | … |
| A0 | RAW_BYTE | byte |
| A1 | RAW_BYTES | bytes… |
| FF | RET | — |

See PROMPT-v3.md Part 4.
