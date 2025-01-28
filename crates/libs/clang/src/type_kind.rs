use super::*;

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq)]
pub struct TypeKind(i32);

impl std::fmt::Debug for TypeKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.spelling())
    }
}

impl std::fmt::Display for TypeKind {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "{}", self.spelling())
    }
}

impl TypeKind {
    pub fn spelling(&self) -> Owned<CXString> {
        link!("libclang.dll" "system" fn clang_getTypeKindSpelling(_: TypeKind) -> CXString);
        unsafe { Owned::new(clang_getTypeKindSpelling(*self)) }
    }

    pub const Invalid: Self = Self(0);
    pub const Unexposed: Self = Self(1);
    pub const Void: Self = Self(2);
    pub const Bool: Self = Self(3);
    pub const Char_U: Self = Self(4);
    pub const UChar: Self = Self(5);
    pub const Char16: Self = Self(6);
    pub const Char32: Self = Self(7);
    pub const UShort: Self = Self(8);
    pub const UInt: Self = Self(9);
    pub const ULong: Self = Self(10);
    pub const ULongLong: Self = Self(11);
    pub const UInt128: Self = Self(12);
    pub const Char_S: Self = Self(13);
    pub const SChar: Self = Self(14);
    pub const WChar: Self = Self(15);
    pub const Short: Self = Self(16);
    pub const Int: Self = Self(17);
    pub const Long: Self = Self(18);
    pub const LongLong: Self = Self(19);
    pub const Int128: Self = Self(20);
    pub const Float: Self = Self(21);
    pub const Double: Self = Self(22);
    pub const LongDouble: Self = Self(23);
    pub const NullPtr: Self = Self(24);
    pub const Overload: Self = Self(25);
    pub const Dependent: Self = Self(26);
    pub const ObjCId: Self = Self(27);
    pub const ObjCClass: Self = Self(28);
    pub const ObjCSel: Self = Self(29);
    pub const Float128: Self = Self(30);
    pub const Half: Self = Self(31);
    pub const Float16: Self = Self(32);
    pub const ShortAccum: Self = Self(33);
    pub const Accum: Self = Self(34);
    pub const LongAccum: Self = Self(35);
    pub const UShortAccum: Self = Self(36);
    pub const UAccum: Self = Self(37);
    pub const ULongAccum: Self = Self(38);
    pub const BFloat16: Self = Self(39);
    pub const Ibm128: Self = Self(40);
    pub const FirstBuiltin: Self = Self::Void;
    pub const LastBuiltin: Self = Self::Ibm128;
    pub const Complex: Self = Self(100);
    pub const Pointer: Self = Self(101);
    pub const BlockPointer: Self = Self(102);
    pub const LValueReference: Self = Self(103);
    pub const RValueReference: Self = Self(104);
    pub const Record: Self = Self(105);
    pub const Enum: Self = Self(106);
    pub const Typedef: Self = Self(107);
    pub const ObjCInterface: Self = Self(108);
    pub const ObjCObjectPointer: Self = Self(109);
    pub const FunctionNoProto: Self = Self(110);
    pub const FunctionProto: Self = Self(111);
    pub const ConstantArray: Self = Self(112);
    pub const Vector: Self = Self(113);
    pub const IncompleteArray: Self = Self(114);
    pub const VariableArray: Self = Self(115);
    pub const DependentSizedArray: Self = Self(116);
    pub const MemberPointer: Self = Self(117);
    pub const Auto: Self = Self(118);
    pub const Elaborated: Self = Self(119);
    pub const Pipe: Self = Self(120);
    pub const OCLImage1dRO: Self = Self(121);
    pub const OCLImage1dArrayRO: Self = Self(122);
    pub const OCLImage1dBufferRO: Self = Self(123);
    pub const OCLImage2dRO: Self = Self(124);
    pub const OCLImage2dArrayRO: Self = Self(125);
    pub const OCLImage2dDepthRO: Self = Self(126);
    pub const OCLImage2dArrayDepthRO: Self = Self(127);
    pub const OCLImage2dMSAARO: Self = Self(128);
    pub const OCLImage2dArrayMSAARO: Self = Self(129);
    pub const OCLImage2dMSAADepthRO: Self = Self(130);
    pub const OCLImage2dArrayMSAADepthRO: Self = Self(131);
    pub const OCLImage3dRO: Self = Self(132);
    pub const OCLImage1dWO: Self = Self(133);
    pub const OCLImage1dArrayWO: Self = Self(134);
    pub const OCLImage1dBufferWO: Self = Self(135);
    pub const OCLImage2dWO: Self = Self(136);
    pub const OCLImage2dArrayWO: Self = Self(137);
    pub const OCLImage2dDepthWO: Self = Self(138);
    pub const OCLImage2dArrayDepthWO: Self = Self(139);
    pub const OCLImage2dMSAAWO: Self = Self(140);
    pub const OCLImage2dArrayMSAAWO: Self = Self(141);
    pub const OCLImage2dMSAADepthWO: Self = Self(142);
    pub const OCLImage2dArrayMSAADepthWO: Self = Self(143);
    pub const OCLImage3dWO: Self = Self(144);
    pub const OCLImage1dRW: Self = Self(145);
    pub const OCLImage1dArrayRW: Self = Self(146);
    pub const OCLImage1dBufferRW: Self = Self(147);
    pub const OCLImage2dRW: Self = Self(148);
    pub const OCLImage2dArrayRW: Self = Self(149);
    pub const OCLImage2dDepthRW: Self = Self(150);
    pub const OCLImage2dArrayDepthRW: Self = Self(151);
    pub const OCLImage2dMSAARW: Self = Self(152);
    pub const OCLImage2dArrayMSAARW: Self = Self(153);
    pub const OCLImage2dMSAADepthRW: Self = Self(154);
    pub const OCLImage2dArrayMSAADepthRW: Self = Self(155);
    pub const OCLImage3dRW: Self = Self(156);
    pub const OCLSampler: Self = Self(157);
    pub const OCLEvent: Self = Self(158);
    pub const OCLQueue: Self = Self(159);
    pub const OCLReserveID: Self = Self(160);
    pub const ObjCObject: Self = Self(161);
    pub const ObjCTypeParam: Self = Self(162);
    pub const Attributed: Self = Self(163);
    pub const OCLIntelSubgroupAVCMcePayload: Self = Self(164);
    pub const OCLIntelSubgroupAVCImePayload: Self = Self(165);
    pub const OCLIntelSubgroupAVCRefPayload: Self = Self(166);
    pub const OCLIntelSubgroupAVCSicPayload: Self = Self(167);
    pub const OCLIntelSubgroupAVCMceResult: Self = Self(168);
    pub const OCLIntelSubgroupAVCImeResult: Self = Self(169);
    pub const OCLIntelSubgroupAVCRefResult: Self = Self(170);
    pub const OCLIntelSubgroupAVCSicResult: Self = Self(171);
    pub const OCLIntelSubgroupAVCImeResultSingleReferenceStreamout: Self = Self(172);
    pub const OCLIntelSubgroupAVCImeResultDualReferenceStreamout: Self = Self(173);
    pub const OCLIntelSubgroupAVCImeSingleReferenceStreamin: Self = Self(174);
    pub const OCLIntelSubgroupAVCImeDualReferenceStreamin: Self = Self(175);
    pub const OCLIntelSubgroupAVCImeResultSingleRefStreamout: Self = Self(172);
    pub const OCLIntelSubgroupAVCImeResultDualRefStreamout: Self = Self(173);
    pub const OCLIntelSubgroupAVCImeSingleRefStreamin: Self = Self(174);
    pub const OCLIntelSubgroupAVCImeDualRefStreamin: Self = Self(175);
    pub const ExtVector: Self = Self(176);
    pub const Atomic: Self = Self(177);
    pub const BTFTagAttributed: Self = Self(178);
}
