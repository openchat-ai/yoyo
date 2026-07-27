/**
 * body-extend-016 rust wiring fix — self_test.rs + main.rs for H_54..H_61
 */
import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';

const ROOT = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '../..');

function writeUtf8(p, s) {
  fs.writeFileSync(p, s, 'utf8');
}

// --- self_test.rs ---
{
  const p = path.join(ROOT, 'yoyo-rust/verifier/src/self_test.rs');
  let c = fs.readFileSync(p, 'utf8');

  // Fix run_self_test calls after set_large
  const callRe =
    /set_large_slot_check\(\)?;[\s\S]*?Ok\(\(\)\)\n\}/;
  const callNew = `set_large_slot_check()?;
    orv_h52_slot_check()?;
    subv_h52_slot_check()?;
    imul_swap_slot_check()?;
    imul_h52_slot_check()?;
    cmp_swap_slot_check()?;
    get_h52_slot_check()?;
    set_deadbeef_slot_check()?;
    ldb_dst51_slot_check()?;
    Ok(())
}`;
  if (!callRe.test(c)) throw new Error('self_test run_self_test block not found');
  c = c.replace(callRe, callNew);

  // Replace from first post-set_large batch-016 fn through cfg(test)
  const fnStartCandidates = [
    'fn set_deadbeef_slot_check()',
    'fn orv_h52_slot_check()',
    '/// body-extend-016 / parallel-batch-10 H_54',
  ];
  let fnStart = -1;
  for (const cand of fnStartCandidates) {
    const i = c.indexOf(cand);
    if (i >= 0) {
      // prefer the earliest occurrence after set_large_slot_check function body
      const afterLarge = c.indexOf('fn set_large_slot_check()');
      const afterLargeEnd = c.indexOf('\n}\n', c.indexOf('SET-LARGE slot stub', afterLarge));
      if (i > afterLargeEnd) {
        fnStart = Math.min(fnStart < 0 ? i : fnStart, i);
      }
    }
  }
  // More robust: find end of set_large_slot_check Ok(())\n}\n then next fn
  const largeOk = c.indexOf('"SET-LARGE slot stub mismatch');
  if (largeOk < 0) throw new Error('SET-LARGE marker missing');
  const largeEnd = c.indexOf('\n}\n\n', largeOk);
  if (largeEnd < 0) throw new Error('SET-LARGE end missing');
  fnStart = largeEnd + 4; // after \n}\n\n

  const cfgIdx = c.indexOf('#[cfg(test)]', fnStart);
  if (cfgIdx < 0) throw new Error('cfg(test) missing');

  const slotFns = `/// body-extend-016 / parallel-batch-10 H_54: 0x69 ORV dst=0x52 src=0x51.
/// Pin: 498b8790020000498b8f880200004809c849898790020000c3 (25B).
fn orv_h52_slot_check() -> IsaResult<()> {
    const DST: u16 = 0x52;
    const SRC: u16 = 0x51;
    const HH: u8 = 0x3C;
    let tir = vec![
        lower_op_checked(0x40, &[HH as u64], 1)?,
        lower_op_checked(0x69, &[DST as u64, SRC as u64], 2)?,
        lower_op_checked(0xFF, &[], 3)?,
    ];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_orv(DST, SRC)?;
    want.extend(assembler::ret());
    if out.code != want || out.code.len() != 25 {
        return Err(IsaError::ParseError {
            line: 0,
            msg: format!("ORV-H52 mismatch: got {:02X?} want {:02X?}", out.code, want),
        });
    }
    Ok(())
}

/// body-extend-016 / parallel-batch-10 H_55: 0x6A SUBV dst=0x52 src=0x51.
/// Pin: 498b8790020000498b8f880200004829c849898790020000c3 (25B).
fn subv_h52_slot_check() -> IsaResult<()> {
    const DST: u16 = 0x52;
    const SRC: u16 = 0x51;
    const HH: u8 = 0x3D;
    let tir = vec![
        lower_op_checked(0x40, &[HH as u64], 1)?,
        lower_op_checked(0x6A, &[DST as u64, SRC as u64], 2)?,
        lower_op_checked(0xFF, &[], 3)?,
    ];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_subv(DST, SRC)?;
    want.extend(assembler::ret());
    if out.code != want || out.code.len() != 25 {
        return Err(IsaError::ParseError {
            line: 0,
            msg: format!("SUBV-H52 mismatch: got {:02X?} want {:02X?}", out.code, want),
        });
    }
    Ok(())
}

/// body-extend-016 / parallel-batch-10 H_56: 0x63 IMUL dst=0x51 src=0x50.
/// Pin: 498b8788020000498b8f80020000480fafc149898788020000c3 (26B).
fn imul_swap_slot_check() -> IsaResult<()> {
    const DST: u16 = 0x51;
    const SRC: u16 = 0x50;
    const HH: u8 = 0x3E;
    let tir = vec![
        lower_op_checked(0x40, &[HH as u64], 1)?,
        lower_op_checked(0x63, &[DST as u64, SRC as u64], 2)?,
        lower_op_checked(0xFF, &[], 3)?,
    ];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_imul(DST, SRC)?;
    want.extend(assembler::ret());
    if out.code != want || out.code.len() != 26 {
        return Err(IsaError::ParseError {
            line: 0,
            msg: format!("IMUL-SWAP mismatch: got {:02X?} want {:02X?}", out.code, want),
        });
    }
    Ok(())
}

/// body-extend-016 / parallel-batch-10 H_57: 0x63 IMUL dst=0x52 src=0x51.
/// Pin: 498b8790020000498b8f88020000480fafc149898790020000c3 (26B).
fn imul_h52_slot_check() -> IsaResult<()> {
    const DST: u16 = 0x52;
    const SRC: u16 = 0x51;
    const HH: u8 = 0x3F;
    let tir = vec![
        lower_op_checked(0x40, &[HH as u64], 1)?,
        lower_op_checked(0x63, &[DST as u64, SRC as u64], 2)?,
        lower_op_checked(0xFF, &[], 3)?,
    ];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_imul(DST, SRC)?;
    want.extend(assembler::ret());
    if out.code != want || out.code.len() != 26 {
        return Err(IsaError::ParseError {
            line: 0,
            msg: format!("IMUL-H52 mismatch: got {:02X?} want {:02X?}", out.code, want),
        });
    }
    Ok(())
}

/// body-extend-016 / parallel-batch-10 H_58: 0x65 CMP a=0x51 b=0x50.
/// Pin: 498b8788020000498b8f800200004839c8c3 (18B).
fn cmp_swap_slot_check() -> IsaResult<()> {
    const A: u16 = 0x51;
    const B: u16 = 0x50;
    const HH: u8 = 0x40;
    let tir = vec![
        lower_op_checked(0x40, &[HH as u64], 1)?,
        lower_op_checked(0x65, &[A as u64, B as u64], 2)?,
        lower_op_checked(0xFF, &[], 3)?,
    ];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_cmp(A, B)?;
    want.extend(assembler::ret());
    if out.code != want || out.code.len() != 18 {
        return Err(IsaError::ParseError {
            line: 0,
            msg: format!("CMP-SWAP mismatch: got {:02X?} want {:02X?}", out.code, want),
        });
    }
    Ok(())
}

/// body-extend-016 / parallel-batch-10 H_59: 0x60 GET dst=0x52 src=0x50.
/// Pin: 498b878002000049898790020000c3 (15B).
fn get_h52_slot_check() -> IsaResult<()> {
    const DST: u16 = 0x52;
    const SRC: u16 = 0x50;
    const HH: u8 = 0x41;
    let tir = vec![
        lower_op_checked(0x40, &[HH as u64], 1)?,
        lower_op_checked(0x60, &[DST as u64, SRC as u64], 2)?,
        lower_op_checked(0xFF, &[], 3)?,
    ];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_get(DST, SRC)?;
    want.extend(assembler::ret());
    if out.code != want || out.code.len() != 15 {
        return Err(IsaError::ParseError {
            line: 0,
            msg: format!("GET-H52 mismatch: got {:02X?} want {:02X?}", out.code, want),
        });
    }
    Ok(())
}

/// body-extend-016 / parallel-batch-10 H_60: 0x30 SET slot=0x51 imm=0xDEADBEEF.
/// Pin: 48b8efbeadde0000000049898788020000c3 (18B).
fn set_deadbeef_slot_check() -> IsaResult<()> {
    const SLOT: u16 = 0x51;
    const IMM: u64 = 0xDEADBEEF;
    const HH: u8 = 0x42;
    let tir = vec![
        lower_op_checked(0x40, &[HH as u64], 1)?,
        lower_op_checked(0x30, &[SLOT as u64, IMM], 2)?,
        lower_op_checked(0xFF, &[], 3)?,
    ];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_set(SLOT, IMM)?;
    want.extend(assembler::ret());
    if out.code != want || out.code.len() != 18 {
        return Err(IsaError::ParseError {
            line: 0,
            msg: format!("SET-DEADBEEF mismatch: got {:02X?} want {:02X?}", out.code, want),
        });
    }
    Ok(())
}

/// body-extend-016 / parallel-batch-10 H_61: 0x80 LDB dd=0x51 ss=0x60 oo=8.
/// Pin: 498b87000300004883c008480fb60049898788020000c3 (23B).
fn ldb_dst51_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x51;
    const SS: u16 = 0x60;
    const OO: u64 = 8;
    const HH: u8 = 0x43;
    let tir = vec![
        lower_op_checked(0x40, &[HH as u64], 1)?,
        lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?,
        lower_op_checked(0xFF, &[], 3)?,
    ];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 23] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x83, 0xc0, 0x08,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x88, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want {
        return Err(IsaError::ParseError {
            line: 0,
            msg: format!("LDB-DST51 mismatch: got {:02X?} want {:02X?}", out.code, want),
        });
    }
    Ok(())
}

`;
  c = c.slice(0, fnStart) + slotFns + c.slice(cfgIdx);
  writeUtf8(p, c);
  console.log('self_test.rs fixed');
}

// --- main.rs ---
{
  const p = path.join(ROOT, 'yoyo-rust/verifier/src/main.rs');
  let c = fs.readFileSync(p, 'utf8');
  // Repair corrupted em-dash mojibake if present
  c = c.replace(/\u20AC\u017D\?/g, '\u2014');
  c = c.replace(/鈥\?/g, '\u2014');

  // Fix match arms between set_large and jmp
  const s = c.indexOf('    match check_selfhost_min_set_large(&root) {');
  const e = c.indexOf('    match check_selfhost_min_jmp(&root) {');
  if (s < 0 || e < 0) throw new Error('main match markers missing');
  const dash = '\u2014';
  const arms = `    match check_selfhost_min_set_large(&root) {
        Ok(detail) => println!("G-SM-SET-LARGE PASS ${dash} {detail}"),
        Err(e) => {
            failed += 1;
            eprintln!("G-SM-SET-LARGE FAIL ${dash} {e}");
        }
    }
    total += 1;
    match check_selfhost_min_orv_h52(&root) {
        Ok(detail) => println!("G-SM-ORV-H52 PASS ${dash} {detail}"),
        Err(e) => {
            failed += 1;
            eprintln!("G-SM-ORV-H52 FAIL ${dash} {e}");
        }
    }
    total += 1;
    match check_selfhost_min_subv_h52(&root) {
        Ok(detail) => println!("G-SM-SUBV-H52 PASS ${dash} {detail}"),
        Err(e) => {
            failed += 1;
            eprintln!("G-SM-SUBV-H52 FAIL ${dash} {e}");
        }
    }
    total += 1;
    match check_selfhost_min_imul_swap(&root) {
        Ok(detail) => println!("G-SM-IMUL-SWAP PASS ${dash} {detail}"),
        Err(e) => {
            failed += 1;
            eprintln!("G-SM-IMUL-SWAP FAIL ${dash} {e}");
        }
    }
    total += 1;
    match check_selfhost_min_imul_h52(&root) {
        Ok(detail) => println!("G-SM-IMUL-H52 PASS ${dash} {detail}"),
        Err(e) => {
            failed += 1;
            eprintln!("G-SM-IMUL-H52 FAIL ${dash} {e}");
        }
    }
    total += 1;
    match check_selfhost_min_cmp_swap(&root) {
        Ok(detail) => println!("G-SM-CMP-SWAP PASS ${dash} {detail}"),
        Err(e) => {
            failed += 1;
            eprintln!("G-SM-CMP-SWAP FAIL ${dash} {e}");
        }
    }
    total += 1;
    match check_selfhost_min_get_h52(&root) {
        Ok(detail) => println!("G-SM-GET-H52 PASS ${dash} {detail}"),
        Err(e) => {
            failed += 1;
            eprintln!("G-SM-GET-H52 FAIL ${dash} {e}");
        }
    }
    total += 1;
    match check_selfhost_min_set_deadbeef(&root) {
        Ok(detail) => println!("G-SM-SET-DEADBEEF PASS ${dash} {detail}"),
        Err(e) => {
            failed += 1;
            eprintln!("G-SM-SET-DEADBEEF FAIL ${dash} {e}");
        }
    }
    total += 1;
    match check_selfhost_min_ldb_dst51(&root) {
        Ok(detail) => println!("G-SM-LDB-DST51 PASS ${dash} {detail}"),
        Err(e) => {
            failed += 1;
            eprintln!("G-SM-LDB-DST51 FAIL ${dash} {e}");
        }
    }
    total += 1;
`;
  c = c.slice(0, s) + arms + c.slice(e);

  // Replace check fns: from first body-extend-016 or set_deadbeef/orv after set_large through jmp doc
  const setLargeOk = c.indexOf('via opcode set-large+FF');
  if (setLargeOk < 0) throw new Error('set-large ok missing');
  const afterSetLargeFn = c.indexOf('\n}\n\n', setLargeOk);
  if (afterSetLargeFn < 0) throw new Error('set-large fn end missing');
  const fnInsert = afterSetLargeFn + 4;
  const jmpDoc = c.indexOf('/// W-SM control flow: Rust-only golden for H_19', fnInsert);
  if (jmpDoc < 0) throw new Error('jmp doc missing');

  const checkFns = fs.readFileSync(
    path.join(ROOT, 'docs/auxdocs/_body_extend_016_main_fns.rs.txt'),
    'utf8'
  );
  c = c.slice(0, fnInsert) + checkFns + '\n' + c.slice(jmpDoc);

  // Summary lines
  c = c.replace(
    /G-SM-SET-LARGE \+ G-SM-[^+]+ \+ G-SM-JMP \+ G-SM-CALL/,
    'G-SM-SET-LARGE + G-SM-ORV-H52 + G-SM-SUBV-H52 + G-SM-IMUL-SWAP + G-SM-IMUL-H52 + G-SM-CMP-SWAP + G-SM-GET-H52 + G-SM-SET-DEADBEEF + G-SM-LDB-DST51 + G-SM-JMP + G-SM-CALL'
  );
  c = c.replace(
    /G-SM-SET-LARGE, G-SM-[^,]+, G-SM-JMP, G-SM-CALL/,
    'G-SM-SET-LARGE, G-SM-ORV-H52, G-SM-SUBV-H52, G-SM-IMUL-SWAP, G-SM-IMUL-H52, G-SM-CMP-SWAP, G-SM-GET-H52, G-SM-SET-DEADBEEF, G-SM-LDB-DST51, G-SM-JMP, G-SM-CALL'
  );

  writeUtf8(p, c);
  console.log('main.rs fixed');
}
