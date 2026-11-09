'use strict';
/**
 * FX-2d-7c: mini-PE stubs — ISA emit1 + ISA/RT emit2 builders.
 * Plant bytes from yoyo-compiler-parse1-x64-plant (PROG/I/EMIT2/RT).
 */
const { u32le } = require('./yoyo-compiler-parse1-native-asm');
const { emit1Stores, emit2Stores } = require('./yoyo-compiler-parse1-x64-plant');

const MINI_PE_LEN = 0x1a00;
const TEXT_OFF = 0x40d;
const TEXT_ZERO_LEN = 0x400;

function emitMiniPePrologue(asm, meta, { entry, jeVa, saveRsi }) {
  asm.lab(entry);
  if (saveRsi) asm.raw(0x48, 0x89, 0x74, 0x24, 0x50);
  asm.raw(0x31, 0xc9);
  asm.raw(0xba, ...u32le(MINI_PE_LEN));
  asm.raw(0x41, 0xb8, 0x00, 0x30, 0x00, 0x00);
  asm.raw(0x41, 0xb9, 0x04, 0x00, 0x00, 0x00);
  asm.raw(0x41, 0xff, 0x97, 0x00, 0x00, 0x00, 0x00);
  asm.raw(0x48, 0x85, 0xc0);
  asm.jccSite(jeVa, 0x84);
  asm.raw(0x48, 0x89, 0xc3);
  asm.raw(0x49, 0x8d, 0xb7, ...u32le(meta.ret_pe_off));
  asm.raw(0x48, 0x89, 0xdf);
  asm.raw(0xb9, ...u32le(MINI_PE_LEN));
  asm.raw(0xf3, 0xa4);
  asm.raw(0x48, 0x8d, 0xbb, ...u32le(TEXT_OFF));
  asm.raw(0x31, 0xc0);
  asm.raw(0xb9, ...u32le(TEXT_ZERO_LEN));
  asm.raw(0xf3, 0xaa);
}

function emitMiniPeEpilogue(asm, jmpW) {
  asm.raw(0x48, 0x89, 0xda);
  asm.raw(0x41, 0xb8, ...u32le(MINI_PE_LEN));
  asm.jmpSite(jmpW);
}

function emitTextStores(asm, stores) {
  for (const s of stores) {
    if (s.op === 'abs64') {
      asm.raw(0x48, 0xb8, ...s.bytes);
      asm.raw(0x48, 0x89, 0x83, ...u32le(s.off));
    } else if (s.op === 'imm8') {
      asm.raw(0xc6, 0x83, ...u32le(s.off), s.bytes[0]);
    } else if (s.op === 'imm16') {
      asm.raw(0x66, 0xc7, 0x83, ...u32le(s.off), s.bytes[0], s.bytes[1]);
    } else if (s.op === 'imm32') {
      asm.raw(0xc7, 0x83, ...u32le(s.off), ...s.bytes);
    } else if (s.op === 'raw') {
      asm.raw(...s.bytes);
    } else throw new Error("unknown store op " + s.op);
  }
}

function emitLive2Nop(asm, meta) {
  asm.lab("live2_nop");
  asm.raw(0x4c, 0x89, 0x44, 0x24, 0x50);
  asm.raw(0x44, 0x89, 0x4c, 0x24, 0x58);
  asm.raw(0x31, 0xc9);
  asm.raw(0xba, ...u32le(MINI_PE_LEN));
  asm.raw(0x41, 0xb8, 0x00, 0x30, 0x00, 0x00);
  asm.raw(0x41, 0xb9, 0x04, 0x00, 0x00, 0x00);
  asm.raw(0x41, 0xff, 0x97, 0x00, 0x00, 0x00, 0x00);
  asm.raw(0x48, 0x85, 0xc0);
  asm.jccSite('je_live2_va', 0x84);
  asm.raw(0x48, 0x89, 0xc3);
  asm.raw(0x49, 0x8d, 0xb7, ...u32le(meta.ret_pe_off));
  asm.raw(0x48, 0x89, 0xdf);
  asm.raw(0xb9, ...u32le(MINI_PE_LEN));
  asm.raw(0xf3, 0xa4);
  asm.raw(0x48, 0x8d, 0xbb, ...u32le(TEXT_OFF));
  asm.raw(0x31, 0xc0);
  asm.raw(0xb9, ...u32le(TEXT_ZERO_LEN));
  asm.raw(0xf3, 0xaa);
  asm.raw(0x48, 0x8b, 0x74, 0x24, 0x50);
  asm.raw(0x48, 0x8d, 0xbb, ...u32le(TEXT_OFF));
  asm.raw(0x8b, 0x4c, 0x24, 0x58);
  asm.raw(0xf3, 0xa4);
  emitMiniPeEpilogue(asm, "jmp_live2_to_write");
}

/** emit1: ISA PROG → packed stores. emit2: skeleton + RT patch ops. */
const TEMPLATES = {
  EMIT1_NOP: emit1Stores("nop"),
  EMIT1_INC: emit1Stores("inc"),
  EMIT1_DEC: emit1Stores("dec"),
  EMIT1_GET: emit1Stores("get"),
  EMIT1_ADD: emit1Stores("add"),
  EMIT1_SUB: emit1Stores("sub"),
  EMIT1_SET: emit1Stores("set"),
  EMIT1_MOV: emit1Stores("mov"),
  EMIT1_CMP: emit1Stores("cmp"),
  EMIT1_SET_ADD: emit1Stores("set_add"),
  EMIT1_SET_GET: emit1Stores("set_get"),
  EMIT1_SET_ADD_SUB: emit1Stores("set_add_sub"),
  EMIT1_NOP_INC_DEC: emit1Stores("nop_inc_dec"),
  EMIT1_SA_SUB_NOP: emit1Stores("sa_sub_nop"),
  EMIT1_SET_MOV: emit1Stores("set_mov"),
  EMIT1_SET_CMP_MOV: emit1Stores("set_cmp_mov"),
  EMIT2_SET: emit2Stores("set"),
  EMIT2_ADD: emit2Stores("add"),
  EMIT2_SUB: emit2Stores("sub"),
  EMIT2_GETMOV: emit2Stores("getmov"),
  EMIT2_CMP: emit2Stores("cmp"),
  EMIT2_INC: emit2Stores("inc"),
  EMIT2_DEC: emit2Stores("dec"),
  EMIT2_SET_ADD: emit2Stores("set_add"),
  EMIT2_SET_GET: emit2Stores("set_get"),
  EMIT2_SCM: emit2Stores("scm"),
  EMIT2_SAS: emit2Stores("sas"),
  EMIT2_SASN: emit2Stores("sasn"),
  EMIT2_NID: emit2Stores("nid"),
};

const STUBS = [
  { entry: "emit1_nop", jeVa: "je_emit1_nop_va", jmpW: "jmp_emit1_nop_w", saveRsi: false, tpl: "EMIT1_NOP" },
  { entry: "emit1_inc", jeVa: "je_emit1_inc_va", jmpW: "jmp_emit1_inc_w", saveRsi: false, tpl: "EMIT1_INC" },
  { entry: "emit1_dec", jeVa: "je_emit1_dec_va", jmpW: "jmp_emit1_dec_w", saveRsi: false, tpl: "EMIT1_DEC" },
  { entry: "emit1_get", jeVa: "je_emit1_get_va", jmpW: "jmp_emit1_get_w", saveRsi: false, tpl: "EMIT1_GET" },
  { entry: "emit1_add", jeVa: "je_emit1_add_va", jmpW: "jmp_emit1_add_w", saveRsi: false, tpl: "EMIT1_ADD" },
  { entry: "emit1_sub", jeVa: "je_emit1_sub_va", jmpW: "jmp_emit1_sub_w", saveRsi: false, tpl: "EMIT1_SUB" },
  { entry: "emit1_set", jeVa: "je_emit1_set_va", jmpW: "jmp_emit1_set_w", saveRsi: false, tpl: "EMIT1_SET" },
  { entry: "emit1_mov", jeVa: "je_emit1_mov_va", jmpW: "jmp_emit1_mov_w", saveRsi: false, tpl: "EMIT1_MOV" },
  { entry: "emit1_cmp", jeVa: "je_emit1_cmp_va", jmpW: "jmp_emit1_cmp_w", saveRsi: false, tpl: "EMIT1_CMP" },
  { entry: "_emit1_set_add", jeVa: "je_emit1_sa_va", jmpW: "jmp_emit1_sa_w", saveRsi: false, tpl: "EMIT1_SET_ADD" },
  { entry: "_emit1_set_get", jeVa: "je_emit1_sg_va", jmpW: "jmp_emit1_sg_w", saveRsi: false, tpl: "EMIT1_SET_GET" },
  { entry: "_emit1_set_add_sub", jeVa: "je_emit1_sas_va", jmpW: "jmp_emit1_sas_w", saveRsi: false, tpl: "EMIT1_SET_ADD_SUB" },
  { entry: "_emit1_nop_inc_dec", jeVa: "je_emit1_nid_va", jmpW: "jmp_emit1_nid_w", saveRsi: false, tpl: "EMIT1_NOP_INC_DEC" },
  { entry: "_emit1_sa_sub_nop", jeVa: "je_emit1_san_va", jmpW: "jmp_emit1_san_w", saveRsi: false, tpl: "EMIT1_SA_SUB_NOP" },
  { entry: "emit1_set_mov", jeVa: "je_emit1_sm_va", jmpW: "jmp_emit1_sm_w", saveRsi: false, tpl: "EMIT1_SET_MOV" },
  { entry: "emit1_set_cmp_mov", jeVa: "je_emit1_scm_va", jmpW: "jmp_emit1_scm_w", saveRsi: false, tpl: "EMIT1_SET_CMP_MOV" },
  { entry: "_emit2_set", jeVa: "je_emit2_set_va", jmpW: "jmp_emit2_set_w", saveRsi: true, tpl: "EMIT2_SET" },
  { entry: "_emit2_add", jeVa: "je_emit2_add_va", jmpW: "jmp_emit2_add_w", saveRsi: true, tpl: "EMIT2_ADD" },
  { entry: "_emit2_sub", jeVa: "je_emit2_sub_va", jmpW: "jmp_emit2_sub_w", saveRsi: true, tpl: "EMIT2_SUB" },
  { entry: "_emit2_getmov", jeVa: "je_emit2_getmov_va", jmpW: "jmp_emit2_getmov_w", saveRsi: true, tpl: "EMIT2_GETMOV" },
  { entry: "_emit2_cmp", jeVa: "je_emit2_cmp_va", jmpW: "jmp_emit2_cmp_w", saveRsi: true, tpl: "EMIT2_CMP" },
  { entry: "_emit2_inc", jeVa: "je_emit2_inc_va", jmpW: "jmp_emit2_inc_w", saveRsi: true, tpl: "EMIT2_INC" },
  { entry: "_emit2_dec", jeVa: "je_emit2_dec_va", jmpW: "jmp_emit2_dec_w", saveRsi: true, tpl: "EMIT2_DEC" },
  { entry: "_emit2_set_add", jeVa: "je_emit2_sa_va", jmpW: "jmp_emit2_sa_w", saveRsi: true, tpl: "EMIT2_SET_ADD" },
  { entry: "_emit2_set_get", jeVa: "je_emit2_sg_va", jmpW: "jmp_emit2_sg_w", saveRsi: true, tpl: "EMIT2_SET_GET" },
  { entry: "_emit2_scm", jeVa: "je_emit2_scm_va", jmpW: "jmp_emit2_scm_w", saveRsi: true, tpl: "EMIT2_SCM" },
  { entry: "_emit2_sas", jeVa: "je_emit2_sas_va", jmpW: "jmp_emit2_sas_w", saveRsi: true, tpl: "EMIT2_SAS" },
  { entry: "_emit2_sasn", jeVa: "je_emit2_sasn_va", jmpW: "jmp_emit2_sasn_w", saveRsi: true, tpl: "EMIT2_SASN" },
  { entry: "_emit2_nid", jeVa: "je_emit2_nid_va", jmpW: "jmp_emit2_nid_w", saveRsi: true, tpl: "EMIT2_NID" },
];

function emitParse1EmitStubs(asm, meta) {
  for (const st of STUBS) {
    emitMiniPePrologue(asm, meta, { entry: st.entry, jeVa: st.jeVa, saveRsi: st.saveRsi });
    emitTextStores(asm, TEMPLATES[st.tpl]);
    emitMiniPeEpilogue(asm, st.jmpW);
  }
  emitLive2Nop(asm, meta);
  return asm;
}

module.exports = {
  emitParse1EmitStubs,
  emitMiniPePrologue,
  emitMiniPeEpilogue,
  emitTextStores,
  emitLive2Nop,
  TEMPLATES,
  STUBS,
  MINI_PE_LEN,
  TEXT_OFF,
  TEXT_ZERO_LEN,
  lineage: 'js_parse1_isa_rt_plant',
};
