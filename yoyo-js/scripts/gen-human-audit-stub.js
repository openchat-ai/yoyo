'use strict';
/**
 * AUTO-HUMANAUDIT: regenerate human-audit pack for place_stub TCB.
 *   node yoyo-js/scripts/gen-human-audit-stub.js
 */
const fs = require('fs');
const path = require('path');
const crypto = require('crypto');

const root = path.join(__dirname, '..', '..');
const stubPath = path.join(root, 'scripts', 'fixtures', 'yoyo_place_stub.exe');
const outDir = path.join(root, 'scripts', 'fixtures', 'audit-stub');
const stubPin = path.join(root, 'scripts', 'fixtures', 'yoyo_place_stub.pin');
const slicePin = path.join(root, 'scripts', 'fixtures', 'yoyo_stub_slices.pin');
const a1Pin = path.join(root, 'scripts', 'fixtures', 'yoyo_a1_imports.pin');

function sha256(buf) {
  return crypto.createHash('sha256').update(buf).digest('hex');
}
function readPin(p) {
  const o = Object.create(null);
  for (const line of fs.readFileSync(p, 'utf8').split(/\r?\n/)) {
    if (!line || line.startsWith('#')) continue;
    const i = line.indexOf('=');
    if (i < 0) continue;
    o[line.slice(0, i).trim()] = line.slice(i + 1).trim();
  }
  return o;
}
function hexdump(buf, base, width = 16) {
  const lines = [];
  for (let i = 0; i < buf.length; i += width) {
    const chunk = buf.slice(i, i + width);
    const hex = [...chunk].map((b) => b.toString(16).padStart(2, '0')).join(' ');
    const asc = [...chunk]
      .map((b) => (b >= 0x20 && b < 0x7f ? String.fromCharCode(b) : '.'))
      .join('');
    lines.push(`${(base + i).toString(16).padStart(8, '0')}  ${hex.padEnd(width * 3 - 1)}  ${asc}`);
  }
  return lines.join('\n');
}

function peLayout(buf) {
  const lfanew = buf.readUInt32LE(0x3c);
  const opt = lfanew + 24;
  const magic = buf.readUInt16LE(opt);
  const soh = buf.readUInt16LE(lfanew + 20);
  const sec = lfanew + 24 + soh;
  const nsec = buf.readUInt16LE(lfanew + 6);
  const entry = buf.readUInt32LE(opt + 16);
  const sections = [];
  for (let i = 0; i < nsec; i++) {
    const s = sec + i * 40;
    sections.push({
      name: buf.slice(s, s + 8).toString('ascii').replace(/\0/g, ''),
      vsize: buf.readUInt32LE(s + 8),
      rva: buf.readUInt32LE(s + 12),
      rawsz: buf.readUInt32LE(s + 16),
      raw: buf.readUInt32LE(s + 20),
    });
  }
  const data = sections.find((x) => x.name.startsWith('.data'));
  const text = sections.find((x) => x.name.startsWith('.text'));
  return { lfanew, magic, entry, sections, data, text };
}

function kernel32Funcs(buf, dataRaw) {
  // Same IAT layout as place stub: names via ILT in .data
  const lfanew = buf.readUInt32LE(0x3c);
  const opt = lfanew + 24;
  const magic = buf.readUInt16LE(opt);
  const dd = magic === 0x20b ? opt + 112 : opt + 96;
  const impRva = buf.readUInt32LE(dd + 8);
  const soh = buf.readUInt16LE(lfanew + 20);
  const sec = lfanew + 24 + soh;
  const nsec = buf.readUInt16LE(lfanew + 6);
  function rvaToRaw(rva) {
    for (let i = 0; i < nsec; i++) {
      const s = sec + i * 40;
      const vr = buf.readUInt32LE(s + 12);
      const vs = buf.readUInt32LE(s + 8);
      const raw = buf.readUInt32LE(s + 20);
      const rs = buf.readUInt32LE(s + 16);
      if (rva >= vr && rva < vr + Math.max(vs, rs)) return raw + (rva - vr);
    }
    return -1;
  }
  const out = [];
  let desc = rvaToRaw(impRva);
  for (;;) {
    const nameRva = buf.readUInt32LE(desc + 12);
    const iatRva = buf.readUInt32LE(desc + 16);
    const iltRva = buf.readUInt32LE(desc);
    if (!nameRva && !iltRva && !iatRva) break;
    let dll = '';
    const nr = rvaToRaw(nameRva);
    for (let i = nr; buf[i]; i++) dll += String.fromCharCode(buf[i]);
    if (dll.toLowerCase() === 'kernel32.dll') {
      let th = rvaToRaw(iltRva || iatRva);
      let slot = 0;
      for (;;) {
        const entry = buf.readBigUInt64LE(th);
        if (entry === 0n) break;
        if ((entry & 0x8000000000000000n) === 0n) {
          const hn = rvaToRaw(Number(entry & 0xffffffffn));
          let nm = '';
          for (let i = hn + 2; buf[i]; i++) nm += String.fromCharCode(buf[i]);
          out.push({ slot, name: nm, iat_off: dataRaw + slot * 8 });
        }
        th += 8;
        slot++;
      }
    }
    desc += 20;
  }
  return out;
}

function main() {
  const buf = fs.readFileSync(stubPath);
  const pin = readPin(stubPin);
  const slices = readPin(slicePin);
  const a1 = readPin(a1Pin);
  const fullSha = sha256(buf);
  if (pin.sha256 && pin.sha256.toLowerCase() !== fullSha) {
    console.log('HUMANAUDIT status=RED reason=stub_pin_DIFF');
    process.exit(1);
  }
  const L = peLayout(buf);
  const hdr = buf.slice(0, 0x200);
  const iat = buf.slice(L.data.raw, L.data.raw + 0x100);
  if (sha256(hdr) !== (slices.hdr_sha256 || '').toLowerCase()) {
    console.log('HUMANAUDIT status=RED reason=hdr_slice_DIFF');
    process.exit(1);
  }
  if (sha256(iat) !== (slices.iat_sha256 || '').toLowerCase()) {
    console.log('HUMANAUDIT status=RED reason=iat_slice_DIFF');
    process.exit(1);
  }
  const imports = kernel32Funcs(buf, L.data.raw);
  const expect = (a1.funcs || '').split(',').map((s) => s.trim());
  if (imports.map((x) => x.name).join(',') !== expect.join(',')) {
    console.log('HUMANAUDIT status=RED reason=import_DIFF');
    process.exit(1);
  }

  // .text file bytes: PE startup (13) + code_place; raw section may be padded
  const textFile = buf.slice(L.text.raw, L.text.raw + L.text.rawsz);
  const codePlace = parseInt(pin.code_trunc || '5876', 10);
  // startup is prepended by linker inside section; extract full raw for offline disasm
  fs.mkdirSync(outDir, { recursive: true });
  // Do not duplicate stub.exe in git — SoT remains scripts/fixtures/yoyo_place_stub.exe
  fs.writeFileSync(path.join(outDir, 'hdr_0x200.bin'), hdr);
  fs.writeFileSync(path.join(outDir, 'iat_0x100.bin'), iat);
  fs.writeFileSync(path.join(outDir, 'text_raw.bin'), textFile);
  fs.writeFileSync(
    path.join(outDir, 'MANIFEST.sha256'),
    [
      `${fullSha}  ../yoyo_place_stub.exe`,
      `${sha256(hdr)}  hdr_0x200.bin`,
      `${sha256(iat)}  iat_0x100.bin`,
      `${sha256(textFile)}  text_raw.bin`,
      '',
    ].join('\n'),
  );

  const importTable = imports
    .map((x) => `| ${x.slot} | \`${x.name}\` | file+0x${x.iat_off.toString(16)} |`)
    .join('\n');

  const md = `# Stub 人眼审核包（冷启 TCB）

> 自动生成：\`node yoyo-js/scripts/gen-human-audit-stub.js\`  
> **审的是公理 \`place_stub\`**，不是证明无后门。≠ Thompson。

## 0. 你要花多少眼睛

| 对象 | 规模 | 人眼现实 |
| --- | --- | --- |
| 本包 stub | **${buf.length} B** PE；\`.text\` raw **${L.text.rawsz} B**（含 pad）；逻辑 place ≈ **${codePlace} B** | 结构+导入可一坐；全文反汇编需外部工具+数小时～数日 |
| code/data bins | ~60KB+ | **不在本包**；人眼极致不覆盖内容体量 |

## 1. 对照钉子（先核对，再读字节）

| pin | 期望 |
| --- | --- |
| \`yoyo_place_stub.pin\` sha256 | \`${fullSha}\` |
| hdr[0x200] | \`${sha256(hdr)}\` |
| iat[.data+0x100] | \`${sha256(iat)}\` |
| A1 导入 | \`${expect.join(', ')}\` |
| entry RVA | **${L.entry}** (0x${L.entry.toString(16)}) |
| code_trunc | **${codePlace}** |

## 2. 节表

| 节 | RVA | vsize | raw | rawsz |
| --- | ---: | ---: | ---: | ---: |
${L.sections.map((s) => `| ${s.name} | 0x${s.rva.toString(16)} | ${s.vsize} | 0x${s.raw.toString(16)} | ${s.rawsz} |`).join('\n')}

## 3. 导入（应仅 kernel32 下列 6 个）

| slot | name | IAT file off |
| --- | --- | --- |
${importTable}

## 4. 头窗 hex（0x000–0x1FF）

\`\`\`
${hexdump(hdr, 0)}
\`\`\`

## 5. IAT 窗 hex（.data 前 0x100）

\`\`\`
${hexdump(iat, L.data.raw)}
\`\`\`

## 6. 离线反汇编入口

本目录文件：

- 完整 PE SoT：\`../yoyo_place_stub.exe\`（pin）
- \`text_raw.bin\` — \`.text\` 文件内容（送 IDA/Ghidra/llvm-objdump）
- \`hdr_0x200.bin\` / \`iat_0x100.bin\` — 与切片 pin 对应

建议命令（若本机有工具）：

\`\`\`text
llvm-objdump -d -M intel ../yoyo_place_stub.exe
\`\`\`

人审焦点：**入口 → try_place_bins 读 code.bin/data.bin → 写 PE → 写 output.exe**；不应出现 LoadLibrary/额外 DLL。

## 7. 人审勾选清单

- [ ] \`Get-FileHash ../yoyo_place_stub.exe\` ≡ \`yoyo_place_stub.pin\`
- [ ] 导入恰好 6 个，无 \`LoadLibraryA\` / 无其它 DLL
- [ ] entry RVA = 0x${L.entry.toString(16)}
- [ ] 头窗 / IAT 窗 sha ≡ \`yoyo_stub_slices.pin\`
- [ ] 反汇编未见明文网络 API / 未见写注册表等越权面（按你的威胁模型）
- [ ] 确认 place 只依赖同目录 \`code.bin\`/\`data.bin\` 文件名契约

## 8. 诚实边界

- 读完 stub ≠ bins 干净 ≠ OS 干净  
- 本包把**可人审的冷启面**推到工程极致；内容 pin 仍要机器检测（TRUSTLIM3/4）

---

*gen-human-audit-stub · AUTO-HUMANAUDIT*
`;

  fs.writeFileSync(path.join(outDir, 'AUDIT-STUB.md'), md);
  console.log(`HUMANAUDIT out=${outDir}`);
  console.log(`HUMANAUDIT stub_sha=${fullSha.slice(0, 16)} text_raw=${textFile.length}`);
  console.log('HUMANAUDIT status=GREEN');
  console.log('HUMANAUDIT doc=AUTO-HUMANAUDIT.md');
}

main();
