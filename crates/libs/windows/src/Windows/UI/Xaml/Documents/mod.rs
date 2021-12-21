#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: 'UI_Xaml_Documents'*"]
#[repr(transparent)]
pub struct Block(::windows::core::IUnknown);
impl Block {
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn TextAlignment(&self) -> ::windows::core::Result<super::TextAlignment> {
        let this = self;
        unsafe {
            let mut result__: super::TextAlignment = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::TextAlignment>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn SetTextAlignment(&self, value: super::TextAlignment) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn LineHeight(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn SetLineHeight(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn LineStackingStrategy(&self) -> ::windows::core::Result<super::LineStackingStrategy> {
        let this = self;
        unsafe {
            let mut result__: super::LineStackingStrategy = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::LineStackingStrategy>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn SetLineStackingStrategy(&self, value: super::LineStackingStrategy) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn Margin(&self) -> ::windows::core::Result<super::Thickness> {
        let this = self;
        unsafe {
            let mut result__: super::Thickness = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Thickness>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn SetMargin<'a, Param0: ::windows::core::IntoParam<'a, super::Thickness>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn HorizontalTextAlignment(&self) -> ::windows::core::Result<super::TextAlignment> {
        let this = &::windows::core::Interface::cast::<IBlock2>(self)?;
        unsafe {
            let mut result__: super::TextAlignment = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::TextAlignment>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn SetHorizontalTextAlignment(&self, value: super::TextAlignment) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IBlock2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn TextAlignmentProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IBlockStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn LineHeightProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IBlockStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn LineStackingStrategyProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IBlockStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn MarginProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IBlockStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn HorizontalTextAlignmentProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IBlockStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IBlockStatics<R, F: FnOnce(&IBlockStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Block, IBlockStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IBlockStatics2<R, F: FnOnce(&IBlockStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Block, IBlockStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for Block {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Block {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Block {}
impl ::core::fmt::Debug for Block {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Block").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for Block {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Documents.Block;{4bce0016-dd47-4350-8cb0-e171600ac896})");
}
unsafe impl ::windows::core::Interface for Block {
    type Vtable = IBlockVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4bce0016_dd47_4350_8cb0_e171600ac896);
}
impl ::windows::core::RuntimeName for Block {
    const NAME: &'static str = "Windows.UI.Xaml.Documents.Block";
}
impl ::core::convert::From<Block> for ::windows::core::IUnknown {
    fn from(value: Block) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Block> for ::windows::core::IUnknown {
    fn from(value: &Block) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for Block {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &Block {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Block> for ::windows::core::IInspectable {
    fn from(value: Block) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Block> for ::windows::core::IInspectable {
    fn from(value: &Block) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for Block {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &Block {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Block> for TextElement {
    fn from(value: Block) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Block> for TextElement {
    fn from(value: &Block) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, TextElement> for Block {
    fn into_param(self) -> ::windows::core::Param<'a, TextElement> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, TextElement> for &Block {
    fn into_param(self) -> ::windows::core::Param<'a, TextElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<TextElement>::into(self))
    }
}
impl ::core::convert::From<Block> for super::DependencyObject {
    fn from(value: Block) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Block> for super::DependencyObject {
    fn from(value: &Block) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for Block {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &Block {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for Block {}
unsafe impl ::core::marker::Sync for Block {}
#[doc = "*Required features: 'UI_Xaml_Documents', 'Foundation_Collections'*"]
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
pub struct BlockCollection(::windows::core::IUnknown);
#[cfg(feature = "Foundation_Collections")]
impl BlockCollection {
    #[doc = "*Required features: 'UI_Xaml_Documents', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterator<Block>> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IIterable<Block>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IIterator<Block>>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAt(&self, index: u32) -> ::windows::core::Result<Block> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), index, &mut result__).from_abi::<Block>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetView(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<Block>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<Block>>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn IndexOf<'a, Param0: ::windows::core::IntoParam<'a, Block>>(&self, value: Param0, index: &mut u32) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi(), index, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetAt<'a, Param1: ::windows::core::IntoParam<'a, Block>>(&self, index: u32, value: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn InsertAt<'a, Param1: ::windows::core::IntoParam<'a, Block>>(&self, index: u32, value: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), index).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Append<'a, Param0: ::windows::core::IntoParam<'a, Block>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAtEnd(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, startindex: u32, items: &mut [<Block as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReplaceAll(&self, items: &[<Block as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), items.len() as u32, ::core::mem::transmute(items.as_ptr())).ok() }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::clone::Clone for BlockCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::PartialEq for BlockCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::Eq for BlockCollection {}
#[cfg(feature = "Foundation_Collections")]
impl ::core::fmt::Debug for BlockCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BlockCollection").field(&self.0).finish()
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::RuntimeType for BlockCollection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Documents.BlockCollection;pinterface({913337e9-11a1-4345-a3a2-4e7f956e222d};rc(Windows.UI.Xaml.Documents.Block;{4bce0016-dd47-4350-8cb0-e171600ac896})))");
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::Interface for BlockCollection {
    type Vtable = super::super::super::Foundation::Collections::IVectorVtbl<Block>;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_signature(<Self as ::windows::core::RuntimeType>::SIGNATURE);
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for BlockCollection {
    const NAME: &'static str = "Windows.UI.Xaml.Documents.BlockCollection";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for BlockCollection {
    type Item = Block;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &BlockCollection {
    type Item = Block;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::super::Foundation::Collections::VectorIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<BlockCollection> for ::windows::core::IUnknown {
    fn from(value: BlockCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&BlockCollection> for ::windows::core::IUnknown {
    fn from(value: &BlockCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BlockCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &BlockCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<BlockCollection> for ::windows::core::IInspectable {
    fn from(value: BlockCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&BlockCollection> for ::windows::core::IInspectable {
    fn from(value: &BlockCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BlockCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &BlockCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<BlockCollection> for super::super::super::Foundation::Collections::IIterable<Block> {
    type Error = ::windows::core::Error;
    fn try_from(value: BlockCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&BlockCollection> for super::super::super::Foundation::Collections::IIterable<Block> {
    type Error = ::windows::core::Error;
    fn try_from(value: &BlockCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<Block>> for BlockCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IIterable<Block>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<Block>> for &BlockCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IIterable<Block>> {
        ::core::convert::TryInto::<super::super::super::Foundation::Collections::IIterable<Block>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<BlockCollection> for super::super::super::Foundation::Collections::IVector<Block> {
    type Error = ::windows::core::Error;
    fn try_from(value: BlockCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&BlockCollection> for super::super::super::Foundation::Collections::IVector<Block> {
    type Error = ::windows::core::Error;
    fn try_from(value: &BlockCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IVector<Block>> for BlockCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IVector<Block>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IVector<Block>> for &BlockCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IVector<Block>> {
        ::core::convert::TryInto::<super::super::super::Foundation::Collections::IVector<Block>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Send for BlockCollection {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Sync for BlockCollection {}
#[doc = "*Required features: 'UI_Xaml_Documents'*"]
#[repr(transparent)]
pub struct Bold(::windows::core::IUnknown);
impl Bold {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Bold, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for Bold {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Bold {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Bold {}
impl ::core::fmt::Debug for Bold {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Bold").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for Bold {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Documents.Bold;{ade73784-1b59-4da4-bb23-0f20e885b4bf})");
}
unsafe impl ::windows::core::Interface for Bold {
    type Vtable = IBoldVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xade73784_1b59_4da4_bb23_0f20e885b4bf);
}
impl ::windows::core::RuntimeName for Bold {
    const NAME: &'static str = "Windows.UI.Xaml.Documents.Bold";
}
impl ::core::convert::From<Bold> for ::windows::core::IUnknown {
    fn from(value: Bold) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Bold> for ::windows::core::IUnknown {
    fn from(value: &Bold) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for Bold {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &Bold {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Bold> for ::windows::core::IInspectable {
    fn from(value: Bold) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Bold> for ::windows::core::IInspectable {
    fn from(value: &Bold) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for Bold {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &Bold {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Bold> for Span {
    fn from(value: Bold) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Bold> for Span {
    fn from(value: &Bold) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Span> for Bold {
    fn into_param(self) -> ::windows::core::Param<'a, Span> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, Span> for &Bold {
    fn into_param(self) -> ::windows::core::Param<'a, Span> {
        ::windows::core::Param::Owned(::core::convert::Into::<Span>::into(self))
    }
}
impl ::core::convert::From<Bold> for Inline {
    fn from(value: Bold) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Bold> for Inline {
    fn from(value: &Bold) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Inline> for Bold {
    fn into_param(self) -> ::windows::core::Param<'a, Inline> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, Inline> for &Bold {
    fn into_param(self) -> ::windows::core::Param<'a, Inline> {
        ::windows::core::Param::Owned(::core::convert::Into::<Inline>::into(self))
    }
}
impl ::core::convert::From<Bold> for TextElement {
    fn from(value: Bold) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Bold> for TextElement {
    fn from(value: &Bold) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, TextElement> for Bold {
    fn into_param(self) -> ::windows::core::Param<'a, TextElement> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, TextElement> for &Bold {
    fn into_param(self) -> ::windows::core::Param<'a, TextElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<TextElement>::into(self))
    }
}
impl ::core::convert::From<Bold> for super::DependencyObject {
    fn from(value: Bold) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Bold> for super::DependencyObject {
    fn from(value: &Bold) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for Bold {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &Bold {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for Bold {}
unsafe impl ::core::marker::Sync for Bold {}
#[doc = "*Required features: 'UI_Xaml_Documents'*"]
#[repr(transparent)]
pub struct ContactContentLinkProvider(::windows::core::IUnknown);
impl ContactContentLinkProvider {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ContactContentLinkProvider, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for ContactContentLinkProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ContactContentLinkProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContactContentLinkProvider {}
impl ::core::fmt::Debug for ContactContentLinkProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactContentLinkProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ContactContentLinkProvider {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Documents.ContactContentLinkProvider;{f92fd29b-589b-4abd-9d37-35a1468f021e})");
}
unsafe impl ::windows::core::Interface for ContactContentLinkProvider {
    type Vtable = IContactContentLinkProviderVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf92fd29b_589b_4abd_9d37_35a1468f021e);
}
impl ::windows::core::RuntimeName for ContactContentLinkProvider {
    const NAME: &'static str = "Windows.UI.Xaml.Documents.ContactContentLinkProvider";
}
impl ::core::convert::From<ContactContentLinkProvider> for ::windows::core::IUnknown {
    fn from(value: ContactContentLinkProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContactContentLinkProvider> for ::windows::core::IUnknown {
    fn from(value: &ContactContentLinkProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ContactContentLinkProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ContactContentLinkProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ContactContentLinkProvider> for ::windows::core::IInspectable {
    fn from(value: ContactContentLinkProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContactContentLinkProvider> for ::windows::core::IInspectable {
    fn from(value: &ContactContentLinkProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ContactContentLinkProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &ContactContentLinkProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ContactContentLinkProvider> for ContentLinkProvider {
    fn from(value: ContactContentLinkProvider) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&ContactContentLinkProvider> for ContentLinkProvider {
    fn from(value: &ContactContentLinkProvider) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ContentLinkProvider> for ContactContentLinkProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ContentLinkProvider> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ContentLinkProvider> for &ContactContentLinkProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ContentLinkProvider> {
        ::windows::core::Param::Owned(::core::convert::Into::<ContentLinkProvider>::into(self))
    }
}
impl ::core::convert::From<ContactContentLinkProvider> for super::DependencyObject {
    fn from(value: ContactContentLinkProvider) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&ContactContentLinkProvider> for super::DependencyObject {
    fn from(value: &ContactContentLinkProvider) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for ContactContentLinkProvider {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &ContactContentLinkProvider {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for ContactContentLinkProvider {}
unsafe impl ::core::marker::Sync for ContactContentLinkProvider {}
#[doc = "*Required features: 'UI_Xaml_Documents'*"]
#[repr(transparent)]
pub struct ContentLink(::windows::core::IUnknown);
impl ContentLink {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ContentLink, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents', 'UI_Text'*"]
    #[cfg(feature = "UI_Text")]
    pub fn Info(&self) -> ::windows::core::Result<super::super::Text::ContentLinkInfo> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Text::ContentLinkInfo>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents', 'UI_Text'*"]
    #[cfg(feature = "UI_Text")]
    pub fn SetInfo<'a, Param0: ::windows::core::IntoParam<'a, super::super::Text::ContentLinkInfo>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents', 'UI_Xaml_Media'*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn Background(&self) -> ::windows::core::Result<super::Media::Brush> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Media::Brush>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents', 'UI_Xaml_Media'*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetBackground<'a, Param0: ::windows::core::IntoParam<'a, super::Media::Brush>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents', 'UI_Core'*"]
    #[cfg(feature = "UI_Core")]
    pub fn Cursor(&self) -> ::windows::core::Result<super::super::Core::CoreCursorType> {
        let this = self;
        unsafe {
            let mut result__: super::super::Core::CoreCursorType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Core::CoreCursorType>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents', 'UI_Core'*"]
    #[cfg(feature = "UI_Core")]
    pub fn SetCursor(&self, value: super::super::Core::CoreCursorType) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn XYFocusLeft(&self) -> ::windows::core::Result<super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyObject>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn SetXYFocusLeft<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn XYFocusRight(&self) -> ::windows::core::Result<super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyObject>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn SetXYFocusRight<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn XYFocusUp(&self) -> ::windows::core::Result<super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyObject>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn SetXYFocusUp<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn XYFocusDown(&self) -> ::windows::core::Result<super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyObject>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn SetXYFocusDown<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn ElementSoundMode(&self) -> ::windows::core::Result<super::ElementSoundMode> {
        let this = self;
        unsafe {
            let mut result__: super::ElementSoundMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::ElementSoundMode>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn SetElementSoundMode(&self, value: super::ElementSoundMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn FocusState(&self) -> ::windows::core::Result<super::FocusState> {
        let this = self;
        unsafe {
            let mut result__: super::FocusState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::FocusState>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents', 'UI_Xaml_Input'*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn XYFocusUpNavigationStrategy(&self) -> ::windows::core::Result<super::Input::XYFocusNavigationStrategy> {
        let this = self;
        unsafe {
            let mut result__: super::Input::XYFocusNavigationStrategy = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Input::XYFocusNavigationStrategy>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents', 'UI_Xaml_Input'*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetXYFocusUpNavigationStrategy(&self, value: super::Input::XYFocusNavigationStrategy) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents', 'UI_Xaml_Input'*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn XYFocusDownNavigationStrategy(&self) -> ::windows::core::Result<super::Input::XYFocusNavigationStrategy> {
        let this = self;
        unsafe {
            let mut result__: super::Input::XYFocusNavigationStrategy = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Input::XYFocusNavigationStrategy>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents', 'UI_Xaml_Input'*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetXYFocusDownNavigationStrategy(&self, value: super::Input::XYFocusNavigationStrategy) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents', 'UI_Xaml_Input'*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn XYFocusLeftNavigationStrategy(&self) -> ::windows::core::Result<super::Input::XYFocusNavigationStrategy> {
        let this = self;
        unsafe {
            let mut result__: super::Input::XYFocusNavigationStrategy = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).27)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Input::XYFocusNavigationStrategy>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents', 'UI_Xaml_Input'*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetXYFocusLeftNavigationStrategy(&self, value: super::Input::XYFocusNavigationStrategy) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).28)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents', 'UI_Xaml_Input'*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn XYFocusRightNavigationStrategy(&self) -> ::windows::core::Result<super::Input::XYFocusNavigationStrategy> {
        let this = self;
        unsafe {
            let mut result__: super::Input::XYFocusNavigationStrategy = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).29)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Input::XYFocusNavigationStrategy>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents', 'UI_Xaml_Input'*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetXYFocusRightNavigationStrategy(&self, value: super::Input::XYFocusNavigationStrategy) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).30)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn IsTabStop(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).31)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn SetIsTabStop(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).32)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn TabIndex(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).33)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn SetTabIndex(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).34)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Invoked<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<ContentLink, ContentLinkInvokedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).35)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveInvoked<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).36)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn GotFocus<'a, Param0: ::windows::core::IntoParam<'a, super::RoutedEventHandler>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).37)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveGotFocus<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).38)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn LostFocus<'a, Param0: ::windows::core::IntoParam<'a, super::RoutedEventHandler>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).39)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveLostFocus<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).40)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn Focus(&self, value: super::FocusState) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).41)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn BackgroundProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IContentLinkStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn CursorProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IContentLinkStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn XYFocusLeftProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IContentLinkStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn XYFocusRightProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IContentLinkStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn XYFocusUpProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IContentLinkStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn XYFocusDownProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IContentLinkStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn ElementSoundModeProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IContentLinkStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn FocusStateProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IContentLinkStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn XYFocusUpNavigationStrategyProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IContentLinkStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn XYFocusDownNavigationStrategyProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IContentLinkStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn XYFocusLeftNavigationStrategyProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IContentLinkStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn XYFocusRightNavigationStrategyProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IContentLinkStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn IsTabStopProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IContentLinkStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn TabIndexProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IContentLinkStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IContentLinkStatics<R, F: FnOnce(&IContentLinkStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ContentLink, IContentLinkStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for ContentLink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ContentLink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContentLink {}
impl ::core::fmt::Debug for ContentLink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContentLink").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ContentLink {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Documents.ContentLink;{6c60c3e1-528c-42f8-92be-34b8c68be304})");
}
unsafe impl ::windows::core::Interface for ContentLink {
    type Vtable = IContentLinkVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6c60c3e1_528c_42f8_92be_34b8c68be304);
}
impl ::windows::core::RuntimeName for ContentLink {
    const NAME: &'static str = "Windows.UI.Xaml.Documents.ContentLink";
}
impl ::core::convert::From<ContentLink> for ::windows::core::IUnknown {
    fn from(value: ContentLink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContentLink> for ::windows::core::IUnknown {
    fn from(value: &ContentLink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ContentLink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ContentLink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ContentLink> for ::windows::core::IInspectable {
    fn from(value: ContentLink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContentLink> for ::windows::core::IInspectable {
    fn from(value: &ContentLink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ContentLink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &ContentLink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ContentLink> for Inline {
    fn from(value: ContentLink) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&ContentLink> for Inline {
    fn from(value: &ContentLink) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Inline> for ContentLink {
    fn into_param(self) -> ::windows::core::Param<'a, Inline> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, Inline> for &ContentLink {
    fn into_param(self) -> ::windows::core::Param<'a, Inline> {
        ::windows::core::Param::Owned(::core::convert::Into::<Inline>::into(self))
    }
}
impl ::core::convert::From<ContentLink> for TextElement {
    fn from(value: ContentLink) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&ContentLink> for TextElement {
    fn from(value: &ContentLink) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, TextElement> for ContentLink {
    fn into_param(self) -> ::windows::core::Param<'a, TextElement> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, TextElement> for &ContentLink {
    fn into_param(self) -> ::windows::core::Param<'a, TextElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<TextElement>::into(self))
    }
}
impl ::core::convert::From<ContentLink> for super::DependencyObject {
    fn from(value: ContentLink) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&ContentLink> for super::DependencyObject {
    fn from(value: &ContentLink) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for ContentLink {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &ContentLink {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for ContentLink {}
unsafe impl ::core::marker::Sync for ContentLink {}
#[doc = "*Required features: 'UI_Xaml_Documents'*"]
#[repr(transparent)]
pub struct ContentLinkInvokedEventArgs(::windows::core::IUnknown);
impl ContentLinkInvokedEventArgs {
    #[doc = "*Required features: 'UI_Xaml_Documents', 'UI_Text'*"]
    #[cfg(feature = "UI_Text")]
    pub fn ContentLinkInfo(&self) -> ::windows::core::Result<super::super::Text::ContentLinkInfo> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Text::ContentLinkInfo>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
}
impl ::core::clone::Clone for ContentLinkInvokedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ContentLinkInvokedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContentLinkInvokedEventArgs {}
impl ::core::fmt::Debug for ContentLinkInvokedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContentLinkInvokedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ContentLinkInvokedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Documents.ContentLinkInvokedEventArgs;{546717c1-e8df-4593-9639-97595fdf8310})");
}
unsafe impl ::windows::core::Interface for ContentLinkInvokedEventArgs {
    type Vtable = IContentLinkInvokedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x546717c1_e8df_4593_9639_97595fdf8310);
}
impl ::windows::core::RuntimeName for ContentLinkInvokedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Documents.ContentLinkInvokedEventArgs";
}
impl ::core::convert::From<ContentLinkInvokedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ContentLinkInvokedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContentLinkInvokedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ContentLinkInvokedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ContentLinkInvokedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ContentLinkInvokedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ContentLinkInvokedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ContentLinkInvokedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContentLinkInvokedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ContentLinkInvokedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ContentLinkInvokedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &ContentLinkInvokedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ContentLinkInvokedEventArgs {}
unsafe impl ::core::marker::Sync for ContentLinkInvokedEventArgs {}
#[doc = "*Required features: 'UI_Xaml_Documents'*"]
#[repr(transparent)]
pub struct ContentLinkProvider(::windows::core::IUnknown);
impl ContentLinkProvider {}
impl ::core::clone::Clone for ContentLinkProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ContentLinkProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContentLinkProvider {}
impl ::core::fmt::Debug for ContentLinkProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContentLinkProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ContentLinkProvider {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Documents.ContentLinkProvider;{730587fd-bfdc-4cb3-904d-b65ab339bbf5})");
}
unsafe impl ::windows::core::Interface for ContentLinkProvider {
    type Vtable = IContentLinkProviderVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x730587fd_bfdc_4cb3_904d_b65ab339bbf5);
}
impl ::windows::core::RuntimeName for ContentLinkProvider {
    const NAME: &'static str = "Windows.UI.Xaml.Documents.ContentLinkProvider";
}
impl ::core::convert::From<ContentLinkProvider> for ::windows::core::IUnknown {
    fn from(value: ContentLinkProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContentLinkProvider> for ::windows::core::IUnknown {
    fn from(value: &ContentLinkProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ContentLinkProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ContentLinkProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ContentLinkProvider> for ::windows::core::IInspectable {
    fn from(value: ContentLinkProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContentLinkProvider> for ::windows::core::IInspectable {
    fn from(value: &ContentLinkProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ContentLinkProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &ContentLinkProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ContentLinkProvider> for super::DependencyObject {
    fn from(value: ContentLinkProvider) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&ContentLinkProvider> for super::DependencyObject {
    fn from(value: &ContentLinkProvider) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for ContentLinkProvider {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &ContentLinkProvider {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for ContentLinkProvider {}
unsafe impl ::core::marker::Sync for ContentLinkProvider {}
#[doc = "*Required features: 'UI_Xaml_Documents'*"]
#[repr(transparent)]
pub struct ContentLinkProviderCollection(::windows::core::IUnknown);
impl ContentLinkProviderCollection {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ContentLinkProviderCollection, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterator<ContentLinkProvider>> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IIterable<ContentLinkProvider>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IIterator<ContentLinkProvider>>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAt(&self, index: u32) -> ::windows::core::Result<ContentLinkProvider> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<ContentLinkProvider>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), index, &mut result__).from_abi::<ContentLinkProvider>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<ContentLinkProvider>>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetView(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<ContentLinkProvider>> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<ContentLinkProvider>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<ContentLinkProvider>>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn IndexOf<'a, Param0: ::windows::core::IntoParam<'a, ContentLinkProvider>>(&self, value: Param0, index: &mut u32) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<ContentLinkProvider>>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi(), index, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetAt<'a, Param1: ::windows::core::IntoParam<'a, ContentLinkProvider>>(&self, index: u32, value: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<ContentLinkProvider>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn InsertAt<'a, Param1: ::windows::core::IntoParam<'a, ContentLinkProvider>>(&self, index: u32, value: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<ContentLinkProvider>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<ContentLinkProvider>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), index).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Append<'a, Param0: ::windows::core::IntoParam<'a, ContentLinkProvider>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<ContentLinkProvider>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAtEnd(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<ContentLinkProvider>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<ContentLinkProvider>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, startindex: u32, items: &mut [<ContentLinkProvider as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<ContentLinkProvider>>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReplaceAll(&self, items: &[<ContentLinkProvider as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<ContentLinkProvider>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), items.len() as u32, ::core::mem::transmute(items.as_ptr())).ok() }
    }
}
impl ::core::clone::Clone for ContentLinkProviderCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ContentLinkProviderCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContentLinkProviderCollection {}
impl ::core::fmt::Debug for ContentLinkProviderCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContentLinkProviderCollection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ContentLinkProviderCollection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Documents.ContentLinkProviderCollection;{f5b84d0c-a9f4-4d1a-a13c-10def1843734})");
}
unsafe impl ::windows::core::Interface for ContentLinkProviderCollection {
    type Vtable = IContentLinkProviderCollectionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf5b84d0c_a9f4_4d1a_a13c_10def1843734);
}
impl ::windows::core::RuntimeName for ContentLinkProviderCollection {
    const NAME: &'static str = "Windows.UI.Xaml.Documents.ContentLinkProviderCollection";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for ContentLinkProviderCollection {
    type Item = ContentLinkProvider;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &ContentLinkProviderCollection {
    type Item = ContentLinkProvider;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::super::Foundation::Collections::VectorIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
impl ::core::convert::From<ContentLinkProviderCollection> for ::windows::core::IUnknown {
    fn from(value: ContentLinkProviderCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContentLinkProviderCollection> for ::windows::core::IUnknown {
    fn from(value: &ContentLinkProviderCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ContentLinkProviderCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ContentLinkProviderCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ContentLinkProviderCollection> for ::windows::core::IInspectable {
    fn from(value: ContentLinkProviderCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContentLinkProviderCollection> for ::windows::core::IInspectable {
    fn from(value: &ContentLinkProviderCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ContentLinkProviderCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &ContentLinkProviderCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<ContentLinkProviderCollection> for super::super::super::Foundation::Collections::IIterable<ContentLinkProvider> {
    type Error = ::windows::core::Error;
    fn try_from(value: ContentLinkProviderCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&ContentLinkProviderCollection> for super::super::super::Foundation::Collections::IIterable<ContentLinkProvider> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ContentLinkProviderCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<ContentLinkProvider>> for ContentLinkProviderCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IIterable<ContentLinkProvider>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<ContentLinkProvider>> for &ContentLinkProviderCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IIterable<ContentLinkProvider>> {
        ::core::convert::TryInto::<super::super::super::Foundation::Collections::IIterable<ContentLinkProvider>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<ContentLinkProviderCollection> for super::super::super::Foundation::Collections::IVector<ContentLinkProvider> {
    type Error = ::windows::core::Error;
    fn try_from(value: ContentLinkProviderCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&ContentLinkProviderCollection> for super::super::super::Foundation::Collections::IVector<ContentLinkProvider> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ContentLinkProviderCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IVector<ContentLinkProvider>> for ContentLinkProviderCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IVector<ContentLinkProvider>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IVector<ContentLinkProvider>> for &ContentLinkProviderCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IVector<ContentLinkProvider>> {
        ::core::convert::TryInto::<super::super::super::Foundation::Collections::IVector<ContentLinkProvider>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for ContentLinkProviderCollection {}
unsafe impl ::core::marker::Sync for ContentLinkProviderCollection {}
#[doc = "*Required features: 'UI_Xaml_Documents'*"]
#[repr(transparent)]
pub struct Glyphs(::windows::core::IUnknown);
impl Glyphs {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Glyphs, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn UnicodeString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn SetUnicodeString<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn Indices(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn SetIndices<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn FontUri(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetFontUri<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents', 'UI_Xaml_Media'*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn StyleSimulations(&self) -> ::windows::core::Result<super::Media::StyleSimulations> {
        let this = self;
        unsafe {
            let mut result__: super::Media::StyleSimulations = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Media::StyleSimulations>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents', 'UI_Xaml_Media'*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetStyleSimulations(&self, value: super::Media::StyleSimulations) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn FontRenderingEmSize(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn SetFontRenderingEmSize(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn OriginX(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn SetOriginX(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn OriginY(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn SetOriginY(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents', 'UI_Xaml_Media'*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn Fill(&self) -> ::windows::core::Result<super::Media::Brush> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Media::Brush>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents', 'UI_Xaml_Media'*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetFill<'a, Param0: ::windows::core::IntoParam<'a, super::Media::Brush>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn IsColorFontEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IGlyphs2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn SetIsColorFontEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IGlyphs2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn ColorFontPaletteIndex(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<IGlyphs2>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn SetColorFontPaletteIndex(&self, value: i32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IGlyphs2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn UnicodeStringProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IGlyphsStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn IndicesProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IGlyphsStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn FontUriProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IGlyphsStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn StyleSimulationsProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IGlyphsStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn FontRenderingEmSizeProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IGlyphsStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn OriginXProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IGlyphsStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn OriginYProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IGlyphsStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn FillProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IGlyphsStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn IsColorFontEnabledProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IGlyphsStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn ColorFontPaletteIndexProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IGlyphsStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IGlyphsStatics<R, F: FnOnce(&IGlyphsStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Glyphs, IGlyphsStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IGlyphsStatics2<R, F: FnOnce(&IGlyphsStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Glyphs, IGlyphsStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for Glyphs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Glyphs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Glyphs {}
impl ::core::fmt::Debug for Glyphs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Glyphs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for Glyphs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Documents.Glyphs;{d079498b-f2b1-4281-99a2-e4d05932b2b5})");
}
unsafe impl ::windows::core::Interface for Glyphs {
    type Vtable = IGlyphsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd079498b_f2b1_4281_99a2_e4d05932b2b5);
}
impl ::windows::core::RuntimeName for Glyphs {
    const NAME: &'static str = "Windows.UI.Xaml.Documents.Glyphs";
}
impl ::core::convert::From<Glyphs> for ::windows::core::IUnknown {
    fn from(value: Glyphs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Glyphs> for ::windows::core::IUnknown {
    fn from(value: &Glyphs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for Glyphs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &Glyphs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Glyphs> for ::windows::core::IInspectable {
    fn from(value: Glyphs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Glyphs> for ::windows::core::IInspectable {
    fn from(value: &Glyphs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for Glyphs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &Glyphs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<Glyphs> for super::super::Composition::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: Glyphs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&Glyphs> for super::super::Composition::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: &Glyphs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Composition::IAnimationObject> for Glyphs {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Composition::IAnimationObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Composition::IAnimationObject> for &Glyphs {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Composition::IAnimationObject> {
        ::core::convert::TryInto::<super::super::Composition::IAnimationObject>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<Glyphs> for super::super::Composition::IVisualElement {
    type Error = ::windows::core::Error;
    fn try_from(value: Glyphs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&Glyphs> for super::super::Composition::IVisualElement {
    type Error = ::windows::core::Error;
    fn try_from(value: &Glyphs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Composition::IVisualElement> for Glyphs {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Composition::IVisualElement> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Composition::IVisualElement> for &Glyphs {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Composition::IVisualElement> {
        ::core::convert::TryInto::<super::super::Composition::IVisualElement>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::From<Glyphs> for super::FrameworkElement {
    fn from(value: Glyphs) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Glyphs> for super::FrameworkElement {
    fn from(value: &Glyphs) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::FrameworkElement> for Glyphs {
    fn into_param(self) -> ::windows::core::Param<'a, super::FrameworkElement> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::FrameworkElement> for &Glyphs {
    fn into_param(self) -> ::windows::core::Param<'a, super::FrameworkElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::FrameworkElement>::into(self))
    }
}
impl ::core::convert::From<Glyphs> for super::UIElement {
    fn from(value: Glyphs) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Glyphs> for super::UIElement {
    fn from(value: &Glyphs) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::UIElement> for Glyphs {
    fn into_param(self) -> ::windows::core::Param<'a, super::UIElement> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::UIElement> for &Glyphs {
    fn into_param(self) -> ::windows::core::Param<'a, super::UIElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::UIElement>::into(self))
    }
}
impl ::core::convert::From<Glyphs> for super::DependencyObject {
    fn from(value: Glyphs) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Glyphs> for super::DependencyObject {
    fn from(value: &Glyphs) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for Glyphs {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &Glyphs {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for Glyphs {}
unsafe impl ::core::marker::Sync for Glyphs {}
#[doc = "*Required features: 'UI_Xaml_Documents'*"]
#[repr(transparent)]
pub struct Hyperlink(::windows::core::IUnknown);
impl Hyperlink {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Hyperlink, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn NavigateUri(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetNavigateUri<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Click<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<Hyperlink, HyperlinkClickEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveClick<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn UnderlineStyle(&self) -> ::windows::core::Result<UnderlineStyle> {
        let this = &::windows::core::Interface::cast::<IHyperlink2>(self)?;
        unsafe {
            let mut result__: UnderlineStyle = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UnderlineStyle>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn SetUnderlineStyle(&self, value: UnderlineStyle) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IHyperlink2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn XYFocusLeft(&self) -> ::windows::core::Result<super::DependencyObject> {
        let this = &::windows::core::Interface::cast::<IHyperlink3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyObject>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn SetXYFocusLeft<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IHyperlink3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn XYFocusRight(&self) -> ::windows::core::Result<super::DependencyObject> {
        let this = &::windows::core::Interface::cast::<IHyperlink3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyObject>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn SetXYFocusRight<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IHyperlink3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn XYFocusUp(&self) -> ::windows::core::Result<super::DependencyObject> {
        let this = &::windows::core::Interface::cast::<IHyperlink3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyObject>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn SetXYFocusUp<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IHyperlink3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn XYFocusDown(&self) -> ::windows::core::Result<super::DependencyObject> {
        let this = &::windows::core::Interface::cast::<IHyperlink3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyObject>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn SetXYFocusDown<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IHyperlink3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn ElementSoundMode(&self) -> ::windows::core::Result<super::ElementSoundMode> {
        let this = &::windows::core::Interface::cast::<IHyperlink3>(self)?;
        unsafe {
            let mut result__: super::ElementSoundMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::ElementSoundMode>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn SetElementSoundMode(&self, value: super::ElementSoundMode) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IHyperlink3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn FocusState(&self) -> ::windows::core::Result<super::FocusState> {
        let this = &::windows::core::Interface::cast::<IHyperlink4>(self)?;
        unsafe {
            let mut result__: super::FocusState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::FocusState>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents', 'UI_Xaml_Input'*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn XYFocusUpNavigationStrategy(&self) -> ::windows::core::Result<super::Input::XYFocusNavigationStrategy> {
        let this = &::windows::core::Interface::cast::<IHyperlink4>(self)?;
        unsafe {
            let mut result__: super::Input::XYFocusNavigationStrategy = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Input::XYFocusNavigationStrategy>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents', 'UI_Xaml_Input'*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetXYFocusUpNavigationStrategy(&self, value: super::Input::XYFocusNavigationStrategy) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IHyperlink4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents', 'UI_Xaml_Input'*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn XYFocusDownNavigationStrategy(&self) -> ::windows::core::Result<super::Input::XYFocusNavigationStrategy> {
        let this = &::windows::core::Interface::cast::<IHyperlink4>(self)?;
        unsafe {
            let mut result__: super::Input::XYFocusNavigationStrategy = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Input::XYFocusNavigationStrategy>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents', 'UI_Xaml_Input'*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetXYFocusDownNavigationStrategy(&self, value: super::Input::XYFocusNavigationStrategy) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IHyperlink4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents', 'UI_Xaml_Input'*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn XYFocusLeftNavigationStrategy(&self) -> ::windows::core::Result<super::Input::XYFocusNavigationStrategy> {
        let this = &::windows::core::Interface::cast::<IHyperlink4>(self)?;
        unsafe {
            let mut result__: super::Input::XYFocusNavigationStrategy = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Input::XYFocusNavigationStrategy>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents', 'UI_Xaml_Input'*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetXYFocusLeftNavigationStrategy(&self, value: super::Input::XYFocusNavigationStrategy) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IHyperlink4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents', 'UI_Xaml_Input'*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn XYFocusRightNavigationStrategy(&self) -> ::windows::core::Result<super::Input::XYFocusNavigationStrategy> {
        let this = &::windows::core::Interface::cast::<IHyperlink4>(self)?;
        unsafe {
            let mut result__: super::Input::XYFocusNavigationStrategy = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Input::XYFocusNavigationStrategy>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents', 'UI_Xaml_Input'*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetXYFocusRightNavigationStrategy(&self, value: super::Input::XYFocusNavigationStrategy) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IHyperlink4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn GotFocus<'a, Param0: ::windows::core::IntoParam<'a, super::RoutedEventHandler>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IHyperlink4>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveGotFocus<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IHyperlink4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn LostFocus<'a, Param0: ::windows::core::IntoParam<'a, super::RoutedEventHandler>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IHyperlink4>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveLostFocus<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IHyperlink4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn Focus(&self, value: super::FocusState) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IHyperlink4>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn IsTabStop(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IHyperlink5>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn SetIsTabStop(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IHyperlink5>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn TabIndex(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<IHyperlink5>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn SetTabIndex(&self, value: i32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IHyperlink5>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn NavigateUriProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IHyperlinkStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn UnderlineStyleProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IHyperlinkStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn XYFocusLeftProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IHyperlinkStatics3(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn XYFocusRightProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IHyperlinkStatics3(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn XYFocusUpProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IHyperlinkStatics3(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn XYFocusDownProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IHyperlinkStatics3(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn ElementSoundModeProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IHyperlinkStatics3(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn FocusStateProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IHyperlinkStatics4(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn XYFocusUpNavigationStrategyProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IHyperlinkStatics4(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn XYFocusDownNavigationStrategyProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IHyperlinkStatics4(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn XYFocusLeftNavigationStrategyProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IHyperlinkStatics4(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn XYFocusRightNavigationStrategyProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IHyperlinkStatics4(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn IsTabStopProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IHyperlinkStatics5(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn TabIndexProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IHyperlinkStatics5(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IHyperlinkStatics<R, F: FnOnce(&IHyperlinkStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Hyperlink, IHyperlinkStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IHyperlinkStatics2<R, F: FnOnce(&IHyperlinkStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Hyperlink, IHyperlinkStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IHyperlinkStatics3<R, F: FnOnce(&IHyperlinkStatics3) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Hyperlink, IHyperlinkStatics3> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IHyperlinkStatics4<R, F: FnOnce(&IHyperlinkStatics4) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Hyperlink, IHyperlinkStatics4> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IHyperlinkStatics5<R, F: FnOnce(&IHyperlinkStatics5) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Hyperlink, IHyperlinkStatics5> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for Hyperlink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Hyperlink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Hyperlink {}
impl ::core::fmt::Debug for Hyperlink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Hyperlink").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for Hyperlink {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Documents.Hyperlink;{0fe2363b-14e9-4152-9e58-5aea5b21f08d})");
}
unsafe impl ::windows::core::Interface for Hyperlink {
    type Vtable = IHyperlinkVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0fe2363b_14e9_4152_9e58_5aea5b21f08d);
}
impl ::windows::core::RuntimeName for Hyperlink {
    const NAME: &'static str = "Windows.UI.Xaml.Documents.Hyperlink";
}
impl ::core::convert::From<Hyperlink> for ::windows::core::IUnknown {
    fn from(value: Hyperlink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Hyperlink> for ::windows::core::IUnknown {
    fn from(value: &Hyperlink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for Hyperlink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &Hyperlink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Hyperlink> for ::windows::core::IInspectable {
    fn from(value: Hyperlink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Hyperlink> for ::windows::core::IInspectable {
    fn from(value: &Hyperlink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for Hyperlink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &Hyperlink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Hyperlink> for Span {
    fn from(value: Hyperlink) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Hyperlink> for Span {
    fn from(value: &Hyperlink) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Span> for Hyperlink {
    fn into_param(self) -> ::windows::core::Param<'a, Span> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, Span> for &Hyperlink {
    fn into_param(self) -> ::windows::core::Param<'a, Span> {
        ::windows::core::Param::Owned(::core::convert::Into::<Span>::into(self))
    }
}
impl ::core::convert::From<Hyperlink> for Inline {
    fn from(value: Hyperlink) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Hyperlink> for Inline {
    fn from(value: &Hyperlink) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Inline> for Hyperlink {
    fn into_param(self) -> ::windows::core::Param<'a, Inline> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, Inline> for &Hyperlink {
    fn into_param(self) -> ::windows::core::Param<'a, Inline> {
        ::windows::core::Param::Owned(::core::convert::Into::<Inline>::into(self))
    }
}
impl ::core::convert::From<Hyperlink> for TextElement {
    fn from(value: Hyperlink) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Hyperlink> for TextElement {
    fn from(value: &Hyperlink) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, TextElement> for Hyperlink {
    fn into_param(self) -> ::windows::core::Param<'a, TextElement> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, TextElement> for &Hyperlink {
    fn into_param(self) -> ::windows::core::Param<'a, TextElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<TextElement>::into(self))
    }
}
impl ::core::convert::From<Hyperlink> for super::DependencyObject {
    fn from(value: Hyperlink) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Hyperlink> for super::DependencyObject {
    fn from(value: &Hyperlink) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for Hyperlink {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &Hyperlink {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for Hyperlink {}
unsafe impl ::core::marker::Sync for Hyperlink {}
#[doc = "*Required features: 'UI_Xaml_Documents'*"]
#[repr(transparent)]
pub struct HyperlinkClickEventArgs(::windows::core::IUnknown);
impl HyperlinkClickEventArgs {}
impl ::core::clone::Clone for HyperlinkClickEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HyperlinkClickEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HyperlinkClickEventArgs {}
impl ::core::fmt::Debug for HyperlinkClickEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HyperlinkClickEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HyperlinkClickEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Documents.HyperlinkClickEventArgs;{c755916b-7bdc-4be7-b373-9240a503d870})");
}
unsafe impl ::windows::core::Interface for HyperlinkClickEventArgs {
    type Vtable = IHyperlinkClickEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc755916b_7bdc_4be7_b373_9240a503d870);
}
impl ::windows::core::RuntimeName for HyperlinkClickEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Documents.HyperlinkClickEventArgs";
}
impl ::core::convert::From<HyperlinkClickEventArgs> for ::windows::core::IUnknown {
    fn from(value: HyperlinkClickEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HyperlinkClickEventArgs> for ::windows::core::IUnknown {
    fn from(value: &HyperlinkClickEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for HyperlinkClickEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &HyperlinkClickEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HyperlinkClickEventArgs> for ::windows::core::IInspectable {
    fn from(value: HyperlinkClickEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HyperlinkClickEventArgs> for ::windows::core::IInspectable {
    fn from(value: &HyperlinkClickEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for HyperlinkClickEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &HyperlinkClickEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HyperlinkClickEventArgs> for super::RoutedEventArgs {
    fn from(value: HyperlinkClickEventArgs) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&HyperlinkClickEventArgs> for super::RoutedEventArgs {
    fn from(value: &HyperlinkClickEventArgs) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::RoutedEventArgs> for HyperlinkClickEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, super::RoutedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::RoutedEventArgs> for &HyperlinkClickEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, super::RoutedEventArgs> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(self))
    }
}
unsafe impl ::core::marker::Send for HyperlinkClickEventArgs {}
unsafe impl ::core::marker::Sync for HyperlinkClickEventArgs {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBlock(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBlock {
    type Vtable = IBlockVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4bce0016_dd47_4350_8cb0_e171600ac896);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBlockVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::TextAlignment) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::TextAlignment) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::LineStackingStrategy) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::LineStackingStrategy) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::Thickness) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::Thickness) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IBlock2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBlock2 {
    type Vtable = IBlock2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5ec7bdf3_1333_4a92_8318_6caedc12ef89);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBlock2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::TextAlignment) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::TextAlignment) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IBlockFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBlockFactory {
    type Vtable = IBlockFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x07110532_4f59_4f3b_9ce5_25784c430507);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBlockFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IBlockStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBlockStatics {
    type Vtable = IBlockStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf86a8c34_8d18_4c53_aebd_91e610a5e010);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBlockStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IBlockStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBlockStatics2 {
    type Vtable = IBlockStatics2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaf01a4d6_03e3_4cee_9b02_2bfc308b27a9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBlockStatics2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IBold(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBold {
    type Vtable = IBoldVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xade73784_1b59_4da4_bb23_0f20e885b4bf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBoldVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactContentLinkProvider(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IContactContentLinkProvider {
    type Vtable = IContactContentLinkProviderVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf92fd29b_589b_4abd_9d37_35a1468f021e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactContentLinkProviderVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IContentLink(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IContentLink {
    type Vtable = IContentLinkVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6c60c3e1_528c_42f8_92be_34b8c68be304);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContentLinkVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Text")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Text"))] usize,
    #[cfg(feature = "UI_Text")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Text"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Core")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Core::CoreCursorType) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Core"))] usize,
    #[cfg(feature = "UI_Core")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Core::CoreCursorType) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Core"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::ElementSoundMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::ElementSoundMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::FocusState) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Input")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::Input::XYFocusNavigationStrategy) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))] usize,
    #[cfg(feature = "UI_Xaml_Input")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::Input::XYFocusNavigationStrategy) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))] usize,
    #[cfg(feature = "UI_Xaml_Input")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::Input::XYFocusNavigationStrategy) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))] usize,
    #[cfg(feature = "UI_Xaml_Input")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::Input::XYFocusNavigationStrategy) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))] usize,
    #[cfg(feature = "UI_Xaml_Input")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::Input::XYFocusNavigationStrategy) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))] usize,
    #[cfg(feature = "UI_Xaml_Input")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::Input::XYFocusNavigationStrategy) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))] usize,
    #[cfg(feature = "UI_Xaml_Input")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::Input::XYFocusNavigationStrategy) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))] usize,
    #[cfg(feature = "UI_Xaml_Input")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::Input::XYFocusNavigationStrategy) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::FocusState, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IContentLinkInvokedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IContentLinkInvokedEventArgs {
    type Vtable = IContentLinkInvokedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x546717c1_e8df_4593_9639_97595fdf8310);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContentLinkInvokedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Text")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Text"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IContentLinkProvider(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IContentLinkProvider {
    type Vtable = IContentLinkProviderVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x730587fd_bfdc_4cb3_904d_b65ab339bbf5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContentLinkProviderVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IContentLinkProviderCollection(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IContentLinkProviderCollection {
    type Vtable = IContentLinkProviderCollectionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf5b84d0c_a9f4_4d1a_a13c_10def1843734);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContentLinkProviderCollectionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IContentLinkProviderFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IContentLinkProviderFactory {
    type Vtable = IContentLinkProviderFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x57d60d3b_ef1a_4e8e_839b_d36ef3a503e0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContentLinkProviderFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IContentLinkStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IContentLinkStatics {
    type Vtable = IContentLinkStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa34e3063_eb16_484e_a3df_522b9a832e6e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContentLinkStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IGlyphs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGlyphs {
    type Vtable = IGlyphsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd079498b_f2b1_4281_99a2_e4d05932b2b5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGlyphsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::Media::StyleSimulations) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::Media::StyleSimulations) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IGlyphs2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGlyphs2 {
    type Vtable = IGlyphs2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaa8bfe5c_3754_4bee_bbe1_4403ee9b86f0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGlyphs2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IGlyphsStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGlyphsStatics {
    type Vtable = IGlyphsStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x225cf4c5_fdf1_43ed_958f_414e86f103f2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGlyphsStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IGlyphsStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGlyphsStatics2 {
    type Vtable = IGlyphsStatics2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x10489aa7_1615_4a33_aa02_d7ef2aefc739);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGlyphsStatics2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IHyperlink(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHyperlink {
    type Vtable = IHyperlinkVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0fe2363b_14e9_4152_9e58_5aea5b21f08d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHyperlinkVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IHyperlink2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHyperlink2 {
    type Vtable = IHyperlink2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4ce9da5f_7cff_4291_b78f_dfec72490576);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHyperlink2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut UnderlineStyle) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: UnderlineStyle) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IHyperlink3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHyperlink3 {
    type Vtable = IHyperlink3Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc3f157d9_e5d3_4fb7_8702_4f6d85dd9e0a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHyperlink3Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::ElementSoundMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::ElementSoundMode) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IHyperlink4(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHyperlink4 {
    type Vtable = IHyperlink4Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf7d02959_82fb_400a_a407_5a4ee677988a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHyperlink4Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::FocusState) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Input")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::Input::XYFocusNavigationStrategy) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))] usize,
    #[cfg(feature = "UI_Xaml_Input")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::Input::XYFocusNavigationStrategy) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))] usize,
    #[cfg(feature = "UI_Xaml_Input")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::Input::XYFocusNavigationStrategy) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))] usize,
    #[cfg(feature = "UI_Xaml_Input")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::Input::XYFocusNavigationStrategy) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))] usize,
    #[cfg(feature = "UI_Xaml_Input")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::Input::XYFocusNavigationStrategy) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))] usize,
    #[cfg(feature = "UI_Xaml_Input")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::Input::XYFocusNavigationStrategy) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))] usize,
    #[cfg(feature = "UI_Xaml_Input")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::Input::XYFocusNavigationStrategy) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))] usize,
    #[cfg(feature = "UI_Xaml_Input")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::Input::XYFocusNavigationStrategy) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::FocusState, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IHyperlink5(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHyperlink5 {
    type Vtable = IHyperlink5Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x607dd7d2_0945_4328_91ee_94ccec2ea6c3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHyperlink5Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IHyperlinkClickEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHyperlinkClickEventArgs {
    type Vtable = IHyperlinkClickEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc755916b_7bdc_4be7_b373_9240a503d870);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHyperlinkClickEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IHyperlinkStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHyperlinkStatics {
    type Vtable = IHyperlinkStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3a44d3d4_fd41_41db_8c72_3b790acd9fd3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHyperlinkStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IHyperlinkStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHyperlinkStatics2 {
    type Vtable = IHyperlinkStatics2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5028d8b7_7adf_43ee_a4ae_9c925f755716);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHyperlinkStatics2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IHyperlinkStatics3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHyperlinkStatics3 {
    type Vtable = IHyperlinkStatics3Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3e15dea0_205e_4947_99a5_74e757e8e1b4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHyperlinkStatics3Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IHyperlinkStatics4(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHyperlinkStatics4 {
    type Vtable = IHyperlinkStatics4Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0476b378_8faa_4e24_b3b6_e9de4d3c708c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHyperlinkStatics4Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IHyperlinkStatics5(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHyperlinkStatics5 {
    type Vtable = IHyperlinkStatics5Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x59308cea_1e49_4921_bd88_a2878d07e30e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHyperlinkStatics5Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IInline(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInline {
    type Vtable = IInlineVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0c92712d_1bc9_4931_8cb1_1aeadf1cc685);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInlineVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IInlineFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInlineFactory {
    type Vtable = IInlineFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4058acd1_2f90_4b8f_99dd_4218ef5f03de);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInlineFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IInlineUIContainer(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInlineUIContainer {
    type Vtable = IInlineUIContainerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1416ce81_28ee_452e_b121_5fc4f60b86a6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInlineUIContainerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IItalic(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IItalic {
    type Vtable = IItalicVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x91f4619c_fcbb_4157_802c_76f63b5fb657);
}
#[repr(C)]
#[doc(hidden)]
pub struct IItalicVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ILineBreak(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILineBreak {
    type Vtable = ILineBreakVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x645589c4_f769_41ed_895b_8a1b2fb31562);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILineBreakVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IParagraph(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IParagraph {
    type Vtable = IParagraphVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf83ef59a_fa61_4bef_ae33_0b0ad756a84d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IParagraphVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IParagraphStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IParagraphStatics {
    type Vtable = IParagraphStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xef08889a_535b_4e4c_8d84_283b33e98a37);
}
#[repr(C)]
#[doc(hidden)]
pub struct IParagraphStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IPlaceContentLinkProvider(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPlaceContentLinkProvider {
    type Vtable = IPlaceContentLinkProviderVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x10348a4c_2366_41be_90c8_3258b53b5483);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlaceContentLinkProviderVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IRun(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IRun {
    type Vtable = IRunVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x59553c83_0e14_49bd_b84b_c526f3034349);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRunVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::FlowDirection) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::FlowDirection) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IRunStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IRunStatics {
    type Vtable = IRunStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe9303cef_65a0_4b8d_a7f7_8fdb287b46f3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRunStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpan(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISpan {
    type Vtable = ISpanVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9839d4a9_02af_4811_aa15_6bef3acac97a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpanVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpanFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISpanFactory {
    type Vtable = ISpanFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5b916f5c_cd2d_40c0_956a_386448322f79);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpanFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ITextElement(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITextElement {
    type Vtable = ITextElementVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe83b0062_d776_4f92_baea_40e77d4791d5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextElementVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Text")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Text::FontWeight) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Text"))] usize,
    #[cfg(feature = "UI_Text")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Text::FontWeight) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Text"))] usize,
    #[cfg(feature = "UI_Text")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Text::FontStyle) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Text"))] usize,
    #[cfg(feature = "UI_Text")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Text::FontStyle) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Text"))] usize,
    #[cfg(feature = "UI_Text")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Text::FontStretch) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Text"))] usize,
    #[cfg(feature = "UI_Text")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Text::FontStretch) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Text"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ITextElement2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITextElement2 {
    type Vtable = ITextElement2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa8076aa8_f892_49f6_8cd2_89addaf06d2d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextElement2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ITextElement3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITextElement3 {
    type Vtable = ITextElement3Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd1db340f_1bc4_4ca8_bcf7_770bff9b27ab);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextElement3Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ITextElement4(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITextElement4 {
    type Vtable = ITextElement4Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb196e222_ca0e_48a9_83bc_36ce50566ac7);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextElement4Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Text")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Text::TextDecorations) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Text"))] usize,
    #[cfg(feature = "UI_Text")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Text::TextDecorations) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Text"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Input")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::Input::KeyTipPlacementMode) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))] usize,
    #[cfg(feature = "UI_Xaml_Input")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::Input::KeyTipPlacementMode) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "UI_Xaml_Input"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Xaml_Input")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Xaml_Input"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Xaml_Input")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Xaml_Input"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Xaml_Input")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ITextElement5(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITextElement5 {
    type Vtable = ITextElement5Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbd9552f3_540d_58bf_b6a8_07556aeda2ea);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextElement5Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ITextElementFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITextElementFactory {
    type Vtable = ITextElementFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x35007285_cf47_4bfe_b1bc_39c93af4ae80);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextElementFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ITextElementOverrides(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITextElementOverrides {
    type Vtable = ITextElementOverridesVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0ce21ee7_4f76_4dd9_bf91_163beccf84bc);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextElementOverridesVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ITextElementStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITextElementStatics {
    type Vtable = ITextElementStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0a2f9b98_6c03_4470_a79b_3298a10482ce);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextElementStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ITextElementStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITextElementStatics2 {
    type Vtable = ITextElementStatics2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x164297b2_982b_49e1_8c03_ca43bc4d5b6d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextElementStatics2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ITextElementStatics3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITextElementStatics3 {
    type Vtable = ITextElementStatics3Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcfefcfaf_0fa1_45ec_9a4e_9b33664dc8b1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextElementStatics3Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ITextElementStatics4(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITextElementStatics4 {
    type Vtable = ITextElementStatics4Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfd8f641e_6b12_40d5_b6ef_d1bd12ac9066);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextElementStatics4Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ITextHighlighter(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITextHighlighter {
    type Vtable = ITextHighlighterVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xba6cb54b_7d75_4535_b30d_a81a00b637a4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextHighlighterVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ITextHighlighterBase(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITextHighlighterBase {
    type Vtable = ITextHighlighterBaseVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd957601a_5f0d_4cdf_9758_97e0eb95c8fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextHighlighterBaseVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ITextHighlighterBaseFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITextHighlighterBaseFactory {
    type Vtable = ITextHighlighterBaseFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9592b2d0_eadc_4c74_92c8_6e896e22506d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextHighlighterBaseFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ITextHighlighterFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITextHighlighterFactory {
    type Vtable = ITextHighlighterFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x70125461_9a8f_4fa0_b235_8ffaa507bef2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextHighlighterFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ITextHighlighterStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITextHighlighterStatics {
    type Vtable = ITextHighlighterStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb3b009c4_3a7e_49cc_ab84_29c405488765);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextHighlighterStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ITextPointer(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITextPointer {
    type Vtable = ITextPointerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xac687aa1_6a41_43ff_851e_45348aa2cf7b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextPointerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut LogicalDirection) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, direction: LogicalDirection, result__: *mut super::super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offset: i32, direction: LogicalDirection, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ITypography(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITypography {
    type Vtable = ITypographyVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x866f65d5_ea97_42ab_9288_9c01aebc7a97);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITypographyVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ITypographyStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITypographyStatics {
    type Vtable = ITypographyStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x67b9ec88_6c57_4ce0_95f1_d4b9ed632fb4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITypographyStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut super::FontEastAsianLanguage) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: super::FontEastAsianLanguage) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut super::FontEastAsianWidths) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: super::FontEastAsianWidths) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut super::FontCapitals) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: super::FontCapitals) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut super::FontFraction) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: super::FontFraction) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut super::FontNumeralStyle) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: super::FontNumeralStyle) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut super::FontNumeralAlignment) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: super::FontNumeralAlignment) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut super::FontVariants) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: super::FontVariants) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IUnderline(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUnderline {
    type Vtable = IUnderlineVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa5fa8202_61c0_47d7_93ef_bc0b577c5f26);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUnderlineVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'UI_Xaml_Documents'*"]
#[repr(transparent)]
pub struct Inline(::windows::core::IUnknown);
impl Inline {}
impl ::core::clone::Clone for Inline {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Inline {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Inline {}
impl ::core::fmt::Debug for Inline {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Inline").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for Inline {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Documents.Inline;{0c92712d-1bc9-4931-8cb1-1aeadf1cc685})");
}
unsafe impl ::windows::core::Interface for Inline {
    type Vtable = IInlineVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0c92712d_1bc9_4931_8cb1_1aeadf1cc685);
}
impl ::windows::core::RuntimeName for Inline {
    const NAME: &'static str = "Windows.UI.Xaml.Documents.Inline";
}
impl ::core::convert::From<Inline> for ::windows::core::IUnknown {
    fn from(value: Inline) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Inline> for ::windows::core::IUnknown {
    fn from(value: &Inline) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for Inline {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &Inline {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Inline> for ::windows::core::IInspectable {
    fn from(value: Inline) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Inline> for ::windows::core::IInspectable {
    fn from(value: &Inline) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for Inline {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &Inline {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Inline> for TextElement {
    fn from(value: Inline) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Inline> for TextElement {
    fn from(value: &Inline) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, TextElement> for Inline {
    fn into_param(self) -> ::windows::core::Param<'a, TextElement> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, TextElement> for &Inline {
    fn into_param(self) -> ::windows::core::Param<'a, TextElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<TextElement>::into(self))
    }
}
impl ::core::convert::From<Inline> for super::DependencyObject {
    fn from(value: Inline) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Inline> for super::DependencyObject {
    fn from(value: &Inline) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for Inline {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &Inline {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for Inline {}
unsafe impl ::core::marker::Sync for Inline {}
#[doc = "*Required features: 'UI_Xaml_Documents', 'Foundation_Collections'*"]
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
pub struct InlineCollection(::windows::core::IUnknown);
#[cfg(feature = "Foundation_Collections")]
impl InlineCollection {
    #[doc = "*Required features: 'UI_Xaml_Documents', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterator<Inline>> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IIterable<Inline>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IIterator<Inline>>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAt(&self, index: u32) -> ::windows::core::Result<Inline> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), index, &mut result__).from_abi::<Inline>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetView(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<Inline>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<Inline>>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn IndexOf<'a, Param0: ::windows::core::IntoParam<'a, Inline>>(&self, value: Param0, index: &mut u32) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi(), index, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetAt<'a, Param1: ::windows::core::IntoParam<'a, Inline>>(&self, index: u32, value: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn InsertAt<'a, Param1: ::windows::core::IntoParam<'a, Inline>>(&self, index: u32, value: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), index).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Append<'a, Param0: ::windows::core::IntoParam<'a, Inline>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAtEnd(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, startindex: u32, items: &mut [<Inline as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReplaceAll(&self, items: &[<Inline as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), items.len() as u32, ::core::mem::transmute(items.as_ptr())).ok() }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::clone::Clone for InlineCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::PartialEq for InlineCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::Eq for InlineCollection {}
#[cfg(feature = "Foundation_Collections")]
impl ::core::fmt::Debug for InlineCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InlineCollection").field(&self.0).finish()
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::RuntimeType for InlineCollection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Documents.InlineCollection;pinterface({913337e9-11a1-4345-a3a2-4e7f956e222d};rc(Windows.UI.Xaml.Documents.Inline;{0c92712d-1bc9-4931-8cb1-1aeadf1cc685})))");
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::Interface for InlineCollection {
    type Vtable = super::super::super::Foundation::Collections::IVectorVtbl<Inline>;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_signature(<Self as ::windows::core::RuntimeType>::SIGNATURE);
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for InlineCollection {
    const NAME: &'static str = "Windows.UI.Xaml.Documents.InlineCollection";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for InlineCollection {
    type Item = Inline;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &InlineCollection {
    type Item = Inline;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::super::Foundation::Collections::VectorIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<InlineCollection> for ::windows::core::IUnknown {
    fn from(value: InlineCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&InlineCollection> for ::windows::core::IUnknown {
    fn from(value: &InlineCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for InlineCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &InlineCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<InlineCollection> for ::windows::core::IInspectable {
    fn from(value: InlineCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&InlineCollection> for ::windows::core::IInspectable {
    fn from(value: &InlineCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for InlineCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &InlineCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<InlineCollection> for super::super::super::Foundation::Collections::IIterable<Inline> {
    type Error = ::windows::core::Error;
    fn try_from(value: InlineCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&InlineCollection> for super::super::super::Foundation::Collections::IIterable<Inline> {
    type Error = ::windows::core::Error;
    fn try_from(value: &InlineCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<Inline>> for InlineCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IIterable<Inline>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<Inline>> for &InlineCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IIterable<Inline>> {
        ::core::convert::TryInto::<super::super::super::Foundation::Collections::IIterable<Inline>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<InlineCollection> for super::super::super::Foundation::Collections::IVector<Inline> {
    type Error = ::windows::core::Error;
    fn try_from(value: InlineCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&InlineCollection> for super::super::super::Foundation::Collections::IVector<Inline> {
    type Error = ::windows::core::Error;
    fn try_from(value: &InlineCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IVector<Inline>> for InlineCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IVector<Inline>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IVector<Inline>> for &InlineCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IVector<Inline>> {
        ::core::convert::TryInto::<super::super::super::Foundation::Collections::IVector<Inline>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Send for InlineCollection {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Sync for InlineCollection {}
#[doc = "*Required features: 'UI_Xaml_Documents'*"]
#[repr(transparent)]
pub struct InlineUIContainer(::windows::core::IUnknown);
impl InlineUIContainer {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<InlineUIContainer, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn Child(&self) -> ::windows::core::Result<super::UIElement> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::UIElement>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn SetChild<'a, Param0: ::windows::core::IntoParam<'a, super::UIElement>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for InlineUIContainer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InlineUIContainer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InlineUIContainer {}
impl ::core::fmt::Debug for InlineUIContainer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InlineUIContainer").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InlineUIContainer {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Documents.InlineUIContainer;{1416ce81-28ee-452e-b121-5fc4f60b86a6})");
}
unsafe impl ::windows::core::Interface for InlineUIContainer {
    type Vtable = IInlineUIContainerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1416ce81_28ee_452e_b121_5fc4f60b86a6);
}
impl ::windows::core::RuntimeName for InlineUIContainer {
    const NAME: &'static str = "Windows.UI.Xaml.Documents.InlineUIContainer";
}
impl ::core::convert::From<InlineUIContainer> for ::windows::core::IUnknown {
    fn from(value: InlineUIContainer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InlineUIContainer> for ::windows::core::IUnknown {
    fn from(value: &InlineUIContainer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for InlineUIContainer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &InlineUIContainer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<InlineUIContainer> for ::windows::core::IInspectable {
    fn from(value: InlineUIContainer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InlineUIContainer> for ::windows::core::IInspectable {
    fn from(value: &InlineUIContainer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for InlineUIContainer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &InlineUIContainer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<InlineUIContainer> for Inline {
    fn from(value: InlineUIContainer) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&InlineUIContainer> for Inline {
    fn from(value: &InlineUIContainer) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Inline> for InlineUIContainer {
    fn into_param(self) -> ::windows::core::Param<'a, Inline> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, Inline> for &InlineUIContainer {
    fn into_param(self) -> ::windows::core::Param<'a, Inline> {
        ::windows::core::Param::Owned(::core::convert::Into::<Inline>::into(self))
    }
}
impl ::core::convert::From<InlineUIContainer> for TextElement {
    fn from(value: InlineUIContainer) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&InlineUIContainer> for TextElement {
    fn from(value: &InlineUIContainer) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, TextElement> for InlineUIContainer {
    fn into_param(self) -> ::windows::core::Param<'a, TextElement> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, TextElement> for &InlineUIContainer {
    fn into_param(self) -> ::windows::core::Param<'a, TextElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<TextElement>::into(self))
    }
}
impl ::core::convert::From<InlineUIContainer> for super::DependencyObject {
    fn from(value: InlineUIContainer) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&InlineUIContainer> for super::DependencyObject {
    fn from(value: &InlineUIContainer) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for InlineUIContainer {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &InlineUIContainer {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for InlineUIContainer {}
unsafe impl ::core::marker::Sync for InlineUIContainer {}
#[doc = "*Required features: 'UI_Xaml_Documents'*"]
#[repr(transparent)]
pub struct Italic(::windows::core::IUnknown);
impl Italic {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Italic, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for Italic {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Italic {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Italic {}
impl ::core::fmt::Debug for Italic {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Italic").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for Italic {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Documents.Italic;{91f4619c-fcbb-4157-802c-76f63b5fb657})");
}
unsafe impl ::windows::core::Interface for Italic {
    type Vtable = IItalicVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x91f4619c_fcbb_4157_802c_76f63b5fb657);
}
impl ::windows::core::RuntimeName for Italic {
    const NAME: &'static str = "Windows.UI.Xaml.Documents.Italic";
}
impl ::core::convert::From<Italic> for ::windows::core::IUnknown {
    fn from(value: Italic) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Italic> for ::windows::core::IUnknown {
    fn from(value: &Italic) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for Italic {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &Italic {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Italic> for ::windows::core::IInspectable {
    fn from(value: Italic) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Italic> for ::windows::core::IInspectable {
    fn from(value: &Italic) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for Italic {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &Italic {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Italic> for Span {
    fn from(value: Italic) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Italic> for Span {
    fn from(value: &Italic) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Span> for Italic {
    fn into_param(self) -> ::windows::core::Param<'a, Span> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, Span> for &Italic {
    fn into_param(self) -> ::windows::core::Param<'a, Span> {
        ::windows::core::Param::Owned(::core::convert::Into::<Span>::into(self))
    }
}
impl ::core::convert::From<Italic> for Inline {
    fn from(value: Italic) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Italic> for Inline {
    fn from(value: &Italic) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Inline> for Italic {
    fn into_param(self) -> ::windows::core::Param<'a, Inline> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, Inline> for &Italic {
    fn into_param(self) -> ::windows::core::Param<'a, Inline> {
        ::windows::core::Param::Owned(::core::convert::Into::<Inline>::into(self))
    }
}
impl ::core::convert::From<Italic> for TextElement {
    fn from(value: Italic) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Italic> for TextElement {
    fn from(value: &Italic) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, TextElement> for Italic {
    fn into_param(self) -> ::windows::core::Param<'a, TextElement> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, TextElement> for &Italic {
    fn into_param(self) -> ::windows::core::Param<'a, TextElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<TextElement>::into(self))
    }
}
impl ::core::convert::From<Italic> for super::DependencyObject {
    fn from(value: Italic) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Italic> for super::DependencyObject {
    fn from(value: &Italic) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for Italic {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &Italic {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for Italic {}
unsafe impl ::core::marker::Sync for Italic {}
#[doc = "*Required features: 'UI_Xaml_Documents'*"]
#[repr(transparent)]
pub struct LineBreak(::windows::core::IUnknown);
impl LineBreak {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<LineBreak, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for LineBreak {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LineBreak {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LineBreak {}
impl ::core::fmt::Debug for LineBreak {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LineBreak").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LineBreak {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Documents.LineBreak;{645589c4-f769-41ed-895b-8a1b2fb31562})");
}
unsafe impl ::windows::core::Interface for LineBreak {
    type Vtable = ILineBreakVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x645589c4_f769_41ed_895b_8a1b2fb31562);
}
impl ::windows::core::RuntimeName for LineBreak {
    const NAME: &'static str = "Windows.UI.Xaml.Documents.LineBreak";
}
impl ::core::convert::From<LineBreak> for ::windows::core::IUnknown {
    fn from(value: LineBreak) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LineBreak> for ::windows::core::IUnknown {
    fn from(value: &LineBreak) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LineBreak {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &LineBreak {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LineBreak> for ::windows::core::IInspectable {
    fn from(value: LineBreak) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LineBreak> for ::windows::core::IInspectable {
    fn from(value: &LineBreak) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LineBreak {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &LineBreak {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LineBreak> for Inline {
    fn from(value: LineBreak) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&LineBreak> for Inline {
    fn from(value: &LineBreak) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Inline> for LineBreak {
    fn into_param(self) -> ::windows::core::Param<'a, Inline> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, Inline> for &LineBreak {
    fn into_param(self) -> ::windows::core::Param<'a, Inline> {
        ::windows::core::Param::Owned(::core::convert::Into::<Inline>::into(self))
    }
}
impl ::core::convert::From<LineBreak> for TextElement {
    fn from(value: LineBreak) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&LineBreak> for TextElement {
    fn from(value: &LineBreak) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, TextElement> for LineBreak {
    fn into_param(self) -> ::windows::core::Param<'a, TextElement> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, TextElement> for &LineBreak {
    fn into_param(self) -> ::windows::core::Param<'a, TextElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<TextElement>::into(self))
    }
}
impl ::core::convert::From<LineBreak> for super::DependencyObject {
    fn from(value: LineBreak) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&LineBreak> for super::DependencyObject {
    fn from(value: &LineBreak) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for LineBreak {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &LineBreak {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for LineBreak {}
unsafe impl ::core::marker::Sync for LineBreak {}
#[doc = "*Required features: 'UI_Xaml_Documents'*"]
#[repr(transparent)]
pub struct LogicalDirection(pub i32);
impl LogicalDirection {
    pub const Backward: Self = Self(0i32);
    pub const Forward: Self = Self(1i32);
}
impl ::core::marker::Copy for LogicalDirection {}
impl ::core::clone::Clone for LogicalDirection {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for LogicalDirection {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for LogicalDirection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LogicalDirection {}
impl ::core::fmt::Debug for LogicalDirection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LogicalDirection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LogicalDirection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Documents.LogicalDirection;i4)");
}
impl ::windows::core::DefaultType for LogicalDirection {
    type DefaultType = Self;
}
#[doc = "*Required features: 'UI_Xaml_Documents'*"]
#[repr(transparent)]
pub struct Paragraph(::windows::core::IUnknown);
impl Paragraph {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Paragraph, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Inlines(&self) -> ::windows::core::Result<InlineCollection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<InlineCollection>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn TextIndent(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn SetTextIndent(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn TextIndentProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IParagraphStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IParagraphStatics<R, F: FnOnce(&IParagraphStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Paragraph, IParagraphStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for Paragraph {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Paragraph {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Paragraph {}
impl ::core::fmt::Debug for Paragraph {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Paragraph").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for Paragraph {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Documents.Paragraph;{f83ef59a-fa61-4bef-ae33-0b0ad756a84d})");
}
unsafe impl ::windows::core::Interface for Paragraph {
    type Vtable = IParagraphVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf83ef59a_fa61_4bef_ae33_0b0ad756a84d);
}
impl ::windows::core::RuntimeName for Paragraph {
    const NAME: &'static str = "Windows.UI.Xaml.Documents.Paragraph";
}
impl ::core::convert::From<Paragraph> for ::windows::core::IUnknown {
    fn from(value: Paragraph) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Paragraph> for ::windows::core::IUnknown {
    fn from(value: &Paragraph) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for Paragraph {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &Paragraph {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Paragraph> for ::windows::core::IInspectable {
    fn from(value: Paragraph) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Paragraph> for ::windows::core::IInspectable {
    fn from(value: &Paragraph) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for Paragraph {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &Paragraph {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Paragraph> for Block {
    fn from(value: Paragraph) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Paragraph> for Block {
    fn from(value: &Paragraph) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Block> for Paragraph {
    fn into_param(self) -> ::windows::core::Param<'a, Block> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, Block> for &Paragraph {
    fn into_param(self) -> ::windows::core::Param<'a, Block> {
        ::windows::core::Param::Owned(::core::convert::Into::<Block>::into(self))
    }
}
impl ::core::convert::From<Paragraph> for TextElement {
    fn from(value: Paragraph) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Paragraph> for TextElement {
    fn from(value: &Paragraph) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, TextElement> for Paragraph {
    fn into_param(self) -> ::windows::core::Param<'a, TextElement> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, TextElement> for &Paragraph {
    fn into_param(self) -> ::windows::core::Param<'a, TextElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<TextElement>::into(self))
    }
}
impl ::core::convert::From<Paragraph> for super::DependencyObject {
    fn from(value: Paragraph) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Paragraph> for super::DependencyObject {
    fn from(value: &Paragraph) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for Paragraph {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &Paragraph {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for Paragraph {}
unsafe impl ::core::marker::Sync for Paragraph {}
#[doc = "*Required features: 'UI_Xaml_Documents'*"]
#[repr(transparent)]
pub struct PlaceContentLinkProvider(::windows::core::IUnknown);
impl PlaceContentLinkProvider {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PlaceContentLinkProvider, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for PlaceContentLinkProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PlaceContentLinkProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PlaceContentLinkProvider {}
impl ::core::fmt::Debug for PlaceContentLinkProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlaceContentLinkProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PlaceContentLinkProvider {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Documents.PlaceContentLinkProvider;{10348a4c-2366-41be-90c8-3258b53b5483})");
}
unsafe impl ::windows::core::Interface for PlaceContentLinkProvider {
    type Vtable = IPlaceContentLinkProviderVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x10348a4c_2366_41be_90c8_3258b53b5483);
}
impl ::windows::core::RuntimeName for PlaceContentLinkProvider {
    const NAME: &'static str = "Windows.UI.Xaml.Documents.PlaceContentLinkProvider";
}
impl ::core::convert::From<PlaceContentLinkProvider> for ::windows::core::IUnknown {
    fn from(value: PlaceContentLinkProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PlaceContentLinkProvider> for ::windows::core::IUnknown {
    fn from(value: &PlaceContentLinkProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PlaceContentLinkProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &PlaceContentLinkProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PlaceContentLinkProvider> for ::windows::core::IInspectable {
    fn from(value: PlaceContentLinkProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PlaceContentLinkProvider> for ::windows::core::IInspectable {
    fn from(value: &PlaceContentLinkProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PlaceContentLinkProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &PlaceContentLinkProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PlaceContentLinkProvider> for ContentLinkProvider {
    fn from(value: PlaceContentLinkProvider) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&PlaceContentLinkProvider> for ContentLinkProvider {
    fn from(value: &PlaceContentLinkProvider) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ContentLinkProvider> for PlaceContentLinkProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ContentLinkProvider> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ContentLinkProvider> for &PlaceContentLinkProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ContentLinkProvider> {
        ::windows::core::Param::Owned(::core::convert::Into::<ContentLinkProvider>::into(self))
    }
}
impl ::core::convert::From<PlaceContentLinkProvider> for super::DependencyObject {
    fn from(value: PlaceContentLinkProvider) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&PlaceContentLinkProvider> for super::DependencyObject {
    fn from(value: &PlaceContentLinkProvider) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for PlaceContentLinkProvider {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &PlaceContentLinkProvider {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for PlaceContentLinkProvider {}
unsafe impl ::core::marker::Sync for PlaceContentLinkProvider {}
#[doc = "*Required features: 'UI_Xaml_Documents'*"]
#[repr(transparent)]
pub struct Run(::windows::core::IUnknown);
impl Run {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Run, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn SetText<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn FlowDirection(&self) -> ::windows::core::Result<super::FlowDirection> {
        let this = self;
        unsafe {
            let mut result__: super::FlowDirection = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::FlowDirection>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn SetFlowDirection(&self, value: super::FlowDirection) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn FlowDirectionProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IRunStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IRunStatics<R, F: FnOnce(&IRunStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Run, IRunStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for Run {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Run {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Run {}
impl ::core::fmt::Debug for Run {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Run").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for Run {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Documents.Run;{59553c83-0e14-49bd-b84b-c526f3034349})");
}
unsafe impl ::windows::core::Interface for Run {
    type Vtable = IRunVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x59553c83_0e14_49bd_b84b_c526f3034349);
}
impl ::windows::core::RuntimeName for Run {
    const NAME: &'static str = "Windows.UI.Xaml.Documents.Run";
}
impl ::core::convert::From<Run> for ::windows::core::IUnknown {
    fn from(value: Run) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Run> for ::windows::core::IUnknown {
    fn from(value: &Run) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for Run {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &Run {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Run> for ::windows::core::IInspectable {
    fn from(value: Run) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Run> for ::windows::core::IInspectable {
    fn from(value: &Run) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for Run {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &Run {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Run> for Inline {
    fn from(value: Run) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Run> for Inline {
    fn from(value: &Run) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Inline> for Run {
    fn into_param(self) -> ::windows::core::Param<'a, Inline> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, Inline> for &Run {
    fn into_param(self) -> ::windows::core::Param<'a, Inline> {
        ::windows::core::Param::Owned(::core::convert::Into::<Inline>::into(self))
    }
}
impl ::core::convert::From<Run> for TextElement {
    fn from(value: Run) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Run> for TextElement {
    fn from(value: &Run) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, TextElement> for Run {
    fn into_param(self) -> ::windows::core::Param<'a, TextElement> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, TextElement> for &Run {
    fn into_param(self) -> ::windows::core::Param<'a, TextElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<TextElement>::into(self))
    }
}
impl ::core::convert::From<Run> for super::DependencyObject {
    fn from(value: Run) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Run> for super::DependencyObject {
    fn from(value: &Run) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for Run {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &Run {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for Run {}
unsafe impl ::core::marker::Sync for Run {}
#[doc = "*Required features: 'UI_Xaml_Documents'*"]
#[repr(transparent)]
pub struct Span(::windows::core::IUnknown);
impl Span {
    #[doc = "*Required features: 'UI_Xaml_Documents', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Inlines(&self) -> ::windows::core::Result<InlineCollection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<InlineCollection>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetInlines<'a, Param0: ::windows::core::IntoParam<'a, InlineCollection>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn new() -> ::windows::core::Result<Span> {
        Self::ISpanFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), ::core::ptr::null_mut(), &mut ::core::option::Option::<::windows::core::IInspectable>::None as *mut _ as _, &mut result__).from_abi::<Span>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISpanFactory<R, F: FnOnce(&ISpanFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Span, ISpanFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for Span {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Span {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Span {}
impl ::core::fmt::Debug for Span {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Span").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for Span {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Documents.Span;{9839d4a9-02af-4811-aa15-6bef3acac97a})");
}
unsafe impl ::windows::core::Interface for Span {
    type Vtable = ISpanVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9839d4a9_02af_4811_aa15_6bef3acac97a);
}
impl ::windows::core::RuntimeName for Span {
    const NAME: &'static str = "Windows.UI.Xaml.Documents.Span";
}
impl ::core::convert::From<Span> for ::windows::core::IUnknown {
    fn from(value: Span) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Span> for ::windows::core::IUnknown {
    fn from(value: &Span) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for Span {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &Span {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Span> for ::windows::core::IInspectable {
    fn from(value: Span) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Span> for ::windows::core::IInspectable {
    fn from(value: &Span) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for Span {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &Span {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Span> for Inline {
    fn from(value: Span) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Span> for Inline {
    fn from(value: &Span) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Inline> for Span {
    fn into_param(self) -> ::windows::core::Param<'a, Inline> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, Inline> for &Span {
    fn into_param(self) -> ::windows::core::Param<'a, Inline> {
        ::windows::core::Param::Owned(::core::convert::Into::<Inline>::into(self))
    }
}
impl ::core::convert::From<Span> for TextElement {
    fn from(value: Span) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Span> for TextElement {
    fn from(value: &Span) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, TextElement> for Span {
    fn into_param(self) -> ::windows::core::Param<'a, TextElement> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, TextElement> for &Span {
    fn into_param(self) -> ::windows::core::Param<'a, TextElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<TextElement>::into(self))
    }
}
impl ::core::convert::From<Span> for super::DependencyObject {
    fn from(value: Span) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Span> for super::DependencyObject {
    fn from(value: &Span) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for Span {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &Span {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for Span {}
unsafe impl ::core::marker::Sync for Span {}
#[doc = "*Required features: 'UI_Xaml_Documents'*"]
#[repr(transparent)]
pub struct TextElement(::windows::core::IUnknown);
impl TextElement {
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn FontSize(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn SetFontSize(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents', 'UI_Xaml_Media'*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn FontFamily(&self) -> ::windows::core::Result<super::Media::FontFamily> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Media::FontFamily>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents', 'UI_Xaml_Media'*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetFontFamily<'a, Param0: ::windows::core::IntoParam<'a, super::Media::FontFamily>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents', 'UI_Text'*"]
    #[cfg(feature = "UI_Text")]
    pub fn FontWeight(&self) -> ::windows::core::Result<super::super::Text::FontWeight> {
        let this = self;
        unsafe {
            let mut result__: super::super::Text::FontWeight = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Text::FontWeight>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents', 'UI_Text'*"]
    #[cfg(feature = "UI_Text")]
    pub fn SetFontWeight<'a, Param0: ::windows::core::IntoParam<'a, super::super::Text::FontWeight>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents', 'UI_Text'*"]
    #[cfg(feature = "UI_Text")]
    pub fn FontStyle(&self) -> ::windows::core::Result<super::super::Text::FontStyle> {
        let this = self;
        unsafe {
            let mut result__: super::super::Text::FontStyle = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Text::FontStyle>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents', 'UI_Text'*"]
    #[cfg(feature = "UI_Text")]
    pub fn SetFontStyle(&self, value: super::super::Text::FontStyle) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents', 'UI_Text'*"]
    #[cfg(feature = "UI_Text")]
    pub fn FontStretch(&self) -> ::windows::core::Result<super::super::Text::FontStretch> {
        let this = self;
        unsafe {
            let mut result__: super::super::Text::FontStretch = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Text::FontStretch>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents', 'UI_Text'*"]
    #[cfg(feature = "UI_Text")]
    pub fn SetFontStretch(&self, value: super::super::Text::FontStretch) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn CharacterSpacing(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn SetCharacterSpacing(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents', 'UI_Xaml_Media'*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn Foreground(&self) -> ::windows::core::Result<super::Media::Brush> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Media::Brush>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents', 'UI_Xaml_Media'*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetForeground<'a, Param0: ::windows::core::IntoParam<'a, super::Media::Brush>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn SetLanguage<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn ContentStart(&self) -> ::windows::core::Result<TextPointer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TextPointer>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn ContentEnd(&self) -> ::windows::core::Result<TextPointer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TextPointer>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn ElementStart(&self) -> ::windows::core::Result<TextPointer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TextPointer>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn ElementEnd(&self) -> ::windows::core::Result<TextPointer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TextPointer>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn FindName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).27)(::core::mem::transmute_copy(this), name.into_param().abi(), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn IsTextScaleFactorEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn SetIsTextScaleFactorEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn AllowFocusOnInteraction(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement3>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn SetAllowFocusOnInteraction(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn AccessKey(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ITextElement3>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn SetAccessKey<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn ExitDisplayModeOnAccessKeyInvoked(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement3>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn SetExitDisplayModeOnAccessKeyInvoked(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents', 'UI_Text'*"]
    #[cfg(feature = "UI_Text")]
    pub fn TextDecorations(&self) -> ::windows::core::Result<super::super::Text::TextDecorations> {
        let this = &::windows::core::Interface::cast::<ITextElement4>(self)?;
        unsafe {
            let mut result__: super::super::Text::TextDecorations = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Text::TextDecorations>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents', 'UI_Text'*"]
    #[cfg(feature = "UI_Text")]
    pub fn SetTextDecorations(&self, value: super::super::Text::TextDecorations) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn IsAccessKeyScope(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement4>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn SetIsAccessKeyScope(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn AccessKeyScopeOwner(&self) -> ::windows::core::Result<super::DependencyObject> {
        let this = &::windows::core::Interface::cast::<ITextElement4>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyObject>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn SetAccessKeyScopeOwner<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents', 'UI_Xaml_Input'*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn KeyTipPlacementMode(&self) -> ::windows::core::Result<super::Input::KeyTipPlacementMode> {
        let this = &::windows::core::Interface::cast::<ITextElement4>(self)?;
        unsafe {
            let mut result__: super::Input::KeyTipPlacementMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Input::KeyTipPlacementMode>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents', 'UI_Xaml_Input'*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetKeyTipPlacementMode(&self, value: super::Input::KeyTipPlacementMode) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn KeyTipHorizontalOffset(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<ITextElement4>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn SetKeyTipHorizontalOffset(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn KeyTipVerticalOffset(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<ITextElement4>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn SetKeyTipVerticalOffset(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents', 'Foundation', 'UI_Xaml_Input'*"]
    #[cfg(all(feature = "Foundation", feature = "UI_Xaml_Input"))]
    pub fn AccessKeyDisplayRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<TextElement, super::Input::AccessKeyDisplayRequestedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ITextElement4>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveAccessKeyDisplayRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents', 'Foundation', 'UI_Xaml_Input'*"]
    #[cfg(all(feature = "Foundation", feature = "UI_Xaml_Input"))]
    pub fn AccessKeyDisplayDismissed<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<TextElement, super::Input::AccessKeyDisplayDismissedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ITextElement4>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveAccessKeyDisplayDismissed<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents', 'Foundation', 'UI_Xaml_Input'*"]
    #[cfg(all(feature = "Foundation", feature = "UI_Xaml_Input"))]
    pub fn AccessKeyInvoked<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<TextElement, super::Input::AccessKeyInvokedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ITextElement4>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveAccessKeyInvoked<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn XamlRoot(&self) -> ::windows::core::Result<super::XamlRoot> {
        let this = &::windows::core::Interface::cast::<ITextElement5>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::XamlRoot>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn SetXamlRoot<'a, Param0: ::windows::core::IntoParam<'a, super::XamlRoot>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement5>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn FontSizeProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITextElementStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn FontFamilyProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITextElementStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn FontWeightProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITextElementStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn FontStyleProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITextElementStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn FontStretchProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITextElementStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn CharacterSpacingProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITextElementStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn ForegroundProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITextElementStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn LanguageProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITextElementStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn IsTextScaleFactorEnabledProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITextElementStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn AllowFocusOnInteractionProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITextElementStatics3(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn AccessKeyProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITextElementStatics3(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn ExitDisplayModeOnAccessKeyInvokedProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITextElementStatics3(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn TextDecorationsProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITextElementStatics4(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn IsAccessKeyScopeProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITextElementStatics4(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn AccessKeyScopeOwnerProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITextElementStatics4(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn KeyTipPlacementModeProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITextElementStatics4(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn KeyTipHorizontalOffsetProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITextElementStatics4(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn KeyTipVerticalOffsetProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITextElementStatics4(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ITextElementStatics<R, F: FnOnce(&ITextElementStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<TextElement, ITextElementStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn ITextElementStatics2<R, F: FnOnce(&ITextElementStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<TextElement, ITextElementStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn ITextElementStatics3<R, F: FnOnce(&ITextElementStatics3) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<TextElement, ITextElementStatics3> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn ITextElementStatics4<R, F: FnOnce(&ITextElementStatics4) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<TextElement, ITextElementStatics4> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for TextElement {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TextElement {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TextElement {}
impl ::core::fmt::Debug for TextElement {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TextElement").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TextElement {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Documents.TextElement;{e83b0062-d776-4f92-baea-40e77d4791d5})");
}
unsafe impl ::windows::core::Interface for TextElement {
    type Vtable = ITextElementVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe83b0062_d776_4f92_baea_40e77d4791d5);
}
impl ::windows::core::RuntimeName for TextElement {
    const NAME: &'static str = "Windows.UI.Xaml.Documents.TextElement";
}
impl ::core::convert::From<TextElement> for ::windows::core::IUnknown {
    fn from(value: TextElement) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TextElement> for ::windows::core::IUnknown {
    fn from(value: &TextElement) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TextElement {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &TextElement {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TextElement> for ::windows::core::IInspectable {
    fn from(value: TextElement) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TextElement> for ::windows::core::IInspectable {
    fn from(value: &TextElement) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TextElement {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &TextElement {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TextElement> for super::DependencyObject {
    fn from(value: TextElement) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&TextElement> for super::DependencyObject {
    fn from(value: &TextElement) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for TextElement {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &TextElement {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for TextElement {}
unsafe impl ::core::marker::Sync for TextElement {}
#[doc = "*Required features: 'UI_Xaml_Documents'*"]
#[repr(transparent)]
pub struct TextHighlighter(::windows::core::IUnknown);
impl TextHighlighter {
    #[doc = "*Required features: 'UI_Xaml_Documents', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Ranges(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<TextRange>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVector<TextRange>>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents', 'UI_Xaml_Media'*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn Foreground(&self) -> ::windows::core::Result<super::Media::Brush> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Media::Brush>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents', 'UI_Xaml_Media'*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetForeground<'a, Param0: ::windows::core::IntoParam<'a, super::Media::Brush>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents', 'UI_Xaml_Media'*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn Background(&self) -> ::windows::core::Result<super::Media::Brush> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Media::Brush>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents', 'UI_Xaml_Media'*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetBackground<'a, Param0: ::windows::core::IntoParam<'a, super::Media::Brush>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn new() -> ::windows::core::Result<TextHighlighter> {
        Self::ITextHighlighterFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), ::core::ptr::null_mut(), &mut ::core::option::Option::<::windows::core::IInspectable>::None as *mut _ as _, &mut result__).from_abi::<TextHighlighter>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn ForegroundProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITextHighlighterStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn BackgroundProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITextHighlighterStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ITextHighlighterFactory<R, F: FnOnce(&ITextHighlighterFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<TextHighlighter, ITextHighlighterFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn ITextHighlighterStatics<R, F: FnOnce(&ITextHighlighterStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<TextHighlighter, ITextHighlighterStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for TextHighlighter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TextHighlighter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TextHighlighter {}
impl ::core::fmt::Debug for TextHighlighter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TextHighlighter").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TextHighlighter {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Documents.TextHighlighter;{ba6cb54b-7d75-4535-b30d-a81a00b637a4})");
}
unsafe impl ::windows::core::Interface for TextHighlighter {
    type Vtable = ITextHighlighterVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xba6cb54b_7d75_4535_b30d_a81a00b637a4);
}
impl ::windows::core::RuntimeName for TextHighlighter {
    const NAME: &'static str = "Windows.UI.Xaml.Documents.TextHighlighter";
}
impl ::core::convert::From<TextHighlighter> for ::windows::core::IUnknown {
    fn from(value: TextHighlighter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TextHighlighter> for ::windows::core::IUnknown {
    fn from(value: &TextHighlighter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TextHighlighter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &TextHighlighter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TextHighlighter> for ::windows::core::IInspectable {
    fn from(value: TextHighlighter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TextHighlighter> for ::windows::core::IInspectable {
    fn from(value: &TextHighlighter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TextHighlighter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &TextHighlighter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for TextHighlighter {}
unsafe impl ::core::marker::Sync for TextHighlighter {}
#[doc = "*Required features: 'UI_Xaml_Documents'*"]
#[repr(transparent)]
pub struct TextHighlighterBase(::windows::core::IUnknown);
impl TextHighlighterBase {}
impl ::core::clone::Clone for TextHighlighterBase {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TextHighlighterBase {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TextHighlighterBase {}
impl ::core::fmt::Debug for TextHighlighterBase {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TextHighlighterBase").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TextHighlighterBase {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Documents.TextHighlighterBase;{d957601a-5f0d-4cdf-9758-97e0eb95c8fa})");
}
unsafe impl ::windows::core::Interface for TextHighlighterBase {
    type Vtable = ITextHighlighterBaseVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd957601a_5f0d_4cdf_9758_97e0eb95c8fa);
}
impl ::windows::core::RuntimeName for TextHighlighterBase {
    const NAME: &'static str = "Windows.UI.Xaml.Documents.TextHighlighterBase";
}
impl ::core::convert::From<TextHighlighterBase> for ::windows::core::IUnknown {
    fn from(value: TextHighlighterBase) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TextHighlighterBase> for ::windows::core::IUnknown {
    fn from(value: &TextHighlighterBase) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TextHighlighterBase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &TextHighlighterBase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TextHighlighterBase> for ::windows::core::IInspectable {
    fn from(value: TextHighlighterBase) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TextHighlighterBase> for ::windows::core::IInspectable {
    fn from(value: &TextHighlighterBase) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TextHighlighterBase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &TextHighlighterBase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TextHighlighterBase> for super::DependencyObject {
    fn from(value: TextHighlighterBase) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&TextHighlighterBase> for super::DependencyObject {
    fn from(value: &TextHighlighterBase) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for TextHighlighterBase {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &TextHighlighterBase {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for TextHighlighterBase {}
unsafe impl ::core::marker::Sync for TextHighlighterBase {}
#[doc = "*Required features: 'UI_Xaml_Documents'*"]
#[repr(transparent)]
pub struct TextPointer(::windows::core::IUnknown);
impl TextPointer {
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn Parent(&self) -> ::windows::core::Result<super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyObject>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn VisualParent(&self) -> ::windows::core::Result<super::FrameworkElement> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::FrameworkElement>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn LogicalDirection(&self) -> ::windows::core::Result<LogicalDirection> {
        let this = self;
        unsafe {
            let mut result__: LogicalDirection = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<LogicalDirection>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn Offset(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn GetCharacterRect(&self, direction: LogicalDirection) -> ::windows::core::Result<super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), direction, &mut result__).from_abi::<super::super::super::Foundation::Rect>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn GetPositionAtOffset(&self, offset: i32, direction: LogicalDirection) -> ::windows::core::Result<TextPointer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), offset, direction, &mut result__).from_abi::<TextPointer>(result__)
        }
    }
}
impl ::core::clone::Clone for TextPointer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TextPointer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TextPointer {}
impl ::core::fmt::Debug for TextPointer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TextPointer").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TextPointer {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Documents.TextPointer;{ac687aa1-6a41-43ff-851e-45348aa2cf7b})");
}
unsafe impl ::windows::core::Interface for TextPointer {
    type Vtable = ITextPointerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xac687aa1_6a41_43ff_851e_45348aa2cf7b);
}
impl ::windows::core::RuntimeName for TextPointer {
    const NAME: &'static str = "Windows.UI.Xaml.Documents.TextPointer";
}
impl ::core::convert::From<TextPointer> for ::windows::core::IUnknown {
    fn from(value: TextPointer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TextPointer> for ::windows::core::IUnknown {
    fn from(value: &TextPointer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TextPointer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &TextPointer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TextPointer> for ::windows::core::IInspectable {
    fn from(value: TextPointer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TextPointer> for ::windows::core::IInspectable {
    fn from(value: &TextPointer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TextPointer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &TextPointer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for TextPointer {}
unsafe impl ::core::marker::Sync for TextPointer {}
#[repr(C)]
#[doc = "*Required features: 'UI_Xaml_Documents'*"]
pub struct TextRange {
    pub StartIndex: i32,
    pub Length: i32,
}
impl ::core::marker::Copy for TextRange {}
impl ::core::clone::Clone for TextRange {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TextRange {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TextRange").field("StartIndex", &self.StartIndex).field("Length", &self.Length).finish()
    }
}
unsafe impl ::windows::core::Abi for TextRange {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for TextRange {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.UI.Xaml.Documents.TextRange;i4;i4)");
}
impl ::windows::core::DefaultType for TextRange {
    type DefaultType = Self;
}
impl ::core::cmp::PartialEq for TextRange {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TextRange>()) == 0 }
    }
}
impl ::core::cmp::Eq for TextRange {}
impl ::core::default::Default for TextRange {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'UI_Xaml_Documents'*"]
#[repr(transparent)]
pub struct Typography(::windows::core::IUnknown);
impl Typography {
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn AnnotationAlternatesProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn GetAnnotationAlternates<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<i32> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<i32>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn SetAnnotationAlternates<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: i32) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn EastAsianExpertFormsProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn GetEastAsianExpertForms<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn SetEastAsianExpertForms<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: bool) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn EastAsianLanguageProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn GetEastAsianLanguage<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<super::FontEastAsianLanguage> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: super::FontEastAsianLanguage = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<super::FontEastAsianLanguage>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn SetEastAsianLanguage<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: super::FontEastAsianLanguage) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn EastAsianWidthsProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn GetEastAsianWidths<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<super::FontEastAsianWidths> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: super::FontEastAsianWidths = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<super::FontEastAsianWidths>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn SetEastAsianWidths<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: super::FontEastAsianWidths) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn StandardLigaturesProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn GetStandardLigatures<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn SetStandardLigatures<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: bool) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe { (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn ContextualLigaturesProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn GetContextualLigatures<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn SetContextualLigatures<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: bool) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe { (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn DiscretionaryLigaturesProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn GetDiscretionaryLigatures<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn SetDiscretionaryLigatures<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: bool) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe { (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn HistoricalLigaturesProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).27)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn GetHistoricalLigatures<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).28)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn SetHistoricalLigatures<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: bool) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe { (::windows::core::Interface::vtable(this).29)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn StandardSwashesProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).30)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn GetStandardSwashes<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<i32> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).31)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<i32>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn SetStandardSwashes<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: i32) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe { (::windows::core::Interface::vtable(this).32)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn ContextualSwashesProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).33)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn GetContextualSwashes<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<i32> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).34)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<i32>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn SetContextualSwashes<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: i32) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe { (::windows::core::Interface::vtable(this).35)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn ContextualAlternatesProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).36)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn GetContextualAlternates<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).37)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn SetContextualAlternates<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: bool) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe { (::windows::core::Interface::vtable(this).38)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn StylisticAlternatesProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).39)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn GetStylisticAlternates<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<i32> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).40)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<i32>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn SetStylisticAlternates<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: i32) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe { (::windows::core::Interface::vtable(this).41)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn StylisticSet1Property() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).42)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn GetStylisticSet1<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).43)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn SetStylisticSet1<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: bool) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe { (::windows::core::Interface::vtable(this).44)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn StylisticSet2Property() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).45)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn GetStylisticSet2<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).46)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn SetStylisticSet2<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: bool) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe { (::windows::core::Interface::vtable(this).47)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn StylisticSet3Property() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).48)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn GetStylisticSet3<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).49)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn SetStylisticSet3<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: bool) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe { (::windows::core::Interface::vtable(this).50)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn StylisticSet4Property() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).51)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn GetStylisticSet4<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).52)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn SetStylisticSet4<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: bool) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe { (::windows::core::Interface::vtable(this).53)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn StylisticSet5Property() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).54)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn GetStylisticSet5<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).55)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn SetStylisticSet5<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: bool) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe { (::windows::core::Interface::vtable(this).56)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn StylisticSet6Property() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).57)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn GetStylisticSet6<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).58)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn SetStylisticSet6<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: bool) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe { (::windows::core::Interface::vtable(this).59)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn StylisticSet7Property() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).60)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn GetStylisticSet7<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).61)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn SetStylisticSet7<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: bool) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe { (::windows::core::Interface::vtable(this).62)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn StylisticSet8Property() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).63)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn GetStylisticSet8<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).64)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn SetStylisticSet8<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: bool) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe { (::windows::core::Interface::vtable(this).65)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn StylisticSet9Property() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).66)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn GetStylisticSet9<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).67)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn SetStylisticSet9<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: bool) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe { (::windows::core::Interface::vtable(this).68)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn StylisticSet10Property() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).69)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn GetStylisticSet10<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).70)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn SetStylisticSet10<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: bool) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe { (::windows::core::Interface::vtable(this).71)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn StylisticSet11Property() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).72)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn GetStylisticSet11<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).73)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn SetStylisticSet11<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: bool) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe { (::windows::core::Interface::vtable(this).74)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn StylisticSet12Property() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).75)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn GetStylisticSet12<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).76)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn SetStylisticSet12<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: bool) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe { (::windows::core::Interface::vtable(this).77)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn StylisticSet13Property() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).78)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn GetStylisticSet13<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).79)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn SetStylisticSet13<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: bool) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe { (::windows::core::Interface::vtable(this).80)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn StylisticSet14Property() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).81)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn GetStylisticSet14<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).82)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn SetStylisticSet14<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: bool) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe { (::windows::core::Interface::vtable(this).83)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn StylisticSet15Property() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).84)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn GetStylisticSet15<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).85)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn SetStylisticSet15<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: bool) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe { (::windows::core::Interface::vtable(this).86)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn StylisticSet16Property() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).87)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn GetStylisticSet16<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).88)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn SetStylisticSet16<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: bool) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe { (::windows::core::Interface::vtable(this).89)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn StylisticSet17Property() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).90)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn GetStylisticSet17<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).91)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn SetStylisticSet17<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: bool) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe { (::windows::core::Interface::vtable(this).92)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn StylisticSet18Property() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).93)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn GetStylisticSet18<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).94)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn SetStylisticSet18<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: bool) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe { (::windows::core::Interface::vtable(this).95)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn StylisticSet19Property() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).96)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn GetStylisticSet19<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).97)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn SetStylisticSet19<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: bool) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe { (::windows::core::Interface::vtable(this).98)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn StylisticSet20Property() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).99)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn GetStylisticSet20<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).100)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn SetStylisticSet20<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: bool) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe { (::windows::core::Interface::vtable(this).101)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn CapitalsProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).102)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn GetCapitals<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<super::FontCapitals> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: super::FontCapitals = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).103)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<super::FontCapitals>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn SetCapitals<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: super::FontCapitals) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe { (::windows::core::Interface::vtable(this).104)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn CapitalSpacingProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).105)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn GetCapitalSpacing<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).106)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn SetCapitalSpacing<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: bool) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe { (::windows::core::Interface::vtable(this).107)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn KerningProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).108)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn GetKerning<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).109)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn SetKerning<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: bool) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe { (::windows::core::Interface::vtable(this).110)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn CaseSensitiveFormsProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).111)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn GetCaseSensitiveForms<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).112)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn SetCaseSensitiveForms<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: bool) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe { (::windows::core::Interface::vtable(this).113)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn HistoricalFormsProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).114)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn GetHistoricalForms<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).115)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn SetHistoricalForms<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: bool) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe { (::windows::core::Interface::vtable(this).116)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn FractionProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).117)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn GetFraction<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<super::FontFraction> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: super::FontFraction = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).118)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<super::FontFraction>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn SetFraction<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: super::FontFraction) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe { (::windows::core::Interface::vtable(this).119)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn NumeralStyleProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).120)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn GetNumeralStyle<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<super::FontNumeralStyle> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: super::FontNumeralStyle = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).121)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<super::FontNumeralStyle>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn SetNumeralStyle<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: super::FontNumeralStyle) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe { (::windows::core::Interface::vtable(this).122)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn NumeralAlignmentProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).123)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn GetNumeralAlignment<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<super::FontNumeralAlignment> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: super::FontNumeralAlignment = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).124)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<super::FontNumeralAlignment>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn SetNumeralAlignment<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: super::FontNumeralAlignment) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe { (::windows::core::Interface::vtable(this).125)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn SlashedZeroProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).126)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn GetSlashedZero<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).127)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn SetSlashedZero<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: bool) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe { (::windows::core::Interface::vtable(this).128)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn MathematicalGreekProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).129)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn GetMathematicalGreek<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).130)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn SetMathematicalGreek<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: bool) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe { (::windows::core::Interface::vtable(this).131)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn VariantsProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).132)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn GetVariants<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<super::FontVariants> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: super::FontVariants = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).133)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<super::FontVariants>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Documents'*"]
    pub fn SetVariants<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: super::FontVariants) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe { (::windows::core::Interface::vtable(this).134)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    #[doc(hidden)]
    pub fn ITypographyStatics<R, F: FnOnce(&ITypographyStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Typography, ITypographyStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for Typography {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Typography {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Typography {}
impl ::core::fmt::Debug for Typography {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Typography").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for Typography {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Documents.Typography;{866f65d5-ea97-42ab-9288-9c01aebc7a97})");
}
unsafe impl ::windows::core::Interface for Typography {
    type Vtable = ITypographyVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x866f65d5_ea97_42ab_9288_9c01aebc7a97);
}
impl ::windows::core::RuntimeName for Typography {
    const NAME: &'static str = "Windows.UI.Xaml.Documents.Typography";
}
impl ::core::convert::From<Typography> for ::windows::core::IUnknown {
    fn from(value: Typography) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Typography> for ::windows::core::IUnknown {
    fn from(value: &Typography) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for Typography {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &Typography {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Typography> for ::windows::core::IInspectable {
    fn from(value: Typography) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Typography> for ::windows::core::IInspectable {
    fn from(value: &Typography) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for Typography {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &Typography {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for Typography {}
unsafe impl ::core::marker::Sync for Typography {}
#[doc = "*Required features: 'UI_Xaml_Documents'*"]
#[repr(transparent)]
pub struct Underline(::windows::core::IUnknown);
impl Underline {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Underline, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for Underline {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Underline {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Underline {}
impl ::core::fmt::Debug for Underline {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Underline").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for Underline {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Documents.Underline;{a5fa8202-61c0-47d7-93ef-bc0b577c5f26})");
}
unsafe impl ::windows::core::Interface for Underline {
    type Vtable = IUnderlineVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa5fa8202_61c0_47d7_93ef_bc0b577c5f26);
}
impl ::windows::core::RuntimeName for Underline {
    const NAME: &'static str = "Windows.UI.Xaml.Documents.Underline";
}
impl ::core::convert::From<Underline> for ::windows::core::IUnknown {
    fn from(value: Underline) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Underline> for ::windows::core::IUnknown {
    fn from(value: &Underline) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for Underline {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &Underline {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Underline> for ::windows::core::IInspectable {
    fn from(value: Underline) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Underline> for ::windows::core::IInspectable {
    fn from(value: &Underline) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for Underline {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &Underline {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Underline> for Span {
    fn from(value: Underline) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Underline> for Span {
    fn from(value: &Underline) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Span> for Underline {
    fn into_param(self) -> ::windows::core::Param<'a, Span> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, Span> for &Underline {
    fn into_param(self) -> ::windows::core::Param<'a, Span> {
        ::windows::core::Param::Owned(::core::convert::Into::<Span>::into(self))
    }
}
impl ::core::convert::From<Underline> for Inline {
    fn from(value: Underline) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Underline> for Inline {
    fn from(value: &Underline) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, Inline> for Underline {
    fn into_param(self) -> ::windows::core::Param<'a, Inline> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, Inline> for &Underline {
    fn into_param(self) -> ::windows::core::Param<'a, Inline> {
        ::windows::core::Param::Owned(::core::convert::Into::<Inline>::into(self))
    }
}
impl ::core::convert::From<Underline> for TextElement {
    fn from(value: Underline) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Underline> for TextElement {
    fn from(value: &Underline) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, TextElement> for Underline {
    fn into_param(self) -> ::windows::core::Param<'a, TextElement> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, TextElement> for &Underline {
    fn into_param(self) -> ::windows::core::Param<'a, TextElement> {
        ::windows::core::Param::Owned(::core::convert::Into::<TextElement>::into(self))
    }
}
impl ::core::convert::From<Underline> for super::DependencyObject {
    fn from(value: Underline) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Underline> for super::DependencyObject {
    fn from(value: &Underline) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for Underline {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &Underline {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for Underline {}
unsafe impl ::core::marker::Sync for Underline {}
#[doc = "*Required features: 'UI_Xaml_Documents'*"]
#[repr(transparent)]
pub struct UnderlineStyle(pub i32);
impl UnderlineStyle {
    pub const None: Self = Self(0i32);
    pub const Single: Self = Self(1i32);
}
impl ::core::marker::Copy for UnderlineStyle {}
impl ::core::clone::Clone for UnderlineStyle {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for UnderlineStyle {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for UnderlineStyle {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UnderlineStyle {}
impl ::core::fmt::Debug for UnderlineStyle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UnderlineStyle").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UnderlineStyle {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Documents.UnderlineStyle;i4)");
}
impl ::windows::core::DefaultType for UnderlineStyle {
    type DefaultType = Self;
}
