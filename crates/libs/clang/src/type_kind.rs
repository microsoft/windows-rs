use super::*;

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq)]
pub struct CXTypeKind(i32);

impl CXTypeKind {
    pub fn name(&self) -> Owned<CXString> {
        link!("libclang.dll" "system" fn clang_getTypeKindSpelling(_: CXTypeKind) -> CXString);
        unsafe { Owned::new(clang_getTypeKindSpelling(*self)) }
    }
}

impl std::fmt::Debug for CXTypeKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl std::fmt::Display for CXTypeKind {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "{}", self.name())
    }
}

pub const CXType_Invalid: CXTypeKind = CXTypeKind(0);
pub const CXType_Unexposed: CXTypeKind = CXTypeKind(1);
pub const CXType_Void: CXTypeKind = CXTypeKind(2);
pub const CXType_Bool: CXTypeKind = CXTypeKind(3);
pub const CXType_Char_U: CXTypeKind = CXTypeKind(4);
pub const CXType_UChar: CXTypeKind = CXTypeKind(5);
pub const CXType_Char16: CXTypeKind = CXTypeKind(6);
pub const CXType_Char32: CXTypeKind = CXTypeKind(7);
pub const CXType_UShort: CXTypeKind = CXTypeKind(8);
pub const CXType_UInt: CXTypeKind = CXTypeKind(9);
pub const CXType_ULong: CXTypeKind = CXTypeKind(10);
pub const CXType_ULongLong: CXTypeKind = CXTypeKind(11);
pub const CXType_UInt128: CXTypeKind = CXTypeKind(12);
pub const CXType_Char_S: CXTypeKind = CXTypeKind(13);
pub const CXType_SChar: CXTypeKind = CXTypeKind(14);
pub const CXType_WChar: CXTypeKind = CXTypeKind(15);
pub const CXType_Short: CXTypeKind = CXTypeKind(16);
pub const CXType_Int: CXTypeKind = CXTypeKind(17);
pub const CXType_Long: CXTypeKind = CXTypeKind(18);
pub const CXType_LongLong: CXTypeKind = CXTypeKind(19);
pub const CXType_Int128: CXTypeKind = CXTypeKind(20);
pub const CXType_Float: CXTypeKind = CXTypeKind(21);
pub const CXType_Double: CXTypeKind = CXTypeKind(22);
pub const CXType_LongDouble: CXTypeKind = CXTypeKind(23);
pub const CXType_NullPtr: CXTypeKind = CXTypeKind(24);
pub const CXType_Overload: CXTypeKind = CXTypeKind(25);
pub const CXType_Dependent: CXTypeKind = CXTypeKind(26);
pub const CXType_ObjCId: CXTypeKind = CXTypeKind(27);
pub const CXType_ObjCClass: CXTypeKind = CXTypeKind(28);
pub const CXType_ObjCSel: CXTypeKind = CXTypeKind(29);
pub const CXType_Float128: CXTypeKind = CXTypeKind(30);
pub const CXType_Half: CXTypeKind = CXTypeKind(31);
pub const CXType_Float16: CXTypeKind = CXTypeKind(32);
pub const CXType_ShortAccum: CXTypeKind = CXTypeKind(33);
pub const CXType_Accum: CXTypeKind = CXTypeKind(34);
pub const CXType_LongAccum: CXTypeKind = CXTypeKind(35);
pub const CXType_UShortAccum: CXTypeKind = CXTypeKind(36);
pub const CXType_UAccum: CXTypeKind = CXTypeKind(37);
pub const CXType_ULongAccum: CXTypeKind = CXTypeKind(38);
pub const CXType_BFloat16: CXTypeKind = CXTypeKind(39);
pub const CXType_Ibm128: CXTypeKind = CXTypeKind(40);
pub const CXType_FirstBuiltin: CXTypeKind = CXType_Void;
pub const CXType_LastBuiltin: CXTypeKind = CXType_Ibm128;
pub const CXType_Complex: CXTypeKind = CXTypeKind(100);
pub const CXType_Pointer: CXTypeKind = CXTypeKind(101);
pub const CXType_BlockPointer: CXTypeKind = CXTypeKind(102);
pub const CXType_LValueReference: CXTypeKind = CXTypeKind(103);
pub const CXType_RValueReference: CXTypeKind = CXTypeKind(104);
pub const CXType_Record: CXTypeKind = CXTypeKind(105);
pub const CXType_Enum: CXTypeKind = CXTypeKind(106);
pub const CXType_Typedef: CXTypeKind = CXTypeKind(107);
pub const CXType_ObjCInterface: CXTypeKind = CXTypeKind(108);
pub const CXType_ObjCObjectPointer: CXTypeKind = CXTypeKind(109);
pub const CXType_FunctionNoProto: CXTypeKind = CXTypeKind(110);
pub const CXType_FunctionProto: CXTypeKind = CXTypeKind(111);
pub const CXType_ConstantArray: CXTypeKind = CXTypeKind(112);
pub const CXType_Vector: CXTypeKind = CXTypeKind(113);
pub const CXType_IncompleteArray: CXTypeKind = CXTypeKind(114);
pub const CXType_VariableArray: CXTypeKind = CXTypeKind(115);
pub const CXType_DependentSizedArray: CXTypeKind = CXTypeKind(116);
pub const CXType_MemberPointer: CXTypeKind = CXTypeKind(117);
pub const CXType_Auto: CXTypeKind = CXTypeKind(118);
pub const CXType_Elaborated: CXTypeKind = CXTypeKind(119);
pub const CXType_Pipe: CXTypeKind = CXTypeKind(120);
pub const CXType_OCLImage1dRO: CXTypeKind = CXTypeKind(121);
pub const CXType_OCLImage1dArrayRO: CXTypeKind = CXTypeKind(122);
pub const CXType_OCLImage1dBufferRO: CXTypeKind = CXTypeKind(123);
pub const CXType_OCLImage2dRO: CXTypeKind = CXTypeKind(124);
pub const CXType_OCLImage2dArrayRO: CXTypeKind = CXTypeKind(125);
pub const CXType_OCLImage2dDepthRO: CXTypeKind = CXTypeKind(126);
pub const CXType_OCLImage2dArrayDepthRO: CXTypeKind = CXTypeKind(127);
pub const CXType_OCLImage2dMSAARO: CXTypeKind = CXTypeKind(128);
pub const CXType_OCLImage2dArrayMSAARO: CXTypeKind = CXTypeKind(129);
pub const CXType_OCLImage2dMSAADepthRO: CXTypeKind = CXTypeKind(130);
pub const CXType_OCLImage2dArrayMSAADepthRO: CXTypeKind = CXTypeKind(131);
pub const CXType_OCLImage3dRO: CXTypeKind = CXTypeKind(132);
pub const CXType_OCLImage1dWO: CXTypeKind = CXTypeKind(133);
pub const CXType_OCLImage1dArrayWO: CXTypeKind = CXTypeKind(134);
pub const CXType_OCLImage1dBufferWO: CXTypeKind = CXTypeKind(135);
pub const CXType_OCLImage2dWO: CXTypeKind = CXTypeKind(136);
pub const CXType_OCLImage2dArrayWO: CXTypeKind = CXTypeKind(137);
pub const CXType_OCLImage2dDepthWO: CXTypeKind = CXTypeKind(138);
pub const CXType_OCLImage2dArrayDepthWO: CXTypeKind = CXTypeKind(139);
pub const CXType_OCLImage2dMSAAWO: CXTypeKind = CXTypeKind(140);
pub const CXType_OCLImage2dArrayMSAAWO: CXTypeKind = CXTypeKind(141);
pub const CXType_OCLImage2dMSAADepthWO: CXTypeKind = CXTypeKind(142);
pub const CXType_OCLImage2dArrayMSAADepthWO: CXTypeKind = CXTypeKind(143);
pub const CXType_OCLImage3dWO: CXTypeKind = CXTypeKind(144);
pub const CXType_OCLImage1dRW: CXTypeKind = CXTypeKind(145);
pub const CXType_OCLImage1dArrayRW: CXTypeKind = CXTypeKind(146);
pub const CXType_OCLImage1dBufferRW: CXTypeKind = CXTypeKind(147);
pub const CXType_OCLImage2dRW: CXTypeKind = CXTypeKind(148);
pub const CXType_OCLImage2dArrayRW: CXTypeKind = CXTypeKind(149);
pub const CXType_OCLImage2dDepthRW: CXTypeKind = CXTypeKind(150);
pub const CXType_OCLImage2dArrayDepthRW: CXTypeKind = CXTypeKind(151);
pub const CXType_OCLImage2dMSAARW: CXTypeKind = CXTypeKind(152);
pub const CXType_OCLImage2dArrayMSAARW: CXTypeKind = CXTypeKind(153);
pub const CXType_OCLImage2dMSAADepthRW: CXTypeKind = CXTypeKind(154);
pub const CXType_OCLImage2dArrayMSAADepthRW: CXTypeKind = CXTypeKind(155);
pub const CXType_OCLImage3dRW: CXTypeKind = CXTypeKind(156);
pub const CXType_OCLSampler: CXTypeKind = CXTypeKind(157);
pub const CXType_OCLEvent: CXTypeKind = CXTypeKind(158);
pub const CXType_OCLQueue: CXTypeKind = CXTypeKind(159);
pub const CXType_OCLReserveID: CXTypeKind = CXTypeKind(160);
pub const CXType_ObjCObject: CXTypeKind = CXTypeKind(161);
pub const CXType_ObjCTypeParam: CXTypeKind = CXTypeKind(162);
pub const CXType_Attributed: CXTypeKind = CXTypeKind(163);
pub const CXType_OCLIntelSubgroupAVCMcePayload: CXTypeKind = CXTypeKind(164);
pub const CXType_OCLIntelSubgroupAVCImePayload: CXTypeKind = CXTypeKind(165);
pub const CXType_OCLIntelSubgroupAVCRefPayload: CXTypeKind = CXTypeKind(166);
pub const CXType_OCLIntelSubgroupAVCSicPayload: CXTypeKind = CXTypeKind(167);
pub const CXType_OCLIntelSubgroupAVCMceResult: CXTypeKind = CXTypeKind(168);
pub const CXType_OCLIntelSubgroupAVCImeResult: CXTypeKind = CXTypeKind(169);
pub const CXType_OCLIntelSubgroupAVCRefResult: CXTypeKind = CXTypeKind(170);
pub const CXType_OCLIntelSubgroupAVCSicResult: CXTypeKind = CXTypeKind(171);
pub const CXType_OCLIntelSubgroupAVCImeResultSingleReferenceStreamout: CXTypeKind = CXTypeKind(172);
pub const CXType_OCLIntelSubgroupAVCImeResultDualReferenceStreamout: CXTypeKind = CXTypeKind(173);
pub const CXType_OCLIntelSubgroupAVCImeSingleReferenceStreamin: CXTypeKind = CXTypeKind(174);
pub const CXType_OCLIntelSubgroupAVCImeDualReferenceStreamin: CXTypeKind = CXTypeKind(175);
pub const CXType_OCLIntelSubgroupAVCImeResultSingleRefStreamout: CXTypeKind = CXTypeKind(172);
pub const CXType_OCLIntelSubgroupAVCImeResultDualRefStreamout: CXTypeKind = CXTypeKind(173);
pub const CXType_OCLIntelSubgroupAVCImeSingleRefStreamin: CXTypeKind = CXTypeKind(174);
pub const CXType_OCLIntelSubgroupAVCImeDualRefStreamin: CXTypeKind = CXTypeKind(175);
pub const CXType_ExtVector: CXTypeKind = CXTypeKind(176);
pub const CXType_Atomic: CXTypeKind = CXTypeKind(177);
pub const CXType_BTFTagAttributed: CXTypeKind = CXTypeKind(178);
