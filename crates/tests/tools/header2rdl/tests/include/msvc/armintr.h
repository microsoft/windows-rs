/***
*   armintr.h - definitions and declarations for ARM specific intrinsics
*
*       Copyright (c) Microsoft Corporation. All rights reserved.
*
*Purpose:
*       This file contains constant definitions and external subroutine
*       declarations for the ARM specific intrinsics
*
****/

#pragma once


#if !defined (_M_ARM)
#error This header is specific to ARM targets
#endif  /* !defined (_M_ARM) */


#if defined (__cplusplus)
extern "C" {
#endif  /* defined (__cplusplus) */

typedef enum _tag_ARMINTR_SHIFT_T
{
    _ARM_LSR = 0,
    _ARM_LSL = 1,
    _ARM_ASR = 2,
    _ARM_ROR = 3
}
_ARMINTR_SHIFT_T;

             int __arm_gen_s_Rd_Rd      (const unsigned int, int);
             int __arm_gen_s_Rd_RdRn    (const unsigned int, int, int);
             int __arm_gen_s_Rd_RdRm    (const unsigned int, int, int);
             int __arm_gen_s_Rd_RnRm_g  (const unsigned int, int, int);
             int __arm_gen_s_Rd_RnRm_q  (const unsigned int, int, int);
             int __arm_gen_s_Rd_Rn      (const unsigned int, int);
             int __arm_gen_s_Rd_Rn_q    (const unsigned int, int);
             int __arm_gen_s_Rd_RnRm    (const unsigned int, int, int);
             int __arm_gen_s_Rd_Rm      (const unsigned int, int);
             int __arm_gen_s_Rd_Rm_q    (const unsigned int, int);
             int __arm_gen_s_Rd_Rm2     (const unsigned int, int);
         __int64 __arm_gen_s_Rdn_RdnRmRs(const unsigned int, __int64, int, int);
         __int64 __arm_gen_s_Rdn_RmRs   (const unsigned int, int, int);
             int __arm_gen_s_Rn_RmRs    (const unsigned int, int, int);
             int __arm_gen_s_Rn_RmRsRd  (const unsigned int, int, int, int);
unsigned     int __arm_gen_u_Rd_Rd      (const unsigned int, unsigned int);
unsigned     int __arm_gen_u_Rd_RdRn    (const unsigned int, unsigned int, unsigned int);
unsigned     int __arm_gen_u_Rd_RdRm    (const unsigned int, unsigned int, unsigned int);
unsigned     int __arm_gen_u_Rd_RnRm_g  (const unsigned int, unsigned int, unsigned int);
unsigned     int __arm_gen_u_Rd_Rn      (const unsigned int, unsigned int);
unsigned     int __arm_gen_u_Rd_RnRm    (const unsigned int, unsigned int, unsigned int);
unsigned     int __arm_gen_u_Rd_Rm      (const unsigned int, unsigned int);
unsigned     int __arm_gen_u_Rd_Rm2     (const unsigned int, unsigned int);
unsigned __int64 __arm_gen_u_Rdn_RdnRmRs(const unsigned int, unsigned  __int64, unsigned int, unsigned int);
unsigned __int64 __arm_gen_u_Rdn_RmRs   (const unsigned int, unsigned int, unsigned int);
unsigned     int __arm_gen_u_Rn_RmRs    (const unsigned int, unsigned int, unsigned int);
unsigned     int __arm_gen_u_Rn_RmRsRd  (const unsigned int, unsigned int, unsigned int, unsigned int);

#define _ARMINTR_ASSERT(exp, msg) __static_assert(exp, msg)

#define _ARMINTR_IN_RANGE(val, low, high) (((val) >= low) && ((val) <= high))

#define _ARMINTR_ASSERT_RANGE(val, low, high, name) \
    (_ARMINTR_ASSERT(_ARMINTR_IN_RANGE((val), (low), (high)), name " is out of range '" #low " - " #high "'"))
#define _ARMINTR_ASSERT_ROT(rot) \
    (_ARMINTR_ASSERT(((rot) & ~0x18U) == 0, "rotation must be 0, 8, 16, or 24"))
#define _ARMINTR_ASSERT_SAT_BIT(sat_bit, low, high) \
    (_ARMINTR_ASSERT_RANGE((sat_bit), low, high, "saturation bit position")) // low, high intentionally not in (parentheses)

#if defined (_M_THUMB)
#define _ARMINTR_ASSERT_SAT_SHIFT(type, amt) \
    (_ARMINTR_ASSERT(((type) == _ARM_LSL) || ((type) == _ARM_ASR), "shift type must be _ARM_LSL or _ARM_ASR"), \
     _ARMINTR_ASSERT(((type) != _ARM_LSL) || _ARMINTR_IN_RANGE((amt), 0, 31), "shift is out of range '0 - 31'"), \
     _ARMINTR_ASSERT(((type) != _ARM_ASR) || _ARMINTR_IN_RANGE((amt), 1, 31), "shift is out of range '1 - 31'"))

#else  /* defined (_M_THUMB) */
#define _ARMINTR_ASSERT_SAT_SHIFT(type, amt) \
    (_ARMINTR_ASSERT(((type) == _ARM_LSL) || ((type) == _ARM_ASR), "shift type must be _ARM_LSL or _ARM_ASR"), \
     _ARMINTR_ASSERT(((type) != _ARM_LSL) || _ARMINTR_IN_RANGE((amt), 0, 31), "shift is out of range '0 - 31'"), \
     _ARMINTR_ASSERT(((type) != _ARM_ASR) || _ARMINTR_IN_RANGE((amt), 1, 32), "shift is out of range '1 - 32'"))
#endif  /* defined (_M_THUMB) */

#if defined (_M_THUMB)

#define _ARMINTR_ENCODE_XTROT(rot)        (((rot) & 0x18U) << 17)
#define _ARMINTR_ENCODE_PKHSHIFT(shift)   ((((shift) & 0x1CU) << 26) | (((shift) & 0x3U) << 22))
#define _ARMINTR_ENCODE_IMM5_16(imm)      (((imm) & 0x1FU) << 16)
#define _ARMINTR_ENCODE_IMM5_7(imm)       ((((imm) & 0x1CU) << 26) | (((imm) & 0x3U) << 22))
#define _ARMINTR_ENCODE_SAT_SH(type, amt) ((((type) & 2U) << 4) | _ARMINTR_ENCODE_IMM5_7(amt))
#define _ARMINTR_ENCODE_IMM4_16(bit)      (((bit) & 0xFU) << 16)

#define _ARMINTR_BFC        0x0000F36FU
#define _ARMINTR_BFI        0x0000F360U
#define _ARMINTR_SBFX       0x0000F340U
#define _ARMINTR_UBFX       0x0000F3C0U
#define _ARMINTR_QADD       0xF080FA80U
#define _ARMINTR_QDADD      0xF090FA80U
#define _ARMINTR_QDSUB      0xF0B0FA80U
#define _ARMINTR_QSUB       0xF0A0FA80U
#define _ARMINTR_QADD16     0xF010FA90U
#define _ARMINTR_QADD8      0xF010FA80U
#define _ARMINTR_QASX       0xF010FAA0U
#define _ARMINTR_QSAX       0xF010FAE0U
#define _ARMINTR_QSUB16     0xF010FAD0U
#define _ARMINTR_QSUB8      0xF010FAC0U
#define _ARMINTR_SHADD16    0xF020FA90U
#define _ARMINTR_SHADD8     0xF020FA80U
#define _ARMINTR_SHASX      0xF020FAA0U
#define _ARMINTR_SHSAX      0xF020FAE0U
#define _ARMINTR_SHSUB16    0xF020FAD0U
#define _ARMINTR_SHSUB8     0xF020FAC0U
#define _ARMINTR_UHADD16    0xF060FA90U
#define _ARMINTR_UHADD8     0xF060FA80U
#define _ARMINTR_UHASX      0xF060FAA0U
#define _ARMINTR_UHSAX      0xF060FAE0U
#define _ARMINTR_UHSUB16    0xF060FAD0U
#define _ARMINTR_UHSUB8     0xF060FAC0U
#define _ARMINTR_UQADD16    0xF050FA90U
#define _ARMINTR_UQADD8     0xF050FA80U
#define _ARMINTR_UQASX      0xF050FAA0U
#define _ARMINTR_UQSAX      0xF050FAE0U
#define _ARMINTR_UQSUB16    0xF050FAD0U
#define _ARMINTR_UQSUB8     0xF050FAC0U
#define _ARMINTR_SXTAB      0xF080FA40U
#define _ARMINTR_SXTAB16    0xF080FA20U
#define _ARMINTR_SXTAH      0xF080FA00U
#define _ARMINTR_UXTAB      0xF080FA50U
#define _ARMINTR_UXTAB16    0xF080FA30U
#define _ARMINTR_UXTAH      0xF080FA10U
#define _ARMINTR_SXTB       0xF080FA4FU
#define _ARMINTR_SXTB16     0xF080FA2FU
#define _ARMINTR_SXTH       0xF080FA0FU
#define _ARMINTR_UXTB       0xF080FA5FU
#define _ARMINTR_UXTB16     0xF080FA3FU
#define _ARMINTR_UXTBH      0xF080FA1FU
#define _ARMINTR_PKHBT      0x0000EAC0U
#define _ARMINTR_PKHTB      0x0020EAC0U
#define _ARMINTR_USAD8      0xF000FB70U
#define _ARMINTR_USADA8     0x0000FB70U
#define _ARMINTR_SADD16     0xF000FA90U
#define _ARMINTR_SADD8      0xF000FA80U
#define _ARMINTR_SASX       0xF000FAA0U
#define _ARMINTR_SSAX       0xF000FAE0U
#define _ARMINTR_SSUB16     0xF000FAD0U
#define _ARMINTR_SSUB8      0xF000FAC0U
#define _ARMINTR_UADD16     0xF040FA90U
#define _ARMINTR_UADD8      0xF040FA80U
#define _ARMINTR_UASX       0xF040FAA0U
#define _ARMINTR_USAX       0xF040FAE0U
#define _ARMINTR_USUB16     0xF040FAD0U
#define _ARMINTR_USUB8      0xF040FAC0U
#define _ARMINTR_SSAT       0x0000F300U
#define _ARMINTR_USAT       0x0000F380U
#define _ARMINTR_SSAT16     0x0000F320U
#define _ARMINTR_USAT16     0x0000F3A0U
#define _ARMINTR_CLZ        0xF080FAB0U
#define _ARMINTR_RBIT       0xF0A0FA90U
#define _ARMINTR_REV        0xF080FA90U
#define _ARMINTR_REV16      0xF090FA90U
#define _ARMINTR_REVSH      0xF0B0FA90U
#define _ARMINTR_SMLAD      0x0000FB20U
#define _ARMINTR_SMLADX     0x0010FB20U
#define _ARMINTR_SMLSD      0x0000FB40U
#define _ARMINTR_SMLSDX     0x0010FB40U
#define _ARMINTR_SMMLA      0x0000FB50U
#define _ARMINTR_SMMLAR     0x0010FB50U
#define _ARMINTR_SMMLS      0x0000FB60U
#define _ARMINTR_SMMLSR     0x0010FB60U
#define _ARMINTR_SMLABB     0x0000FB10U
#define _ARMINTR_SMLABT     0x0010FB10U
#define _ARMINTR_SMLATB     0x0020FB10U
#define _ARMINTR_SMLATT     0x0030FB10U
#define _ARMINTR_SMLAWB     0x0000FB30U
#define _ARMINTR_SMLAWT     0x0010FB30U
#define _ARMINTR_SMULWB     0xF000FB30U
#define _ARMINTR_SMULWT     0xF010FB30U
#define _ARMINTR_SMULBB     0xF000FB10U
#define _ARMINTR_SMULBT     0xF010FB10U
#define _ARMINTR_SMULTB     0xF020FB10U
#define _ARMINTR_SMULTT     0xF030FB10U
#define _ARMINTR_SMULL      0x0000FB80U
#define _ARMINTR_SMMUL      0xF000FB50U
#define _ARMINTR_SMMULR     0xF010FB50U
#define _ARMINTR_SMUAD      0xF000FB20U
#define _ARMINTR_SMUADX     0xF010FB20U
#define _ARMINTR_SMUSD      0xF000FB40U
#define _ARMINTR_SMUSDX     0xF010FB40U
#define _ARMINTR_SMLALBB    0x0080FBC0U
#define _ARMINTR_SMLALBT    0x0090FBC0U
#define _ARMINTR_SMLALTB    0x00A0FBC0U
#define _ARMINTR_SMLALTT    0x00B0FBC0U
#define _ARMINTR_SMLALD     0x00C0FBC0U
#define _ARMINTR_SMLALDX    0x00D0FBC0U
#define _ARMINTR_SMLSLD     0x00C0FBD0U
#define _ARMINTR_SMLSLDX    0x00D0FBD0U
#define _ARMINTR_SMLAL      0x0000FBC0U
#define _ARMINTR_UMLAL      0x0000FBE0U
#define _ARMINTR_UMAAL      0x0060FBE0U
#define _ARMINTR_UMULL      0x0000FBA0U
#define _ARMINTR_SDIV       0xF0F0FB90U
#define _ARMINTR_UDIV       0xF0F0FBB0U

#define _ARMINTR_U_DDMx __arm_gen_u_Rd_RdRn
#define _ARMINTR_U_DMx  __arm_gen_u_Rd_Rn
#define _ARMINTR_S_DMx  __arm_gen_s_Rd_Rn

#else  /* defined (_M_THUMB) */

#define _ARMINTR_ENCODE_XTROT(rot)        (((rot) & 0x18U) << 7)
#define _ARMINTR_ENCODE_PKHSHIFT(shift)   (((shift) & 0x1FU) << 7)
#define _ARMINTR_ENCODE_IMM5_16(imm)      (((imm) & 0x1FU) << 16)
#define _ARMINTR_ENCODE_IMM5_7(imm)       (((imm) & 0x1FU) << 7)
#define _ARMINTR_ENCODE_SAT_SH(type, amt) ((((type) & 2U) << 5) | _ARMINTR_ENCODE_IMM5_7(amt))
#define _ARMINTR_ENCODE_IMM4_16(bit)      (((bit) & 0xFU) << 16)

#define _ARMINTR_BFC        0x07C0001FU
#define _ARMINTR_BFI        0x07C00010U
#define _ARMINTR_SBFX       0x07A00050U
#define _ARMINTR_UBFX       0x07E00050U
#define _ARMINTR_QADD       0x01000050U
#define _ARMINTR_QDADD      0x01400050U
#define _ARMINTR_QDSUB      0x01600050U
#define _ARMINTR_QSUB       0x01200050U
#define _ARMINTR_QADD16     0x06200F10U
#define _ARMINTR_QADD8      0x06200F90U
#define _ARMINTR_QASX       0x06200F30U
#define _ARMINTR_QSAX       0x06200F50U
#define _ARMINTR_QSUB16     0x06200F70U
#define _ARMINTR_QSUB8      0x06200FF0U
#define _ARMINTR_SHADD16    0x06300F10U
#define _ARMINTR_SHADD8     0x06300F90U
#define _ARMINTR_SHASX      0x06300F30U
#define _ARMINTR_SHSAX      0x06300F50U
#define _ARMINTR_SHSUB16    0x06300F70U
#define _ARMINTR_SHSUB8     0x06300FF0U
#define _ARMINTR_UHADD16    0x06700F10U
#define _ARMINTR_UHADD8     0x06700F90U
#define _ARMINTR_UHASX      0x06700F30U
#define _ARMINTR_UHSAX      0x06700F50U
#define _ARMINTR_UHSUB16    0x06700F70U
#define _ARMINTR_UHSUB8     0x06700FF0U
#define _ARMINTR_UQADD16    0x06600F10U
#define _ARMINTR_UQADD8     0x06600F90U
#define _ARMINTR_UQASX      0x06600F30U
#define _ARMINTR_UQSAX      0x06600F50U
#define _ARMINTR_UQSUB16    0x06600F70U
#define _ARMINTR_UQSUB8     0x06600FF0U
#define _ARMINTR_SXTAB      0x06A00070U
#define _ARMINTR_SXTAB16    0x06800070U
#define _ARMINTR_SXTAH      0x06B00070U
#define _ARMINTR_UXTAB      0x06E00070U
#define _ARMINTR_UXTAB16    0x06C00070U
#define _ARMINTR_UXTAH      0x06F00070U
#define _ARMINTR_SXTB       0x06AF0070U
#define _ARMINTR_SXTB16     0x068F0070U
#define _ARMINTR_SXTH       0x06BF0070U
#define _ARMINTR_UXTB       0x06EF0070U
#define _ARMINTR_UXTB16     0x06CF0070U
#define _ARMINTR_UXTBH      0x06FF0070U
#define _ARMINTR_PKHBT      0x06800010U
#define _ARMINTR_PKHTB      0x06800050U
#define _ARMINTR_USAD8      0x0780F010U
#define _ARMINTR_USADA8     0x07800010U
#define _ARMINTR_SADD16     0x06100F10U
#define _ARMINTR_SADD8      0x06100F90U
#define _ARMINTR_SASX       0x06100F30U
#define _ARMINTR_SSAX       0x06100F50U
#define _ARMINTR_SSUB16     0x06100F70U
#define _ARMINTR_SSUB8      0x06100FF0U
#define _ARMINTR_UADD16     0x06500F10U
#define _ARMINTR_UADD8      0x06500F90U
#define _ARMINTR_UASX       0x06500F30U
#define _ARMINTR_USAX       0x06500F50U
#define _ARMINTR_USUB16     0x06500F70U
#define _ARMINTR_USUB8      0x06500FF0U
#define _ARMINTR_SSAT       0x06A00010U
#define _ARMINTR_USAT       0x06E00010U
#define _ARMINTR_SSAT16     0x06A00F30U
#define _ARMINTR_USAT16     0x06E00F30U
#define _ARMINTR_CLZ        0x016F0F10U
#define _ARMINTR_RBIT       0x06FF0F30U
#define _ARMINTR_REV        0x06BF0F30U
#define _ARMINTR_REV16      0x06BF0FB0U
#define _ARMINTR_REVSH      0x06FF0FB0U
#define _ARMINTR_SMLAD      0x07000010U
#define _ARMINTR_SMLADX     0x07000030U
#define _ARMINTR_SMLSD      0x07000050U
#define _ARMINTR_SMLSDX     0x07000070U
#define _ARMINTR_SMMLA      0x07500010U
#define _ARMINTR_SMMLAR     0x07500030U
#define _ARMINTR_SMMLS      0x075000D0U
#define _ARMINTR_SMMLSR     0x075000F0U
#define _ARMINTR_SMLABB     0x01000080U
#define _ARMINTR_SMLABT     0x010000C0U
#define _ARMINTR_SMLATB     0x010000A0U
#define _ARMINTR_SMLATT     0x010000E0U
#define _ARMINTR_SMLAWB     0x01200080U
#define _ARMINTR_SMLAWT     0x012000C0U
#define _ARMINTR_SMULWB     0x012000A0U
#define _ARMINTR_SMULWT     0x012000E0U
#define _ARMINTR_SMULBB     0x01600080U
#define _ARMINTR_SMULBT     0x016000C0U
#define _ARMINTR_SMULTB     0x016000A0U
#define _ARMINTR_SMULTT     0x016000E0U
#define _ARMINTR_SMULL      0x00C00090U
#define _ARMINTR_SMMUL      0x0750F010U
#define _ARMINTR_SMMULR     0x0750F030U
#define _ARMINTR_SMUAD      0x0700F010U
#define _ARMINTR_SMUADX     0x0700F030U
#define _ARMINTR_SMUSD      0x0700F050U
#define _ARMINTR_SMUSDX     0x0700F070U
#define _ARMINTR_SMLALBB    0x01400080U
#define _ARMINTR_SMLALBT    0x014000C0U
#define _ARMINTR_SMLALTB    0x014000A0U
#define _ARMINTR_SMLALTT    0x014000E0U
#define _ARMINTR_SMLALD     0x07400010U
#define _ARMINTR_SMLALDX    0x07400030U
#define _ARMINTR_SMLSLD     0x07400050U
#define _ARMINTR_SMLSLDX    0x07400070U
#define _ARMINTR_SMLAL      0x00E00090U
#define _ARMINTR_UMLAL      0x00A00090U
#define _ARMINTR_UMAAL      0x00400090U
#define _ARMINTR_UMULL      0x00800090U

#define _ARMINTR_U_DDMx __arm_gen_u_Rd_RdRm
#define _ARMINTR_U_DMx  __arm_gen_u_Rd_Rm
#define _ARMINTR_S_DMx  __arm_gen_s_Rd_Rm

#endif  /* defined (_M_THUMB) */


/* ARMv4 */

#define _arm_smlal(_RdHiLo, _Rn, _Rm)         (__arm_gen_s_Rdn_RdnRmRs(_ARMINTR_SMLAL, (_RdHiLo), (_Rn), (_Rm)))
#define _arm_umlal(_RdHiLo, _Rn, _Rm)         (__arm_gen_u_Rdn_RdnRmRs(_ARMINTR_UMLAL, (_RdHiLo), (_Rn), (_Rm)))

/* ARMv5E */

#define _arm_clz(_Rm)                       (__arm_gen_u_Rd_Rm2(_ARMINTR_CLZ, (_Rm)))

#define _arm_qadd(_Rm, _Rn)                 (__arm_gen_s_Rd_RnRm_q(_ARMINTR_QADD,   (_Rn), (_Rm)))
#define _arm_qdadd(_Rm, _Rn)                (__arm_gen_s_Rd_RnRm_q(_ARMINTR_QDADD,  (_Rn), (_Rm)))
#define _arm_qdsub(_Rm, _Rn)                (__arm_gen_s_Rd_RnRm_q(_ARMINTR_QDSUB,  (_Rn), (_Rm)))
#define _arm_qsub(_Rm, _Rn)                 (__arm_gen_s_Rd_RnRm_q(_ARMINTR_QSUB,   (_Rn), (_Rm)))

#define _arm_smlabb(_Rn, _Rm, _Ra)          (__arm_gen_s_Rn_RmRsRd(_ARMINTR_SMLABB, (_Rn), (_Rm), (_Ra)))
#define _arm_smlabt(_Rn, _Rm, _Ra)          (__arm_gen_s_Rn_RmRsRd(_ARMINTR_SMLABT, (_Rn), (_Rm), (_Ra)))
#define _arm_smlatb(_Rn, _Rm, _Ra)          (__arm_gen_s_Rn_RmRsRd(_ARMINTR_SMLATB, (_Rn), (_Rm), (_Ra)))
#define _arm_smlatt(_Rn, _Rm, _Ra)          (__arm_gen_s_Rn_RmRsRd(_ARMINTR_SMLATT, (_Rn), (_Rm), (_Ra)))

#define _arm_smlalbb(_RdHiLo, _Rn, _Rm)     (__arm_gen_s_Rdn_RdnRmRs(_ARMINTR_SMLALBB, (_RdHiLo), (_Rn), (_Rm)))
#define _arm_smlalbt(_RdHiLo, _Rn, _Rm)     (__arm_gen_s_Rdn_RdnRmRs(_ARMINTR_SMLALBT, (_RdHiLo), (_Rn), (_Rm)))
#define _arm_smlaltb(_RdHiLo, _Rn, _Rm)     (__arm_gen_s_Rdn_RdnRmRs(_ARMINTR_SMLALTB, (_RdHiLo), (_Rn), (_Rm)))
#define _arm_smlaltt(_RdHiLo, _Rn, _Rm)     (__arm_gen_s_Rdn_RdnRmRs(_ARMINTR_SMLALTT, (_RdHiLo), (_Rn), (_Rm)))

#define _arm_smlawb(_Rn, _Rm, _Ra)          (__arm_gen_s_Rn_RmRsRd(_ARMINTR_SMLAWB, (_Rn), (_Rm), (_Ra)))
#define _arm_smlawt(_Rn, _Rm, _Ra)          (__arm_gen_s_Rn_RmRsRd(_ARMINTR_SMLAWT, (_Rn), (_Rm), (_Ra)))

#define _arm_smulbb(_Rn, _Rm)               (__arm_gen_s_Rn_RmRs(_ARMINTR_SMULBB, (_Rn), (_Rm)))
#define _arm_smulbt(_Rn, _Rm)               (__arm_gen_s_Rn_RmRs(_ARMINTR_SMULBT, (_Rn), (_Rm)))
#define _arm_smultb(_Rn, _Rm)               (__arm_gen_s_Rn_RmRs(_ARMINTR_SMULTB, (_Rn), (_Rm)))
#define _arm_smultt(_Rn, _Rm)               (__arm_gen_s_Rn_RmRs(_ARMINTR_SMULTT, (_Rn), (_Rm)))

#define _arm_smulwb(_Rn, _Rm)               (__arm_gen_s_Rn_RmRs(_ARMINTR_SMULWB, (_Rn), (_Rm)))
#define _arm_smulwt(_Rn, _Rm)               (__arm_gen_s_Rn_RmRs(_ARMINTR_SMULWT, (_Rn), (_Rm)))

/* ARMv6 */

#define _arm_sadd16(_Rn, _Rm)               (__arm_gen_s_Rd_RnRm_g(_ARMINTR_SADD16, (_Rn), (_Rm)))
#define _arm_sadd8(_Rn, _Rm)                (__arm_gen_s_Rd_RnRm_g(_ARMINTR_SADD8,  (_Rn), (_Rm)))
#define _arm_sasx(_Rn, _Rm)                 (__arm_gen_s_Rd_RnRm_g(_ARMINTR_SASX,   (_Rn), (_Rm)))
#define _arm_ssax(_Rn, _Rm)                 (__arm_gen_s_Rd_RnRm_g(_ARMINTR_SSAX,   (_Rn), (_Rm)))
#define _arm_ssub16(_Rn, _Rm)               (__arm_gen_s_Rd_RnRm_g(_ARMINTR_SSUB16, (_Rn), (_Rm)))
#define _arm_ssub8(_Rn, _Rm)                (__arm_gen_s_Rd_RnRm_g(_ARMINTR_SSUB8,  (_Rn), (_Rm)))

#define _arm_shadd16(_Rn, _Rm)              (__arm_gen_s_Rd_RnRm(_ARMINTR_SHADD16, (_Rn), (_Rm)))
#define _arm_shadd8(_Rn, _Rm)               (__arm_gen_s_Rd_RnRm(_ARMINTR_SHADD8,  (_Rn), (_Rm)))
#define _arm_shasx(_Rn, _Rm)                (__arm_gen_s_Rd_RnRm(_ARMINTR_SHASX,   (_Rn), (_Rm)))
#define _arm_shsax(_Rn, _Rm)                (__arm_gen_s_Rd_RnRm(_ARMINTR_SHSAX,   (_Rn), (_Rm)))
#define _arm_shsub16(_Rn, _Rm)              (__arm_gen_s_Rd_RnRm(_ARMINTR_SHSUB16, (_Rn), (_Rm)))
#define _arm_shsub8(_Rn, _Rm)               (__arm_gen_s_Rd_RnRm(_ARMINTR_SHSUB8,  (_Rn), (_Rm)))

#define _arm_qadd16(_Rn, _Rm)               (__arm_gen_s_Rd_RnRm(_ARMINTR_QADD16, (_Rn), (_Rm)))
#define _arm_qadd8(_Rn, _Rm)                (__arm_gen_s_Rd_RnRm(_ARMINTR_QADD8,  (_Rn), (_Rm)))
#define _arm_qasx(_Rn, _Rm)                 (__arm_gen_s_Rd_RnRm(_ARMINTR_QASX,   (_Rn), (_Rm)))
#define _arm_qsax(_Rn, _Rm)                 (__arm_gen_s_Rd_RnRm(_ARMINTR_QSAX,   (_Rn), (_Rm)))
#define _arm_qsub16(_Rn, _Rm)               (__arm_gen_s_Rd_RnRm(_ARMINTR_QSUB16, (_Rn), (_Rm)))
#define _arm_qsub8(_Rn, _Rm)                (__arm_gen_s_Rd_RnRm(_ARMINTR_QSUB8,  (_Rn), (_Rm)))

#define _arm_uadd16(_Rn, _Rm)               (__arm_gen_u_Rd_RnRm_g(_ARMINTR_UADD16, (_Rn), (_Rm)))
#define _arm_uadd8(_Rn, _Rm)                (__arm_gen_u_Rd_RnRm_g(_ARMINTR_UADD8,  (_Rn), (_Rm)))
#define _arm_uasx(_Rn, _Rm)                 (__arm_gen_u_Rd_RnRm_g(_ARMINTR_UASX,   (_Rn), (_Rm)))
#define _arm_usax(_Rn, _Rm)                 (__arm_gen_u_Rd_RnRm_g(_ARMINTR_USAX,   (_Rn), (_Rm)))
#define _arm_usub16(_Rn, _Rm)               (__arm_gen_u_Rd_RnRm_g(_ARMINTR_USUB16, (_Rn), (_Rm)))
#define _arm_usub8(_Rn, _Rm)                (__arm_gen_u_Rd_RnRm_g(_ARMINTR_USUB8,  (_Rn), (_Rm)))

#define _arm_uhadd16(_Rn, _Rm)              (__arm_gen_u_Rd_RnRm(_ARMINTR_UHADD16, (_Rn), (_Rm)))
#define _arm_uhadd8(_Rn, _Rm)               (__arm_gen_u_Rd_RnRm(_ARMINTR_UHADD8,  (_Rn), (_Rm)))
#define _arm_uhasx(_Rn, _Rm)                (__arm_gen_u_Rd_RnRm(_ARMINTR_UHASX,   (_Rn), (_Rm)))
#define _arm_uhsax(_Rn, _Rm)                (__arm_gen_u_Rd_RnRm(_ARMINTR_UHSAX,   (_Rn), (_Rm)))
#define _arm_uhsub16(_Rn, _Rm)              (__arm_gen_u_Rd_RnRm(_ARMINTR_UHSUB16, (_Rn), (_Rm)))
#define _arm_uhsub8(_Rn, _Rm)               (__arm_gen_u_Rd_RnRm(_ARMINTR_UHSUB8,  (_Rn), (_Rm)))

#define _arm_uqadd16(_Rn, _Rm)              (__arm_gen_u_Rd_RnRm(_ARMINTR_UQADD16, (_Rn), (_Rm)))
#define _arm_uqadd8(_Rn, _Rm)               (__arm_gen_u_Rd_RnRm(_ARMINTR_UQADD8,  (_Rn), (_Rm)))
#define _arm_uqasx(_Rn, _Rm)                (__arm_gen_u_Rd_RnRm(_ARMINTR_UQASX,   (_Rn), (_Rm)))
#define _arm_uqsax(_Rn, _Rm)                (__arm_gen_u_Rd_RnRm(_ARMINTR_UQSAX,   (_Rn), (_Rm)))
#define _arm_uqsub16(_Rn, _Rm)              (__arm_gen_u_Rd_RnRm(_ARMINTR_UQSUB16, (_Rn), (_Rm)))
#define _arm_uqsub8(_Rn, _Rm)               (__arm_gen_u_Rd_RnRm(_ARMINTR_UQSUB8,  (_Rn), (_Rm)))

#define _arm_sxtab(_Rn, _Rm, _Rotation)     (_ARMINTR_ASSERT_ROT(_Rotation), __arm_gen_s_Rd_RnRm(_ARMINTR_SXTAB   | _ARMINTR_ENCODE_XTROT(_Rotation), (_Rn), (_Rm)))
#define _arm_sxtab16(_Rn, _Rm, _Rotation)   (_ARMINTR_ASSERT_ROT(_Rotation), __arm_gen_s_Rd_RnRm(_ARMINTR_SXTAB16 | _ARMINTR_ENCODE_XTROT(_Rotation), (_Rn), (_Rm)))
#define _arm_sxtah(_Rn, _Rm, _Rotation)     (_ARMINTR_ASSERT_ROT(_Rotation), __arm_gen_s_Rd_RnRm(_ARMINTR_SXTAH   | _ARMINTR_ENCODE_XTROT(_Rotation), (_Rn), (_Rm)))

#define _arm_uxtab(_Rn, _Rm, _Rotation)     (_ARMINTR_ASSERT_ROT(_Rotation), __arm_gen_u_Rd_RnRm(_ARMINTR_UXTAB   | _ARMINTR_ENCODE_XTROT(_Rotation), (_Rn), (_Rm)))
#define _arm_uxtab16(_Rn, _Rm, _Rotation)   (_ARMINTR_ASSERT_ROT(_Rotation), __arm_gen_u_Rd_RnRm(_ARMINTR_UXTAB16 | _ARMINTR_ENCODE_XTROT(_Rotation), (_Rn), (_Rm)))
#define _arm_uxtah(_Rn, _Rm, _Rotation)     (_ARMINTR_ASSERT_ROT(_Rotation), __arm_gen_u_Rd_RnRm(_ARMINTR_UXTAH   | _ARMINTR_ENCODE_XTROT(_Rotation), (_Rn), (_Rm)))

#define _arm_sxtb(_Rn, _Rotation)           (_ARMINTR_ASSERT_ROT(_Rotation), __arm_gen_s_Rd_Rm(_ARMINTR_SXTB   | _ARMINTR_ENCODE_XTROT(_Rotation), (_Rn)))
#define _arm_sxtb16(_Rn, _Rotation)         (_ARMINTR_ASSERT_ROT(_Rotation), __arm_gen_s_Rd_Rm(_ARMINTR_SXTB16 | _ARMINTR_ENCODE_XTROT(_Rotation), (_Rn)))
#define _arm_sxth(_Rn, _Rotation)           (_ARMINTR_ASSERT_ROT(_Rotation), __arm_gen_s_Rd_Rm(_ARMINTR_SXTH   | _ARMINTR_ENCODE_XTROT(_Rotation), (_Rn)))

#define _arm_uxtb(_Rn, _Rotation)           (_ARMINTR_ASSERT_ROT(_Rotation), __arm_gen_u_Rd_Rm(_ARMINTR_UXTB   | _ARMINTR_ENCODE_XTROT(_Rotation), (_Rn)))
#define _arm_uxtb16(_Rn, _Rotation)         (_ARMINTR_ASSERT_ROT(_Rotation), __arm_gen_u_Rd_Rm(_ARMINTR_UXTB16 | _ARMINTR_ENCODE_XTROT(_Rotation), (_Rn)))
#define _arm_uxth(_Rn, _Rotation)           (_ARMINTR_ASSERT_ROT(_Rotation), __arm_gen_u_Rd_Rm(_ARMINTR_UXTBH  | _ARMINTR_ENCODE_XTROT(_Rotation), (_Rn)))

#define _arm_pkhbt(_Rn, _Rm, _Lsl_imm) (_ARMINTR_ASSERT_RANGE((_Lsl_imm), 0, 31, "logical left shift"), \
    __arm_gen_s_Rd_RnRm(_ARMINTR_PKHBT | _ARMINTR_ENCODE_PKHSHIFT(_Lsl_imm), (_Rn), (_Rm)))
#define _arm_pkhtb(_Rn, _Rm, _Asr_imm) (_ARMINTR_ASSERT_RANGE((_Asr_imm), 1, 32, "arithmetic right shift"), \
    __arm_gen_s_Rd_RnRm(_ARMINTR_PKHTB | _ARMINTR_ENCODE_PKHSHIFT(_Asr_imm), (_Rn), (_Rm)))

#define _arm_usad8(_Rn, _Rm)                (__arm_gen_u_Rn_RmRs(_ARMINTR_USAD8, (_Rn), (_Rm)))

#define _arm_usada8(_Rn, _Rm, _Ra)          (__arm_gen_u_Rn_RmRsRd(_ARMINTR_USADA8, (_Rn), (_Rm), (_Ra)))

#define _arm_ssat(_Sat_imm, _Rn, _Shift_type, _Shift_imm) (_ARMINTR_ASSERT_SAT_BIT((_Sat_imm), 1, 32), _ARMINTR_ASSERT_SAT_SHIFT((_Shift_type), (_Shift_imm)), \
    _ARMINTR_S_DMx(_ARMINTR_SSAT | _ARMINTR_ENCODE_IMM5_16((_Sat_imm) - 1) | _ARMINTR_ENCODE_SAT_SH((_Shift_type), (_Shift_imm)), (_Rn)))
#define _arm_usat(_Sat_imm, _Rn, _Shift_type, _Shift_imm) (_ARMINTR_ASSERT_SAT_BIT((_Sat_imm), 0, 31), _ARMINTR_ASSERT_SAT_SHIFT((_Shift_type), (_Shift_imm)), \
    _ARMINTR_S_DMx(_ARMINTR_USAT | _ARMINTR_ENCODE_IMM5_16(_Sat_imm) | _ARMINTR_ENCODE_SAT_SH((_Shift_type), (_Shift_imm)), (_Rn)))

#define _arm_ssat16(_Sat_imm, _Rn) (_ARMINTR_ASSERT_SAT_BIT((_Sat_imm), 1, 16), \
    _ARMINTR_S_DMx(_ARMINTR_SSAT16 | _ARMINTR_ENCODE_IMM4_16((_Sat_imm) - 1), (_Rn)))
#define _arm_usat16(_Sat_imm, _Rn) (_ARMINTR_ASSERT_SAT_BIT((_Sat_imm), 0, 15), \
    _ARMINTR_S_DMx(_ARMINTR_USAT16 | _ARMINTR_ENCODE_IMM4_16(_Sat_imm), (_Rn)))

#define _arm_rev(_Rm)                       (__arm_gen_u_Rd_Rm2(_ARMINTR_REV, (_Rm)))
#define _arm_rev16(_Rm)                     (__arm_gen_u_Rd_Rm2(_ARMINTR_REV16, (_Rm)))
#define _arm_revsh(_Rm)                     (__arm_gen_s_Rd_Rm2(_ARMINTR_REVSH, (_Rm)))

#define _arm_smlad(_Rn, _Rm, _Ra)           (__arm_gen_s_Rn_RmRsRd(_ARMINTR_SMLAD,  (_Rn), (_Rm), (_Ra)))
#define _arm_smladx(_Rn, _Rm, _Ra)          (__arm_gen_s_Rn_RmRsRd(_ARMINTR_SMLADX, (_Rn), (_Rm), (_Ra)))
#define _arm_smlsd(_Rn, _Rm, _Ra)           (__arm_gen_s_Rn_RmRsRd(_ARMINTR_SMLSD,  (_Rn), (_Rm), (_Ra)))
#define _arm_smlsdx(_Rn, _Rm, _Ra)          (__arm_gen_s_Rn_RmRsRd(_ARMINTR_SMLSDX, (_Rn), (_Rm), (_Ra)))

#define _arm_smmla(_Rn, _Rm, _Ra)           (__arm_gen_s_Rn_RmRsRd(_ARMINTR_SMMLA,  (_Rn), (_Rm), (_Ra)))
#define _arm_smmlar(_Rn, _Rm, _Ra)          (__arm_gen_s_Rn_RmRsRd(_ARMINTR_SMMLAR, (_Rn), (_Rm), (_Ra)))
#define _arm_smmls(_Rn, _Rm, _Ra)           (__arm_gen_s_Rn_RmRsRd(_ARMINTR_SMMLS,  (_Rn), (_Rm), (_Ra)))
#define _arm_smmlsr(_Rn, _Rm, _Ra)          (__arm_gen_s_Rn_RmRsRd(_ARMINTR_SMMLSR, (_Rn), (_Rm), (_Ra)))

#define _arm_smmul(_Rn, _Rm)                (__arm_gen_s_Rn_RmRs(_ARMINTR_SMMUL,  (_Rn), (_Rm)))
#define _arm_smmulr(_Rn, _Rm)               (__arm_gen_s_Rn_RmRs(_ARMINTR_SMMULR, (_Rn), (_Rm)))

#define _arm_smlald(_RdHiLo, _Rn, _Rm)      (__arm_gen_s_Rdn_RdnRmRs(_ARMINTR_SMLALD,  (_RdHiLo), (_Rn), (_Rm)))
#define _arm_smlaldx(_RdHiLo, _Rn, _Rm)     (__arm_gen_s_Rdn_RdnRmRs(_ARMINTR_SMLALDX, (_RdHiLo), (_Rn), (_Rm)))
#define _arm_smlsld(_RdHiLo, _Rn, _Rm)      (__arm_gen_s_Rdn_RdnRmRs(_ARMINTR_SMLSLD,  (_RdHiLo), (_Rn), (_Rm)))
#define _arm_smlsldx(_RdHiLo, _Rn, _Rm)     (__arm_gen_s_Rdn_RdnRmRs(_ARMINTR_SMLSLDX, (_RdHiLo), (_Rn), (_Rm)))

#define _arm_smuad(_Rn, _Rm)                (__arm_gen_s_Rn_RmRs(_ARMINTR_SMUAD,  (_Rn), (_Rm)))
#define _arm_smuadx(_Rn, _Rm)               (__arm_gen_s_Rn_RmRs(_ARMINTR_SMUADX, (_Rn), (_Rm)))
#define _arm_smusd(_Rn, _Rm)                (__arm_gen_s_Rn_RmRs(_ARMINTR_SMUSD,  (_Rn), (_Rm)))
#define _arm_smusdx(_Rn, _Rm)               (__arm_gen_s_Rn_RmRs(_ARMINTR_SMUSDX, (_Rn), (_Rm)))

#define _arm_smull(_Rn, _Rm)                (__arm_gen_s_Rdn_RmRs(_ARMINTR_SMULL, (_Rn), (_Rm)))
#define _arm_umull(_Rn, _Rm)                (__arm_gen_u_Rdn_RmRs(_ARMINTR_UMULL, (_Rn), (_Rm)))

#define _arm_umaal(_RdLo, _RdHi, _Rn, _Rm)  (__arm_gen_u_Rdn_RdnRmRs(_ARMINTR_UMAAL, ((unsigned __int64)(_RdHi) << 32) | (_RdLo), (_Rn), (_Rm)))

/* ARMv6T2 */

#define _arm_bfc(_Rd, _Lsb, _Width) (_ARMINTR_ASSERT_RANGE((_Lsb), 0, 31, "least significant bit"), _ARMINTR_ASSERT_RANGE((_Width), 1, 32-(_Lsb), "width"), \
    __arm_gen_u_Rd_Rd(_ARMINTR_BFC | _ARMINTR_ENCODE_IMM5_7(_Lsb) | _ARMINTR_ENCODE_IMM5_16((_Lsb) + (_Width) - 1), (_Rd)))
#define _arm_bfi(_Rd, _Rn, _Lsb, _Width) (_ARMINTR_ASSERT_RANGE((_Lsb), 0, 31, "least significant bit"), _ARMINTR_ASSERT_RANGE((_Width), 1, 32-(_Lsb), "width"), \
    _ARMINTR_U_DDMx(_ARMINTR_BFI | _ARMINTR_ENCODE_IMM5_7(_Lsb) | _ARMINTR_ENCODE_IMM5_16((_Lsb) + (_Width) - 1), (_Rd), (_Rn)))

#define _arm_rbit(_Rm)                      (__arm_gen_u_Rd_Rm2(_ARMINTR_RBIT, (_Rm)))

#define _arm_sbfx(_Rn, _Lsb, _Width) (_ARMINTR_ASSERT_RANGE((_Lsb), 0, 31, "least significant bit"), _ARMINTR_ASSERT_RANGE((_Width), 1, 32-(_Lsb), "width"), \
    _ARMINTR_S_DMx(_ARMINTR_SBFX | _ARMINTR_ENCODE_IMM5_7(_Lsb) | _ARMINTR_ENCODE_IMM5_16((_Width) - 1), (_Rn)))
#define _arm_ubfx(_Rn, _Lsb, _Width) (_ARMINTR_ASSERT_RANGE((_Lsb), 0, 31, "least significant bit"), _ARMINTR_ASSERT_RANGE((_Width), 1, 32-(_Lsb), "width"), \
    _ARMINTR_U_DMx(_ARMINTR_UBFX | _ARMINTR_ENCODE_IMM5_7(_Lsb) | _ARMINTR_ENCODE_IMM5_16((_Width) - 1), (_Rn)))

/* ARMv7 */

#if defined (_M_THUMB)

#define _arm_sdiv(_Rn, _Rm)                 (__arm_gen_s_Rd_RnRm(_ARMINTR_SDIV, (_Rn), (_Rm)))
#define _arm_udiv(_Rn, _Rm)                 (__arm_gen_u_Rd_RnRm(_ARMINTR_UDIV, (_Rn), (_Rm)))

#endif  /* defined (_M_THUMB) */

typedef enum _tag_ARMINTR_CPS_OP
{
    _ARM_CPS_ENABLE_INTERRUPTS  = 1,
    _ARM_CPS_DISABLE_INTERRUPTS = 2,
    _ARM_CPS_CHANGE_MODE        = 4
}
_ARMINTR_CPS_OP;

typedef enum _tag_ARMINTR_CPS_FLAG
{
    _ARM_CPS_INTERRUPT_FLAG_F  = 1,
    _ARM_CPS_INTERRUPT_FLAG_I  = 2,
    _ARM_CPS_INTERRUPT_FLAG_A  = 4
}
_ARMINTR_CPS_FLAG;

void __cps(unsigned int _Ops, unsigned int _Flags, unsigned int _Mode);


typedef enum _tag_ARMINTR_BARRIER_TYPE
{
    _ARM_BARRIER_SY    = 0xF,
    _ARM_BARRIER_ST    = 0xE,
    _ARM_BARRIER_ISH   = 0xB,
    _ARM_BARRIER_ISHST = 0xA,
    _ARM_BARRIER_NSH   = 0x7,
    _ARM_BARRIER_NSHST = 0x6,
    _ARM_BARRIER_OSH   = 0x3,
    _ARM_BARRIER_OSHST = 0x2
}
_ARMINTR_BARRIER_TYPE;

void __dmb(unsigned int _Type);
void __dsb(unsigned int _Type);
void __isb(unsigned int _Type);

/* ARMv7VE */

typedef enum _tag_ARMINTR_BANKED_REG
{
    _ARM_BANKED_R8_USR   = 0x0,
    _ARM_BANKED_R9_USR   = 0x1,
    _ARM_BANKED_R10_USR  = 0x2,
    _ARM_BANKED_R11_USR  = 0x3,
    _ARM_BANKED_R12_USR  = 0x4,
    _ARM_BANKED_R13_USR  = 0x5,
    _ARM_BANKED_SP_USR   = 0x5,
    _ARM_BANKED_R14_USR  = 0x6,
    _ARM_BANKED_LR_USR   = 0x6,
    _ARM_BANKED_R8_FIQ   = 0x8,
    _ARM_BANKED_R9_FIQ   = 0x9,
    _ARM_BANKED_R10_FIQ  = 0xA,
    _ARM_BANKED_R11_FIQ  = 0xB,
    _ARM_BANKED_R12_FIQ  = 0xC,
    _ARM_BANKED_R13_FIQ  = 0xD,
    _ARM_BANKED_SP_FIQ   = 0xD,
    _ARM_BANKED_R14_FIQ  = 0xE,
    _ARM_BANKED_LR_FIQ   = 0xE,
    _ARM_BANKED_R14_IRQ  = 0x10,
    _ARM_BANKED_LR_IRQ   = 0x10,
    _ARM_BANKED_R13_IRQ  = 0x11,
    _ARM_BANKED_SP_IRQ   = 0x11,
    _ARM_BANKED_R14_SVC  = 0x12,
    _ARM_BANKED_LR_SVC   = 0x12,
    _ARM_BANKED_R13_SVC  = 0x13,
    _ARM_BANKED_SP_SVC   = 0x13,
    _ARM_BANKED_R14_ABT  = 0x14,
    _ARM_BANKED_LR_ABT   = 0x14,
    _ARM_BANKED_R13_ABT  = 0x15,
    _ARM_BANKED_SP_ABT   = 0x15,
    _ARM_BANKED_R14_UND  = 0x16,
    _ARM_BANKED_LR_UND   = 0x16,
    _ARM_BANKED_R13_UND  = 0x17,
    _ARM_BANKED_SP_UND   = 0x17,
    _ARM_BANKED_R14_MON  = 0x1C,
    _ARM_BANKED_LR_MON   = 0x1C,
    _ARM_BANKED_R13_MON  = 0x1D,
    _ARM_BANKED_SP_MON   = 0x1D,
    _ARM_BANKED_ELR_HYP  = 0x1E,
    _ARM_BANKED_R13_HYP  = 0x1F,
    _ARM_BANKED_SP_HYP   = 0x1F,
    _ARM_BANKED_SPSR_FIQ = 0x2E,
    _ARM_BANKED_SPSR_IRQ = 0x30,
    _ARM_BANKED_SPSR_SVC = 0x32,
    _ARM_BANKED_SPSR_ABT = 0x34,
    _ARM_BANKED_SPSR_UND = 0x36,
    _ARM_BANKED_SPSR_MON = 0x3C,
    _ARM_BANKED_SPSR_HYP = 0x3E
}
_ARMINTR_BANKED_REG;

void _WriteBankedReg(int _Value, int _Reg);
int _ReadBankedReg(int _Reg);

#ifdef __cplusplus
}
#endif  /* __cplusplus */
