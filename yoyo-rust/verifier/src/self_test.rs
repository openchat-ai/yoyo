//! Startup self-test (Decision #12 / Part 9.2.3).
//! Includes Appendix F G01 harness: SET/GET slot round-trip at emit level.

use crate::assembler::{
    self, call_rel32, emit_addv, emit_cmp, emit_dec, emit_get, emit_imul, emit_inc, emit_orv,
    emit_set, emit_subv, jmp_rel32, load_state, store_state,
};
use crate::emit;
use crate::platform::PlatformKind;
use crate::tir::{lower_op_checked, opcode_from_u8, TirOp};
use crate::types::{IsaError, IsaResult, Reg};

pub fn run_self_test() -> IsaResult<()> {
    primitive_correctness_check()?;
    isa_table_check()?;
    set_get_roundtrip_check()?;
    addv_orv_distinct_check()?;
    raw_byte_compile_stub_check()?;
    raw_byte_chained_check()?;
    raw_byte_triple_check()?;
    raw_byte_quad_check()?;
    raw_byte_quint_check()?;
    raw_byte_sextuple_check()?;
    raw_byte_septet_check()?;
    raw_byte_octet_check()?;
    raw_byte_nonet_check()?;
    raw_byte_decuplet_check()?;
    raw_byte_undecuplet_check()?;
    raw_byte_duodecuplet_check()?;
    inc_slot_check()?;
    dec_slot_check()?;
    jmp_branch_check()?;
    call_branch_check()?;
    je_branch_check()?;
    jcc_all_branch_check()?;
    io_backend_check()?;
    add_imm_slot_check()?;
    sub_imm_slot_check()?;
    movrr_slot_check()?;
    orv_slot_check()?;
    nop_slot_check()?;
    raw_bytes_slot_check()?;
    imul_slot_check()?;
    subv_slot_check()?;
    cmp_slot_check()?;
    ldb_body_slot_check()?;
    set_control_slot_check()?;
    get_slot_check()?;
    ldb_off8_handler_slot_check()?;
    ldb_off127_handler_slot_check()?;
    ldb_offm128_handler_slot_check()?;
    ldb_off64_handler_slot_check()?;
    ldb_off16_handler_slot_check()?;
    ldb_off32_handler_slot_check()?;
    ldb_off96_handler_slot_check()?;
    ldb_off112_handler_slot_check()?;
    addv_swap_slot_check()?;
    orv_swap_slot_check()?;
    subv_swap_slot_check()?;
    get_alt_slot_check()?;
    addv_h52_slot_check()?;
    set_large_slot_check()?;
    orv_h52_slot_check()?;
    subv_h52_slot_check()?;
    imul_swap_slot_check()?;
    imul_h52_slot_check()?;
    cmp_swap_slot_check()?;
    get_h52_slot_check()?;
    set_deadbeef_slot_check()?;
    ldb_dst51_slot_check()?;
    inc_h51_slot_check()?;
    dec_h51_slot_check()?;
    addimm_h51_slot_check()?;
    cmp_h52_slot_check()?;
    addv_5052_slot_check()?;
    get_5150_slot_check()?;
    set_12345678_slot_check()?;
    ldb_dst52_slot_check()?;
    subimm_h51_slot_check()?;
    dec_h52_slot_check()?;
    inc_h52_slot_check()?;
    orv_5052_slot_check()?;
    subv_5052_slot_check()?;
    get_5251_slot_check()?;
    set_f00dbabe_slot_check()?;
    cmp_5250_slot_check()?;
    addimm_h52_slot_check()?;
    subimm_h52_03_slot_check()?;
    addimm_h51_0a_slot_check()?;
    subimm_h50_05_slot_check()?;
    orv_5250_slot_check()?;
    subv_5250_slot_check()?;
    addv_5152_slot_check()?;
    imul_5052_slot_check()?;
    set_feedface_slot_check()?;
    set_aabbccdd_slot_check()?;
    get_5052_slot_check()?;
    cmp_5052_slot_check()?;
    ldb_5160_10_slot_check()?;
    imul_5250_slot_check()?;
    orv_5152_slot_check()?;
    addimm_h50_0f_slot_check()?;
    set_beefcafe_slot_check()?;
    set_11111111_slot_check()?;
    subimm_h50_08_slot_check()?;
    addimm_h52_0a_slot_check()?;
    ldb_5260_10_slot_check()?;
    ldb_5060_18_slot_check()?;
    subv_5152_slot_check()?;
    addv_5250_slot_check()?;
    cmp_5152_slot_check()?;
    ldb_5160_18_slot_check()?;
    ldb_5260_18_slot_check()?;
    set_c0ffee00_slot_check()?;
    subimm_h52_08_slot_check()?;
    imul_5152_slot_check()?;
    addimm_h50_14_slot_check()?;
    set_50_c0ffee00_slot_check()?;
    set_52_deadf00d_slot_check()?;
    addimm_h51_14_slot_check()?;
    subimm_h51_0a_slot_check()?;
    ldb_5160_20_slot_check()?;
    ldb_5260_20_slot_check()?;
    addimm_h52_14_slot_check()?;
    subimm_h50_0a_slot_check()?;
    set_51_deadf00d_slot_check()?;
    set_50_facefeed_slot_check()?;
    addimm_h51_1e_slot_check()?;
    subimm_h52_0a_slot_check()?;
    ldb_5060_28_slot_check()?;
    set_52_facefeed_slot_check()?;
    addimm_h50_1e_slot_check()?;
    subimm_h51_05_slot_check()?;
    ldb_5160_28_slot_check()?;
    ldb_5260_28_slot_check()?;
    ldb_5060_30_slot_check()?;
    set_51_baadf00d_slot_check()?;
    addimm_h52_1e_slot_check()?;
    subimm_h50_14_slot_check()?;
    ldb_5160_30_slot_check()?;
    set_52_baadf00d_slot_check()?;
    subimm_h52_14_slot_check()?;
    ldb_5260_30_slot_check()?;
    ldb_5060_38_slot_check()?;
    set_50_0badf00d_slot_check()?;
    addimm_h51_28_slot_check()?;
    subimm_h51_1e_slot_check()?;
    ldb_5160_38_slot_check()?;
    addimm_h50_28_slot_check()?;
    subimm_h52_1e_slot_check()?;
    ldb_5260_38_slot_check()?;
    set_51_feedc0de_slot_check()?;
    addimm_h52_28_slot_check()?;
    subimm_h50_1e_slot_check()?;
    ldb_5160_40_slot_check()?;
    ldb_5260_40_slot_check()?;
    set_52_feedc0de_slot_check()?;
    subimm_h51_28_slot_check()?;
    set_50_feedc0de_slot_check()?;
    addimm_h50_32_slot_check()?;
    subimm_h52_28_slot_check()?;
    ldb_5060_48_slot_check()?;
    ldb_5160_48_slot_check()?;
    ldb_5260_48_slot_check()?;
    addimm_h51_32_slot_check()?;
    subimm_h50_28_slot_check()?;
    ldb_5160_50_slot_check()?;
    ldb_5260_50_slot_check()?;
    set_51_cafef00d_slot_check()?;
    addimm_h52_32_slot_check()?;
    subimm_h51_32_slot_check()?;
    set_50_cafef00d_slot_check()?;
    subimm_h52_32_slot_check()?;
    addimm_h50_3c_slot_check()?;
    set_52_cafef00d_slot_check()?;
    ldb_5060_58_slot_check()?;
    addimm_h51_3c_slot_check()?;
    subimm_h50_3c_slot_check()?;
    ldb_5260_58_slot_check()?;
    ldb_5160_58_slot_check()?;
    addimm_h52_3c_slot_check()?;
    subimm_h51_3c_slot_check()?;
    set_50_deadc0de_slot_check()?;
    ldb_5160_60_slot_check()?;
    ldb_5260_60_slot_check()?;
    addimm_h50_40_slot_check()?;
    addimm_h51_40_slot_check()?;
    addimm_h52_40_slot_check()?;
    subimm_h52_3c_slot_check()?;
    set_51_deadc0de_slot_check()?;
    set_52_deadc0de_slot_check()?;
    ldb_5060_68_slot_check()?;
    ldb_5160_68_slot_check()?;
    ldb_5260_68_slot_check()?;
    addimm_h50_48_slot_check()?;
    addimm_h51_48_slot_check()?;
    subimm_h50_40_slot_check()?;
    subimm_h51_40_slot_check()?;
    addimm_h52_48_slot_check()?;
    subimm_h52_40_slot_check()?;
    ldb_5160_70_slot_check()?;
    ldb_5260_70_slot_check()?;
    set_50_c0dec0de_slot_check()?;
    addimm_h50_50_slot_check()?;
    subimm_h51_48_slot_check()?;
    addimm_h51_50_slot_check()?;
    addimm_h52_50_slot_check()?;
    subimm_h50_48_slot_check()?;
    subimm_h52_48_slot_check()?;
    ldb_5060_78_slot_check()?;
    set_51_c0dec0de_slot_check()?;
    addimm_h50_58_slot_check()?;
    subimm_h51_50_slot_check()?;
    ldb_5160_78_slot_check()?;
    addimm_h51_58_slot_check()?;
    addimm_h52_58_slot_check()?;
    subimm_h50_50_slot_check()?;
    subimm_h52_50_slot_check()?;
    ldb_5260_78_slot_check()?;
    set_52_c0dec0de_slot_check()?;
    addimm_h50_60_slot_check()?;
    ldb_5060_80_slot_check()?;
    addimm_h51_60_slot_check()?;
    addimm_h52_60_slot_check()?;
    subimm_h50_58_slot_check()?;
    subimm_h51_58_slot_check()?;
    ldb_5160_80_slot_check()?;
    ldb_5260_80_slot_check()?;
    subimm_h52_58_slot_check()?;
    addimm_h50_68_slot_check()?;
    addimm_h51_68_slot_check()?;
    addimm_h52_68_slot_check()?;
    subimm_h50_60_slot_check()?;
    subimm_h51_60_slot_check()?;
    subimm_h52_60_slot_check()?;
    ldb_5060_88_slot_check()?;
    ldb_5160_88_slot_check()?;
    ldb_5260_88_slot_check()?;
    addimm_h50_70_slot_check()?;
    addimm_h51_70_slot_check()?;
    addimm_h52_70_slot_check()?;
    subimm_h50_68_slot_check()?;
    subimm_h51_68_slot_check()?;
    subimm_h52_68_slot_check()?;
    ldb_5060_90_slot_check()?;
    ldb_5160_90_slot_check()?;
    ldb_5260_90_slot_check()?;
    subimm_h50_70_slot_check()?;
    subimm_h51_70_slot_check()?;
    subimm_h52_70_slot_check()?;
    addimm_h50_78_slot_check()?;
    addimm_h51_78_slot_check()?;
    addimm_h52_78_slot_check()?;
    ldb_5060_98_slot_check()?;
    ldb_5160_98_slot_check()?;
    ldb_5260_98_slot_check()?;
    subimm_h50_78_slot_check()?;
    subimm_h51_78_slot_check()?;
    subimm_h52_78_slot_check()?;
    addimm_h50_80_slot_check()?;
    addimm_h51_80_slot_check()?;
    addimm_h52_80_slot_check()?;
    ldb_5060_a0_slot_check()?;
    ldb_5160_a0_slot_check()?;
    ldb_5260_a0_slot_check()?;
    subimm_h50_80_slot_check()?;
    subimm_h51_80_slot_check()?;
    subimm_h52_80_slot_check()?;
    addimm_h50_88_slot_check()?;
    addimm_h51_88_slot_check()?;
    addimm_h52_88_slot_check()?;
    subimm_h50_88_slot_check()?;
    subimm_h51_88_slot_check()?;
    subimm_h52_88_slot_check()?;
    ldb_5060_a8_slot_check()?;
    ldb_5160_a8_slot_check()?;
    ldb_5260_a8_slot_check()?;
    addimm_h50_90_slot_check()?;
    addimm_h51_90_slot_check()?;
    addimm_h52_90_slot_check()?;
    subimm_h50_90_slot_check()?;
    subimm_h51_90_slot_check()?;
    subimm_h52_90_slot_check()?;
    ldb_5060_b0_slot_check()?;
    ldb_5160_b0_slot_check()?;
    ldb_5260_b0_slot_check()?;
    addimm_h50_98_slot_check()?;
    addimm_h51_98_slot_check()?;
    addimm_h52_98_slot_check()?;
    subimm_h50_98_slot_check()?;
    subimm_h51_98_slot_check()?;
    subimm_h52_98_slot_check()?;
    ldb_5060_b8_slot_check()?;
    ldb_5160_b8_slot_check()?;
    ldb_5260_b8_slot_check()?;
    addimm_h50_a0_slot_check()?;
    addimm_h51_a0_slot_check()?;
    addimm_h52_a0_slot_check()?;
    subimm_h50_a0_slot_check()?;
    subimm_h51_a0_slot_check()?;
    subimm_h52_a0_slot_check()?;
    ldb_5060_c0_slot_check()?;
    ldb_5160_c0_slot_check()?;
    ldb_5260_c0_slot_check()?;
    addimm_h50_a8_slot_check()?;
    addimm_h51_a8_slot_check()?;
    addimm_h52_a8_slot_check()?;
    subimm_h50_a8_slot_check()?;
    subimm_h51_a8_slot_check()?;
    subimm_h52_a8_slot_check()?;
    ldb_5060_c8_slot_check()?;
    ldb_5160_c8_slot_check()?;
    ldb_5260_c8_slot_check()?;
    addimm_h50_b0_slot_check()?;
    addimm_h51_b0_slot_check()?;
    addimm_h52_b0_slot_check()?;
    subimm_h50_b0_slot_check()?;
    subimm_h51_b0_slot_check()?;
    subimm_h52_b0_slot_check()?;
    addimm_h50_b8_slot_check()?;
    addimm_h51_b8_slot_check()?;
    addimm_h52_b8_slot_check()?;
    subimm_h50_b8_slot_check()?;
    subimm_h51_b8_slot_check()?;
    subimm_h52_b8_slot_check()?;
    ldb_5060_d0_slot_check()?;
    ldb_5160_d0_slot_check()?;
    ldb_5260_d0_slot_check()?;
    addimm_h50_c0_slot_check()?;
    addimm_h51_c0_slot_check()?;
    addimm_h52_c0_slot_check()?;
    subimm_h50_c0_slot_check()?;
    subimm_h51_c0_slot_check()?;
    subimm_h52_c0_slot_check()?;
    ldb_5060_d8_slot_check()?;
    ldb_5160_d8_slot_check()?;
    ldb_5260_d8_slot_check()?;
    addimm_h50_c8_slot_check()?;
    addimm_h51_c8_slot_check()?;
    addimm_h52_c8_slot_check()?;
    subimm_h50_c8_slot_check()?;
    subimm_h51_c8_slot_check()?;
    subimm_h52_c8_slot_check()?;
    addimm_h50_d0_slot_check()?;
    addimm_h51_d0_slot_check()?;
    addimm_h52_d0_slot_check()?;
    subimm_h50_d0_slot_check()?;
    subimm_h51_d0_slot_check()?;
    subimm_h52_d0_slot_check()?;
    ldb_5060_e0_slot_check()?;
    ldb_5160_e0_slot_check()?;
    ldb_5260_e0_slot_check()?;
    addimm_h50_d8_slot_check()?;
    addimm_h51_d8_slot_check()?;
    addimm_h52_d8_slot_check()?;
    subimm_h50_d8_slot_check()?;
    subimm_h51_d8_slot_check()?;
    subimm_h52_d8_slot_check()?;
    ldb_5060_e8_slot_check()?;
    ldb_5160_e8_slot_check()?;
    ldb_5260_e8_slot_check()?;
    addimm_h50_e0_slot_check()?;
    addimm_h51_e0_slot_check()?;
    addimm_h52_e0_slot_check()?;
    subimm_h50_e0_slot_check()?;
    subimm_h51_e0_slot_check()?;
    subimm_h52_e0_slot_check()?;
    addimm_h50_e8_slot_check()?;
    addimm_h51_e8_slot_check()?;
    addimm_h52_e8_slot_check()?;
    subimm_h50_e8_slot_check()?;
    subimm_h51_e8_slot_check()?;
    subimm_h52_e8_slot_check()?;
    ldb_5060_f0_slot_check()?;
    ldb_5160_f0_slot_check()?;
    ldb_5260_f0_slot_check()?;
    addimm_h50_f0_slot_check()?;
    addimm_h51_f0_slot_check()?;
    addimm_h52_f0_slot_check()?;
    subimm_h50_f0_slot_check()?;
    subimm_h51_f0_slot_check()?;
    subimm_h52_f0_slot_check()?;
    ldb_5060_f8_slot_check()?;
    ldb_5160_f8_slot_check()?;
    ldb_5260_f8_slot_check()?;
    addimm_h50_f8_slot_check()?;
    addimm_h51_f8_slot_check()?;
    addimm_h52_f8_slot_check()?;
    subimm_h50_f8_slot_check()?;
    subimm_h51_f8_slot_check()?;
    subimm_h52_f8_slot_check()?;
    ldb_5060_100_slot_check()?;
    ldb_5160_100_slot_check()?;
    ldb_5260_100_slot_check()?;
    addimm_h50_100_slot_check()?;
    addimm_h51_100_slot_check()?;
    addimm_h52_100_slot_check()?;
    subimm_h50_100_slot_check()?;
    subimm_h51_100_slot_check()?;
    subimm_h52_100_slot_check()?;
    ldb_5060_108_slot_check()?;
    ldb_5160_108_slot_check()?;
    ldb_5260_108_slot_check()?;
    addimm_h50_108_slot_check()?;
    addimm_h51_108_slot_check()?;
    addimm_h52_108_slot_check()?;
    subimm_h50_108_slot_check()?;
    subimm_h51_108_slot_check()?;
    subimm_h52_108_slot_check()?;
    ldb_5060_110_slot_check()?;
    ldb_5160_110_slot_check()?;
    ldb_5260_110_slot_check()?;
    addimm_h50_110_slot_check()?;
    addimm_h51_110_slot_check()?;
    addimm_h52_110_slot_check()?;
    subimm_h50_110_slot_check()?;
    subimm_h51_110_slot_check()?;
    subimm_h52_110_slot_check()?;
    ldb_5060_118_slot_check()?;
    ldb_5160_118_slot_check()?;
    ldb_5260_118_slot_check()?;
    addimm_h50_118_slot_check()?;
    addimm_h51_118_slot_check()?;
    addimm_h52_118_slot_check()?;
    subimm_h50_118_slot_check()?;
    subimm_h51_118_slot_check()?;
    subimm_h52_118_slot_check()?;
    ldb_5060_120_slot_check()?;
    ldb_5160_120_slot_check()?;
    ldb_5260_120_slot_check()?;
    addimm_h50_120_slot_check()?;
    addimm_h51_120_slot_check()?;
    addimm_h52_120_slot_check()?;
    subimm_h50_120_slot_check()?;
    subimm_h51_120_slot_check()?;
    subimm_h52_120_slot_check()?;
    ldb_5060_128_slot_check()?;
    ldb_5160_128_slot_check()?;
    ldb_5260_128_slot_check()?;
    addimm_h50_128_slot_check()?;
    addimm_h51_128_slot_check()?;
    addimm_h52_128_slot_check()?;
    subimm_h50_128_slot_check()?;
    subimm_h51_128_slot_check()?;
    subimm_h52_128_slot_check()?;
    ldb_5060_130_slot_check()?;
    ldb_5160_130_slot_check()?;
    ldb_5260_130_slot_check()?;
    addimm_h50_130_slot_check()?;
    addimm_h51_130_slot_check()?;
    addimm_h52_130_slot_check()?;
    subimm_h50_130_slot_check()?;
    subimm_h51_130_slot_check()?;
    subimm_h52_130_slot_check()?;
    ldb_5060_138_slot_check()?;
    ldb_5160_138_slot_check()?;
    ldb_5260_138_slot_check()?;
    addimm_h50_138_slot_check()?;
    addimm_h51_138_slot_check()?;
    addimm_h52_138_slot_check()?;
    subimm_h50_138_slot_check()?;
    subimm_h51_138_slot_check()?;
    subimm_h52_138_slot_check()?;
    ldb_5060_140_slot_check()?;
    ldb_5160_140_slot_check()?;
    ldb_5260_140_slot_check()?;
    addimm_h50_140_slot_check()?;
    addimm_h51_140_slot_check()?;
    addimm_h52_140_slot_check()?;
    subimm_h50_140_slot_check()?;
    subimm_h51_140_slot_check()?;
    subimm_h52_140_slot_check()?;
    ldb_5060_148_slot_check()?;
    ldb_5160_148_slot_check()?;
    ldb_5260_148_slot_check()?;
    addimm_h50_148_slot_check()?;
    addimm_h51_148_slot_check()?;
    addimm_h52_148_slot_check()?;
    subimm_h50_148_slot_check()?;
    subimm_h51_148_slot_check()?;
    subimm_h52_148_slot_check()?;
    ldb_5060_150_slot_check()?;
    ldb_5160_150_slot_check()?;
    ldb_5260_150_slot_check()?;
    addimm_h50_150_slot_check()?;
    addimm_h51_150_slot_check()?;
    addimm_h52_150_slot_check()?;
    subimm_h50_150_slot_check()?;
    subimm_h51_150_slot_check()?;
    subimm_h52_150_slot_check()?;
    ldb_5060_158_slot_check()?;
    ldb_5160_158_slot_check()?;
    ldb_5260_158_slot_check()?;
    addimm_h50_158_slot_check()?;
    addimm_h51_158_slot_check()?;
    addimm_h52_158_slot_check()?;
    subimm_h50_158_slot_check()?;
    subimm_h51_158_slot_check()?;
    subimm_h52_158_slot_check()?;
    ldb_5060_160_slot_check()?;
    ldb_5160_160_slot_check()?;
    ldb_5260_160_slot_check()?;
    addimm_h50_160_slot_check()?;
    addimm_h51_160_slot_check()?;
    addimm_h52_160_slot_check()?;
    subimm_h50_160_slot_check()?;
    subimm_h51_160_slot_check()?;
    subimm_h52_160_slot_check()?;
    ldb_5060_168_slot_check()?;
    ldb_5160_168_slot_check()?;
    ldb_5260_168_slot_check()?;
    addimm_h50_168_slot_check()?;
    addimm_h51_168_slot_check()?;
    addimm_h52_168_slot_check()?;
    subimm_h50_168_slot_check()?;
    subimm_h51_168_slot_check()?;
    subimm_h52_168_slot_check()?;
    ldb_5060_170_slot_check()?;
    ldb_5160_170_slot_check()?;
    ldb_5260_170_slot_check()?;
    addimm_h50_170_slot_check()?;
    addimm_h51_170_slot_check()?;
    addimm_h52_170_slot_check()?;
    subimm_h50_170_slot_check()?;
    subimm_h51_170_slot_check()?;
    subimm_h52_170_slot_check()?;
    ldb_5060_178_slot_check()?;
    ldb_5160_178_slot_check()?;
    ldb_5260_178_slot_check()?;
    addimm_h50_178_slot_check()?;
    addimm_h51_178_slot_check()?;
    addimm_h52_178_slot_check()?;
    subimm_h50_178_slot_check()?;
    subimm_h51_178_slot_check()?;
    subimm_h52_178_slot_check()?;
    ldb_5060_180_slot_check()?;
    ldb_5160_180_slot_check()?;
    ldb_5260_180_slot_check()?;
    addimm_h50_180_slot_check()?;
    addimm_h51_180_slot_check()?;
    addimm_h52_180_slot_check()?;
    subimm_h50_180_slot_check()?;
    subimm_h51_180_slot_check()?;
    subimm_h52_180_slot_check()?;
    ldb_5060_188_slot_check()?;
    ldb_5160_188_slot_check()?;
    ldb_5260_188_slot_check()?;
    addimm_h50_188_slot_check()?;
    addimm_h51_188_slot_check()?;
    addimm_h52_188_slot_check()?;
    subimm_h50_188_slot_check()?;
    subimm_h51_188_slot_check()?;
    subimm_h52_188_slot_check()?;
    ldb_5060_190_slot_check()?;
    ldb_5160_190_slot_check()?;
    ldb_5260_190_slot_check()?;
    addimm_h50_190_slot_check()?;
    addimm_h51_190_slot_check()?;
    addimm_h52_190_slot_check()?;
    subimm_h50_190_slot_check()?;
    subimm_h51_190_slot_check()?;
    subimm_h52_190_slot_check()?;
    ldb_5060_198_slot_check()?;
    ldb_5160_198_slot_check()?;
    ldb_5260_198_slot_check()?;
    addimm_h50_198_slot_check()?;
    addimm_h51_198_slot_check()?;
    addimm_h52_198_slot_check()?;
    subimm_h50_198_slot_check()?;
    subimm_h51_198_slot_check()?;
    subimm_h52_198_slot_check()?;
    ldb_5060_1a0_slot_check()?;
    ldb_5160_1a0_slot_check()?;
    ldb_5260_1a0_slot_check()?;
    addimm_h50_1a0_slot_check()?;
    addimm_h51_1a0_slot_check()?;
    addimm_h52_1a0_slot_check()?;
    subimm_h50_1a0_slot_check()?;
    subimm_h51_1a0_slot_check()?;
    subimm_h52_1a0_slot_check()?;
    ldb_5060_1a8_slot_check()?;
    ldb_5160_1a8_slot_check()?;
    ldb_5260_1a8_slot_check()?;
    addimm_h50_1a8_slot_check()?;
    addimm_h51_1a8_slot_check()?;
    addimm_h52_1a8_slot_check()?;
    subimm_h50_1a8_slot_check()?;
    subimm_h51_1a8_slot_check()?;
    subimm_h52_1a8_slot_check()?;
    ldb_5060_1b0_slot_check()?;
    ldb_5160_1b0_slot_check()?;
    ldb_5260_1b0_slot_check()?;
    addimm_h50_1b0_slot_check()?;
    addimm_h51_1b0_slot_check()?;
    addimm_h52_1b0_slot_check()?;
    subimm_h50_1b0_slot_check()?;
    subimm_h51_1b0_slot_check()?;
    subimm_h52_1b0_slot_check()?;
    ldb_5060_1b8_slot_check()?;
    ldb_5160_1b8_slot_check()?;
    ldb_5260_1b8_slot_check()?;
    addimm_h50_1b8_slot_check()?;
    addimm_h51_1b8_slot_check()?;
    addimm_h52_1b8_slot_check()?;
    subimm_h50_1b8_slot_check()?;
    subimm_h51_1b8_slot_check()?;
    subimm_h52_1b8_slot_check()?;
    ldb_5060_1c0_slot_check()?;
    ldb_5160_1c0_slot_check()?;
    ldb_5260_1c0_slot_check()?;
    addimm_h50_1c0_slot_check()?;
    addimm_h51_1c0_slot_check()?;
    addimm_h52_1c0_slot_check()?;
    subimm_h50_1c0_slot_check()?;
    subimm_h51_1c0_slot_check()?;
    subimm_h52_1c0_slot_check()?;
    ldb_5060_1c8_slot_check()?;
    ldb_5160_1c8_slot_check()?;
    ldb_5260_1c8_slot_check()?;
    addimm_h50_1c8_slot_check()?;
    addimm_h51_1c8_slot_check()?;
    addimm_h52_1c8_slot_check()?;
    subimm_h50_1c8_slot_check()?;
    subimm_h51_1c8_slot_check()?;
    subimm_h52_1c8_slot_check()?;
    ldb_5060_1d0_slot_check()?;
    ldb_5160_1d0_slot_check()?;
    ldb_5260_1d0_slot_check()?;
    addimm_h50_1d0_slot_check()?;
    addimm_h51_1d0_slot_check()?;
    addimm_h52_1d0_slot_check()?;
    subimm_h50_1d0_slot_check()?;
    subimm_h51_1d0_slot_check()?;
    subimm_h52_1d0_slot_check()?;
    ldb_5060_1d8_slot_check()?;
    ldb_5160_1d8_slot_check()?;
    ldb_5260_1d8_slot_check()?;
    addimm_h50_1d8_slot_check()?;
    addimm_h51_1d8_slot_check()?;
    addimm_h52_1d8_slot_check()?;
    subimm_h50_1d8_slot_check()?;
    subimm_h51_1d8_slot_check()?;
    subimm_h52_1d8_slot_check()?;
    ldb_5060_1e0_slot_check()?;
    ldb_5160_1e0_slot_check()?;
    ldb_5260_1e0_slot_check()?;
    addimm_h50_1e0_slot_check()?;
    addimm_h51_1e0_slot_check()?;
    addimm_h52_1e0_slot_check()?;
    subimm_h50_1e0_slot_check()?;
    subimm_h51_1e0_slot_check()?;
    subimm_h52_1e0_slot_check()?;
    ldb_5060_1e8_slot_check()?;
    ldb_5160_1e8_slot_check()?;
    ldb_5260_1e8_slot_check()?;
    addimm_h50_1e8_slot_check()?;
    addimm_h51_1e8_slot_check()?;
    addimm_h52_1e8_slot_check()?;
    subimm_h50_1e8_slot_check()?;
    subimm_h51_1e8_slot_check()?;
    subimm_h52_1e8_slot_check()?;
    ldb_5060_1f0_slot_check()?;
    ldb_5160_1f0_slot_check()?;
    ldb_5260_1f0_slot_check()?;
    addimm_h50_1f0_slot_check()?;
    addimm_h51_1f0_slot_check()?;
    addimm_h52_1f0_slot_check()?;
    subimm_h50_1f0_slot_check()?;
    subimm_h51_1f0_slot_check()?;
    subimm_h52_1f0_slot_check()?;
    ldb_5060_1f8_slot_check()?;
    ldb_5160_1f8_slot_check()?;
    ldb_5260_1f8_slot_check()?;
    addimm_h50_1f8_slot_check()?;
    addimm_h51_1f8_slot_check()?;
    addimm_h52_1f8_slot_check()?;
    subimm_h50_1f8_slot_check()?;
    subimm_h51_1f8_slot_check()?;
    subimm_h52_1f8_slot_check()?;
    ldb_5060_200_slot_check()?;
    ldb_5160_200_slot_check()?;
    ldb_5260_200_slot_check()?;
    addimm_h50_200_slot_check()?;
    addimm_h51_200_slot_check()?;
    addimm_h52_200_slot_check()?;
    subimm_h50_200_slot_check()?;
    subimm_h51_200_slot_check()?;
    subimm_h52_200_slot_check()?;
    ldb_5060_208_slot_check()?;
    ldb_5160_208_slot_check()?;
    ldb_5260_208_slot_check()?;
    addimm_h50_208_slot_check()?;
    addimm_h51_208_slot_check()?;
    addimm_h52_208_slot_check()?;
    subimm_h50_208_slot_check()?;
    subimm_h51_208_slot_check()?;
    subimm_h52_208_slot_check()?;
    ldb_5060_210_slot_check()?;
    ldb_5160_210_slot_check()?;
    ldb_5260_210_slot_check()?;
    addimm_h50_210_slot_check()?;
    addimm_h51_210_slot_check()?;
    addimm_h52_210_slot_check()?;
    subimm_h50_210_slot_check()?;
    subimm_h51_210_slot_check()?;
    subimm_h52_210_slot_check()?;
    ldb_5060_218_slot_check()?;
    ldb_5160_218_slot_check()?;
    ldb_5260_218_slot_check()?;
    addimm_h50_218_slot_check()?;
    addimm_h51_218_slot_check()?;
    addimm_h52_218_slot_check()?;
    subimm_h50_218_slot_check()?;
    subimm_h51_218_slot_check()?;
    subimm_h52_218_slot_check()?;
    ldb_5060_220_slot_check()?;
    ldb_5160_220_slot_check()?;
    ldb_5260_220_slot_check()?;
    addimm_h50_220_slot_check()?;
    addimm_h51_220_slot_check()?;
    addimm_h52_220_slot_check()?;
    subimm_h50_220_slot_check()?;
    subimm_h51_220_slot_check()?;
    subimm_h52_220_slot_check()?;
    ldb_5060_228_slot_check()?;
    ldb_5160_228_slot_check()?;
    ldb_5260_228_slot_check()?;
    addimm_h50_228_slot_check()?;
    addimm_h51_228_slot_check()?;
    addimm_h52_228_slot_check()?;
    subimm_h50_228_slot_check()?;
    subimm_h51_228_slot_check()?;
    subimm_h52_228_slot_check()?;
    ldb_5060_230_slot_check()?;
    ldb_5160_230_slot_check()?;
    ldb_5260_230_slot_check()?;
    addimm_h50_230_slot_check()?;
    addimm_h51_230_slot_check()?;
    addimm_h52_230_slot_check()?;
    subimm_h50_230_slot_check()?;
    subimm_h51_230_slot_check()?;
    subimm_h52_230_slot_check()?;
    ldb_5060_232_slot_check()?;
    ldb_5160_232_slot_check()?;
    ldb_5260_232_slot_check()?;
    addimm_h50_232_slot_check()?;
    addimm_h51_232_slot_check()?;
    addimm_h52_232_slot_check()?;
    subimm_h50_232_slot_check()?;
    Ok(())
}





fn primitive_correctness_check() -> IsaResult<()> {
    let r = assembler::ret();
    if r != [0xC3] {
        return Err(IsaError::ParseError {
            line: 0,
            msg: "ret() != C3".into(),
        });
    }
    let m = assembler::movabs(Reg::Rax, 0x42)?;
    if m.len() != 10 || m[0] != 0x48 || m[1] != 0xB8 {
        return Err(IsaError::ParseError {
            line: 0,
            msg: "movabs encoding broken".into(),
        });
    }
    let ls = assembler::load_state(0, Reg::Rax)?;
    if ls != [0x49, 0x8B, 0x47, 0x00] {
        return Err(IsaError::ParseError {
            line: 0,
            msg: format!("load_state(0) unexpected: {ls:02X?}"),
        });
    }
    Ok(())
}

fn isa_table_check() -> IsaResult<()> {
    if opcode_from_u8(0x30) != Some(TirOp::Set) {
        return Err(IsaError::DuplicateOpcode { op: 0x30 });
    }
    if opcode_from_u8(0xFF) != Some(TirOp::Ret) {
        return Err(IsaError::DuplicateOpcode { op: 0xFF });
    }
    if opcode_from_u8(0xA0) != Some(TirOp::RawByte) {
        return Err(IsaError::DuplicateOpcode { op: 0xA0 });
    }
    // Spot-check no accidental holes for core set (incl. G01–G02 + INC/DEC)
    for op in [0x30u8, 0x40, 0x60, 0x65, 0x66, 0x67, 0x68, 0x69, 0x70, 0x71, 0x80, 0xA0, 0xFF] {
        if opcode_from_u8(op).is_none() {
            return Err(IsaError::ParseError {
                line: 0,
                msg: format!("missing opcode 0x{op:02X}"),
            });
        }
    }
    Ok(())
}

/// G01 harness: SET then GET share slot displacement for the transferred slot.
fn set_get_roundtrip_check() -> IsaResult<()> {
    const SLOT_SRC: u16 = 0x50;
    const SLOT_DST: u16 = 0x51;
    const IMM: u64 = 0x2A;

    let set = emit_set(SLOT_SRC, IMM)?;
    let get = emit_get(SLOT_DST, SLOT_SRC)?;
    if set.len() < 12 || set[0] != 0x48 || set[1] != 0xB8 {
        return Err(IsaError::ParseError {
            line: 0,
            msg: "SET missing movabs rax".into(),
        });
    }
    let store_src = store_state(SLOT_SRC, Reg::Rax)?;
    let load_src = load_state(SLOT_SRC, Reg::Rax)?;
    if !set.ends_with(&store_src) {
        return Err(IsaError::ParseError {
            line: 0,
            msg: "SET does not end with store_state(src)".into(),
        });
    }
    if !get.starts_with(&load_src) {
        return Err(IsaError::ParseError {
            line: 0,
            msg: "GET does not start with load_state(src)".into(),
        });
    }
    // Displacement bytes (after REX+opcode+modrm) must match for same slot
    if store_src.len() < 4 || load_src.len() < 4 || store_src[3..] != load_src[3..] {
        return Err(IsaError::ParseError {
            line: 0,
            msg: format!(
                "SET/GET slot disp mismatch store={store_src:02X?} load={load_src:02X?}"
            ),
        });
    }
    Ok(())
}

/// G02 support: ORV emit must not alias ADDV.
fn addv_orv_distinct_check() -> IsaResult<()> {
    let addv = emit_addv(0x50, 0x51)?;
    let orv = emit_orv(0x50, 0x51)?;
    if addv == orv {
        return Err(IsaError::ParseError {
            line: 0,
            msg: "ORV aliases ADDV (MUST differ)".into(),
        });
    }
    let or_pat = [0x48u8, 0x09, 0xC8];
    let add_pat = [0x48u8, 0x01, 0xC8];
    if !orv.windows(3).any(|w| w == or_pat) {
        return Err(IsaError::ParseError {
            line: 0,
            msg: "ORV missing or rax,rcx".into(),
        });
    }
    if !addv.windows(3).any(|w| w == add_pat) {
        return Err(IsaError::ParseError {
            line: 0,
            msg: "ADDV missing add rax,rcx".into(),
        });
    }
    Ok(())
}

/// W-SM.3: newly exercised RAW_BYTE must compile one tiny NOP+RET stub.
fn raw_byte_compile_stub_check() -> IsaResult<()> {
    let tir = vec![
        lower_op_checked(0x40, &[0x05], 1)?,
        lower_op_checked(0xA0, &[0x90], 2)?,
        lower_op_checked(0xFF, &[], 3)?,
    ];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    if out.code != [0x90, 0xC3] {
        return Err(IsaError::ParseError {
            line: 0,
            msg: format!("RAW_BYTE compile stub != 90 C3: {:02X?}", out.code),
        });
    }
    Ok(())
}

/// W-SM chained: emit must produce 90 90 C3 for H_06 (two RAW_BYTE NOPs + RET).
/// Pinned as a self-test guard so a future regression in RawByte / Tir handling
/// fails the suite before any golden runner complains.
fn raw_byte_chained_check() -> IsaResult<()> {
    let tir = vec![
        lower_op_checked(0x40, &[0x06], 1)?,
        lower_op_checked(0xA0, &[0x90], 2)?,
        lower_op_checked(0xA0, &[0x90], 3)?,
        lower_op_checked(0xFF, &[], 4)?,
    ];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    if out.code != [0x90, 0x90, 0xC3] {
        return Err(IsaError::ParseError {
            line: 0,
            msg: format!("RAW_BYTE chained stub != 90 90 C3: {:02X?}", out.code),
        });
    }
    Ok(())
}

/// W-SM chained3: emit must produce 90 90 90 C3 for H_07 (three RAW_BYTE NOPs + RET).
/// Same opcode class as H_05/H_06 (0xA0 + 0xFF); the chain length grows by one
/// NOP. Pinned as a self-test guard so a future regression in RawByte / Tir
/// handling fails the suite before any golden runner complains, and so the
/// three-handler golden cannot silently fall back to the two-handler path.
fn raw_byte_triple_check() -> IsaResult<()> {
    let tir = vec![
        lower_op_checked(0x40, &[0x07], 1)?,
        lower_op_checked(0xA0, &[0x90], 2)?,
        lower_op_checked(0xA0, &[0x90], 3)?,
        lower_op_checked(0xA0, &[0x90], 4)?,
        lower_op_checked(0xFF, &[], 5)?,
    ];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    if out.code != [0x90, 0x90, 0x90, 0xC3] {
        return Err(IsaError::ParseError {
            line: 0,
            msg: format!("RAW_BYTE triple stub != 90 90 90 C3: {:02X?}", out.code),
        });
    }
    Ok(())
}

/// W-SM chained4: emit must produce 90 90 90 90 C3 for H_08 (four RAW_BYTE
/// NOPs + RET). Same opcode class as H_05/H_06/H_07 (0xA0 + 0xFF); the chain
/// length grows by one NOP. Pinned as a self-test guard so a future regression
/// in RawByte / Tir handling fails the suite before any golden runner
/// complains, and so the four-handler golden cannot silently fall back to the
/// three-handler path.
fn raw_byte_quad_check() -> IsaResult<()> {
    let tir = vec![
        lower_op_checked(0x40, &[0x08], 1)?,
        lower_op_checked(0xA0, &[0x90], 2)?,
        lower_op_checked(0xA0, &[0x90], 3)?,
        lower_op_checked(0xA0, &[0x90], 4)?,
        lower_op_checked(0xA0, &[0x90], 5)?,
        lower_op_checked(0xFF, &[], 6)?,
    ];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    if out.code != [0x90, 0x90, 0x90, 0x90, 0xC3] {
        return Err(IsaError::ParseError {
            line: 0,
            msg: format!("RAW_BYTE quad stub != 90 90 90 90 C3: {:02X?}", out.code),
        });
    }
    Ok(())
}

/// W-SM chained5: emit must produce 90 90 90 90 90 C3 for H_09 (five RAW_BYTE
/// NOPs + RET). Same opcode class as H_05/H_06/H_07/H_08 (0xA0 + 0xFF); the
/// chain length grows by one NOP. Pinned as a self-test guard so a future
/// regression in RawByte / Tir handling fails the suite before any golden
/// runner complains, and so the five-handler golden cannot silently fall back
/// to the four-handler path.
fn raw_byte_quint_check() -> IsaResult<()> {
    let tir = vec![
        lower_op_checked(0x40, &[0x09], 1)?,
        lower_op_checked(0xA0, &[0x90], 2)?,
        lower_op_checked(0xA0, &[0x90], 3)?,
        lower_op_checked(0xA0, &[0x90], 4)?,
        lower_op_checked(0xA0, &[0x90], 5)?,
        lower_op_checked(0xA0, &[0x90], 6)?,
        lower_op_checked(0xFF, &[], 7)?,
    ];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    if out.code != [0x90, 0x90, 0x90, 0x90, 0x90, 0xC3] {
        return Err(IsaError::ParseError {
            line: 0,
            msg: format!(
                "RAW_BYTE quint stub != 90 90 90 90 90 C3: {:02X?}",
                out.code
            ),
        });
    }
    Ok(())
}

/// W-SM chained6: emit must produce 90 90 90 90 90 90 C3 for H_10 (six RAW_BYTE
/// NOPs + RET). Same opcode class as H_05/H_06/H_07/H_08/H_09 (0xA0 + 0xFF);
/// the chain length grows by one NOP. Pinned as a self-test guard so a future
/// regression in RawByte / Tir handling fails the suite before any golden
/// runner complains, and so the six-handler golden cannot silently fall back
/// to the five-handler path.
fn raw_byte_sextuple_check() -> IsaResult<()> {
    let tir = vec![
        lower_op_checked(0x40, &[0x0A], 1)?,
        lower_op_checked(0xA0, &[0x90], 2)?,
        lower_op_checked(0xA0, &[0x90], 3)?,
        lower_op_checked(0xA0, &[0x90], 4)?,
        lower_op_checked(0xA0, &[0x90], 5)?,
        lower_op_checked(0xA0, &[0x90], 6)?,
        lower_op_checked(0xA0, &[0x90], 7)?,
        lower_op_checked(0xFF, &[], 8)?,
    ];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    if out.code != [0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0xC3] {
        return Err(IsaError::ParseError {
            line: 0,
            msg: format!(
                "RAW_BYTE sextuple stub != 90 90 90 90 90 90 C3: {:02X?}",
                out.code
            ),
        });
    }
    Ok(())
}

/// W-SM chained7: emit must produce 90 90 90 90 90 90 90 C3 for H_11 (seven
/// RAW_BYTE NOPs + RET). Same opcode class as H_05/H_06/H_07/H_08/H_09/H_10
/// (0xA0 + 0xFF); the chain length grows by one NOP. Pinned as a self-test
/// guard so a future regression in RawByte / Tir handling fails the suite
/// before any golden runner complains, and so the seven-handler golden
/// cannot silently fall back to the six-handler path.
fn raw_byte_septet_check() -> IsaResult<()> {
    let tir = vec![
        lower_op_checked(0x40, &[0x0B], 1)?,
        lower_op_checked(0xA0, &[0x90], 2)?,
        lower_op_checked(0xA0, &[0x90], 3)?,
        lower_op_checked(0xA0, &[0x90], 4)?,
        lower_op_checked(0xA0, &[0x90], 5)?,
        lower_op_checked(0xA0, &[0x90], 6)?,
        lower_op_checked(0xA0, &[0x90], 7)?,
        lower_op_checked(0xA0, &[0x90], 8)?,
        lower_op_checked(0xFF, &[], 9)?,
    ];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    if out.code != [0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0xC3] {
        return Err(IsaError::ParseError {
            line: 0,
            msg: format!(
                "RAW_BYTE septet stub != 90 90 90 90 90 90 90 C3: {:02X?}",
                out.code
            ),
        });
    }
    Ok(())
}

fn raw_byte_chain_check(hh: u8, nops: usize, label: &str) -> IsaResult<()> {
    let mut tir = Vec::with_capacity(nops + 2);
    tir.push(lower_op_checked(0x40, &[hh as u64], 1)?);
    for line in 0..nops {
        tir.push(lower_op_checked(0xA0, &[0x90], line + 2)?);
    }
    tir.push(lower_op_checked(0xFF, &[], nops + 2)?);
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = vec![0x90; nops];
    want.push(0xC3);
    if out.code != want {
        return Err(IsaError::ParseError {
            line: 0,
            msg: format!("RAW_BYTE {label} stub != {:02X?}: {:02X?}", want, out.code),
        });
    }
    Ok(())
}

/// W-SM chained8: H_12, eight RAW_BYTE NOPs + RET.
fn raw_byte_octet_check() -> IsaResult<()> {
    raw_byte_chain_check(0x0C, 8, "octet")
}

/// W-SM chained9: H_13, nine RAW_BYTE NOPs + RET.
fn raw_byte_nonet_check() -> IsaResult<()> {
    raw_byte_chain_check(0x0D, 9, "nonet")
}

/// W-SM chained10: H_14, ten RAW_BYTE NOPs + RET.
fn raw_byte_decuplet_check() -> IsaResult<()> {
    raw_byte_chain_check(0x0E, 10, "decuplet")
}

/// W-SM chained11: H_15, eleven RAW_BYTE NOPs + RET.
fn raw_byte_undecuplet_check() -> IsaResult<()> {
    raw_byte_chain_check(0x0F, 11, "undecuplet")
}

/// W-SM chained12: H_16, twelve RAW_BYTE NOPs + RET.
fn raw_byte_duodecuplet_check() -> IsaResult<()> {
    raw_byte_chain_check(0x10, 12, "duodecuplet")
}

/// W-SM control flow: CALL via TIR must emit E8 + rel32 + RET.
fn call_branch_check() -> IsaResult<()> {
    let tir = vec![
        lower_op_checked(0x40, &[0x00], 1)?,
        lower_op_checked(0x30, &[0x50, 0x00], 2)?,
        lower_op_checked(0xFF, &[], 3)?,
        lower_op_checked(0x40, &[0x14], 4)?,
        lower_op_checked(0x41, &[0x00], 5)?,
        lower_op_checked(0xFF, &[], 6)?,
    ];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let call_idx = out.code.iter().position(|b| *b == 0xE8)
        .ok_or_else(|| IsaError::ParseError {
            line: 0,
            msg: "CALL branch test: E8 missing".into(),
        })?;
    let rel32 = i32::from_le_bytes(out.code[call_idx + 1..call_idx + 5].try_into().unwrap());
    let target = (call_idx as i32) + 5 + rel32;
    if target != 0 {
        return Err(IsaError::ParseError {
            line: 0,
            msg: format!("CALL branch test: target={target} != 0 (rel32={rel32})"),
        });
    }
    if out.code[call_idx + 5] != 0xC3 {
        return Err(IsaError::ParseError {
            line: 0,
            msg: "CALL branch test: missing RET after CALL".into(),
        });
    }
    let call_bytes = call_rel32(0x42).map_err(|e| IsaError::ParseError {
        line: 0,
        msg: format!("call_rel32 failure: {e}"),
    })?;
    if call_bytes.len() != 5 || call_bytes[0] != 0xE8 {
        return Err(IsaError::ParseError {
            line: 0,
            msg: "call_rel32 must be E8 + rel32 (5 bytes)".into(),
        });
    }
    Ok(())
}

/// W-SM I/O: ALLOC/LOAD_FILE/WRITE_FILE via Stub backend emit movabs+store.
fn io_backend_check() -> IsaResult<()> {
    // ALLOC
    let tir = vec![
        lower_op_checked(0x40, &[0x1F], 1)?,
        lower_op_checked(0x20, &[0x50, 0x1000], 2)?,
        lower_op_checked(0xFF, &[], 3)?,
    ];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    // Must be movabs(0x1000) + store_state(S[0x50]) + ret = 18 bytes
    if out.code.len() != 18 || out.code[0] != 0x48 || out.code[1] != 0xB8 {
        return Err(IsaError::ParseError {
            line: 0,
            msg: format!("ALLOC stub must be 18B movabs+store+ret, got {:02X?}", out.code),
        });
    }
    // LOAD_FILE
    let tir2 = vec![
        lower_op_checked(0x40, &[0x20], 1)?,
        lower_op_checked(0x50, &[0x50, 0x00], 2)?,
        lower_op_checked(0xFF, &[], 3)?,
    ];
    let out2 = emit::emit(&tir2, PlatformKind::Stub)?;
    if out2.code.len() != 18 || out2.code[1] != 0xB8 {
        return Err(IsaError::ParseError {
            line: 0,
            msg: format!("LOAD_FILE stub must be 18B, got {:02X?}", out2.code),
        });
    }
    // WRITE_FILE
    let tir3 = vec![
        lower_op_checked(0x40, &[0x21], 1)?,
        lower_op_checked(0x51, &[0x50, 0x00, 0x51], 2)?,
        lower_op_checked(0xFF, &[], 3)?,
    ];
    let out3 = emit::emit(&tir3, PlatformKind::Stub)?;
    if out3.code.len() != 18 || out3.code[1] != 0xB8 {
        return Err(IsaError::ParseError {
            line: 0,
            msg: format!("WRITE_FILE stub must be 18B, got {:02X?}", out3.code),
        });
    }
    // ALLOC has 0x1000, LOAD/WRITE have 0x00 — must differ
    if out.code == out2.code {
        return Err(IsaError::ParseError {
            line: 0,
            msg: "ALLOC and LOAD_FILE emit must differ (different imm)".into(),
        });
    }
    Ok(())
}

/// W-SM control flow: all 9 Jcc variants (0x72-0x7A) via jcc_rel32.
fn jcc_all_branch_check() -> IsaResult<()> {
    let pairs: [(u8, u8, &str); 9] = [
        (0x72, 0x85, "JNE"),
        (0x73, 0x8C, "JL"),
        (0x74, 0x8D, "JGE"),
        (0x75, 0x8E, "JLE"),
        (0x76, 0x8F, "JG"),
        (0x77, 0x82, "JB"),
        (0x78, 0x83, "JAE"),
        (0x79, 0x86, "JBE"),
        (0x7A, 0x87, "JA"),
    ];
    for &(op, x64_suffix, name) in &pairs {
        let tir = vec![
            lower_op_checked(0x40, &[0x00], 1)?,
            lower_op_checked(0x30, &[0x50, 0x00], 2)?,
            lower_op_checked(0xFF, &[], 3)?,
            lower_op_checked(0x40, &[0x99], 4)?,
            lower_op_checked(0x30, &[0x50, 0x00], 5)?,
            lower_op_checked(0x30, &[0x51, 0x00], 6)?,
            lower_op_checked(0x65, &[0x50, 0x51], 7)?,
            lower_op_checked(op, &[0x00], 8)?,
            lower_op_checked(0xFF, &[], 9)?,
        ];
        let out = emit::emit(&tir, PlatformKind::Stub)?;
        let pat = [0x0F, x64_suffix];
        let idx = out.code.windows(2).position(|w| w == pat)
            .ok_or_else(|| IsaError::ParseError {
                line: 0,
                msg: format!("{name} branch test: 0F {:02X} missing", x64_suffix),
            })?;
        let rel32 = i32::from_le_bytes(out.code[idx + 2..idx + 6].try_into().unwrap());
        let target = (idx as i32) + 6 + rel32;
        if target != 0 {
            return Err(IsaError::ParseError {
                line: 0,
                msg: format!("{name} branch test: target={target} != 0 (rel32={rel32})"),
            });
        }
        // Verify jcc_rel32 helper
        let jcc_bytes = assembler::jcc_rel32(x64_suffix, 0x42).map_err(|e| IsaError::ParseError {
            line: 0,
            msg: format!("jcc_rel32({name}) failure: {e}"),
        })?;
        if jcc_bytes.len() != 6 || jcc_bytes[0] != 0x0F || jcc_bytes[1] != x64_suffix {
            return Err(IsaError::ParseError {
                line: 0,
                msg: format!("jcc_rel32({name}) must be 0F {:02X} + rel32 (6 bytes)", x64_suffix),
            });
        }
    }
    Ok(())
}

/// W-SM control flow: JE via TIR must emit 0F 84 + rel32 + RET.
fn je_branch_check() -> IsaResult<()> {
    let tir = vec![
        lower_op_checked(0x40, &[0x00], 1)?,
        lower_op_checked(0x30, &[0x50, 0x00], 2)?,
        lower_op_checked(0xFF, &[], 3)?,
        lower_op_checked(0x40, &[0x15], 4)?,
        lower_op_checked(0x30, &[0x50, 0x00], 5)?,
        lower_op_checked(0x30, &[0x51, 0x00], 6)?,
        lower_op_checked(0x65, &[0x50, 0x51], 7)?,
        lower_op_checked(0x71, &[0x00], 8)?,
        lower_op_checked(0xFF, &[], 9)?,
    ];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let je_idx = match out.code.windows(2).position(|w| w == [0x0F, 0x84]) {
        Some(i) => i,
        None => return Err(IsaError::ParseError {
            line: 0,
            msg: "JE branch test: 0F 84 missing".into(),
        }),
    };
    let rel32 = i32::from_le_bytes(out.code[je_idx + 2..je_idx + 6].try_into().unwrap());
    let target = (je_idx as i32) + 6 + rel32;
    if target != 0 {
        return Err(IsaError::ParseError {
            line: 0,
            msg: format!("JE branch test: target={target} != 0 (rel32={rel32})"),
        });
    }
    Ok(())
}

/// W-SM control flow: JMP via TIR must emit E9 + rel32 + RET.
fn jmp_branch_check() -> IsaResult<()> {
    // Two-handler sequence: H_00 (SET + RET), H_19 (JMP to 0x00 + RET)
    let tir = vec![
        lower_op_checked(0x40, &[0x00], 1)?,
        lower_op_checked(0x30, &[0x50, 0x00], 2)?,
        lower_op_checked(0xFF, &[], 3)?,
        lower_op_checked(0x40, &[0x13], 4)?,
        lower_op_checked(0x70, &[0x00], 5)?,
        lower_op_checked(0xFF, &[], 6)?,
    ];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    // JMP opcode E9 must be present
    let jmp_idx = out.code.iter().position(|b| *b == 0xE9)
        .ok_or_else(|| IsaError::ParseError {
            line: 0,
            msg: "JMP branch test: E9 missing".into(),
        })?;
    let rel32 = i32::from_le_bytes(out.code[jmp_idx + 1..jmp_idx + 5].try_into().unwrap());
    let target = (jmp_idx as i32) + 5 + rel32;
    if target != 0 {
        return Err(IsaError::ParseError {
            line: 0,
            msg: format!("JMP branch test: target={target} != 0 (rel32={rel32})"),
        });
    }
    // Byte after JMP must be RET
    if out.code[jmp_idx + 5] != 0xC3 {
        return Err(IsaError::ParseError {
            line: 0,
            msg: "JMP branch test: missing RET after JMP".into(),
        });
    }
    // jmp_rel32 helper must produce correct opcode
    let jmp_bytes = jmp_rel32(0x42).map_err(|e| IsaError::ParseError {
        line: 0,
        msg: format!("jmp_rel32 failure: {e}"),
    })?;
    if jmp_bytes.len() != 5 || jmp_bytes[0] != 0xE9 {
        return Err(IsaError::ParseError {
            line: 0,
            msg: "jmp_rel32 must be E9 + rel32 (5 bytes)".into(),
        });
    }
    Ok(())
}

/// W-SM arithmetic: H_17 INC slot must compose load + inc rax + store + ret.
fn inc_slot_check() -> IsaResult<()> {
    arith_slot_check(0x11, 0x66, "INC", emit_inc, &[0x48, 0xFF, 0xC0])
}

/// W-SM arithmetic: H_18 DEC slot must compose load + dec rax + store + ret.
fn dec_slot_check() -> IsaResult<()> {
    arith_slot_check(0x12, 0x67, "DEC", emit_dec, &[0x48, 0xFF, 0xC8])
}

fn arith_slot_check(
    hh: u8,
    op: u8,
    name: &str,
    emit_fn: fn(u16) -> IsaResult<Vec<u8>>,
    opcode_pat: &[u8],
) -> IsaResult<()> {
    const SLOT: u16 = 0x50;
    let tir = vec![
        lower_op_checked(0x40, &[hh as u64], 1)?,
        lower_op_checked(op, &[SLOT as u64], 2)?,
        lower_op_checked(0xFF, &[], 3)?,
    ];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_fn(SLOT)?;
    want.extend(assembler::ret());
    if out.code != want {
        return Err(IsaError::ParseError {
            line: 0,
            msg: format!("{name} slot stub mismatch: got {:02X?} want {:02X?}", out.code, want),
        });
    }
    if !out.code.windows(3).any(|w| w == opcode_pat) {
        return Err(IsaError::ParseError {
            line: 0,
            msg: format!("{name} missing {name:} rax ({})", hex_encode(opcode_pat)),
        });
    }
    let load = load_state(SLOT, Reg::Rax)?;
    let store = store_state(SLOT, Reg::Rax)?;
    if !out.code.starts_with(&load) {
        return Err(IsaError::ParseError {
            line: 0,
            msg: format!("{name} does not start with load_state(slot)"),
        });
    }
    if !contains_tail_before_ret(&out.code, &store) {
        return Err(IsaError::ParseError {
            line: 0,
            msg: format!("{name} does not store_state(slot) before ret"),
        });
    }
    Ok(())
}

fn contains_tail_before_ret(hay: &[u8], needle: &[u8]) -> bool {
    if hay.last() != Some(&0xC3) || hay.len() < needle.len() + 1 {
        return false;
    }
    &hay[hay.len() - 1 - needle.len()..hay.len() - 1] == needle
}

fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

/// body-extend-001: H_2E `0x62 ADD slot imm` MUST compose
/// load_state(0x50) + add_imm(rax, 3) + store_state(0x50) + ret.
/// Pin: 498b87800200004883c00349898780020000c3 (19B).
/// Mirrors inc_slot_check / dec_slot_check template but exercises
/// the add_imm primitive (imm8 path: 3 ∈ [-128, 127]).
fn add_imm_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x50;
    const IMM: u64 = 3;
    const HH: u8 = 0x22;
    const ADD_RAX_IMM8: [u8; 4] = [0x48, 0x83, 0xC0, 0x03];
    let tir = vec![
        lower_op_checked(0x40, &[HH as u64], 1)?,
        lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?,
        lower_op_checked(0xFF, &[], 3)?,
    ];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?;
    want.extend(assembler::ret());
    if out.code != want {
        return Err(IsaError::ParseError {
            line: 0,
            msg: format!(
                "ADD-IMM slot stub mismatch: got {:02X?} want {:02X?}",
                out.code, want
            ),
        });
    }
    if !out.code.windows(4).any(|w| w == ADD_RAX_IMM8) {
        return Err(IsaError::ParseError {
            line: 0,
            msg: "ADD-IMM missing add rax,imm8 signature 48 83 c0 03".into(),
        });
    }
    let load = load_state(SLOT, Reg::Rax)?;
    let store = store_state(SLOT, Reg::Rax)?;
    if !out.code.starts_with(&load) {
        return Err(IsaError::ParseError {
            line: 0,
            msg: "ADD-IMM does not start with load_state(slot)".into(),
        });
    }
    if !contains_tail_before_ret(&out.code, &store) {
        return Err(IsaError::ParseError {
            line: 0,
            msg: "ADD-IMM does not store_state(slot) before ret".into(),
        });
    }
    if out.code.len() != 19 {
        return Err(IsaError::ParseError {
            line: 0,
            msg: format!("ADD-IMM slot stub must be 19B, got {}B", out.code.len()),
        });
    }
    Ok(())
}

/// D-2 MOVRR Phase 2: independent emit_movrr route (same slot-copy semantics as GET).
fn movrr_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_movrr;
    const DST: u16 = 0x50;
    const SRC: u16 = 0x51;
    let tir = vec![
        lower_op_checked(0x40, &[0x24], 1)?,
        lower_op_checked(0x64, &[DST as u64, SRC as u64], 2)?,
        lower_op_checked(0xFF, &[], 3)?,
    ];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_movrr(DST, SRC)?;
    want.extend(assembler::ret());
    if out.code != want || out.code.len() != 15 {
        return Err(IsaError::ParseError { line: 0, msg: format!("MOVRR slot stub mismatch: got {:02X?} want {:02X?}", out.code, want) });
    }
    Ok(())
}

/// Mirrors add_imm_slot_check template but exercises
/// the sub_imm primitive (imm8 path: 3 ∈ [-128, 127]).
fn sub_imm_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x50;
    const IMM: u64 = 3;
    const HH: u8 = 0x23;
    const SUB_RAX_IMM8: [u8; 4] = [0x48, 0x83, 0xE8, 0x03];
    let tir = vec![
        lower_op_checked(0x40, &[HH as u64], 1)?,
        lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?,
        lower_op_checked(0xFF, &[], 3)?,
    ];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?;
    want.extend(assembler::ret());
    if out.code != want {
        return Err(IsaError::ParseError {
            line: 0,
            msg: format!(
                "SUB-IMM slot stub mismatch: got {:02X?} want {:02X?}",
                out.code, want
            ),
        });
    }
    if !out.code.windows(4).any(|w| w == SUB_RAX_IMM8) {
        return Err(IsaError::ParseError {
            line: 0,
            msg: "SUB-IMM missing sub rax,imm8 signature 48 83 e8 03".into(),
        });
    }
    let load = load_state(SLOT, Reg::Rax)?;
    let store = store_state(SLOT, Reg::Rax)?;
    if !out.code.starts_with(&load) {
        return Err(IsaError::ParseError {
            line: 0,
            msg: "SUB-IMM does not start with load_state(slot)".into(),
        });
    }
    if !contains_tail_before_ret(&out.code, &store) {
        return Err(IsaError::ParseError {
            line: 0,
            msg: "SUB-IMM does not store_state(slot) before ret".into(),
        });
    }
    if out.code.len() != 19 {
        return Err(IsaError::ParseError {
            line: 0,
            msg: format!("SUB-IMM slot stub must be 19B, got {}B", out.code.len()),
        });
    }
    Ok(())
}

/// body-extend-004: H_31 `0x69 ORV dst src` MUST compose
/// load_state(0x50) + load_state(0x51,rcx) + or_reg(rax,rcx) + store_state(0x50) + ret.
/// Pin: 498b8780020000498b8f880200004809c849898780020000c3 (25B).
/// Mirrors movrr_slot_check template but exercises the or_reg primitive
/// (NOT add_reg — the audit-defect flag in PROMPT Part 4.1 is satisfied).
/// Distinct from ADDV at byte 16: 48 09 c8 (OR) vs 48 01 c8 (ADD).
fn orv_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_orv;
    const DST: u16 = 0x50;
    const SRC: u16 = 0x51;
    const HH: u8 = 0x25;
    const OR_RAX_RCX: [u8; 3] = [0x48, 0x09, 0xC8];
    let tir = vec![
        lower_op_checked(0x40, &[HH as u64], 1)?,
        lower_op_checked(0x69, &[DST as u64, SRC as u64], 2)?,
        lower_op_checked(0xFF, &[], 3)?,
    ];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_orv(DST, SRC)?;
    want.extend(assembler::ret());
    if out.code != want {
        return Err(IsaError::ParseError {
            line: 0,
            msg: format!(
                "ORV slot stub mismatch: got {:02X?} want {:02X?}",
                out.code, want
            ),
        });
    }
    if !out.code.windows(3).any(|w| w == OR_RAX_RCX) {
        return Err(IsaError::ParseError {
            line: 0,
            msg: "ORV missing or rax,rcx signature 48 09 c8 (audit defect: must NOT alias ADDV add rax,rcx 48 01 c8)".into(),
        });
    }
    // Verify ORV does NOT alias ADDV — both must be in same peer, distinct primitives
    let addv = emit_addv(DST, SRC).map_err(|e| IsaError::ParseError {
        line: 0,
        msg: format!("emit_addv helper: {e}"),
    })?;
    let orv = emit_orv(DST, SRC).map_err(|e| IsaError::ParseError {
        line: 0,
        msg: format!("emit_orv helper: {e}"),
    })?;
    if out.code[..out.code.len() - 1] == addv {
        return Err(IsaError::ParseError {
            line: 0,
            msg: "ORV must NOT alias ADDV (PROMPT Part 4.1 audit rule)".into(),
        });
    }
    let load_dst = load_state(DST, Reg::Rax)?;
    let load_src = load_state(SRC, Reg::Rcx)?;
    let store_dst = store_state(DST, Reg::Rax)?;
    if !out.code.starts_with(&load_dst) {
        return Err(IsaError::ParseError {
            line: 0,
            msg: "ORV does not start with load_state(dst,rax)".into(),
        });
    }
    if !out.code.windows(load_src.len()).any(|w| w == load_src.as_slice()) {
        return Err(IsaError::ParseError {
            line: 0,
            msg: "ORV missing load_state(src,rcx) — bitwise OR needs both operands in registers".into(),
        });
    }
    if !contains_tail_before_ret(&out.code, &store_dst) {
        return Err(IsaError::ParseError {
            line: 0,
            msg: "ORV does not store_state(dst) before ret".into(),
        });
    }
    if out.code.len() != 25 {
        return Err(IsaError::ParseError {
            line: 0,
            msg: format!("ORV slot stub must be 25B, got {}B", out.code.len()),
        });
    }
    let _ = orv; // silence unused
    Ok(())
}

fn nop_slot_check() -> IsaResult<()> {
    const HH: u8 = 0x26;
    let tir = vec![
        lower_op_checked(0x40, &[HH as u64], 1)?,
        lower_op_checked(0x00, &[], 2)?,
        lower_op_checked(0xFF, &[], 3)?,
    ];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    if out.code != [0x90, 0xC3] {
        return Err(IsaError::ParseError {
            line: 0,
            msg: format!("NOP slot stub must be 90 C3, got {:02X?}", out.code),
        });
    }
    Ok(())
}

/// body-extend-005 H_33: `0xA1 RAW_BYTES`, variadic literal-byte primitive.
/// Pin: ccdd c3 (3B). NOT RAW_BYTE 0xA0 filler — opcode 0xA1 routes through
/// the real variadic literal-byte emit path (TirOp::RawBytes → args as bytes).
/// Since `emit::emit_raw_bytes` is private to emit.rs, pin against the
/// expected byte sequence directly.
fn raw_bytes_slot_check() -> IsaResult<()> {
    const HH: u8 = 0x27;
    let tir = vec![
        lower_op_checked(0x40, &[HH as u64], 1)?,
        lower_op_checked(0xA1, &[0xCC, 0xDD], 2)?,
        lower_op_checked(0xFF, &[], 3)?,
    ];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 3] = [0xCC, 0xDD, 0xC3];
    if out.code != want {
        return Err(IsaError::ParseError {
            line: 0,
            msg: format!(
                "RAW-BYTES slot stub mismatch: got {:02X?} want {:02X?}",
                out.code, want
            ),
        });
    }
    Ok(())
}

/// body-extend-005 H_34: `0x63 IMUL`, 2-arg ALU. Pin 26B.
fn imul_slot_check() -> IsaResult<()> {
    const DST: u16 = 0x50;
    const SRC: u16 = 0x51;
    const HH: u8 = 0x28;
    let tir = vec![
        lower_op_checked(0x40, &[HH as u64], 1)?,
        lower_op_checked(0x63, &[DST as u64, SRC as u64], 2)?,
        lower_op_checked(0xFF, &[], 3)?,
    ];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_imul(DST, SRC)?;
    want.extend(assembler::ret());
    if out.code != want {
        return Err(IsaError::ParseError {
            line: 0,
            msg: format!(
                "IMUL slot stub mismatch: got {:02X?} want {:02X?}",
                out.code, want
            ),
        });
    }
    if !out.code.windows(4).any(|w| w == [0x48, 0x0F, 0xAF, 0xC1]) {
        return Err(IsaError::ParseError {
            line: 0,
            msg: "IMUL missing imul rax,rcx signature 48 0F AF C1".into(),
        });
    }
    Ok(())
}

/// body-extend-005 H_35: `0x6A SUBV`, 2-arg ALU. Pin 25B.
fn subv_slot_check() -> IsaResult<()> {
    const DST: u16 = 0x50;
    const SRC: u16 = 0x51;
    const HH: u8 = 0x29;
    let tir = vec![
        lower_op_checked(0x40, &[HH as u64], 1)?,
        lower_op_checked(0x6A, &[DST as u64, SRC as u64], 2)?,
        lower_op_checked(0xFF, &[], 3)?,
    ];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_subv(DST, SRC)?;
    want.extend(assembler::ret());
    if out.code != want {
        return Err(IsaError::ParseError {
            line: 0,
            msg: format!(
                "SUBV slot stub mismatch: got {:02X?} want {:02X?}",
                out.code, want
            ),
        });
    }
    if !out.code.windows(3).any(|w| w == [0x48, 0x29, 0xC8]) {
        return Err(IsaError::ParseError {
            line: 0,
            msg: "SUBV missing sub rax,rcx signature 48 29 C8".into(),
        });
    }
    Ok(())
}

/// body-extend-005 H_36: `0x65 CMP`, 2-arg compare, no store. Pin 18B.
fn cmp_slot_check() -> IsaResult<()> {
    const DST: u16 = 0x50;
    const SRC: u16 = 0x51;
    const HH: u8 = 0x2A;
    let tir = vec![
        lower_op_checked(0x40, &[HH as u64], 1)?,
        lower_op_checked(0x65, &[DST as u64, SRC as u64], 2)?,
        lower_op_checked(0xFF, &[], 3)?,
    ];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_cmp(DST, SRC)?;
    want.extend(assembler::ret());
    if out.code != want {
        return Err(IsaError::ParseError {
            line: 0,
            msg: format!(
                "CMP slot stub mismatch: got {:02X?} want {:02X?}",
                out.code, want
            ),
        });
    }
    if !out.code.windows(3).any(|w| w == [0x48, 0x39, 0xC8]) {
        return Err(IsaError::ParseError {
            line: 0,
            msg: "CMP missing cmp rax,rcx signature 48 39 C8".into(),
        });
    }
    let store50 = store_state(DST, Reg::Rax)?;
    if out.code.windows(store50.len()).any(|w| w == store50.as_slice()) {
        return Err(IsaError::ParseError {
            line: 0,
            msg: "CMP must NOT store_state (compare-only, no store)".into(),
        });
    }
    Ok(())
}

/// body-extend-005 H_37: `0x80 LDB`, 3-arg load-byte. dd=0x50, ss=0x60, oo=0.
/// Pin 19B. load_state(0x60,rax) + movzx rax,byte[rax] + store_state(0x50,rax)
/// + ret. emit::emit_ldb is private; pin against the expected byte sequence.
fn ldb_body_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x50;
    const SS: u16 = 0x60;
    const OO: u64 = 0;
    const HH: u8 = 0x2B;
    let tir = vec![
        lower_op_checked(0x40, &[HH as u64], 1)?,
        lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?,
        lower_op_checked(0xFF, &[], 3)?,
    ];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    // Canonical 19B pin (independently derived in scratch probe).
    let want: [u8; 19] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00, // load_state(0x60, rax) — disp 0x300 LE
        0x48, 0x0f, 0xb6, 0x00,                   // movzx rax, byte[rax]
        0x49, 0x89, 0x87, 0x80, 0x02, 0x00, 0x00, // store_state(0x50, rax) — disp 0x280 LE
        0xc3,                                     // ret
    ];
    if out.code != want {
        return Err(IsaError::ParseError {
            line: 0,
            msg: format!(
                "LDB-BODY slot stub mismatch: got {:02X?} want {:02X?}",
                out.code, want
            ),
        });
    }
    Ok(())
}

/// body-extend-005 H_38: `0x30 SET`, CONTROL: already opcode-covered.
/// Identical 18B pin to H_00; no regression.
fn set_control_slot_check() -> IsaResult<()> {
    const SLOT: u16 = 0x50;
    const IMM: u64 = 0;
    const HH: u8 = 0x2C;
    let tir = vec![
        lower_op_checked(0x40, &[HH as u64], 1)?,
        lower_op_checked(0x30, &[SLOT as u64, IMM], 2)?,
        lower_op_checked(0xFF, &[], 3)?,
    ];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_set(SLOT, IMM)?;
    want.extend(assembler::ret());
    if out.code != want {
        return Err(IsaError::ParseError {
            line: 0,
            msg: format!(
                "SET-CONTROL slot stub mismatch: got {:02X?} want {:02X?}",
                out.code, want
            ),
        });
    }
    Ok(())
}

/// body-extend-006 H_39: `0x60 GET dst src`, 2-arg state-slot copy.
/// Pin: 498b878802000049898780020000c3 (15B). Mirrors movrr_slot_check
/// template (which uses 0x64 MOVRR and emit_movrr) but exercises the GET
/// opcode directly. Both peers route GET through emit_get and MOVRR through
/// emit_movrr (D-2 Phase 2 decoupling); bytes match for this pin but the
/// opcode surfaces are distinct.
fn get_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_get;
    const DST: u16 = 0x50;
    const SRC: u16 = 0x51;
    const HH: u8 = 0x2D;
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
            msg: format!(
                "GET slot stub mismatch: got {:02X?} want {:02X?}",
                out.code, want
            ),
        });
    }
    Ok(())
}

/// body-extend-007 H_40: `0x80 LDB dd ss oo`, 3-arg load-byte with oo=8
/// at selector 0x2E (imm8 path: 8 ∈ [-128, 127]). Mirrors ldb_body_slot_check
/// template (H_37 at oo=0, selector 0x2B) but exercises the add_imm imm8
/// code path. Pin: 498b87000300004883c008480fb60049898780020000c3 (23B).
/// load_state(0x60, rax) + add_imm rax, 8 (imm8 path) + movzx rax, byte
/// [rax] + store_state(0x50, rax) + ret = 7 + 4 + 4 + 7 + 1 = 23B.
/// Both peers compose identical bytes via the same x86-64 primitives:
/// JS encodeOp(0x80, [0x50, 0x60, 8]) and Rust emit_ldb(0x50, 0x60, 8).
/// Disjoint from H_37: H_37 oo=0 short-circuits add_imm (19B), H_40 oo=8
/// takes the imm8 path (23B). No D-1/D-2/D-3 aliasing.
fn ldb_off8_handler_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x50;
    const SS: u16 = 0x60;
    const OO: u64 = 8;
    const HH: u8 = 0x2E;
    let tir = vec![
        lower_op_checked(0x40, &[HH as u64], 1)?,
        lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?,
        lower_op_checked(0xFF, &[], 3)?,
    ];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    // Canonical 23B pin (independently derived in JS scratch probe).
    let want: [u8; 23] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00, // load_state(0x60, rax) — disp 0x300 LE
        0x48, 0x83, 0xc0, 0x08,                   // add rax, 8 (imm8 path active)
        0x48, 0x0f, 0xb6, 0x00,                   // movzx rax, byte [rax]
        0x49, 0x89, 0x87, 0x80, 0x02, 0x00, 0x00, // store_state(0x50, rax) — disp 0x280 LE
        0xc3,                                     // ret
    ];
    if out.code != want {
        return Err(IsaError::ParseError {
            line: 0,
            msg: format!(
                "LDB-OFF8-HANDLER slot stub mismatch: got {:02X?} want {:02X?}",
                out.code, want
            ),
        });
    }
    if out.code.len() != 23 {
        return Err(IsaError::ParseError {
            line: 0,
            msg: format!("LDB-OFF8-HANDLER slot stub must be 23B, got {}B", out.code.len()),
        });
    }
    if !out.code.windows(4).any(|w| w == [0x48, 0x83, 0xC0, 0x08]) {
        return Err(IsaError::ParseError {
            line: 0,
            msg: "LDB-OFF8-HANDLER missing add rax, imm8 signature 48 83 c0 08".into(),
        });
    }
    // Pin absence of imm32 path (48 81 c0): the encoder MUST stay on
    // the imm8 path for oo=8.
    if out.code.windows(3).any(|w| w == [0x48, 0x81, 0xC0]) {
        return Err(IsaError::ParseError {
            line: 0,
            msg: "LDB-OFF8-HANDLER emitted imm32 path for oo=8 (premature imm32 escalation)".into(),
        });
    }
    Ok(())
}

fn ldb_off127_handler_slot_check() -> IsaResult<()> {
    let tir = vec![
        lower_op_checked(0x40, &[0x2F], 1)?,
        lower_op_checked(0x80, &[0x50, 0x60, 127], 2)?,
        lower_op_checked(0xFF, &[], 3)?,
    ];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 23] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x83, 0xc0, 0x7f,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x80, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.windows(7).any(|w| w == [0x48, 0x81, 0xc0, 0x7f, 0, 0, 0]) {
        return Err(IsaError::ParseError { line: 0, msg: format!("LDB-OFF127-HANDLER imm8 boundary mismatch: {:02X?}", out.code) });
    }
    Ok(())
}

/// body-extend-009 H_42: `0x80 LDB dd ss oo`, 3-arg load-byte with oo=0x50
/// at selector 0x30 (imm8 path: 0x50 = 80 decimal, positive). Mirrors
/// ldb_off127_handler_slot_check template (H_41 at oo=127, selector 0x2F)
/// but exercises the LEFT-side imm8 byte (0x50 vs H_41's 0x7f). Pin:
/// 498b87000300004883c050480fb60049898780020000c3 (23B).
/// load_state(0x60, rax) + add_imm rax, 0x50 (imm8 path)
/// + movzx rax, byte [rax] + store_state(0x50, rax) + ret = 7 + 4 + 4 + 7 + 1 = 23B.
/// Both peers compose identical bytes via the same x86-64 primitives:
/// JS encodeOp(0x80, [0x50, 0x60, 0x50]) and Rust emit_ldb(0x50, 0x60, 0x50).
/// Disjoint from H_41: H_41 oo=127 imm8 byte=0x7f, H_42 oo=0x50 imm8 byte=0x50.
/// No D-1/D-2/D-3 aliasing. Note: the signed-imm8 LEFT-edge signed-token
/// semantic (-128 → imm8 byte 0x80) is covered by the JS-only checkLDBoffm128
/// probe on selfhost_min_ldb_offm128.ty because Rust ty_parser lacks
/// signed-hex support and emit.rs treats oo as unsigned u16 per dispatch
/// contract — byte-equal peer emission at H_42 uses oo=0x50 (positive).
fn ldb_offm128_handler_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x50;
    const SS: u16 = 0x60;
    const OO: u64 = 0x50;
    const HH: u8 = 0x30;
    let tir = vec![
        lower_op_checked(0x40, &[HH as u64], 1)?,
        lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?,
        lower_op_checked(0xFF, &[], 3)?,
    ];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    // Canonical 23B pin (independently derived in JS scratch probe).
    let want: [u8; 23] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00, // load_state(0x60, rax) — disp 0x300 LE
        0x48, 0x83, 0xc0, 0x50,                   // add rax, 0x50 (imm8 path active: 0x50 byte)
        0x48, 0x0f, 0xb6, 0x00,                   // movzx rax, byte [rax]
        0x49, 0x89, 0x87, 0x80, 0x02, 0x00, 0x00, // store_state(0x50, rax) — disp 0x280 LE
        0xc3,                                     // ret
    ];
    if out.code != want {
        return Err(IsaError::ParseError {
            line: 0,
            msg: format!(
                "LDB-OFFM128-HANDLER slot stub mismatch: got {:02X?} want {:02X?}",
                out.code, want
            ),
        });
    }
    if out.code.len() != 23 {
        return Err(IsaError::ParseError {
            line: 0,
            msg: format!("LDB-OFFM128-HANDLER slot stub must be 23B, got {}B", out.code.len()),
        });
    }
    if !out.code.windows(4).any(|w| w == [0x48, 0x83, 0xC0, 0x50]) {
        return Err(IsaError::ParseError {
            line: 0,
            msg: "LDB-OFFM128-HANDLER missing add rax, imm8=0x50 signature 48 83 c0 50".into(),
        });
    }
    // Pin absence of imm32 path (48 81 c0): the encoder MUST stay on
    // the imm8 path for oo=0x50.
    if out.code.windows(3).any(|w| w == [0x48, 0x81, 0xC0]) {
        return Err(IsaError::ParseError {
            line: 0,
            msg: "LDB-OFFM128-HANDLER emitted imm32 path for oo=0x50 (premature imm32 escalation)".into(),
        });
    }
    Ok(())
}

/// body-extend-010 H_43: `0x80 LDB dd ss oo`, 3-arg load-byte with oo=0x40
/// (=64 decimal; positive imm8 byte) at selector 0x31. Mirrors
/// ldb_offm128_handler_slot_check template but exercises a fresh imm8 byte
/// (0x40) symmetric to H_42 (0x50). Pin:
/// 498b87000300004883c040480fb60049898780020000c3 (23B).
/// load_state(0x60, rax) + add_imm rax, 0x40 (imm8 path)
/// + movzx rax, byte [rax] + store_state(0x50, rax) + ret = 7 + 4 + 4 + 7 + 1 = 23B.
/// Both peers compose identical bytes via the same x86-64 primitives:
/// JS encodeOp(0x80, [0x50, 0x60, 0x40]) and Rust emit_ldb(0x50, 0x60, 0x40).
/// Disjoint from H_42: H_42 oo=0x50 imm8 byte=0x50, H_43 oo=0x40 imm8 byte=0x40.
/// No D-1/D-2/D-3 aliasing. The signed-imm8 LEFT-edge signed-token semantic
/// (-128 -> imm8 byte 0x80) is covered by the JS-only checkLDBoffm128 probe
/// on selfhost_min_ldb_offm128.ty because Rust ty_parser lacks signed-hex
/// support and emit.rs treats oo as unsigned u16 per dispatch contract.
fn ldb_off64_handler_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x50;
    const SS: u16 = 0x60;
    const OO: u64 = 0x40;
    const HH: u8 = 0x31;
    let tir = vec![
        lower_op_checked(0x40, &[HH as u64], 1)?,
        lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?,
        lower_op_checked(0xFF, &[], 3)?,
    ];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    // Canonical 23B pin (independently derived in JS scratch probe).
    let want: [u8; 23] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00, // load_state(0x60, rax) - disp 0x300 LE
        0x48, 0x83, 0xc0, 0x40,                   // add rax, 0x40 (imm8 path active: 0x40=64 in [-128, 127])
        0x48, 0x0f, 0xb6, 0x00,                   // movzx rax, byte [rax]
        0x49, 0x89, 0x87, 0x80, 0x02, 0x00, 0x00, // store_state(0x50, rax) - disp 0x280 LE
        0xc3,                                     // ret
    ];
    if out.code != want {
        return Err(IsaError::ParseError {
            line: 0,
            msg: format!(
                "LDB-OFF64-HANDLER slot stub mismatch: got {:02X?} want {:02X?}",
                out.code, want
            ),
        });
    }
    if out.code.len() != 23 {
        return Err(IsaError::ParseError {
            line: 0,
            msg: format!("LDB-OFF64-HANDLER slot stub must be 23B, got {}B", out.code.len()),
        });
    }
    if !out.code.windows(4).any(|w| w == [0x48, 0x83, 0xC0, 0x40]) {
        return Err(IsaError::ParseError {
            line: 0,
            msg: "LDB-OFF64-HANDLER missing add rax, imm8=0x40 signature 48 83 c0 40".into(),
        });
    }
    // Pin absence of imm32 path (48 81 c0): the encoder MUST stay on
    // the imm8 path for oo=0x40.
    if out.code.windows(3).any(|w| w == [0x48, 0x81, 0xC0]) {
        return Err(IsaError::ParseError {
            line: 0,
            msg: "LDB-OFF64-HANDLER emitted imm32 path for oo=0x40 (premature imm32 escalation)".into(),
        });
    }
    Ok(())
}

/// body-extend-011 H_44: `0x80 LDB dd ss oo`, 3-arg load-byte with oo=0x10
/// (=16 decimal; positive imm8 byte) at selector 0x32. Mirrors
/// ldb_off64_handler_slot_check template but exercises a fresh imm8 byte
/// (0x10) at a fresh selector (0x32). Pin:
/// 498b87000300004883c010480fb60049898780020000c3 (23B).
/// load_state(0x60, rax) + add_imm rax, 0x10 (imm8 path)
/// + movzx rax, byte [rax] + store_state(0x50, rax) + ret = 7 + 4 + 4 + 7 + 1 = 23B.
/// Both peers compose identical bytes via the same x86-64 primitives:
/// JS encodeOp(0x80, [0x50, 0x60, 0x10]) and Rust emit_ldb(0x50, 0x60, 0x10).
/// Disjoint from H_43: H_43 oo=0x40 imm8 byte=0x40, H_44 oo=0x10 imm8 byte=0x10.
/// No D-1/D-2/D-3 aliasing. The signed-imm8 LEFT-edge signed-token semantic
/// (-128 -> imm8 byte 0x80) is covered by the JS-only checkLDBoffm128 probe
/// on selfhost_min_ldb_offm128.ty because Rust ty_parser lacks signed-hex
/// support and emit.rs treats oo as unsigned u16 per dispatch contract.
fn ldb_off16_handler_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x50;
    const SS: u16 = 0x60;
    const OO: u64 = 0x10;
    const HH: u8 = 0x32;
    let tir = vec![
        lower_op_checked(0x40, &[HH as u64], 1)?,
        lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?,
        lower_op_checked(0xFF, &[], 3)?,
    ];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    // Canonical 23B pin (independently derived in JS scratch probe).
    let want: [u8; 23] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00, // load_state(0x60, rax) - disp 0x300 LE
        0x48, 0x83, 0xc0, 0x10,                   // add rax, 0x10 (imm8 path active: 0x10=16 in [-128, 127])
        0x48, 0x0f, 0xb6, 0x00,                   // movzx rax, byte [rax]
        0x49, 0x89, 0x87, 0x80, 0x02, 0x00, 0x00, // store_state(0x50, rax) - disp 0x280 LE
        0xc3,                                     // ret
    ];
    if out.code != want {
        return Err(IsaError::ParseError {
            line: 0,
            msg: format!(
                "LDB-OFF16-HANDLER slot stub mismatch: got {:02X?} want {:02X?}",
                out.code, want
            ),
        });
    }
    if out.code.len() != 23 {
        return Err(IsaError::ParseError {
            line: 0,
            msg: format!("LDB-OFF16-HANDLER slot stub must be 23B, got {}B", out.code.len()),
        });
    }
    if !out.code.windows(4).any(|w| w == [0x48, 0x83, 0xC0, 0x10]) {
        return Err(IsaError::ParseError {
            line: 0,
            msg: "LDB-OFF16-HANDLER missing add rax, imm8=0x10 signature 48 83 c0 10".into(),
        });
    }
    // Pin absence of imm32 path (48 81 c0): the encoder MUST stay on
    // the imm8 path for oo=0x10.
    if out.code.windows(3).any(|w| w == [0x48, 0x81, 0xC0]) {
        return Err(IsaError::ParseError {
            line: 0,
            msg: "LDB-OFF16-HANDLER emitted imm32 path for oo=0x10 (premature imm32 escalation)".into(),
        });
    }
    Ok(())
}

/// body-extend-012 H_45: `0x80 LDB dd ss oo`, 3-arg load-byte with oo=0x20
/// (=32 decimal; positive imm8 byte) at selector 0x33. Mirrors
/// ldb_off16_handler_slot_check template but exercises a fresh imm8 byte
/// (0x20) at a fresh selector (0x33). Pin:
/// 498b87000300004883c020480fb60049898780020000c3 (23B).
/// load_state(0x60, rax) + add_imm rax, 0x20 (imm8 path)
/// + movzx rax, byte [rax] + store_state(0x50, rax) + ret = 7 + 4 + 4 + 7 + 1 = 23B.
/// Both peers compose identical bytes via the same x86-64 primitives:
/// JS encodeOp(0x80, [0x50, 0x60, 0x20]) and Rust emit_ldb(0x50, 0x60, 0x20).
/// Disjoint from H_44: H_44 oo=0x10 imm8 byte=0x10, H_45 oo=0x20 imm8 byte=0x20.
/// No D-1/D-2/D-3 aliasing. The signed-imm8 LEFT-edge signed-token semantic
/// (-128 -> imm8 byte 0x80) is covered by the JS-only checkLDBoffm128 probe
/// on selfhost_min_ldb_offm128.ty because Rust ty_parser lacks signed-hex
/// support and emit.rs treats oo as unsigned u16 per dispatch contract.
fn ldb_off32_handler_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x50;
    const SS: u16 = 0x60;
    const OO: u64 = 0x20;
    const HH: u8 = 0x33;
    let tir = vec![
        lower_op_checked(0x40, &[HH as u64], 1)?,
        lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?,
        lower_op_checked(0xFF, &[], 3)?,
    ];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    // Canonical 23B pin (independently derived in JS scratch probe).
    let want: [u8; 23] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00, // load_state(0x60, rax) - disp 0x300 LE
        0x48, 0x83, 0xc0, 0x20,                   // add rax, 0x20 (imm8 path active: 0x20=32 in [-128, 127])
        0x48, 0x0f, 0xb6, 0x00,                   // movzx rax, byte [rax]
        0x49, 0x89, 0x87, 0x80, 0x02, 0x00, 0x00, // store_state(0x50, rax) - disp 0x280 LE
        0xc3,                                     // ret
    ];
    if out.code != want {
        return Err(IsaError::ParseError {
            line: 0,
            msg: format!(
                "LDB-OFF32-HANDLER slot stub mismatch: got {:02X?} want {:02X?}",
                out.code, want
            ),
        });
    }
    if out.code.len() != 23 {
        return Err(IsaError::ParseError {
            line: 0,
            msg: format!("LDB-OFF32-HANDLER slot stub must be 23B, got {}B", out.code.len()),
        });
    }
    if !out.code.windows(4).any(|w| w == [0x48, 0x83, 0xC0, 0x20]) {
        return Err(IsaError::ParseError {
            line: 0,
            msg: "LDB-OFF32-HANDLER missing add rax, imm8=0x20 signature 48 83 c0 20".into(),
        });
    }
    // Pin absence of imm32 path (48 81 c0): the encoder MUST stay on
    // the imm8 path for oo=0x20.
    if out.code.windows(3).any(|w| w == [0x48, 0x81, 0xC0]) {
        return Err(IsaError::ParseError {
            line: 0,
            msg: "LDB-OFF32-HANDLER emitted imm32 path for oo=0x20 (premature imm32 escalation)".into(),
        });
    }
    Ok(())
}

/// body-extend-013 H_46: LDB dd=0x50 ss=0x60 oo=0x60 at selector 0x34.
/// The positive offset 96 must use the four-byte imm8 encoding 48 83 c0 60.
fn ldb_off96_handler_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x50;
    const SS: u16 = 0x60;
    const OO: u64 = 0x60;
    const HH: u8 = 0x34;
    let tir = vec![
        lower_op_checked(0x40, &[HH as u64], 1)?,
        lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?,
        lower_op_checked(0xFF, &[], 3)?,
    ];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 23] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x83, 0xc0, 0x60,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x80, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want {
        return Err(IsaError::ParseError {
            line: 0,
            msg: format!(
                "LDB-OFF96-HANDLER slot stub mismatch: got {:02X?} want {:02X?}",
                out.code, want
            ),
        });
    }
    if !out.code.windows(4).any(|w| w == [0x48, 0x83, 0xC0, 0x60]) {
        return Err(IsaError::ParseError {
            line: 0,
            msg: "LDB-OFF96-HANDLER missing add rax, imm8=0x60 signature".into(),
        });
    }
    if out.code.windows(3).any(|w| w == [0x48, 0x81, 0xC0]) {
        return Err(IsaError::ParseError {
            line: 0,
            msg: "LDB-OFF96-HANDLER emitted imm32 path for oo=0x60".into(),
        });
    }
    Ok(())
}

/// body-extend-014 H_47: LDB dd=0x50 ss=0x60 oo=0x70 (= 112 decimal) at
/// selector 0x35. Mirrors ldb_off96_handler_slot_check template but
/// exercises a fresh imm8 byte (0x70) at a fresh selector (0x35).
/// The encoder MUST stay on the imm8 path because 112 ∈ [-128, 127],
/// and MUST NOT escalate to the 7B imm32 `48 81 c0 70 00 00 00` form.
/// Pin: 498b87000300004883c070480fb60049898780020000c3 (23B).
/// load_state(0x60, rax) + add_imm rax, 0x70 (imm8 path)
/// + movzx rax, byte[rax] + store_state(0x50, rax) + ret = 7 + 4 + 4 + 7 + 1 = 23B.
/// Both peers compose identical bytes via the same x86-64 primitives:
/// JS encodeOp(0x80, [0x50, 0x60, 0x70]) and Rust emit_ldb(0x50, 0x60, 0x70).
/// Disjoint from H_46: H_46 oo=0x60 imm8 byte=0x60, H_47 oo=0x70 imm8 byte=0x70.
/// No D-1/D-2/D-3 aliasing.
fn ldb_off112_handler_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x50;
    const SS: u16 = 0x60;
    const OO: u64 = 0x70;
    const HH: u8 = 0x35;
    let tir = vec![
        lower_op_checked(0x40, &[HH as u64], 1)?,
        lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?,
        lower_op_checked(0xFF, &[], 3)?,
    ];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    // Canonical 23B pin (independently derived in JS scratch probe).
    let want: [u8; 23] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00, // load_state(0x60, rax) — disp 0x300 LE
        0x48, 0x83, 0xc0, 0x70,                   // add rax, 0x70 (imm8 path: 0x70=112 in [-128, 127])
        0x48, 0x0f, 0xb6, 0x00,                   // movzx rax, byte[rax]
        0x49, 0x89, 0x87, 0x80, 0x02, 0x00, 0x00, // store_state(0x50, rax) — disp 0x280 LE
        0xc3,                                     // ret
    ];
    if out.code != want {
        return Err(IsaError::ParseError {
            line: 0,
            msg: format!(
                "LDB-OFF112-HANDLER slot stub mismatch: got {:02X?} want {:02X?}",
                out.code, want
            ),
        });
    }
    if out.code.len() != 23 {
        return Err(IsaError::ParseError {
            line: 0,
            msg: format!("LDB-OFF112-HANDLER slot stub must be 23B, got {}B", out.code.len()),
        });
    }
    if !out.code.windows(4).any(|w| w == [0x48, 0x83, 0xC0, 0x70]) {
        return Err(IsaError::ParseError {
            line: 0,
            msg: "LDB-OFF112-HANDLER missing add rax, imm8=0x70 signature 48 83 c0 70".into(),
        });
    }
    // Pin absence of imm32 path (48 81 c0): the encoder MUST stay on
    // the imm8 path for oo=0x70.
    if out.code.windows(3).any(|w| w == [0x48, 0x81, 0xC0]) {
        return Err(IsaError::ParseError {
            line: 0,
            msg: "LDB-OFF112-HANDLER emitted imm32 path for oo=0x70 (premature imm32 escalation)".into(),
        });
    }
    Ok(())
}

/// body-extend-015 / parallel-batch-09 H_48: 0x68 ADDV dst=0x51 src=0x50.
/// Pin: 498b8788020000498b8f800200004801c849898788020000c3 (25B).
fn addv_swap_slot_check() -> IsaResult<()> {
    const DST: u16 = 0x51;
    const SRC: u16 = 0x50;
    const HH: u8 = 0x36;
    let tir = vec![
        lower_op_checked(0x40, &[HH as u64], 1)?,
        lower_op_checked(0x68, &[DST as u64, SRC as u64], 2)?,
        lower_op_checked(0xFF, &[], 3)?,
    ];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_addv(DST, SRC)?;
    want.extend(assembler::ret());
    if out.code != want || out.code.len() != 25 {
        return Err(IsaError::ParseError {
            line: 0,
            msg: format!(
                "ADDV-SWAP slot stub mismatch: got {:02X?} want {:02X?}",
                out.code, want
            ),
        });
    }
    Ok(())
}

/// body-extend-015 / parallel-batch-09 H_49: 0x69 ORV dst=0x51 src=0x50.
/// Pin: 498b8788020000498b8f800200004809c849898788020000c3 (25B).
fn orv_swap_slot_check() -> IsaResult<()> {
    const DST: u16 = 0x51;
    const SRC: u16 = 0x50;
    const HH: u8 = 0x37;
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
            msg: format!(
                "ORV-SWAP slot stub mismatch: got {:02X?} want {:02X?}",
                out.code, want
            ),
        });
    }
    Ok(())
}

/// body-extend-015 / parallel-batch-09 H_50: 0x6A SUBV dst=0x51 src=0x50.
/// Pin: 498b8788020000498b8f800200004829c849898788020000c3 (25B).
fn subv_swap_slot_check() -> IsaResult<()> {
    const DST: u16 = 0x51;
    const SRC: u16 = 0x50;
    const HH: u8 = 0x38;
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
            msg: format!(
                "SUBV-SWAP slot stub mismatch: got {:02X?} want {:02X?}",
                out.code, want
            ),
        });
    }
    Ok(())
}

/// body-extend-015 / parallel-batch-09 H_51: 0x60 GET dst=0x51 src=0x52.
/// Pin: 498b879002000049898788020000c3 (15B).
fn get_alt_slot_check() -> IsaResult<()> {
    const DST: u16 = 0x51;
    const SRC: u16 = 0x52;
    const HH: u8 = 0x39;
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
            msg: format!(
                "GET-ALT slot stub mismatch: got {:02X?} want {:02X?}",
                out.code, want
            ),
        });
    }
    Ok(())
}

/// body-extend-015 / parallel-batch-09 H_52: 0x68 ADDV dst=0x52 src=0x51.
/// Pin: 498b8790020000498b8f880200004801c849898790020000c3 (25B).
fn addv_h52_slot_check() -> IsaResult<()> {
    const DST: u16 = 0x52;
    const SRC: u16 = 0x51;
    const HH: u8 = 0x3A;
    let tir = vec![
        lower_op_checked(0x40, &[HH as u64], 1)?,
        lower_op_checked(0x68, &[DST as u64, SRC as u64], 2)?,
        lower_op_checked(0xFF, &[], 3)?,
    ];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_addv(DST, SRC)?;
    want.extend(assembler::ret());
    if out.code != want || out.code.len() != 25 {
        return Err(IsaError::ParseError {
            line: 0,
            msg: format!(
                "ADDV-H52 slot stub mismatch: got {:02X?} want {:02X?}",
                out.code, want
            ),
        });
    }
    Ok(())
}

/// body-extend-015 / parallel-batch-09 H_53: 0x30 SET slot=0x52 imm=0xCAFEBABE.
/// Pin: 48b8bebafeca0000000049898790020000c3 (18B).
fn set_large_slot_check() -> IsaResult<()> {
    const SLOT: u16 = 0x52;
    const IMM: u64 = 0xCAFEBABE;
    const HH: u8 = 0x3B;
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
            msg: format!(
                "SET-LARGE slot stub mismatch: got {:02X?} want {:02X?}",
                out.code, want
            ),
        });
    }
    Ok(())
}

/// body-extend-016 / parallel-batch-10 H_54: 0x69 ORV dst=0x52 src=0x51.
/// Pin: 498b8790020000498b8f880200004809c849898790020000c3 (25B).
fn orv_h52_slot_check() -> IsaResult<()> {
    const DST: u16 = 0x52; const SRC: u16 = 0x51; const HH: u8 = 0x3C;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x69, &[DST as u64, SRC as u64], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_orv(DST, SRC)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 25 { return Err(IsaError::ParseError { line: 0, msg: format!("ORV-H52 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subv_h52_slot_check() -> IsaResult<()> {
    const DST: u16 = 0x52; const SRC: u16 = 0x51; const HH: u8 = 0x3D;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x6A, &[DST as u64, SRC as u64], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_subv(DST, SRC)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 25 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBV-H52 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn imul_swap_slot_check() -> IsaResult<()> {
    const DST: u16 = 0x51; const SRC: u16 = 0x50; const HH: u8 = 0x3E;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x63, &[DST as u64, SRC as u64], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_imul(DST, SRC)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("IMUL-SWAP mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn imul_h52_slot_check() -> IsaResult<()> {
    const DST: u16 = 0x52; const SRC: u16 = 0x51; const HH: u8 = 0x3F;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x63, &[DST as u64, SRC as u64], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_imul(DST, SRC)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("IMUL-H52 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn cmp_swap_slot_check() -> IsaResult<()> {
    const A: u16 = 0x51; const B: u16 = 0x50; const HH: u8 = 0x40;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x65, &[A as u64, B as u64], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_cmp(A, B)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 18 { return Err(IsaError::ParseError { line: 0, msg: format!("CMP-SWAP mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn get_h52_slot_check() -> IsaResult<()> {
    const DST: u16 = 0x52; const SRC: u16 = 0x50; const HH: u8 = 0x41;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x60, &[DST as u64, SRC as u64], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_get(DST, SRC)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 15 { return Err(IsaError::ParseError { line: 0, msg: format!("GET-H52 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn set_deadbeef_slot_check() -> IsaResult<()> {
    const SLOT: u16 = 0x51; const IMM: u64 = 0xDEADBEEF; const HH: u8 = 0x42;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x30, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_set(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 18 { return Err(IsaError::ParseError { line: 0, msg: format!("SET-DEADBEEF mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_dst51_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x51; const SS: u16 = 0x60; const OO: u64 = 8; const HH: u8 = 0x43;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 23] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x83, 0xc0, 0x08,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x88, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 23 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-DST51 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}

fn inc_h51_slot_check() -> IsaResult<()> {
    const SLOT: u16 = 0x51; const HH: u8 = 0x44;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x66, &[SLOT as u64], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_inc(SLOT)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 18 { return Err(IsaError::ParseError { line: 0, msg: format!("INC-H51 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn dec_h51_slot_check() -> IsaResult<()> {
    const SLOT: u16 = 0x51; const HH: u8 = 0x45;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x67, &[SLOT as u64], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_dec(SLOT)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 18 { return Err(IsaError::ParseError { line: 0, msg: format!("DEC-H51 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h51_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0x07; const HH: u8 = 0x46;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 19 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H51 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn cmp_h52_slot_check() -> IsaResult<()> {
    const A: u16 = 0x52; const B: u16 = 0x51; const HH: u8 = 0x47;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x65, &[A as u64, B as u64], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_cmp(A, B)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 18 { return Err(IsaError::ParseError { line: 0, msg: format!("CMP-H52 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addv_5052_slot_check() -> IsaResult<()> {
    const DST: u16 = 0x50; const SRC: u16 = 0x52; const HH: u8 = 0x48;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x68, &[DST as u64, SRC as u64], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_addv(DST, SRC)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 25 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDV-5052 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn get_5150_slot_check() -> IsaResult<()> {
    const DST: u16 = 0x51; const SRC: u16 = 0x50; const HH: u8 = 0x49;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x60, &[DST as u64, SRC as u64], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_get(DST, SRC)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 15 { return Err(IsaError::ParseError { line: 0, msg: format!("GET-5150 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn set_12345678_slot_check() -> IsaResult<()> {
    const SLOT: u16 = 0x50; const IMM: u64 = 0x12345678; const HH: u8 = 0x4A;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x30, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_set(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 18 { return Err(IsaError::ParseError { line: 0, msg: format!("SET-12345678 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_dst52_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x52; const SS: u16 = 0x60; const OO: u64 = 8; const HH: u8 = 0x4B;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 23] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x83, 0xc0, 0x08,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x90, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 23 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-DST52 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}

fn subimm_h51_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0x03; const HH: u8 = 0x4C;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 19 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H51 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn dec_h52_slot_check() -> IsaResult<()> {
    const SLOT: u16 = 0x52; const HH: u8 = 0x4D;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x67, &[SLOT as u64], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_dec(SLOT)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 18 { return Err(IsaError::ParseError { line: 0, msg: format!("DEC-H52 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn inc_h52_slot_check() -> IsaResult<()> {
    const SLOT: u16 = 0x52; const HH: u8 = 0x4E;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x66, &[SLOT as u64], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_inc(SLOT)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 18 { return Err(IsaError::ParseError { line: 0, msg: format!("INC-H52 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn orv_5052_slot_check() -> IsaResult<()> {
    const DST: u16 = 0x50; const SRC: u16 = 0x52; const HH: u8 = 0x4F;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x69, &[DST as u64, SRC as u64], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_orv(DST, SRC)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 25 { return Err(IsaError::ParseError { line: 0, msg: format!("ORV-5052 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subv_5052_slot_check() -> IsaResult<()> {
    const DST: u16 = 0x50; const SRC: u16 = 0x52; const HH: u8 = 0x50;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x6A, &[DST as u64, SRC as u64], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_subv(DST, SRC)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 25 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBV-5052 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn get_5251_slot_check() -> IsaResult<()> {
    const DST: u16 = 0x52; const SRC: u16 = 0x51; const HH: u8 = 0x51;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x60, &[DST as u64, SRC as u64], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_get(DST, SRC)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 15 { return Err(IsaError::ParseError { line: 0, msg: format!("GET-5251 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn set_f00dbabe_slot_check() -> IsaResult<()> {
    const SLOT: u16 = 0x50; const IMM: u64 = 0xF00DBABE; const HH: u8 = 0x52;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x30, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_set(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 18 { return Err(IsaError::ParseError { line: 0, msg: format!("SET-F00DBABE mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn cmp_5250_slot_check() -> IsaResult<()> {
    const A: u16 = 0x52; const B: u16 = 0x50; const HH: u8 = 0x53;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x65, &[A as u64, B as u64], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_cmp(A, B)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 18 { return Err(IsaError::ParseError { line: 0, msg: format!("CMP-5250 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}

fn addimm_h52_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0x07; const HH: u8 = 0x54;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 19 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H52 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h52_03_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0x03; const HH: u8 = 0x55;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 19 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H52-03 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h51_0a_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0x0A; const HH: u8 = 0x56;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 19 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H51-0A mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h50_05_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0x05; const HH: u8 = 0x57;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 19 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H50-05 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn orv_5250_slot_check() -> IsaResult<()> {
    const DST: u16 = 0x52; const SRC: u16 = 0x50; const HH: u8 = 0x58;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x69, &[DST as u64, SRC as u64], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_orv(DST, SRC)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 25 { return Err(IsaError::ParseError { line: 0, msg: format!("ORV-5250 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subv_5250_slot_check() -> IsaResult<()> {
    const DST: u16 = 0x52; const SRC: u16 = 0x50; const HH: u8 = 0x59;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x6A, &[DST as u64, SRC as u64], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_subv(DST, SRC)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 25 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBV-5250 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addv_5152_slot_check() -> IsaResult<()> {
    const DST: u16 = 0x51; const SRC: u16 = 0x52; const HH: u8 = 0x5A;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x68, &[DST as u64, SRC as u64], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_addv(DST, SRC)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 25 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDV-5152 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn imul_5052_slot_check() -> IsaResult<()> {
    const DST: u16 = 0x50; const SRC: u16 = 0x52; const HH: u8 = 0x5B;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x63, &[DST as u64, SRC as u64], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_imul(DST, SRC)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("IMUL-5052 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}

fn set_feedface_slot_check() -> IsaResult<()> {
    const SLOT: u16 = 0x52; const IMM: u64 = 0xFEEDFACE; const HH: u8 = 0x5C;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x30, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_set(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 18 { return Err(IsaError::ParseError { line: 0, msg: format!("SET-FEEDFACE mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn set_aabbccdd_slot_check() -> IsaResult<()> {
    const SLOT: u16 = 0x51; const IMM: u64 = 0xAABBCCDD; const HH: u8 = 0x5D;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x30, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_set(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 18 { return Err(IsaError::ParseError { line: 0, msg: format!("SET-AABBCCDD mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn get_5052_slot_check() -> IsaResult<()> {
    const DST: u16 = 0x50; const SRC: u16 = 0x52; const HH: u8 = 0x5E;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x60, &[DST as u64, SRC as u64], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_get(DST, SRC)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 15 { return Err(IsaError::ParseError { line: 0, msg: format!("GET-5052 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn cmp_5052_slot_check() -> IsaResult<()> {
    const A: u16 = 0x50; const B: u16 = 0x52; const HH: u8 = 0x5F;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x65, &[A as u64, B as u64], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_cmp(A, B)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 18 { return Err(IsaError::ParseError { line: 0, msg: format!("CMP-5052 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5160_10_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x51; const SS: u16 = 0x60; const OO: u64 = 0x10; const HH: u8 = 0x60;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 23] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x83, 0xc0, 0x10,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x88, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 23 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5160-10 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn imul_5250_slot_check() -> IsaResult<()> {
    const DST: u16 = 0x52; const SRC: u16 = 0x50; const HH: u8 = 0x61;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x63, &[DST as u64, SRC as u64], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_imul(DST, SRC)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("IMUL-5250 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn orv_5152_slot_check() -> IsaResult<()> {
    const DST: u16 = 0x51; const SRC: u16 = 0x52; const HH: u8 = 0x62;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x69, &[DST as u64, SRC as u64], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_orv(DST, SRC)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 25 { return Err(IsaError::ParseError { line: 0, msg: format!("ORV-5152 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h50_0f_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0x0F; const HH: u8 = 0x63;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 19 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H50-0F mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn set_beefcafe_slot_check() -> IsaResult<()> {
    const SLOT: u16 = 0x50; const IMM: u64 = 0xBEEFCAFE; const HH: u8 = 0x64;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x30, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_set(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 18 { return Err(IsaError::ParseError { line: 0, msg: format!("SET-BEEFCAFE mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn set_11111111_slot_check() -> IsaResult<()> {
    const SLOT: u16 = 0x52; const IMM: u64 = 0x11111111; const HH: u8 = 0x65;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x30, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_set(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 18 { return Err(IsaError::ParseError { line: 0, msg: format!("SET-11111111 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h50_08_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0x08; const HH: u8 = 0x66;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 19 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H50-08 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h52_0a_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0x0A; const HH: u8 = 0x67;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 19 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H52-0A mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5260_10_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x52; const SS: u16 = 0x60; const OO: u64 = 0x10; const HH: u8 = 0x68;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 23] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x83, 0xc0, 0x10,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x90, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 23 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5260-10 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5060_18_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x50; const SS: u16 = 0x60; const OO: u64 = 0x18; const HH: u8 = 0x69;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 23] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x83, 0xc0, 0x18,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x80, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 23 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5060-18 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subv_5152_slot_check() -> IsaResult<()> {
    const DST: u16 = 0x51; const SRC: u16 = 0x52; const HH: u8 = 0x6A;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x6A, &[DST as u64, SRC as u64], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_subv(DST, SRC)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 25 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBV-5152 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addv_5250_slot_check() -> IsaResult<()> {
    const DST: u16 = 0x52; const SRC: u16 = 0x50; const HH: u8 = 0x6B;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x68, &[DST as u64, SRC as u64], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_addv(DST, SRC)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 25 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDV-5250 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn cmp_5152_slot_check() -> IsaResult<()> {
    const A: u16 = 0x51; const B: u16 = 0x52; const HH: u8 = 0x6C;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x65, &[A as u64, B as u64], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_cmp(A, B)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 18 { return Err(IsaError::ParseError { line: 0, msg: format!("CMP-5152 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5160_18_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x51; const SS: u16 = 0x60; const OO: u64 = 0x18; const HH: u8 = 0x6D;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 23] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x83, 0xc0, 0x18,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x88, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 23 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5160-18 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5260_18_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x52; const SS: u16 = 0x60; const OO: u64 = 0x18; const HH: u8 = 0x6E;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 23] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x83, 0xc0, 0x18,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x90, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 23 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5260-18 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn set_c0ffee00_slot_check() -> IsaResult<()> {
    const SLOT: u16 = 0x51; const IMM: u64 = 0xC0FFEE00; const HH: u8 = 0x6F;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x30, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_set(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 18 { return Err(IsaError::ParseError { line: 0, msg: format!("SET-C0FFEE00 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h52_08_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0x08; const HH: u8 = 0x70;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 19 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H52-08 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn imul_5152_slot_check() -> IsaResult<()> {
    const DST: u16 = 0x51; const SRC: u16 = 0x52; const HH: u8 = 0x71;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x63, &[DST as u64, SRC as u64], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_imul(DST, SRC)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("IMUL-5152 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h50_14_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0x14; const HH: u8 = 0x72;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 19 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H50-14 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn set_50_c0ffee00_slot_check() -> IsaResult<()> {
    const SLOT: u16 = 0x50; const IMM: u64 = 0xC0FFEE00; const HH: u8 = 0x73;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x30, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_set(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 18 { return Err(IsaError::ParseError { line: 0, msg: format!("SET-50-C0FFEE00 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn set_52_deadf00d_slot_check() -> IsaResult<()> {
    const SLOT: u16 = 0x52; const IMM: u64 = 0xDEADF00D; const HH: u8 = 0x74;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x30, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_set(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 18 { return Err(IsaError::ParseError { line: 0, msg: format!("SET-52-DEADF00D mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h51_14_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0x14; const HH: u8 = 0x75;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 19 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H51-14 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h51_0a_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0x0A; const HH: u8 = 0x76;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 19 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H51-0A mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5160_20_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x51; const SS: u16 = 0x60; const OO: u64 = 0x20; const HH: u8 = 0x77;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 23] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x83, 0xc0, 0x20,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x88, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 23 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5160-20 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5260_20_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x52; const SS: u16 = 0x60; const OO: u64 = 0x20; const HH: u8 = 0x78;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 23] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x83, 0xc0, 0x20,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x90, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 23 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5260-20 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h52_14_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0x14; const HH: u8 = 0x79;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 19 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H52-14 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h50_0a_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0x0A; const HH: u8 = 0x7A;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 19 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H50-0A mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn set_51_deadf00d_slot_check() -> IsaResult<()> {
    const SLOT: u16 = 0x51; const IMM: u64 = 0xDEADF00D; const HH: u8 = 0x7B;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x30, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_set(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 18 { return Err(IsaError::ParseError { line: 0, msg: format!("SET-51-DEADF00D mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn set_50_facefeed_slot_check() -> IsaResult<()> {
    const SLOT: u16 = 0x50; const IMM: u64 = 0xFACEFEED; const HH: u8 = 0x7C;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x30, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_set(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 18 { return Err(IsaError::ParseError { line: 0, msg: format!("SET-50-FACEFEED mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h51_1e_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0x1E; const HH: u8 = 0x7D;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 19 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H51-1E mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h52_0a_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0x0A; const HH: u8 = 0x7E;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 19 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H52-0A mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5060_28_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x50; const SS: u16 = 0x60; const OO: u64 = 0x28; const HH: u8 = 0x7F;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 23] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x83, 0xc0, 0x28,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x80, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 23 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5060-28 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn set_52_facefeed_slot_check() -> IsaResult<()> {
    const SLOT: u16 = 0x52; const IMM: u64 = 0xFACEFEED; const HH: u8 = 0x80;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x30, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_set(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 18 { return Err(IsaError::ParseError { line: 0, msg: format!("SET-52-FACEFEED mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h50_1e_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0x1E; const HH: u8 = 0x81;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 19 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H50-1E mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h51_05_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0x05; const HH: u8 = 0x82;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 19 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H51-05 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5160_28_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x51; const SS: u16 = 0x60; const OO: u64 = 0x28; const HH: u8 = 0x83;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 23] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x83, 0xc0, 0x28,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x88, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 23 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5160-28 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5260_28_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x52; const SS: u16 = 0x60; const OO: u64 = 0x28; const HH: u8 = 0x84;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 23] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x83, 0xc0, 0x28,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x90, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 23 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5260-28 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5060_30_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x50; const SS: u16 = 0x60; const OO: u64 = 0x30; const HH: u8 = 0x85;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 23] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x83, 0xc0, 0x30,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x80, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 23 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5060-30 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn set_51_baadf00d_slot_check() -> IsaResult<()> {
    const SLOT: u16 = 0x51; const IMM: u64 = 0xBAADF00D; const HH: u8 = 0x86;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x30, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_set(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 18 { return Err(IsaError::ParseError { line: 0, msg: format!("SET-51-BAADF00D mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h52_1e_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0x1E; const HH: u8 = 0x87;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 19 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H52-1E mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h50_14_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0x14; const HH: u8 = 0x88;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 19 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H50-14 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5160_30_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x51; const SS: u16 = 0x60; const OO: u64 = 0x30; const HH: u8 = 0x89;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 23] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x83, 0xc0, 0x30,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x88, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 23 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5160-30 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn set_52_baadf00d_slot_check() -> IsaResult<()> {
    const SLOT: u16 = 0x52; const IMM: u64 = 0xBAADF00D; const HH: u8 = 0x8A;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x30, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_set(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 18 { return Err(IsaError::ParseError { line: 0, msg: format!("SET-52-BAADF00D mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h52_14_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0x14; const HH: u8 = 0x8B;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 19 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H52-14 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5260_30_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x52; const SS: u16 = 0x60; const OO: u64 = 0x30; const HH: u8 = 0x8C;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 23] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x83, 0xc0, 0x30,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x90, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 23 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5260-30 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5060_38_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x50; const SS: u16 = 0x60; const OO: u64 = 0x38; const HH: u8 = 0x8D;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 23] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x83, 0xc0, 0x38,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x80, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 23 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5060-38 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn set_50_0badf00d_slot_check() -> IsaResult<()> {
    const SLOT: u16 = 0x50; const IMM: u64 = 0x0BADF00D; const HH: u8 = 0x8E;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x30, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_set(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 18 { return Err(IsaError::ParseError { line: 0, msg: format!("SET-50-0BADF00D mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h51_28_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0x28; const HH: u8 = 0x8F;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 19 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H51-28 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h51_1e_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0x1E; const HH: u8 = 0x90;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 19 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H51-1E mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5160_38_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x51; const SS: u16 = 0x60; const OO: u64 = 0x38; const HH: u8 = 0x91;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 23] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x83, 0xc0, 0x38,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x88, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 23 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5160-38 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h50_28_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0x28; const HH: u8 = 0x92;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 19 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H50-28 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h52_1e_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0x1E; const HH: u8 = 0x93;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 19 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H52-1E mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5260_38_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x52; const SS: u16 = 0x60; const OO: u64 = 0x38; const HH: u8 = 0x94;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 23] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x83, 0xc0, 0x38,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x90, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 23 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5260-38 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn set_51_feedc0de_slot_check() -> IsaResult<()> {
    const SLOT: u16 = 0x51; const IMM: u64 = 0xFEEDC0DE; const HH: u8 = 0x95;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x30, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_set(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 18 { return Err(IsaError::ParseError { line: 0, msg: format!("SET-51-FEEDC0DE mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h52_28_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0x28; const HH: u8 = 0x96;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 19 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H52-28 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h50_1e_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0x1E; const HH: u8 = 0x97;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 19 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H50-1E mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5160_40_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x51; const SS: u16 = 0x60; const OO: u64 = 0x40; const HH: u8 = 0x98;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 23] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x83, 0xc0, 0x40,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x88, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 23 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5160-40 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5260_40_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x52; const SS: u16 = 0x60; const OO: u64 = 0x40; const HH: u8 = 0x99;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 23] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x83, 0xc0, 0x40,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x90, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 23 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5260-40 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn set_52_feedc0de_slot_check() -> IsaResult<()> {
    const SLOT: u16 = 0x52; const IMM: u64 = 0xFEEDC0DE; const HH: u8 = 0x9A;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x30, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_set(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 18 { return Err(IsaError::ParseError { line: 0, msg: format!("SET-52-FEEDC0DE mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h51_28_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0x28; const HH: u8 = 0x9B;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 19 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H51-28 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn set_50_feedc0de_slot_check() -> IsaResult<()> {
    const SLOT: u16 = 0x50; const IMM: u64 = 0xFEEDC0DE; const HH: u8 = 0x9C;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x30, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_set(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 18 { return Err(IsaError::ParseError { line: 0, msg: format!("SET-50-FEEDC0DE mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h50_32_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0x32; const HH: u8 = 0x9D;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 19 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H50-32 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h52_28_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0x28; const HH: u8 = 0x9E;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 19 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H52-28 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5060_48_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x50; const SS: u16 = 0x60; const OO: u64 = 0x48; const HH: u8 = 0x9F;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 23] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x83, 0xc0, 0x48,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x80, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 23 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5060-48 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5160_48_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x51; const SS: u16 = 0x60; const OO: u64 = 0x48; const HH: u8 = 0xA0;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 23] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x83, 0xc0, 0x48,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x88, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 23 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5160-48 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5260_48_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x52; const SS: u16 = 0x60; const OO: u64 = 0x48; const HH: u8 = 0xA1;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 23] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x83, 0xc0, 0x48,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x90, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 23 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5260-48 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h51_32_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0x32; const HH: u8 = 0xA2;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 19 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H51-32 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h50_28_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0x28; const HH: u8 = 0xA3;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 19 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H50-28 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5160_50_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x51; const SS: u16 = 0x60; const OO: u64 = 0x50; const HH: u8 = 0xA4;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 23] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x83, 0xc0, 0x50,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x88, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 23 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5160-50 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5260_50_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x52; const SS: u16 = 0x60; const OO: u64 = 0x50; const HH: u8 = 0xA5;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 23] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x83, 0xc0, 0x50,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x90, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 23 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5260-50 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn set_51_cafef00d_slot_check() -> IsaResult<()> {
    const SLOT: u16 = 0x51; const IMM: u64 = 0xCAFEF00D; const HH: u8 = 0xA6;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x30, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_set(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 18 { return Err(IsaError::ParseError { line: 0, msg: format!("SET-51-CAFEF00D mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h52_32_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0x32; const HH: u8 = 0xA7;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 19 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H52-32 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h51_32_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0x32; const HH: u8 = 0xA8;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 19 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H51-32 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn set_50_cafef00d_slot_check() -> IsaResult<()> {
    const SLOT: u16 = 0x50; const IMM: u64 = 0xCAFEF00D; const HH: u8 = 0xA9;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x30, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_set(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 18 { return Err(IsaError::ParseError { line: 0, msg: format!("SET-50-CAFEF00D mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h52_32_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0x32; const HH: u8 = 0xAA;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 19 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H52-32 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h50_3c_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0x3C; const HH: u8 = 0xAB;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 19 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H50-3C mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn set_52_cafef00d_slot_check() -> IsaResult<()> {
    const SLOT: u16 = 0x52; const IMM: u64 = 0xCAFEF00D; const HH: u8 = 0xAC;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x30, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_set(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 18 { return Err(IsaError::ParseError { line: 0, msg: format!("SET-52-CAFEF00D mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5060_58_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x50; const SS: u16 = 0x60; const OO: u64 = 0x58; const HH: u8 = 0xAD;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 23] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x83, 0xc0, 0x58,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x80, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 23 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5060-58 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h51_3c_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0x3C; const HH: u8 = 0xAE;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 19 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H51-3C mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h50_3c_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0x3C; const HH: u8 = 0xAF;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 19 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H50-3C mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5260_58_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x52; const SS: u16 = 0x60; const OO: u64 = 0x58; const HH: u8 = 0xB0;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 23] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x83, 0xc0, 0x58,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x90, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 23 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5260-58 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5160_58_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x51; const SS: u16 = 0x60; const OO: u64 = 0x58; const HH: u8 = 0xB1;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 23] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x83, 0xc0, 0x58,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x88, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 23 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5160-58 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h52_3c_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0x3C; const HH: u8 = 0xB2;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 19 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H52-3C mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h51_3c_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0x3C; const HH: u8 = 0xB3;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 19 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H51-3C mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn set_50_deadc0de_slot_check() -> IsaResult<()> {
    const SLOT: u16 = 0x50; const IMM: u64 = 0xDEADC0DE; const HH: u8 = 0xB4;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x30, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_set(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 18 { return Err(IsaError::ParseError { line: 0, msg: format!("SET-50-DEADC0DE mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5160_60_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x51; const SS: u16 = 0x60; const OO: u64 = 0x60; const HH: u8 = 0xB5;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 23] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x83, 0xc0, 0x60,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x88, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 23 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5160-60 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5260_60_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x52; const SS: u16 = 0x60; const OO: u64 = 0x60; const HH: u8 = 0xB6;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 23] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x83, 0xc0, 0x60,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x90, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 23 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5260-60 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h50_40_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0x40; const HH: u8 = 0xB7;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 19 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H50-40 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h51_40_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0x40; const HH: u8 = 0xB8;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 19 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H51-40 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h52_40_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0x40; const HH: u8 = 0xB9;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 19 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H52-40 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h52_3c_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0x3C; const HH: u8 = 0xBA;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 19 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H52-3C mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn set_51_deadc0de_slot_check() -> IsaResult<()> {
    const SLOT: u16 = 0x51; const IMM: u64 = 0xDEADC0DE; const HH: u8 = 0xBB;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x30, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_set(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 18 { return Err(IsaError::ParseError { line: 0, msg: format!("SET-51-DEADC0DE mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn set_52_deadc0de_slot_check() -> IsaResult<()> {
    const SLOT: u16 = 0x52; const IMM: u64 = 0xDEADC0DE; const HH: u8 = 0xBC;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x30, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_set(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 18 { return Err(IsaError::ParseError { line: 0, msg: format!("SET-52-DEADC0DE mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5060_68_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x50; const SS: u16 = 0x60; const OO: u64 = 0x68; const HH: u8 = 0xBD;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 23] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x83, 0xc0, 0x68,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x80, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 23 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5060-68 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5160_68_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x51; const SS: u16 = 0x60; const OO: u64 = 0x68; const HH: u8 = 0xBE;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 23] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x83, 0xc0, 0x68,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x88, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 23 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5160-68 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5260_68_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x52; const SS: u16 = 0x60; const OO: u64 = 0x68; const HH: u8 = 0xBF;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 23] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x83, 0xc0, 0x68,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x90, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 23 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5260-68 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h50_48_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0x48; const HH: u8 = 0xC0;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 19 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H50-48 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h51_48_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0x48; const HH: u8 = 0xC1;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 19 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H51-48 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h50_40_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0x40; const HH: u8 = 0xC2;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 19 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H50-40 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h51_40_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0x40; const HH: u8 = 0xC3;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 19 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H51-40 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h52_48_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0x48; const HH: u8 = 0xC4;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 19 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H52-48 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h52_40_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0x40; const HH: u8 = 0xC5;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 19 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H52-40 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5160_70_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x51; const SS: u16 = 0x60; const OO: u64 = 0x70; const HH: u8 = 0xC6;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 23] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x83, 0xc0, 0x70,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x88, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 23 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5160-70 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5260_70_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x52; const SS: u16 = 0x60; const OO: u64 = 0x70; const HH: u8 = 0xC7;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 23] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x83, 0xc0, 0x70,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x90, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 23 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5260-70 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn set_50_c0dec0de_slot_check() -> IsaResult<()> {
    const SLOT: u16 = 0x50; const IMM: u64 = 0xC0DEC0DE; const HH: u8 = 0xC8;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x30, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_set(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 18 { return Err(IsaError::ParseError { line: 0, msg: format!("SET-50-C0DEC0DE mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h50_50_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0x50; const HH: u8 = 0xC9;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 19 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H50-50 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h51_48_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0x48; const HH: u8 = 0xCA;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 19 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H51-48 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h51_50_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0x50; const HH: u8 = 0xCB;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 19 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H51-50 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h52_50_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0x50; const HH: u8 = 0xCC;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 19 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H52-50 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h50_48_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0x48; const HH: u8 = 0xCD;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 19 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H50-48 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h52_48_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0x48; const HH: u8 = 0xCE;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 19 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H52-48 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5060_78_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x50; const SS: u16 = 0x60; const OO: u64 = 0x78; const HH: u8 = 0xCF;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 23] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x83, 0xc0, 0x78,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x80, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 23 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5060-78 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn set_51_c0dec0de_slot_check() -> IsaResult<()> {
    const SLOT: u16 = 0x51; const IMM: u64 = 0xC0DEC0DE; const HH: u8 = 0xD0;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x30, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_set(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 18 { return Err(IsaError::ParseError { line: 0, msg: format!("SET-51-C0DEC0DE mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h50_58_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0x58; const HH: u8 = 0xD1;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 19 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H50-58 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h51_50_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0x50; const HH: u8 = 0xD2;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 19 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H51-50 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5160_78_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x51; const SS: u16 = 0x60; const OO: u64 = 0x78; const HH: u8 = 0xD3;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 23] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x83, 0xc0, 0x78,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x88, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 23 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5160-78 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h51_58_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0x58; const HH: u8 = 0xD4;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 19 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H51-58 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h52_58_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0x58; const HH: u8 = 0xD5;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 19 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H52-58 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h50_50_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0x50; const HH: u8 = 0xD6;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 19 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H50-50 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h52_50_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0x50; const HH: u8 = 0xD7;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 19 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H52-50 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5260_78_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x52; const SS: u16 = 0x60; const OO: u64 = 0x78; const HH: u8 = 0xD8;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 23] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x83, 0xc0, 0x78,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x90, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 23 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5260-78 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn set_52_c0dec0de_slot_check() -> IsaResult<()> {
    const SLOT: u16 = 0x52; const IMM: u64 = 0xC0DEC0DE; const HH: u8 = 0xD9;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x30, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_set(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 18 { return Err(IsaError::ParseError { line: 0, msg: format!("SET-52-C0DEC0DE mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h50_60_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0x60; const HH: u8 = 0xDA;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 19 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H50-60 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5060_80_slot_check() -> IsaResult<()> {
    // oo=0x80 → imm32 path (48 81 c0 …), not imm8 — pin 26B
    const DD: u16 = 0x50; const SS: u16 = 0x60; const OO: u64 = 0x80; const HH: u8 = 0xDB;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0x80, 0x00, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x80, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5060-80 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h51_60_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0x60; const HH: u8 = 0xDC;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 19 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H51-60 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h52_60_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0x60; const HH: u8 = 0xDD;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 19 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H52-60 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h50_58_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0x58; const HH: u8 = 0xDE;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 19 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H50-58 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h51_58_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0x58; const HH: u8 = 0xDF;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 19 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H51-58 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5160_80_slot_check() -> IsaResult<()> {
    // oo=0x80 → imm32 path (48 81 c0 …), not imm8 — pin 26B
    const DD: u16 = 0x51; const SS: u16 = 0x60; const OO: u64 = 0x80; const HH: u8 = 0xE0;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0x80, 0x00, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x88, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5160-80 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5260_80_slot_check() -> IsaResult<()> {
    // oo=0x80 → imm32 path (48 81 c0 …), not imm8 — pin 26B
    const DD: u16 = 0x52; const SS: u16 = 0x60; const OO: u64 = 0x80; const HH: u8 = 0xE1;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0x80, 0x00, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x90, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5260-80 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h52_58_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0x58; const HH: u8 = 0xE2;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 19 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H52-58 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h50_68_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0x68; const HH: u8 = 0xE3;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 19 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H50-68 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h51_68_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0x68; const HH: u8 = 0xE4;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 19 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H51-68 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h52_68_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0x68; const HH: u8 = 0xE5;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 19 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H52-68 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h50_60_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0x60; const HH: u8 = 0xE6;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 19 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H50-60 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h51_60_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0x60; const HH: u8 = 0xE7;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 19 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H51-60 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h52_60_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0x60; const HH: u8 = 0xE8;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 19 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H52-60 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5060_88_slot_check() -> IsaResult<()> {
    // oo=0x88 → imm32 path (48 81 c0 …), not imm8 — pin 26B
    const DD: u16 = 0x50; const SS: u16 = 0x60; const OO: u64 = 0x88; const HH: u8 = 0xE9;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0x88, 0x00, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x80, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5060-88 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5160_88_slot_check() -> IsaResult<()> {
    // oo=0x88 → imm32 path (48 81 c0 …), not imm8 — pin 26B
    const DD: u16 = 0x51; const SS: u16 = 0x60; const OO: u64 = 0x88; const HH: u8 = 0xEA;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0x88, 0x00, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x88, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5160-88 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5260_88_slot_check() -> IsaResult<()> {
    // oo=0x88 → imm32 path (48 81 c0 …), not imm8 — pin 26B
    const DD: u16 = 0x52; const SS: u16 = 0x60; const OO: u64 = 0x88; const HH: u8 = 0xEB;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0x88, 0x00, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x90, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5260-88 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h50_70_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0x70; const HH: u8 = 0xEC;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 19 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H50-70 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h51_70_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0x70; const HH: u8 = 0xED;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 19 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H51-70 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h52_70_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0x70; const HH: u8 = 0xEE;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 19 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H52-70 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h50_68_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0x68; const HH: u8 = 0xEF;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 19 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H50-68 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h51_68_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0x68; const HH: u8 = 0xF0;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 19 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H51-68 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h52_68_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0x68; const HH: u8 = 0xF1;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 19 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H52-68 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5060_90_slot_check() -> IsaResult<()> {
    // oo=0x90 → imm32 path (48 81 c0 …), not imm8 — pin 26B
    const DD: u16 = 0x50; const SS: u16 = 0x60; const OO: u64 = 0x90; const HH: u8 = 0xF2;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0x90, 0x00, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x80, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5060-90 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5160_90_slot_check() -> IsaResult<()> {
    // oo=0x90 → imm32 path (48 81 c0 …), not imm8 — pin 26B
    const DD: u16 = 0x51; const SS: u16 = 0x60; const OO: u64 = 0x90; const HH: u8 = 0xF3;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0x90, 0x00, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x88, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5160-90 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5260_90_slot_check() -> IsaResult<()> {
    // oo=0x90 → imm32 path (48 81 c0 …), not imm8 — pin 26B
    const DD: u16 = 0x52; const SS: u16 = 0x60; const OO: u64 = 0x90; const HH: u8 = 0xF4;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0x90, 0x00, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x90, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5260-90 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h50_70_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0x70; const HH: u8 = 0xF5;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 19 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H50-70 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h51_70_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0x70; const HH: u8 = 0xF6;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 19 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H51-70 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h52_70_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0x70; const HH: u8 = 0xF7;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 19 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H52-70 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h50_78_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0x78; const HH: u8 = 0xF8;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 19 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H50-78 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h51_78_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0x78; const HH: u8 = 0xF9;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 19 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H51-78 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h52_78_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0x78; const HH: u8 = 0xFA;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 19 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H52-78 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5060_98_slot_check() -> IsaResult<()> {
    // oo=0x98 → imm32 path (48 81 c0 …), not imm8 — pin 26B
    const DD: u16 = 0x50; const SS: u16 = 0x60; const OO: u64 = 0x98; const HH: u8 = 0xFB;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0x98, 0x00, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x80, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5060-98 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5160_98_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x51; const SS: u16 = 0x60; const OO: u64 = 0x98; const HH: u16 = 0xFC;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0x98, 0x00, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x88, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5160-98 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5260_98_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x52; const SS: u16 = 0x60; const OO: u64 = 0x98; const HH: u16 = 0xFD;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0x98, 0x00, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x90, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5260-98 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h50_78_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0x78; const HH: u16 = 0xFE;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 19 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H50-78 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h51_78_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0x78; const HH: u16 = 0xFF;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 19 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H51-78 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h52_78_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0x78; const HH: u16 = 0x100;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 19 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H52-78 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h50_80_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0x80; const HH: u16 = 0x101;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H50-80 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h51_80_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0x80; const HH: u16 = 0x102;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H51-80 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h52_80_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0x80; const HH: u16 = 0x103;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H52-80 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5060_a0_slot_check() -> IsaResult<()> {
    // oo=0xA0 → imm32 path (48 81 c0 …), not imm8 — pin 26B
    const DD: u16 = 0x50; const SS: u16 = 0x60; const OO: u64 = 0xA0; const HH: u16 = 0x104;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0xa0, 0x00, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x80, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5060-A0 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5160_a0_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x51; const SS: u16 = 0x60; const OO: u64 = 0xA0; const HH: u16 = 0x105;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0xa0, 0x00, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x88, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5160-A0 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5260_a0_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x52; const SS: u16 = 0x60; const OO: u64 = 0xA0; const HH: u16 = 0x106;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0xa0, 0x00, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x90, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5260-A0 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h50_80_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0x80; const HH: u16 = 0x107;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H50-80 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h51_80_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0x80; const HH: u16 = 0x108;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H51-80 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h52_80_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0x80; const HH: u16 = 0x109;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H52-80 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h50_88_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0x88; const HH: u16 = 0x10A;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H50-88 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h51_88_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0x88; const HH: u16 = 0x10B;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H51-88 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h52_88_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0x88; const HH: u16 = 0x10C;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H52-88 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h50_88_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0x88; const HH: u16 = 0x10D;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H50-88 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h51_88_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0x88; const HH: u16 = 0x10E;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H51-88 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h52_88_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0x88; const HH: u16 = 0x10F;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H52-88 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5060_a8_slot_check() -> IsaResult<()> {
    // oo=0xA8 → imm32 path (48 81 c0 …), not imm8 — pin 26B
    const DD: u16 = 0x50; const SS: u16 = 0x60; const OO: u64 = 0xA8; const HH: u16 = 0x110;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0xa8, 0x00, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x80, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5060-A8 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5160_a8_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x51; const SS: u16 = 0x60; const OO: u64 = 0xA8; const HH: u16 = 0x111;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0xa8, 0x00, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x88, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5160-A8 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5260_a8_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x52; const SS: u16 = 0x60; const OO: u64 = 0xA8; const HH: u16 = 0x112;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0xa8, 0x00, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x90, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5260-A8 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h50_90_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0x90; const HH: u16 = 0x113;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H50-90 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h51_90_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0x90; const HH: u16 = 0x114;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H51-90 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h52_90_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0x90; const HH: u16 = 0x115;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H52-90 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h50_90_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0x90; const HH: u16 = 0x116;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H50-90 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h51_90_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0x90; const HH: u16 = 0x117;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H51-90 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h52_90_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0x90; const HH: u16 = 0x118;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H52-90 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5060_b0_slot_check() -> IsaResult<()> {
    // oo=0xB0 → imm32 path (48 81 c0 …), not imm8 — pin 26B
    const DD: u16 = 0x50; const SS: u16 = 0x60; const OO: u64 = 0xB0; const HH: u16 = 0x119;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0xb0, 0x00, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x80, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5060-B0 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5160_b0_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x51; const SS: u16 = 0x60; const OO: u64 = 0xB0; const HH: u16 = 0x11A;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0xb0, 0x00, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x88, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5160-B0 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5260_b0_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x52; const SS: u16 = 0x60; const OO: u64 = 0xB0; const HH: u16 = 0x11B;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0xb0, 0x00, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x90, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5260-B0 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h50_98_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0x98; const HH: u16 = 0x11C;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H50-98 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h51_98_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0x98; const HH: u16 = 0x11D;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H51-98 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h52_98_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0x98; const HH: u16 = 0x11E;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H52-98 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h50_98_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0x98; const HH: u16 = 0x11F;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H50-98 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h51_98_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0x98; const HH: u16 = 0x120;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H51-98 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h52_98_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0x98; const HH: u16 = 0x121;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H52-98 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5060_b8_slot_check() -> IsaResult<()> {
    // oo=0xB8 → imm32 path (48 81 c0 …), not imm8 — pin 26B
    const DD: u16 = 0x50; const SS: u16 = 0x60; const OO: u64 = 0xB8; const HH: u16 = 0x122;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0xb8, 0x00, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x80, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5060-B8 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5160_b8_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x51; const SS: u16 = 0x60; const OO: u64 = 0xB8; const HH: u16 = 0x123;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0xb8, 0x00, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x88, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5160-B8 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5260_b8_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x52; const SS: u16 = 0x60; const OO: u64 = 0xB8; const HH: u16 = 0x124;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0xb8, 0x00, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x90, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5260-B8 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h50_a0_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0xA0; const HH: u16 = 0x125;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H50-A0 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h51_a0_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0xA0; const HH: u16 = 0x126;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H51-A0 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h52_a0_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0xA0; const HH: u16 = 0x127;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H52-A0 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h50_a0_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0xA0; const HH: u16 = 0x128;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H50-A0 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h51_a0_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0xA0; const HH: u16 = 0x129;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H51-A0 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h52_a0_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0xA0; const HH: u16 = 0x12A;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H52-A0 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5060_c0_slot_check() -> IsaResult<()> {
    // oo=0xC0 → imm32 path (48 81 c0 …), not imm8 — pin 26B
    const DD: u16 = 0x50; const SS: u16 = 0x60; const OO: u64 = 0xC0; const HH: u16 = 0x12B;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0xc0, 0x00, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x80, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5060-C0 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5160_c0_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x51; const SS: u16 = 0x60; const OO: u64 = 0xC0; const HH: u16 = 0x12C;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0xc0, 0x00, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x88, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5160-C0 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5260_c0_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x52; const SS: u16 = 0x60; const OO: u64 = 0xC0; const HH: u16 = 0x12D;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0xc0, 0x00, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x90, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5260-C0 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h50_a8_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0xA8; const HH: u16 = 0x12E;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H50-A8 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h51_a8_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0xA8; const HH: u16 = 0x12F;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H51-A8 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h52_a8_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0xA8; const HH: u16 = 0x130;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H52-A8 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h50_a8_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0xA8; const HH: u16 = 0x131;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H50-A8 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h51_a8_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0xA8; const HH: u16 = 0x132;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H51-A8 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h52_a8_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0xA8; const HH: u16 = 0x133;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H52-A8 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5060_c8_slot_check() -> IsaResult<()> {
    // oo=0xC8 → imm32 path (48 81 c0 …), not imm8 — pin 26B
    const DD: u16 = 0x50; const SS: u16 = 0x60; const OO: u64 = 0xC8; const HH: u16 = 0x134;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0xc8, 0x00, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x80, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5060-C8 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5160_c8_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x51; const SS: u16 = 0x60; const OO: u64 = 0xC8; const HH: u16 = 0x135;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0xc8, 0x00, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x88, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5160-C8 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5260_c8_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x52; const SS: u16 = 0x60; const OO: u64 = 0xC8; const HH: u16 = 0x136;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0xc8, 0x00, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x90, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5260-C8 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h50_b0_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0xB0; const HH: u16 = 0x137;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H50-B0 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h51_b0_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0xB0; const HH: u16 = 0x138;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H51-B0 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h52_b0_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0xB0; const HH: u16 = 0x139;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H52-B0 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h50_b0_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0xB0; const HH: u16 = 0x13A;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H50-B0 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h51_b0_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0xB0; const HH: u16 = 0x13B;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H51-B0 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h52_b0_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0xB0; const HH: u16 = 0x13C;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H52-B0 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h50_b8_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0xB8; const HH: u16 = 0x13D;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H50-B8 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h51_b8_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0xB8; const HH: u16 = 0x13E;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H51-B8 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h52_b8_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0xB8; const HH: u16 = 0x13F;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H52-B8 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h50_b8_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0xB8; const HH: u16 = 0x140;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H50-B8 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h51_b8_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0xB8; const HH: u16 = 0x141;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H51-B8 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h52_b8_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0xB8; const HH: u16 = 0x142;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H52-B8 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5060_d0_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x50; const SS: u16 = 0x60; const OO: u64 = 0xD0; const HH: u16 = 0x143;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0xd0, 0x00, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x80, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5060-D0 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5160_d0_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x51; const SS: u16 = 0x60; const OO: u64 = 0xD0; const HH: u16 = 0x144;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0xd0, 0x00, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x88, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5160-D0 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5260_d0_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x52; const SS: u16 = 0x60; const OO: u64 = 0xD0; const HH: u16 = 0x145;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0xd0, 0x00, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x90, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5260-D0 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h50_c0_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0xC0; const HH: u16 = 0x146;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H50-C0 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h51_c0_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0xC0; const HH: u16 = 0x147;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H51-C0 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h52_c0_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0xC0; const HH: u16 = 0x148;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H52-C0 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h50_c0_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0xC0; const HH: u16 = 0x149;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H50-C0 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h51_c0_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0xC0; const HH: u16 = 0x14A;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H51-C0 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h52_c0_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0xC0; const HH: u16 = 0x14B;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H52-C0 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5060_d8_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x50; const SS: u16 = 0x60; const OO: u64 = 0xD8; const HH: u16 = 0x14C;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0xd8, 0x00, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x80, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5060-D8 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5160_d8_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x51; const SS: u16 = 0x60; const OO: u64 = 0xD8; const HH: u16 = 0x14D;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0xd8, 0x00, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x88, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5160-D8 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5260_d8_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x52; const SS: u16 = 0x60; const OO: u64 = 0xD8; const HH: u16 = 0x14E;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0xd8, 0x00, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x90, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5260-D8 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h50_c8_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0xC8; const HH: u16 = 0x14F;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H50-C8 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h51_c8_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0xC8; const HH: u16 = 0x150;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H51-C8 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h52_c8_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0xC8; const HH: u16 = 0x151;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H52-C8 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h50_c8_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0xC8; const HH: u16 = 0x152;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H50-C8 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h51_c8_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0xC8; const HH: u16 = 0x153;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H51-C8 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h52_c8_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0xC8; const HH: u16 = 0x154;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H52-C8 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h50_d0_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0xD0; const HH: u16 = 0x155;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H50-D0 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h51_d0_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0xD0; const HH: u16 = 0x156;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H51-D0 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h52_d0_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0xD0; const HH: u16 = 0x157;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H52-D0 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h50_d0_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0xD0; const HH: u16 = 0x158;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H50-D0 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h51_d0_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0xD0; const HH: u16 = 0x159;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H51-D0 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h52_d0_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0xD0; const HH: u16 = 0x15A;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H52-D0 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5060_e0_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x50; const SS: u16 = 0x60; const OO: u64 = 0xE0; const HH: u16 = 0x15B;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0xe0, 0x00, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x80, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5060-E0 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5160_e0_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x51; const SS: u16 = 0x60; const OO: u64 = 0xE0; const HH: u16 = 0x15C;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0xe0, 0x00, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x88, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5160-E0 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5260_e0_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x52; const SS: u16 = 0x60; const OO: u64 = 0xE0; const HH: u16 = 0x15D;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0xe0, 0x00, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x90, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5260-E0 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h50_d8_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0xD8; const HH: u16 = 0x15E;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H50-D8 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h51_d8_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0xD8; const HH: u16 = 0x15F;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H51-D8 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h52_d8_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0xD8; const HH: u16 = 0x160;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H52-D8 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h50_d8_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0xD8; const HH: u16 = 0x161;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H50-D8 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h51_d8_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0xD8; const HH: u16 = 0x162;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H51-D8 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h52_d8_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0xD8; const HH: u16 = 0x163;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H52-D8 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5060_e8_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x50; const SS: u16 = 0x60; const OO: u64 = 0xE8; const HH: u16 = 0x164;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0xe8, 0x00, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x80, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5060-E8 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5160_e8_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x51; const SS: u16 = 0x60; const OO: u64 = 0xE8; const HH: u16 = 0x165;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0xe8, 0x00, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x88, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5160-E8 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5260_e8_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x52; const SS: u16 = 0x60; const OO: u64 = 0xE8; const HH: u16 = 0x166;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0xe8, 0x00, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x90, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5260-E8 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h50_e0_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0xE0; const HH: u16 = 0x167;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H50-E0 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h51_e0_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0xE0; const HH: u16 = 0x168;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H51-E0 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h52_e0_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0xE0; const HH: u16 = 0x169;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H52-E0 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h50_e0_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0xE0; const HH: u16 = 0x16A;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H50-E0 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h51_e0_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0xE0; const HH: u16 = 0x16B;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H51-E0 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h52_e0_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0xE0; const HH: u16 = 0x16C;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H52-E0 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h50_e8_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0xE8; const HH: u16 = 0x16D;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H50-E8 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h51_e8_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0xE8; const HH: u16 = 0x16E;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H51-E8 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h52_e8_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0xE8; const HH: u16 = 0x16F;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H52-E8 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h50_e8_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0xE8; const HH: u16 = 0x170;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H50-E8 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h51_e8_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0xE8; const HH: u16 = 0x171;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H51-E8 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h52_e8_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0xE8; const HH: u16 = 0x172;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H52-E8 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5060_f0_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x50; const SS: u16 = 0x60; const OO: u64 = 0xF0; const HH: u16 = 0x173;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0xf0, 0x00, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x80, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5060-F0 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5160_f0_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x51; const SS: u16 = 0x60; const OO: u64 = 0xF0; const HH: u16 = 0x174;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0xf0, 0x00, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x88, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5160-F0 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5260_f0_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x52; const SS: u16 = 0x60; const OO: u64 = 0xF0; const HH: u16 = 0x175;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0xf0, 0x00, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x90, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5260-F0 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h50_f0_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0xF0; const HH: u16 = 0x176;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H50-F0 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h51_f0_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0xF0; const HH: u16 = 0x177;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H51-F0 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h52_f0_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0xF0; const HH: u16 = 0x178;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H52-F0 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h50_f0_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0xF0; const HH: u16 = 0x179;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H50-F0 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h51_f0_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0xF0; const HH: u16 = 0x17A;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H51-F0 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h52_f0_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0xF0; const HH: u16 = 0x17B;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H52-F0 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5060_f8_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x50; const SS: u16 = 0x60; const OO: u64 = 0xF8; const HH: u16 = 0x17C;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0xf8, 0x00, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x80, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5060-F8 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5160_f8_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x51; const SS: u16 = 0x60; const OO: u64 = 0xF8; const HH: u16 = 0x17D;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0xf8, 0x00, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x88, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5160-F8 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5260_f8_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x52; const SS: u16 = 0x60; const OO: u64 = 0xF8; const HH: u16 = 0x17E;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0xf8, 0x00, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x90, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5260-F8 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h50_f8_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0xF8; const HH: u16 = 0x17F;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H50-F8 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h51_f8_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0xF8; const HH: u16 = 0x180;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H51-F8 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h52_f8_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0xF8; const HH: u16 = 0x181;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H52-F8 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h50_f8_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0xF8; const HH: u16 = 0x182;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H50-F8 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h51_f8_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0xF8; const HH: u16 = 0x183;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H51-F8 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h52_f8_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0xF8; const HH: u16 = 0x184;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H52-F8 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5060_100_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x50; const SS: u16 = 0x60; const OO: u64 = 0x100; const HH: u16 = 0x185;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0x00, 0x01, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x80, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5060-100 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5160_100_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x51; const SS: u16 = 0x60; const OO: u64 = 0x100; const HH: u16 = 0x186;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0x00, 0x01, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x88, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5160-100 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5260_100_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x52; const SS: u16 = 0x60; const OO: u64 = 0x100; const HH: u16 = 0x187;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0x00, 0x01, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x90, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5260-100 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h50_100_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0x100; const HH: u16 = 0x188;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H50-100 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h51_100_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0x100; const HH: u16 = 0x189;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H51-100 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h52_100_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0x100; const HH: u16 = 0x18A;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H52-100 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h50_100_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0x100; const HH: u16 = 0x18B;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H50-100 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h51_100_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0x100; const HH: u16 = 0x18C;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H51-100 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h52_100_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0x100; const HH: u16 = 0x18D;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H52-100 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5060_108_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x50; const SS: u16 = 0x60; const OO: u64 = 0x108; const HH: u16 = 0x18E;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0x08, 0x01, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x80, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5060-108 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5160_108_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x51; const SS: u16 = 0x60; const OO: u64 = 0x108; const HH: u16 = 0x18F;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0x08, 0x01, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x88, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5160-108 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5260_108_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x52; const SS: u16 = 0x60; const OO: u64 = 0x108; const HH: u16 = 0x190;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0x08, 0x01, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x90, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5260-108 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h50_108_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0x108; const HH: u16 = 0x191;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H50-108 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h51_108_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0x108; const HH: u16 = 0x192;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H51-108 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h52_108_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0x108; const HH: u16 = 0x193;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H52-108 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h50_108_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0x108; const HH: u16 = 0x194;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H50-108 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h51_108_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0x108; const HH: u16 = 0x195;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H51-108 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h52_108_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0x108; const HH: u16 = 0x196;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H52-108 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5060_110_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x50; const SS: u16 = 0x60; const OO: u64 = 0x110; const HH: u16 = 0x197;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0x10, 0x01, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x80, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5060-110 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5160_110_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x51; const SS: u16 = 0x60; const OO: u64 = 0x110; const HH: u16 = 0x198;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0x10, 0x01, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x88, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5160-110 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5260_110_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x52; const SS: u16 = 0x60; const OO: u64 = 0x110; const HH: u16 = 0x199;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0x10, 0x01, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x90, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5260-110 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h50_110_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0x110; const HH: u16 = 0x19A;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H50-110 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h51_110_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0x110; const HH: u16 = 0x19B;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H51-110 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h52_110_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0x110; const HH: u16 = 0x19C;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H52-110 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h50_110_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0x110; const HH: u16 = 0x19D;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H50-110 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h51_110_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0x110; const HH: u16 = 0x19E;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H51-110 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h52_110_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0x110; const HH: u16 = 0x19F;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H52-110 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5060_118_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x50; const SS: u16 = 0x60; const OO: u64 = 0x118; const HH: u16 = 0x1A0;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0x18, 0x01, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x80, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5060-118 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5160_118_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x51; const SS: u16 = 0x60; const OO: u64 = 0x118; const HH: u16 = 0x1A1;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0x18, 0x01, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x88, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5160-118 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5260_118_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x52; const SS: u16 = 0x60; const OO: u64 = 0x118; const HH: u16 = 0x1A2;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0x18, 0x01, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x90, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5260-118 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h50_118_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0x118; const HH: u16 = 0x1A3;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H50-118 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h51_118_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0x118; const HH: u16 = 0x1A4;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H51-118 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h52_118_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0x118; const HH: u16 = 0x1A5;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H52-118 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h50_118_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0x118; const HH: u16 = 0x1A6;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H50-118 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h51_118_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0x118; const HH: u16 = 0x1A7;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H51-118 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h52_118_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0x118; const HH: u16 = 0x1A8;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H52-118 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5060_120_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x50; const SS: u16 = 0x60; const OO: u64 = 0x120; const HH: u16 = 0x1A9;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0x20, 0x01, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x80, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5060-120 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5160_120_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x51; const SS: u16 = 0x60; const OO: u64 = 0x120; const HH: u16 = 0x1AA;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0x20, 0x01, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x88, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5160-120 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5260_120_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x52; const SS: u16 = 0x60; const OO: u64 = 0x120; const HH: u16 = 0x1AB;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0x20, 0x01, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x90, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5260-120 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h50_120_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0x120; const HH: u16 = 0x1AC;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H50-120 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h51_120_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0x120; const HH: u16 = 0x1AD;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H51-120 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h52_120_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0x120; const HH: u16 = 0x1AE;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H52-120 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h50_120_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0x120; const HH: u16 = 0x1AF;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H50-120 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h51_120_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0x120; const HH: u16 = 0x1B0;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H51-120 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h52_120_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0x120; const HH: u16 = 0x1B1;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H52-120 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5060_128_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x50; const SS: u16 = 0x60; const OO: u64 = 0x128; const HH: u16 = 0x1B2;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0x28, 0x01, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x80, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5060-128 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5160_128_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x51; const SS: u16 = 0x60; const OO: u64 = 0x128; const HH: u16 = 0x1B3;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0x28, 0x01, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x88, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5160-128 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5260_128_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x52; const SS: u16 = 0x60; const OO: u64 = 0x128; const HH: u16 = 0x1B4;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0x28, 0x01, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x90, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5260-128 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h50_128_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0x128; const HH: u16 = 0x1B5;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H50-128 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h51_128_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0x128; const HH: u16 = 0x1B6;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H51-128 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h52_128_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0x128; const HH: u16 = 0x1B7;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H52-128 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h50_128_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0x128; const HH: u16 = 0x1B8;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H50-128 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h51_128_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0x128; const HH: u16 = 0x1B9;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H51-128 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h52_128_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0x128; const HH: u16 = 0x1BA;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H52-128 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5060_130_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x50; const SS: u16 = 0x60; const OO: u64 = 0x130; const HH: u16 = 0x1BB;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0x30, 0x01, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x80, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5060-130 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5160_130_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x51; const SS: u16 = 0x60; const OO: u64 = 0x130; const HH: u16 = 0x1BC;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0x30, 0x01, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x88, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5160-130 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5260_130_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x52; const SS: u16 = 0x60; const OO: u64 = 0x130; const HH: u16 = 0x1BD;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0x30, 0x01, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x90, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5260-130 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h50_130_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0x130; const HH: u16 = 0x1BE;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H50-130 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h51_130_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0x130; const HH: u16 = 0x1BF;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H51-130 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h52_130_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0x130; const HH: u16 = 0x1C0;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H52-130 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h50_130_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0x130; const HH: u16 = 0x1C1;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H50-130 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h51_130_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0x130; const HH: u16 = 0x1C2;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H51-130 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h52_130_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0x130; const HH: u16 = 0x1C3;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H52-130 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5060_138_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x50; const SS: u16 = 0x60; const OO: u64 = 0x138; const HH: u16 = 0x1C4;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0x38, 0x01, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x80, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5060-138 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5160_138_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x51; const SS: u16 = 0x60; const OO: u64 = 0x138; const HH: u16 = 0x1C5;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0x38, 0x01, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x88, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5160-138 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5260_138_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x52; const SS: u16 = 0x60; const OO: u64 = 0x138; const HH: u16 = 0x1C6;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0x38, 0x01, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x90, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5260-138 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h50_138_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0x138; const HH: u16 = 0x1C7;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H50-138 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h51_138_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0x138; const HH: u16 = 0x1C8;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H51-138 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h52_138_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0x138; const HH: u16 = 0x1C9;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H52-138 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h50_138_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0x138; const HH: u16 = 0x1CA;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H50-138 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h51_138_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0x138; const HH: u16 = 0x1CB;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H51-138 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h52_138_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0x138; const HH: u16 = 0x1CC;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H52-138 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5060_140_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x50; const SS: u16 = 0x60; const OO: u64 = 0x140; const HH: u16 = 0x1CD;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0x40, 0x01, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x80, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5060-140 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5160_140_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x51; const SS: u16 = 0x60; const OO: u64 = 0x140; const HH: u16 = 0x1CE;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0x40, 0x01, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x88, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5160-140 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5260_140_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x52; const SS: u16 = 0x60; const OO: u64 = 0x140; const HH: u16 = 0x1CF;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0x40, 0x01, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x90, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5260-140 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h50_140_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0x140; const HH: u16 = 0x1D0;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H50-140 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h51_140_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0x140; const HH: u16 = 0x1D1;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H51-140 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h52_140_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0x140; const HH: u16 = 0x1D2;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H52-140 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h50_140_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0x140; const HH: u16 = 0x1D3;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H50-140 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h51_140_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0x140; const HH: u16 = 0x1D4;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H51-140 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h52_140_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0x140; const HH: u16 = 0x1D5;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H52-140 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5060_148_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x50; const SS: u16 = 0x60; const OO: u64 = 0x148; const HH: u16 = 0x1D6;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0x48, 0x01, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x80, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5060-148 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5160_148_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x51; const SS: u16 = 0x60; const OO: u64 = 0x148; const HH: u16 = 0x1D7;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0x48, 0x01, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x88, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5160-148 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5260_148_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x52; const SS: u16 = 0x60; const OO: u64 = 0x148; const HH: u16 = 0x1D8;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0x48, 0x01, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x90, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5260-148 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h50_148_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0x148; const HH: u16 = 0x1D9;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H50-148 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h51_148_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0x148; const HH: u16 = 0x1DA;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H51-148 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h52_148_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0x148; const HH: u16 = 0x1DB;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H52-148 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h50_148_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0x148; const HH: u16 = 0x1DC;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H50-148 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h51_148_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0x148; const HH: u16 = 0x1DD;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H51-148 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h52_148_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0x148; const HH: u16 = 0x1DE;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H52-148 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5060_150_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x50; const SS: u16 = 0x60; const OO: u64 = 0x150; const HH: u16 = 0x1DF;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0x50, 0x01, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x80, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5060-150 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5160_150_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x51; const SS: u16 = 0x60; const OO: u64 = 0x150; const HH: u16 = 0x1E0;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0x50, 0x01, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x88, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5160-150 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5260_150_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x52; const SS: u16 = 0x60; const OO: u64 = 0x150; const HH: u16 = 0x1E1;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0x50, 0x01, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x90, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5260-150 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h50_150_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0x150; const HH: u16 = 0x1E2;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H50-150 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h51_150_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0x150; const HH: u16 = 0x1E3;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H51-150 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h52_150_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0x150; const HH: u16 = 0x1E4;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H52-150 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h50_150_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0x150; const HH: u16 = 0x1E5;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H50-150 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h51_150_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0x150; const HH: u16 = 0x1E6;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H51-150 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h52_150_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0x150; const HH: u16 = 0x1E7;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H52-150 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5060_158_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x50; const SS: u16 = 0x60; const OO: u64 = 0x158; const HH: u16 = 0x1E8;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0x58, 0x01, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x80, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5060-158 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5160_158_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x51; const SS: u16 = 0x60; const OO: u64 = 0x158; const HH: u16 = 0x1E9;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0x58, 0x01, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x88, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5160-158 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5260_158_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x52; const SS: u16 = 0x60; const OO: u64 = 0x158; const HH: u16 = 0x1EA;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0x58, 0x01, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x90, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5260-158 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h50_158_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0x158; const HH: u16 = 0x1EB;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H50-158 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h51_158_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0x158; const HH: u16 = 0x1EC;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H51-158 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h52_158_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0x158; const HH: u16 = 0x1ED;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H52-158 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h50_158_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0x158; const HH: u16 = 0x1EE;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H50-158 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h51_158_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0x158; const HH: u16 = 0x1EF;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H51-158 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h52_158_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0x158; const HH: u16 = 0x1F0;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H52-158 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5060_160_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x50; const SS: u16 = 0x60; const OO: u64 = 0x160; const HH: u16 = 0x1F1;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0x60, 0x01, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x80, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5060-160 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5160_160_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x51; const SS: u16 = 0x60; const OO: u64 = 0x160; const HH: u16 = 0x1F2;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0x60, 0x01, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x88, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5160-160 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5260_160_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x52; const SS: u16 = 0x60; const OO: u64 = 0x160; const HH: u16 = 0x1F3;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0x60, 0x01, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x90, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5260-160 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h50_160_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0x160; const HH: u16 = 0x1F4;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H50-160 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h51_160_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0x160; const HH: u16 = 0x1F5;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H51-160 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h52_160_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0x160; const HH: u16 = 0x1F6;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H52-160 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h50_160_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0x160; const HH: u16 = 0x1F7;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H50-160 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h51_160_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0x160; const HH: u16 = 0x1F8;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H51-160 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h52_160_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0x160; const HH: u16 = 0x1F9;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H52-160 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5060_168_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x50; const SS: u16 = 0x60; const OO: u64 = 0x168; const HH: u16 = 0x1FA;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0x68, 0x01, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x80, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5060-168 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5160_168_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x51; const SS: u16 = 0x60; const OO: u64 = 0x168; const HH: u16 = 0x1FB;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0x68, 0x01, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x88, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5160-168 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5260_168_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x52; const SS: u16 = 0x60; const OO: u64 = 0x168; const HH: u16 = 0x1FC;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0x68, 0x01, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x90, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5260-168 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h50_168_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0x168; const HH: u16 = 0x1FD;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H50-168 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h51_168_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0x168; const HH: u16 = 0x1FE;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H51-168 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h52_168_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0x168; const HH: u16 = 0x1FF;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H52-168 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h50_168_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0x168; const HH: u16 = 0x200;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H50-168 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h51_168_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0x168; const HH: u16 = 0x201;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H51-168 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h52_168_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0x168; const HH: u16 = 0x202;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H52-168 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5060_170_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x50; const SS: u16 = 0x60; const OO: u64 = 0x170; const HH: u16 = 0x203;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0x70, 0x01, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x80, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5060-170 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5160_170_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x51; const SS: u16 = 0x60; const OO: u64 = 0x170; const HH: u16 = 0x204;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0x70, 0x01, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x88, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5160-170 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5260_170_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x52; const SS: u16 = 0x60; const OO: u64 = 0x170; const HH: u16 = 0x205;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0x70, 0x01, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x90, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5260-170 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h50_170_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0x170; const HH: u16 = 0x206;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H50-170 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h51_170_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0x170; const HH: u16 = 0x207;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H51-170 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h52_170_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0x170; const HH: u16 = 0x208;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H52-170 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h50_170_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0x170; const HH: u16 = 0x209;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H50-170 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h51_170_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0x170; const HH: u16 = 0x20A;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H51-170 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h52_170_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0x170; const HH: u16 = 0x20B;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H52-170 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5060_178_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x50; const SS: u16 = 0x60; const OO: u64 = 0x178; const HH: u16 = 0x20C;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0x78, 0x01, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x80, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5060-178 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5160_178_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x51; const SS: u16 = 0x60; const OO: u64 = 0x178; const HH: u16 = 0x20D;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0x78, 0x01, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x88, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5160-178 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5260_178_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x52; const SS: u16 = 0x60; const OO: u64 = 0x178; const HH: u16 = 0x20E;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0x78, 0x01, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x90, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5260-178 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h50_178_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0x178; const HH: u16 = 0x20F;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H50-178 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h51_178_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0x178; const HH: u16 = 0x210;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H51-178 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h52_178_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0x178; const HH: u16 = 0x211;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H52-178 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h50_178_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0x178; const HH: u16 = 0x212;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H50-178 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h51_178_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0x178; const HH: u16 = 0x213;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H51-178 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h52_178_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0x178; const HH: u16 = 0x214;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H52-178 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5060_180_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x50; const SS: u16 = 0x60; const OO: u64 = 0x180; const HH: u16 = 0x215;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0x80, 0x01, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x80, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5060-180 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5160_180_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x51; const SS: u16 = 0x60; const OO: u64 = 0x180; const HH: u16 = 0x216;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0x80, 0x01, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x88, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5160-180 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5260_180_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x52; const SS: u16 = 0x60; const OO: u64 = 0x180; const HH: u16 = 0x217;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0x80, 0x01, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x90, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5260-180 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h50_180_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0x180; const HH: u16 = 0x218;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H50-180 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h51_180_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0x180; const HH: u16 = 0x219;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H51-180 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h52_180_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0x180; const HH: u16 = 0x21A;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H52-180 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h50_180_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0x180; const HH: u16 = 0x21B;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H50-180 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h51_180_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0x180; const HH: u16 = 0x21C;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H51-180 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h52_180_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0x180; const HH: u16 = 0x21D;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H52-180 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5060_188_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x50; const SS: u16 = 0x60; const OO: u64 = 0x188; const HH: u16 = 0x21E;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0x88, 0x01, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x80, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5060-188 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5160_188_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x51; const SS: u16 = 0x60; const OO: u64 = 0x188; const HH: u16 = 0x21F;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0x88, 0x01, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x88, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5160-188 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5260_188_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x52; const SS: u16 = 0x60; const OO: u64 = 0x188; const HH: u16 = 0x220;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0x88, 0x01, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x90, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5260-188 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h50_188_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0x188; const HH: u16 = 0x221;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H50-188 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h51_188_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0x188; const HH: u16 = 0x222;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H51-188 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h52_188_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0x188; const HH: u16 = 0x223;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H52-188 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h50_188_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0x188; const HH: u16 = 0x224;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H50-188 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h51_188_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0x188; const HH: u16 = 0x225;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H51-188 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h52_188_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0x188; const HH: u16 = 0x226;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H52-188 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5060_190_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x50; const SS: u16 = 0x60; const OO: u64 = 0x190; const HH: u16 = 0x227;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0x90, 0x01, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x80, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5060-190 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5160_190_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x51; const SS: u16 = 0x60; const OO: u64 = 0x190; const HH: u16 = 0x228;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0x90, 0x01, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x88, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5160-190 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5260_190_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x52; const SS: u16 = 0x60; const OO: u64 = 0x190; const HH: u16 = 0x229;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0x90, 0x01, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x90, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5260-190 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h50_190_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0x190; const HH: u16 = 0x22A;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H50-190 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h51_190_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0x190; const HH: u16 = 0x22B;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H51-190 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h52_190_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0x190; const HH: u16 = 0x22C;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H52-190 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h50_190_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0x190; const HH: u16 = 0x22D;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H50-190 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h51_190_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0x190; const HH: u16 = 0x22E;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H51-190 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h52_190_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0x190; const HH: u16 = 0x22F;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H52-190 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5060_198_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x50; const SS: u16 = 0x60; const OO: u64 = 0x198; const HH: u16 = 0x230;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0x98, 0x01, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x80, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5060-198 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5160_198_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x51; const SS: u16 = 0x60; const OO: u64 = 0x198; const HH: u16 = 0x231;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0x98, 0x01, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x88, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5160-198 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5260_198_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x52; const SS: u16 = 0x60; const OO: u64 = 0x198; const HH: u16 = 0x232;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0x98, 0x01, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x90, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5260-198 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h50_198_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0x198; const HH: u16 = 0x233;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H50-198 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h51_198_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0x198; const HH: u16 = 0x234;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H51-198 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h52_198_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0x198; const HH: u16 = 0x235;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H52-198 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h50_198_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0x198; const HH: u16 = 0x236;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H50-198 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h51_198_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0x198; const HH: u16 = 0x237;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H51-198 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h52_198_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0x198; const HH: u16 = 0x238;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H52-198 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5060_1a0_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x50; const SS: u16 = 0x60; const OO: u64 = 0x1A0; const HH: u16 = 0x239;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0xa0, 0x01, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x80, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5060-1A0 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5160_1a0_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x51; const SS: u16 = 0x60; const OO: u64 = 0x1A0; const HH: u16 = 0x23A;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0xa0, 0x01, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x88, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5160-1A0 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5260_1a0_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x52; const SS: u16 = 0x60; const OO: u64 = 0x1A0; const HH: u16 = 0x23B;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0xa0, 0x01, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x90, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5260-1A0 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h50_1a0_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0x1A0; const HH: u16 = 0x23C;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H50-1A0 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h51_1a0_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0x1A0; const HH: u16 = 0x23D;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H51-1A0 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h52_1a0_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0x1A0; const HH: u16 = 0x23E;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H52-1A0 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h50_1a0_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0x1A0; const HH: u16 = 0x23F;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H50-1A0 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h51_1a0_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0x1A0; const HH: u16 = 0x240;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H51-1A0 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h52_1a0_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0x1A0; const HH: u16 = 0x241;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H52-1A0 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5060_1a8_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x50; const SS: u16 = 0x60; const OO: u64 = 0x1A8; const HH: u16 = 0x242;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0xa8, 0x01, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x80, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5060-1A8 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5160_1a8_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x51; const SS: u16 = 0x60; const OO: u64 = 0x1A8; const HH: u16 = 0x243;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0xa8, 0x01, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x88, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5160-1A8 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5260_1a8_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x52; const SS: u16 = 0x60; const OO: u64 = 0x1A8; const HH: u16 = 0x244;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0xa8, 0x01, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x90, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5260-1A8 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h50_1a8_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0x1A8; const HH: u16 = 0x245;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H50-1A8 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h51_1a8_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0x1A8; const HH: u16 = 0x246;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H51-1A8 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h52_1a8_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0x1A8; const HH: u16 = 0x247;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H52-1A8 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h50_1a8_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0x1A8; const HH: u16 = 0x248;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H50-1A8 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h51_1a8_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0x1A8; const HH: u16 = 0x249;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H51-1A8 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h52_1a8_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0x1A8; const HH: u16 = 0x24A;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H52-1A8 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5060_1b0_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x50; const SS: u16 = 0x60; const OO: u64 = 0x1B0; const HH: u16 = 0x24B;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0xb0, 0x01, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x80, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5060-1B0 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5160_1b0_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x51; const SS: u16 = 0x60; const OO: u64 = 0x1B0; const HH: u16 = 0x24C;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0xb0, 0x01, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x88, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5160-1B0 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5260_1b0_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x52; const SS: u16 = 0x60; const OO: u64 = 0x1B0; const HH: u16 = 0x24D;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0xb0, 0x01, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x90, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5260-1B0 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h50_1b0_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0x1B0; const HH: u16 = 0x24E;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H50-1B0 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h51_1b0_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0x1B0; const HH: u16 = 0x24F;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H51-1B0 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h52_1b0_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0x1B0; const HH: u16 = 0x250;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H52-1B0 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h50_1b0_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0x1B0; const HH: u16 = 0x251;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H50-1B0 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h51_1b0_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0x1B0; const HH: u16 = 0x252;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H51-1B0 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h52_1b0_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0x1B0; const HH: u16 = 0x253;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H52-1B0 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5060_1b8_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x50; const SS: u16 = 0x60; const OO: u64 = 0x1B8; const HH: u16 = 0x254;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0xb8, 0x01, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x80, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5060-1B8 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5160_1b8_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x51; const SS: u16 = 0x60; const OO: u64 = 0x1B8; const HH: u16 = 0x255;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0xb8, 0x01, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x88, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5160-1B8 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5260_1b8_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x52; const SS: u16 = 0x60; const OO: u64 = 0x1B8; const HH: u16 = 0x256;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0xb8, 0x01, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x90, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5260-1B8 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h50_1b8_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0x1B8; const HH: u16 = 0x257;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H50-1B8 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h51_1b8_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0x1B8; const HH: u16 = 0x258;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H51-1B8 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h52_1b8_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0x1B8; const HH: u16 = 0x259;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H52-1B8 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h50_1b8_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0x1B8; const HH: u16 = 0x25A;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H50-1B8 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h51_1b8_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0x1B8; const HH: u16 = 0x25B;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H51-1B8 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h52_1b8_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0x1B8; const HH: u16 = 0x25C;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H52-1B8 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5060_1c0_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x50; const SS: u16 = 0x60; const OO: u64 = 0x1C0; const HH: u16 = 0x25D;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0xc0, 0x01, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x80, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5060-1C0 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5160_1c0_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x51; const SS: u16 = 0x60; const OO: u64 = 0x1C0; const HH: u16 = 0x25E;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0xc0, 0x01, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x88, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5160-1C0 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5260_1c0_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x52; const SS: u16 = 0x60; const OO: u64 = 0x1C0; const HH: u16 = 0x25F;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0xc0, 0x01, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x90, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5260-1C0 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h50_1c0_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0x1C0; const HH: u16 = 0x260;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H50-1C0 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h51_1c0_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0x1C0; const HH: u16 = 0x261;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H51-1C0 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h52_1c0_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0x1C0; const HH: u16 = 0x262;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H52-1C0 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h50_1c0_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0x1C0; const HH: u16 = 0x263;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H50-1C0 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h51_1c0_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0x1C0; const HH: u16 = 0x264;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H51-1C0 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h52_1c0_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0x1C0; const HH: u16 = 0x265;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H52-1C0 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5060_1c8_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x50; const SS: u16 = 0x60; const OO: u64 = 0x1C8; const HH: u16 = 0x266;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0xc8, 0x01, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x80, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5060-1C8 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5160_1c8_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x51; const SS: u16 = 0x60; const OO: u64 = 0x1C8; const HH: u16 = 0x267;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0xc8, 0x01, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x88, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5160-1C8 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5260_1c8_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x52; const SS: u16 = 0x60; const OO: u64 = 0x1C8; const HH: u16 = 0x268;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0xc8, 0x01, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x90, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5260-1C8 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h50_1c8_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0x1C8; const HH: u16 = 0x269;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H50-1C8 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h51_1c8_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0x1C8; const HH: u16 = 0x26A;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H51-1C8 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h52_1c8_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0x1C8; const HH: u16 = 0x26B;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H52-1C8 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h50_1c8_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0x1C8; const HH: u16 = 0x26C;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H50-1C8 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h51_1c8_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0x1C8; const HH: u16 = 0x26D;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H51-1C8 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h52_1c8_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0x1C8; const HH: u16 = 0x26E;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H52-1C8 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5060_1d0_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x50; const SS: u16 = 0x60; const OO: u64 = 0x1D0; const HH: u16 = 0x26F;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0xd0, 0x01, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x80, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5060-1D0 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5160_1d0_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x51; const SS: u16 = 0x60; const OO: u64 = 0x1D0; const HH: u16 = 0x270;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0xd0, 0x01, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x88, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5160-1D0 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5260_1d0_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x52; const SS: u16 = 0x60; const OO: u64 = 0x1D0; const HH: u16 = 0x271;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0xd0, 0x01, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x90, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5260-1D0 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h50_1d0_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0x1D0; const HH: u16 = 0x272;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H50-1D0 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h51_1d0_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0x1D0; const HH: u16 = 0x273;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H51-1D0 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h52_1d0_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0x1D0; const HH: u16 = 0x274;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H52-1D0 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h50_1d0_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0x1D0; const HH: u16 = 0x275;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H50-1D0 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h51_1d0_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0x1D0; const HH: u16 = 0x276;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H51-1D0 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h52_1d0_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0x1D0; const HH: u16 = 0x277;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H52-1D0 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5060_1d8_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x50; const SS: u16 = 0x60; const OO: u64 = 0x1D8; const HH: u16 = 0x278;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0xd8, 0x01, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x80, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5060-1D8 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5160_1d8_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x51; const SS: u16 = 0x60; const OO: u64 = 0x1D8; const HH: u16 = 0x279;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0xd8, 0x01, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x88, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5160-1D8 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5260_1d8_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x52; const SS: u16 = 0x60; const OO: u64 = 0x1D8; const HH: u16 = 0x27A;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0xd8, 0x01, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x90, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5260-1D8 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h50_1d8_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0x1D8; const HH: u16 = 0x27B;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H50-1D8 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h51_1d8_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0x1D8; const HH: u16 = 0x27C;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H51-1D8 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h52_1d8_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0x1D8; const HH: u16 = 0x27D;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H52-1D8 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h50_1d8_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0x1D8; const HH: u16 = 0x27E;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H50-1D8 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h51_1d8_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0x1D8; const HH: u16 = 0x27F;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H51-1D8 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h52_1d8_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0x1D8; const HH: u16 = 0x280;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H52-1D8 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5060_1e0_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x50; const SS: u16 = 0x60; const OO: u64 = 0x1E0; const HH: u16 = 0x281;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0xe0, 0x01, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x80, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5060-1E0 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5160_1e0_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x51; const SS: u16 = 0x60; const OO: u64 = 0x1E0; const HH: u16 = 0x282;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0xe0, 0x01, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x88, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5160-1E0 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5260_1e0_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x52; const SS: u16 = 0x60; const OO: u64 = 0x1E0; const HH: u16 = 0x283;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0xe0, 0x01, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x90, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5260-1E0 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h50_1e0_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0x1E0; const HH: u16 = 0x284;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H50-1E0 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h51_1e0_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0x1E0; const HH: u16 = 0x285;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H51-1E0 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h52_1e0_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0x1E0; const HH: u16 = 0x286;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H52-1E0 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h50_1e0_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0x1E0; const HH: u16 = 0x287;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H50-1E0 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h51_1e0_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0x1E0; const HH: u16 = 0x288;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H51-1E0 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h52_1e0_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0x1E0; const HH: u16 = 0x289;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H52-1E0 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5060_1e8_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x50; const SS: u16 = 0x60; const OO: u64 = 0x1E8; const HH: u16 = 0x28A;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0xe8, 0x01, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x80, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5060-1E8 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5160_1e8_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x51; const SS: u16 = 0x60; const OO: u64 = 0x1E8; const HH: u16 = 0x28B;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0xe8, 0x01, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x88, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5160-1E8 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5260_1e8_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x52; const SS: u16 = 0x60; const OO: u64 = 0x1E8; const HH: u16 = 0x28C;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0xe8, 0x01, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x90, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5260-1E8 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h50_1e8_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0x1E8; const HH: u16 = 0x28D;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H50-1E8 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h51_1e8_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0x1E8; const HH: u16 = 0x28E;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H51-1E8 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h52_1e8_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0x1E8; const HH: u16 = 0x28F;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H52-1E8 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h50_1e8_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0x1E8; const HH: u16 = 0x290;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H50-1E8 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h51_1e8_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0x1E8; const HH: u16 = 0x291;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H51-1E8 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h52_1e8_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0x1E8; const HH: u16 = 0x292;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H52-1E8 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5060_1f0_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x50; const SS: u16 = 0x60; const OO: u64 = 0x1F0; const HH: u16 = 0x293;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0xf0, 0x01, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x80, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5060-1F0 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5160_1f0_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x51; const SS: u16 = 0x60; const OO: u64 = 0x1F0; const HH: u16 = 0x294;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0xf0, 0x01, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x88, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5160-1F0 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5260_1f0_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x52; const SS: u16 = 0x60; const OO: u64 = 0x1F0; const HH: u16 = 0x295;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0xf0, 0x01, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x90, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5260-1F0 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h50_1f0_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0x1F0; const HH: u16 = 0x296;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H50-1F0 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h51_1f0_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0x1F0; const HH: u16 = 0x297;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H51-1F0 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h52_1f0_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0x1F0; const HH: u16 = 0x298;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H52-1F0 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h50_1f0_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0x1F0; const HH: u16 = 0x299;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H50-1F0 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h51_1f0_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0x1F0; const HH: u16 = 0x29A;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H51-1F0 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h52_1f0_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0x1F0; const HH: u16 = 0x29B;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H52-1F0 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5060_1f8_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x50; const SS: u16 = 0x60; const OO: u64 = 0x1F8; const HH: u16 = 0x29C;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0xf8, 0x01, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x80, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5060-1F8 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5160_1f8_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x51; const SS: u16 = 0x60; const OO: u64 = 0x1F8; const HH: u16 = 0x29D;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0xf8, 0x01, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x88, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5160-1F8 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5260_1f8_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x52; const SS: u16 = 0x60; const OO: u64 = 0x1F8; const HH: u16 = 0x29E;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0xf8, 0x01, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x90, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5260-1F8 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h50_1f8_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0x1F8; const HH: u16 = 0x29F;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H50-1F8 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h51_1f8_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0x1F8; const HH: u16 = 0x2A0;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H51-1F8 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h52_1f8_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0x1F8; const HH: u16 = 0x2A1;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H52-1F8 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h50_1f8_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0x1F8; const HH: u16 = 0x2A2;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H50-1F8 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h51_1f8_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0x1F8; const HH: u16 = 0x2A3;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H51-1F8 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h52_1f8_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0x1F8; const HH: u16 = 0x2A4;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H52-1F8 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5060_200_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x50; const SS: u16 = 0x60; const OO: u64 = 0x200; const HH: u16 = 0x2A5;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0x00, 0x02, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x80, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5060-200 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5160_200_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x51; const SS: u16 = 0x60; const OO: u64 = 0x200; const HH: u16 = 0x2A6;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0x00, 0x02, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x88, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5160-200 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5260_200_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x52; const SS: u16 = 0x60; const OO: u64 = 0x200; const HH: u16 = 0x2A7;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0x00, 0x02, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x90, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5260-200 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h50_200_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0x200; const HH: u16 = 0x2A8;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H50-200 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h51_200_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0x200; const HH: u16 = 0x2A9;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H51-200 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h52_200_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0x200; const HH: u16 = 0x2AA;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H52-200 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h50_200_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0x200; const HH: u16 = 0x2AB;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H50-200 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h51_200_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0x200; const HH: u16 = 0x2AC;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H51-200 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h52_200_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0x200; const HH: u16 = 0x2AD;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H52-200 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5060_208_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x50; const SS: u16 = 0x60; const OO: u64 = 0x208; const HH: u16 = 0x2AE;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0x08, 0x02, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x80, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5060-208 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5160_208_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x51; const SS: u16 = 0x60; const OO: u64 = 0x208; const HH: u16 = 0x2AF;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0x08, 0x02, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x88, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5160-208 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5260_208_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x52; const SS: u16 = 0x60; const OO: u64 = 0x208; const HH: u16 = 0x2B0;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0x08, 0x02, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x90, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5260-208 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h50_208_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0x208; const HH: u16 = 0x2B1;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H50-208 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h51_208_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0x208; const HH: u16 = 0x2B2;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H51-208 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h52_208_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0x208; const HH: u16 = 0x2B3;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H52-208 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h50_208_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0x208; const HH: u16 = 0x2B4;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H50-208 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h51_208_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0x208; const HH: u16 = 0x2B5;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H51-208 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h52_208_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0x208; const HH: u16 = 0x2B6;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H52-208 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5060_210_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x50; const SS: u16 = 0x60; const OO: u64 = 0x210; const HH: u16 = 0x2B7;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0x10, 0x02, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x80, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5060-210 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5160_210_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x51; const SS: u16 = 0x60; const OO: u64 = 0x210; const HH: u16 = 0x2B8;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0x10, 0x02, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x88, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5160-210 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5260_210_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x52; const SS: u16 = 0x60; const OO: u64 = 0x210; const HH: u16 = 0x2B9;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0x10, 0x02, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x90, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5260-210 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h50_210_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0x210; const HH: u16 = 0x2BA;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H50-210 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h51_210_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0x210; const HH: u16 = 0x2BB;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H51-210 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h52_210_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0x210; const HH: u16 = 0x2BC;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H52-210 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h50_210_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0x210; const HH: u16 = 0x2BD;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H50-210 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h51_210_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0x210; const HH: u16 = 0x2BE;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H51-210 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h52_210_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0x210; const HH: u16 = 0x2BF;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H52-210 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5060_218_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x50; const SS: u16 = 0x60; const OO: u64 = 0x218; const HH: u16 = 0x2C0;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0x18, 0x02, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x80, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5060-218 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5160_218_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x51; const SS: u16 = 0x60; const OO: u64 = 0x218; const HH: u16 = 0x2C1;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0x18, 0x02, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x88, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5160-218 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5260_218_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x52; const SS: u16 = 0x60; const OO: u64 = 0x218; const HH: u16 = 0x2C2;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0x18, 0x02, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x90, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5260-218 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h50_218_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0x218; const HH: u16 = 0x2C3;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H50-218 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h51_218_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0x218; const HH: u16 = 0x2C4;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H51-218 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h52_218_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0x218; const HH: u16 = 0x2C5;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H52-218 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h50_218_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0x218; const HH: u16 = 0x2C6;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H50-218 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h51_218_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0x218; const HH: u16 = 0x2C7;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H51-218 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h52_218_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0x218; const HH: u16 = 0x2C8;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H52-218 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5060_220_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x50; const SS: u16 = 0x60; const OO: u64 = 0x220; const HH: u16 = 0x2C9;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0x20, 0x02, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x80, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5060-220 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5160_220_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x51; const SS: u16 = 0x60; const OO: u64 = 0x220; const HH: u16 = 0x2CA;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0x20, 0x02, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x88, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5160-220 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5260_220_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x52; const SS: u16 = 0x60; const OO: u64 = 0x220; const HH: u16 = 0x2CB;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0x20, 0x02, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x90, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5260-220 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h50_220_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0x220; const HH: u16 = 0x2CC;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H50-220 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h51_220_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0x220; const HH: u16 = 0x2CD;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H51-220 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h52_220_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0x220; const HH: u16 = 0x2CE;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H52-220 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h50_220_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0x220; const HH: u16 = 0x2CF;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H50-220 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h51_220_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0x220; const HH: u16 = 0x2D0;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H51-220 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h52_220_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0x220; const HH: u16 = 0x2D1;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H52-220 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5060_228_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x50; const SS: u16 = 0x60; const OO: u64 = 0x228; const HH: u16 = 0x2D2;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0x28, 0x02, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x80, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5060-228 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5160_228_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x51; const SS: u16 = 0x60; const OO: u64 = 0x228; const HH: u16 = 0x2D3;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0x28, 0x02, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x88, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5160-228 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5260_228_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x52; const SS: u16 = 0x60; const OO: u64 = 0x228; const HH: u16 = 0x2D4;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0x28, 0x02, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x90, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5260-228 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h50_228_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0x228; const HH: u16 = 0x2D5;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H50-228 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h51_228_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0x228; const HH: u16 = 0x2D6;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H51-228 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h52_228_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0x228; const HH: u16 = 0x2D7;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H52-228 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h50_228_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0x228; const HH: u16 = 0x2D8;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H50-228 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h51_228_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0x228; const HH: u16 = 0x2D9;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H51-228 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h52_228_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0x228; const HH: u16 = 0x2DA;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H52-228 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5060_230_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x50; const SS: u16 = 0x60; const OO: u64 = 0x230; const HH: u16 = 0x2DB;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0x30, 0x02, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x80, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5060-230 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5160_230_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x51; const SS: u16 = 0x60; const OO: u64 = 0x230; const HH: u16 = 0x2DC;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0x30, 0x02, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x88, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5160-230 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5260_230_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x52; const SS: u16 = 0x60; const OO: u64 = 0x230; const HH: u16 = 0x2DD;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0x30, 0x02, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x90, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5260-230 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h50_230_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0x230; const HH: u16 = 0x2DE;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H50-230 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h51_230_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0x230; const HH: u16 = 0x2DF;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H51-230 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h52_230_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0x230; const HH: u16 = 0x2E0;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H52-230 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h50_230_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0x230; const HH: u16 = 0x2E1;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H50-230 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h51_230_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0x230; const HH: u16 = 0x2E2;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H51-230 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h52_230_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0x230; const HH: u16 = 0x2E3;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H52-230 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5060_232_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x50; const SS: u16 = 0x60; const OO: u64 = 0x232; const HH: u16 = 0x2E4;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0x32, 0x02, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x80, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5060-232 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5160_232_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x51; const SS: u16 = 0x60; const OO: u64 = 0x232; const HH: u16 = 0x2E5;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0x32, 0x02, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x88, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5160-232 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn ldb_5260_232_slot_check() -> IsaResult<()> {
    const DD: u16 = 0x52; const SS: u16 = 0x60; const OO: u64 = 0x232; const HH: u16 = 0x2E6;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x80, &[DD as u64, SS as u64, OO], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let want: [u8; 26] = [
        0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
        0x48, 0x81, 0xc0, 0x32, 0x02, 0x00, 0x00,
        0x48, 0x0f, 0xb6, 0x00,
        0x49, 0x89, 0x87, 0x90, 0x02, 0x00, 0x00,
        0xc3,
    ];
    if out.code != want || out.code.len() != 26 { return Err(IsaError::ParseError { line: 0, msg: format!("LDB-5260-232 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h50_232_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0x232; const HH: u16 = 0x2E7;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H50-232 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h51_232_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x51; const IMM: u64 = 0x232; const HH: u16 = 0x2E8;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H51-232 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn addimm_h52_232_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_add_imm;
    const SLOT: u16 = 0x52; const IMM: u64 = 0x232; const HH: u16 = 0x2E9;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x62, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_add_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("ADDIMM-H52-232 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}
fn subimm_h50_232_slot_check() -> IsaResult<()> {
    use crate::assembler::emit_sub_imm;
    const SLOT: u16 = 0x50; const IMM: u64 = 0x232; const HH: u16 = 0x2EA;
    let tir = vec![lower_op_checked(0x40, &[HH as u64], 1)?, lower_op_checked(0x61, &[SLOT as u64, IMM], 2)?, lower_op_checked(0xFF, &[], 3)?];
    let out = emit::emit(&tir, PlatformKind::Stub)?;
    let mut want = emit_sub_imm(SLOT, IMM)?; want.extend(assembler::ret());
    if out.code != want || out.code.len() != 22 { return Err(IsaError::ParseError { line: 0, msg: format!("SUBIMM-H50-232 mismatch: got {:02X?}", out.code) }); }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn self_test_passes() {
        run_self_test().unwrap();
    }
}
