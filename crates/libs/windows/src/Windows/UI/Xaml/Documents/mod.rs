#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
#[repr(transparent)]
pub struct Block(::windows::core::IUnknown);
impl Block {
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn TextAlignment(&self) -> ::windows::core::Result<super::TextAlignment> {
        let this = self;
        unsafe {
            let mut result__: super::TextAlignment = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TextAlignment)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::TextAlignment>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetTextAlignment(&self, value: super::TextAlignment) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetTextAlignment)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn LineHeight(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LineHeight)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetLineHeight(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetLineHeight)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn LineStackingStrategy(&self) -> ::windows::core::Result<super::LineStackingStrategy> {
        let this = self;
        unsafe {
            let mut result__: super::LineStackingStrategy = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LineStackingStrategy)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::LineStackingStrategy>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetLineStackingStrategy(&self, value: super::LineStackingStrategy) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetLineStackingStrategy)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn Margin(&self) -> ::windows::core::Result<super::Thickness> {
        let this = self;
        unsafe {
            let mut result__: super::Thickness = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Margin)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Thickness>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetMargin<'a, Param0: ::windows::core::IntoParam<'a, super::Thickness>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetMargin)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn HorizontalTextAlignment(&self) -> ::windows::core::Result<super::TextAlignment> {
        let this = &::windows::core::Interface::cast::<IBlock2>(self)?;
        unsafe {
            let mut result__: super::TextAlignment = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).HorizontalTextAlignment)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::TextAlignment>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetHorizontalTextAlignment(&self, value: super::TextAlignment) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IBlock2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetHorizontalTextAlignment)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn TextAlignmentProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IBlockStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TextAlignmentProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn LineHeightProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IBlockStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LineHeightProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn LineStackingStrategyProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IBlockStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LineStackingStrategyProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn MarginProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IBlockStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MarginProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn HorizontalTextAlignmentProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IBlockStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).HorizontalTextAlignmentProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
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
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for Block {
    type Vtable = IBlock_Vtbl;
    const IID: ::windows::core::GUID = <IBlock as ::windows::core::Interface>::IID;
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a Block {
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a Block {
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
#[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"Foundation_Collections\"`*"]
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
pub struct BlockCollection(::windows::core::IUnknown);
#[cfg(feature = "Foundation_Collections")]
impl BlockCollection {
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterator<Block>> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IIterable<Block>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).First)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IIterator<Block>>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAt(&self, index: u32) -> ::windows::core::Result<Block> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetAt)(::core::mem::transmute_copy(this), index, &mut result__).from_abi::<Block>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Size)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetView(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<Block>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetView)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<Block>>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn IndexOf<'a, Param0: ::windows::core::IntoParam<'a, Block>>(&self, value: Param0, index: &mut u32) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IndexOf)(::core::mem::transmute_copy(this), value.into_param().abi(), index, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetAt<'a, Param1: ::windows::core::IntoParam<'a, Block>>(&self, index: u32, value: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetAt)(::core::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn InsertAt<'a, Param1: ::windows::core::IntoParam<'a, Block>>(&self, index: u32, value: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).InsertAt)(::core::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveAt)(::core::mem::transmute_copy(this), index).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Append<'a, Param0: ::windows::core::IntoParam<'a, Block>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Append)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAtEnd(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveAtEnd)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Clear)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, startindex: u32, items: &mut [::core::option::Option<Block>]) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetMany)(::core::mem::transmute_copy(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReplaceAll(&self, items: &[::core::option::Option<Block>]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).ReplaceAll)(::core::mem::transmute_copy(this), items.len() as u32, ::core::mem::transmute(items.as_ptr())).ok() }
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
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::Interface for BlockCollection {
    type Vtable = super::super::super::Foundation::Collections::IVector_Vtbl<Block>;
    const IID: ::windows::core::GUID = <super::super::super::Foundation::Collections::IVector<Block> as ::windows::core::Interface>::IID;
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a BlockCollection {
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a BlockCollection {
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
#[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
#[repr(transparent)]
pub struct Bold(::windows::core::IUnknown);
impl Bold {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
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
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for Bold {
    type Vtable = IBold_Vtbl;
    const IID: ::windows::core::GUID = <IBold as ::windows::core::Interface>::IID;
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a Bold {
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a Bold {
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
#[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
#[repr(transparent)]
pub struct ContactContentLinkProvider(::windows::core::IUnknown);
impl ContactContentLinkProvider {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
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
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ContactContentLinkProvider {
    type Vtable = IContactContentLinkProvider_Vtbl;
    const IID: ::windows::core::GUID = <IContactContentLinkProvider as ::windows::core::Interface>::IID;
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ContactContentLinkProvider {
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ContactContentLinkProvider {
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
#[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
#[repr(transparent)]
pub struct ContentLink(::windows::core::IUnknown);
impl ContentLink {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ContentLink, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Text\"`*"]
    #[cfg(feature = "UI_Text")]
    pub fn Info(&self) -> ::windows::core::Result<super::super::Text::ContentLinkInfo> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Info)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Text::ContentLinkInfo>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Text\"`*"]
    #[cfg(feature = "UI_Text")]
    pub fn SetInfo<'a, Param0: ::windows::core::IntoParam<'a, super::super::Text::ContentLinkInfo>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetInfo)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn Background(&self) -> ::windows::core::Result<super::Media::Brush> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Background)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Media::Brush>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetBackground<'a, Param0: ::windows::core::IntoParam<'a, super::Media::Brush>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetBackground)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Core\"`*"]
    #[cfg(feature = "UI_Core")]
    pub fn Cursor(&self) -> ::windows::core::Result<super::super::Core::CoreCursorType> {
        let this = self;
        unsafe {
            let mut result__: super::super::Core::CoreCursorType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Cursor)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Core::CoreCursorType>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Core\"`*"]
    #[cfg(feature = "UI_Core")]
    pub fn SetCursor(&self, value: super::super::Core::CoreCursorType) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetCursor)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn XYFocusLeft(&self) -> ::windows::core::Result<super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).XYFocusLeft)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyObject>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetXYFocusLeft<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetXYFocusLeft)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn XYFocusRight(&self) -> ::windows::core::Result<super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).XYFocusRight)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyObject>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetXYFocusRight<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetXYFocusRight)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn XYFocusUp(&self) -> ::windows::core::Result<super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).XYFocusUp)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyObject>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetXYFocusUp<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetXYFocusUp)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn XYFocusDown(&self) -> ::windows::core::Result<super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).XYFocusDown)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyObject>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetXYFocusDown<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetXYFocusDown)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ElementSoundMode(&self) -> ::windows::core::Result<super::ElementSoundMode> {
        let this = self;
        unsafe {
            let mut result__: super::ElementSoundMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ElementSoundMode)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::ElementSoundMode>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetElementSoundMode(&self, value: super::ElementSoundMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetElementSoundMode)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn FocusState(&self) -> ::windows::core::Result<super::FocusState> {
        let this = self;
        unsafe {
            let mut result__: super::FocusState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FocusState)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::FocusState>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn XYFocusUpNavigationStrategy(&self) -> ::windows::core::Result<super::Input::XYFocusNavigationStrategy> {
        let this = self;
        unsafe {
            let mut result__: super::Input::XYFocusNavigationStrategy = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).XYFocusUpNavigationStrategy)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Input::XYFocusNavigationStrategy>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetXYFocusUpNavigationStrategy(&self, value: super::Input::XYFocusNavigationStrategy) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetXYFocusUpNavigationStrategy)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn XYFocusDownNavigationStrategy(&self) -> ::windows::core::Result<super::Input::XYFocusNavigationStrategy> {
        let this = self;
        unsafe {
            let mut result__: super::Input::XYFocusNavigationStrategy = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).XYFocusDownNavigationStrategy)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Input::XYFocusNavigationStrategy>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetXYFocusDownNavigationStrategy(&self, value: super::Input::XYFocusNavigationStrategy) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetXYFocusDownNavigationStrategy)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn XYFocusLeftNavigationStrategy(&self) -> ::windows::core::Result<super::Input::XYFocusNavigationStrategy> {
        let this = self;
        unsafe {
            let mut result__: super::Input::XYFocusNavigationStrategy = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).XYFocusLeftNavigationStrategy)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Input::XYFocusNavigationStrategy>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetXYFocusLeftNavigationStrategy(&self, value: super::Input::XYFocusNavigationStrategy) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetXYFocusLeftNavigationStrategy)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn XYFocusRightNavigationStrategy(&self) -> ::windows::core::Result<super::Input::XYFocusNavigationStrategy> {
        let this = self;
        unsafe {
            let mut result__: super::Input::XYFocusNavigationStrategy = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).XYFocusRightNavigationStrategy)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Input::XYFocusNavigationStrategy>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetXYFocusRightNavigationStrategy(&self, value: super::Input::XYFocusNavigationStrategy) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetXYFocusRightNavigationStrategy)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn IsTabStop(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsTabStop)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetIsTabStop(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIsTabStop)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn TabIndex(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TabIndex)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetTabIndex(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetTabIndex)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Invoked<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<ContentLink, ContentLinkInvokedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Invoked)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveInvoked<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveInvoked)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GotFocus<'a, Param0: ::windows::core::IntoParam<'a, super::RoutedEventHandler>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GotFocus)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveGotFocus<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveGotFocus)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LostFocus<'a, Param0: ::windows::core::IntoParam<'a, super::RoutedEventHandler>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LostFocus)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveLostFocus<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveLostFocus)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn Focus(&self, value: super::FocusState) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Focus)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn BackgroundProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IContentLinkStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).BackgroundProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn CursorProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IContentLinkStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CursorProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn XYFocusLeftProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IContentLinkStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).XYFocusLeftProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn XYFocusRightProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IContentLinkStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).XYFocusRightProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn XYFocusUpProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IContentLinkStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).XYFocusUpProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn XYFocusDownProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IContentLinkStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).XYFocusDownProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ElementSoundModeProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IContentLinkStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ElementSoundModeProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn FocusStateProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IContentLinkStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FocusStateProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn XYFocusUpNavigationStrategyProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IContentLinkStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).XYFocusUpNavigationStrategyProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn XYFocusDownNavigationStrategyProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IContentLinkStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).XYFocusDownNavigationStrategyProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn XYFocusLeftNavigationStrategyProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IContentLinkStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).XYFocusLeftNavigationStrategyProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn XYFocusRightNavigationStrategyProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IContentLinkStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).XYFocusRightNavigationStrategyProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn IsTabStopProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IContentLinkStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsTabStopProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn TabIndexProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IContentLinkStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TabIndexProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
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
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ContentLink {
    type Vtable = IContentLink_Vtbl;
    const IID: ::windows::core::GUID = <IContentLink as ::windows::core::Interface>::IID;
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ContentLink {
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ContentLink {
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
#[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
#[repr(transparent)]
pub struct ContentLinkInvokedEventArgs(::windows::core::IUnknown);
impl ContentLinkInvokedEventArgs {
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Text\"`*"]
    #[cfg(feature = "UI_Text")]
    pub fn ContentLinkInfo(&self) -> ::windows::core::Result<super::super::Text::ContentLinkInfo> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ContentLinkInfo)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Text::ContentLinkInfo>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Handled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetHandled)(::core::mem::transmute_copy(this), value).ok() }
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
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ContentLinkInvokedEventArgs {
    type Vtable = IContentLinkInvokedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IContentLinkInvokedEventArgs as ::windows::core::Interface>::IID;
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ContentLinkInvokedEventArgs {
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ContentLinkInvokedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ContentLinkInvokedEventArgs {}
unsafe impl ::core::marker::Sync for ContentLinkInvokedEventArgs {}
#[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
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
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ContentLinkProvider {
    type Vtable = IContentLinkProvider_Vtbl;
    const IID: ::windows::core::GUID = <IContentLinkProvider as ::windows::core::Interface>::IID;
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ContentLinkProvider {
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ContentLinkProvider {
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
#[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
#[repr(transparent)]
pub struct ContentLinkProviderCollection(::windows::core::IUnknown);
impl ContentLinkProviderCollection {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ContentLinkProviderCollection, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterator<ContentLinkProvider>> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IIterable<ContentLinkProvider>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).First)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IIterator<ContentLinkProvider>>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAt(&self, index: u32) -> ::windows::core::Result<ContentLinkProvider> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<ContentLinkProvider>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetAt)(::core::mem::transmute_copy(this), index, &mut result__).from_abi::<ContentLinkProvider>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<ContentLinkProvider>>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Size)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetView(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<ContentLinkProvider>> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<ContentLinkProvider>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetView)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<ContentLinkProvider>>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn IndexOf<'a, Param0: ::windows::core::IntoParam<'a, ContentLinkProvider>>(&self, value: Param0, index: &mut u32) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<ContentLinkProvider>>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IndexOf)(::core::mem::transmute_copy(this), value.into_param().abi(), index, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetAt<'a, Param1: ::windows::core::IntoParam<'a, ContentLinkProvider>>(&self, index: u32, value: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<ContentLinkProvider>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetAt)(::core::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn InsertAt<'a, Param1: ::windows::core::IntoParam<'a, ContentLinkProvider>>(&self, index: u32, value: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<ContentLinkProvider>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).InsertAt)(::core::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<ContentLinkProvider>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveAt)(::core::mem::transmute_copy(this), index).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Append<'a, Param0: ::windows::core::IntoParam<'a, ContentLinkProvider>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<ContentLinkProvider>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Append)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAtEnd(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<ContentLinkProvider>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveAtEnd)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<ContentLinkProvider>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Clear)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, startindex: u32, items: &mut [::core::option::Option<ContentLinkProvider>]) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<ContentLinkProvider>>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetMany)(::core::mem::transmute_copy(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReplaceAll(&self, items: &[::core::option::Option<ContentLinkProvider>]) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<ContentLinkProvider>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).ReplaceAll)(::core::mem::transmute_copy(this), items.len() as u32, ::core::mem::transmute(items.as_ptr())).ok() }
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
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ContentLinkProviderCollection {
    type Vtable = IContentLinkProviderCollection_Vtbl;
    const IID: ::windows::core::GUID = <IContentLinkProviderCollection as ::windows::core::Interface>::IID;
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ContentLinkProviderCollection {
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ContentLinkProviderCollection {
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
#[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
#[repr(transparent)]
pub struct Glyphs(::windows::core::IUnknown);
impl Glyphs {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Glyphs, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn UnicodeString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UnicodeString)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetUnicodeString<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetUnicodeString)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn Indices(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Indices)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetIndices<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIndices)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FontUri(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FontUri)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetFontUri<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetFontUri)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn StyleSimulations(&self) -> ::windows::core::Result<super::Media::StyleSimulations> {
        let this = self;
        unsafe {
            let mut result__: super::Media::StyleSimulations = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StyleSimulations)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Media::StyleSimulations>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetStyleSimulations(&self, value: super::Media::StyleSimulations) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetStyleSimulations)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn FontRenderingEmSize(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FontRenderingEmSize)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetFontRenderingEmSize(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetFontRenderingEmSize)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn OriginX(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OriginX)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetOriginX(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetOriginX)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn OriginY(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OriginY)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetOriginY(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetOriginY)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn Fill(&self) -> ::windows::core::Result<super::Media::Brush> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Fill)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Media::Brush>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetFill<'a, Param0: ::windows::core::IntoParam<'a, super::Media::Brush>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetFill)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn IsColorFontEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IGlyphs2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsColorFontEnabled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetIsColorFontEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IGlyphs2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetIsColorFontEnabled)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ColorFontPaletteIndex(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<IGlyphs2>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ColorFontPaletteIndex)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetColorFontPaletteIndex(&self, value: i32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IGlyphs2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetColorFontPaletteIndex)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn UnicodeStringProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IGlyphsStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UnicodeStringProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn IndicesProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IGlyphsStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IndicesProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn FontUriProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IGlyphsStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FontUriProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn StyleSimulationsProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IGlyphsStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StyleSimulationsProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn FontRenderingEmSizeProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IGlyphsStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FontRenderingEmSizeProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn OriginXProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IGlyphsStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OriginXProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn OriginYProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IGlyphsStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OriginYProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn FillProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IGlyphsStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FillProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn IsColorFontEnabledProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IGlyphsStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsColorFontEnabledProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ColorFontPaletteIndexProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IGlyphsStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ColorFontPaletteIndexProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
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
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for Glyphs {
    type Vtable = IGlyphs_Vtbl;
    const IID: ::windows::core::GUID = <IGlyphs as ::windows::core::Interface>::IID;
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a Glyphs {
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a Glyphs {
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
#[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
#[repr(transparent)]
pub struct Hyperlink(::windows::core::IUnknown);
impl Hyperlink {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Hyperlink, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn NavigateUri(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).NavigateUri)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetNavigateUri<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetNavigateUri)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Click<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<Hyperlink, HyperlinkClickEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Click)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveClick<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveClick)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn UnderlineStyle(&self) -> ::windows::core::Result<UnderlineStyle> {
        let this = &::windows::core::Interface::cast::<IHyperlink2>(self)?;
        unsafe {
            let mut result__: UnderlineStyle = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UnderlineStyle)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UnderlineStyle>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetUnderlineStyle(&self, value: UnderlineStyle) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IHyperlink2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetUnderlineStyle)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn XYFocusLeft(&self) -> ::windows::core::Result<super::DependencyObject> {
        let this = &::windows::core::Interface::cast::<IHyperlink3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).XYFocusLeft)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyObject>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetXYFocusLeft<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IHyperlink3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetXYFocusLeft)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn XYFocusRight(&self) -> ::windows::core::Result<super::DependencyObject> {
        let this = &::windows::core::Interface::cast::<IHyperlink3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).XYFocusRight)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyObject>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetXYFocusRight<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IHyperlink3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetXYFocusRight)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn XYFocusUp(&self) -> ::windows::core::Result<super::DependencyObject> {
        let this = &::windows::core::Interface::cast::<IHyperlink3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).XYFocusUp)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyObject>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetXYFocusUp<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IHyperlink3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetXYFocusUp)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn XYFocusDown(&self) -> ::windows::core::Result<super::DependencyObject> {
        let this = &::windows::core::Interface::cast::<IHyperlink3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).XYFocusDown)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyObject>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetXYFocusDown<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IHyperlink3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetXYFocusDown)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ElementSoundMode(&self) -> ::windows::core::Result<super::ElementSoundMode> {
        let this = &::windows::core::Interface::cast::<IHyperlink3>(self)?;
        unsafe {
            let mut result__: super::ElementSoundMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ElementSoundMode)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::ElementSoundMode>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetElementSoundMode(&self, value: super::ElementSoundMode) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IHyperlink3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetElementSoundMode)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn FocusState(&self) -> ::windows::core::Result<super::FocusState> {
        let this = &::windows::core::Interface::cast::<IHyperlink4>(self)?;
        unsafe {
            let mut result__: super::FocusState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FocusState)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::FocusState>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn XYFocusUpNavigationStrategy(&self) -> ::windows::core::Result<super::Input::XYFocusNavigationStrategy> {
        let this = &::windows::core::Interface::cast::<IHyperlink4>(self)?;
        unsafe {
            let mut result__: super::Input::XYFocusNavigationStrategy = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).XYFocusUpNavigationStrategy)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Input::XYFocusNavigationStrategy>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetXYFocusUpNavigationStrategy(&self, value: super::Input::XYFocusNavigationStrategy) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IHyperlink4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetXYFocusUpNavigationStrategy)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn XYFocusDownNavigationStrategy(&self) -> ::windows::core::Result<super::Input::XYFocusNavigationStrategy> {
        let this = &::windows::core::Interface::cast::<IHyperlink4>(self)?;
        unsafe {
            let mut result__: super::Input::XYFocusNavigationStrategy = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).XYFocusDownNavigationStrategy)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Input::XYFocusNavigationStrategy>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetXYFocusDownNavigationStrategy(&self, value: super::Input::XYFocusNavigationStrategy) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IHyperlink4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetXYFocusDownNavigationStrategy)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn XYFocusLeftNavigationStrategy(&self) -> ::windows::core::Result<super::Input::XYFocusNavigationStrategy> {
        let this = &::windows::core::Interface::cast::<IHyperlink4>(self)?;
        unsafe {
            let mut result__: super::Input::XYFocusNavigationStrategy = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).XYFocusLeftNavigationStrategy)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Input::XYFocusNavigationStrategy>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetXYFocusLeftNavigationStrategy(&self, value: super::Input::XYFocusNavigationStrategy) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IHyperlink4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetXYFocusLeftNavigationStrategy)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn XYFocusRightNavigationStrategy(&self) -> ::windows::core::Result<super::Input::XYFocusNavigationStrategy> {
        let this = &::windows::core::Interface::cast::<IHyperlink4>(self)?;
        unsafe {
            let mut result__: super::Input::XYFocusNavigationStrategy = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).XYFocusRightNavigationStrategy)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Input::XYFocusNavigationStrategy>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetXYFocusRightNavigationStrategy(&self, value: super::Input::XYFocusNavigationStrategy) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IHyperlink4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetXYFocusRightNavigationStrategy)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GotFocus<'a, Param0: ::windows::core::IntoParam<'a, super::RoutedEventHandler>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IHyperlink4>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GotFocus)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveGotFocus<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IHyperlink4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveGotFocus)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LostFocus<'a, Param0: ::windows::core::IntoParam<'a, super::RoutedEventHandler>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IHyperlink4>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LostFocus)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveLostFocus<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IHyperlink4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveLostFocus)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn Focus(&self, value: super::FocusState) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IHyperlink4>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Focus)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn IsTabStop(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IHyperlink5>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsTabStop)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetIsTabStop(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IHyperlink5>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetIsTabStop)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn TabIndex(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<IHyperlink5>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TabIndex)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetTabIndex(&self, value: i32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IHyperlink5>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetTabIndex)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn NavigateUriProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IHyperlinkStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).NavigateUriProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn UnderlineStyleProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IHyperlinkStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UnderlineStyleProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn XYFocusLeftProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IHyperlinkStatics3(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).XYFocusLeftProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn XYFocusRightProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IHyperlinkStatics3(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).XYFocusRightProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn XYFocusUpProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IHyperlinkStatics3(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).XYFocusUpProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn XYFocusDownProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IHyperlinkStatics3(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).XYFocusDownProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ElementSoundModeProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IHyperlinkStatics3(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ElementSoundModeProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn FocusStateProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IHyperlinkStatics4(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FocusStateProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn XYFocusUpNavigationStrategyProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IHyperlinkStatics4(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).XYFocusUpNavigationStrategyProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn XYFocusDownNavigationStrategyProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IHyperlinkStatics4(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).XYFocusDownNavigationStrategyProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn XYFocusLeftNavigationStrategyProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IHyperlinkStatics4(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).XYFocusLeftNavigationStrategyProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn XYFocusRightNavigationStrategyProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IHyperlinkStatics4(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).XYFocusRightNavigationStrategyProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn IsTabStopProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IHyperlinkStatics5(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsTabStopProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn TabIndexProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IHyperlinkStatics5(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TabIndexProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
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
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for Hyperlink {
    type Vtable = IHyperlink_Vtbl;
    const IID: ::windows::core::GUID = <IHyperlink as ::windows::core::Interface>::IID;
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a Hyperlink {
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a Hyperlink {
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
#[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
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
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for HyperlinkClickEventArgs {
    type Vtable = IHyperlinkClickEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IHyperlinkClickEventArgs as ::windows::core::Interface>::IID;
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a HyperlinkClickEventArgs {
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a HyperlinkClickEventArgs {
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
    type Vtable = IBlock_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4bce0016_dd47_4350_8cb0_e171600ac896);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBlock_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub TextAlignment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::TextAlignment) -> ::windows::core::HRESULT,
    pub SetTextAlignment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::TextAlignment) -> ::windows::core::HRESULT,
    pub LineHeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetLineHeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub LineStackingStrategy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::LineStackingStrategy) -> ::windows::core::HRESULT,
    pub SetLineStackingStrategy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::LineStackingStrategy) -> ::windows::core::HRESULT,
    pub Margin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::Thickness) -> ::windows::core::HRESULT,
    pub SetMargin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::Thickness) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBlock2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBlock2 {
    type Vtable = IBlock2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5ec7bdf3_1333_4a92_8318_6caedc12ef89);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBlock2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub HorizontalTextAlignment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::TextAlignment) -> ::windows::core::HRESULT,
    pub SetHorizontalTextAlignment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::TextAlignment) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBlockFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBlockFactory {
    type Vtable = IBlockFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x07110532_4f59_4f3b_9ce5_25784c430507);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBlockFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBlockStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBlockStatics {
    type Vtable = IBlockStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf86a8c34_8d18_4c53_aebd_91e610a5e010);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBlockStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub TextAlignmentProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub LineHeightProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub LineStackingStrategyProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub MarginProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBlockStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBlockStatics2 {
    type Vtable = IBlockStatics2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaf01a4d6_03e3_4cee_9b02_2bfc308b27a9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBlockStatics2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub HorizontalTextAlignmentProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBold(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBold {
    type Vtable = IBold_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xade73784_1b59_4da4_bb23_0f20e885b4bf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBold_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactContentLinkProvider(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IContactContentLinkProvider {
    type Vtable = IContactContentLinkProvider_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf92fd29b_589b_4abd_9d37_35a1468f021e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactContentLinkProvider_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContentLink(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IContentLink {
    type Vtable = IContentLink_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6c60c3e1_528c_42f8_92be_34b8c68be304);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContentLink_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "UI_Text")]
    pub Info: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Text"))]
    Info: usize,
    #[cfg(feature = "UI_Text")]
    pub SetInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Text"))]
    SetInfo: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub Background: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    Background: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetBackground: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetBackground: usize,
    #[cfg(feature = "UI_Core")]
    pub Cursor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Core::CoreCursorType) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Core"))]
    Cursor: usize,
    #[cfg(feature = "UI_Core")]
    pub SetCursor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Core::CoreCursorType) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Core"))]
    SetCursor: usize,
    pub XYFocusLeft: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetXYFocusLeft: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub XYFocusRight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetXYFocusRight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub XYFocusUp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetXYFocusUp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub XYFocusDown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetXYFocusDown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub ElementSoundMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::ElementSoundMode) -> ::windows::core::HRESULT,
    pub SetElementSoundMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::ElementSoundMode) -> ::windows::core::HRESULT,
    pub FocusState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::FocusState) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Input")]
    pub XYFocusUpNavigationStrategy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::Input::XYFocusNavigationStrategy) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    XYFocusUpNavigationStrategy: usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub SetXYFocusUpNavigationStrategy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::Input::XYFocusNavigationStrategy) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    SetXYFocusUpNavigationStrategy: usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub XYFocusDownNavigationStrategy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::Input::XYFocusNavigationStrategy) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    XYFocusDownNavigationStrategy: usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub SetXYFocusDownNavigationStrategy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::Input::XYFocusNavigationStrategy) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    SetXYFocusDownNavigationStrategy: usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub XYFocusLeftNavigationStrategy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::Input::XYFocusNavigationStrategy) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    XYFocusLeftNavigationStrategy: usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub SetXYFocusLeftNavigationStrategy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::Input::XYFocusNavigationStrategy) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    SetXYFocusLeftNavigationStrategy: usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub XYFocusRightNavigationStrategy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::Input::XYFocusNavigationStrategy) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    XYFocusRightNavigationStrategy: usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub SetXYFocusRightNavigationStrategy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::Input::XYFocusNavigationStrategy) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    SetXYFocusRightNavigationStrategy: usize,
    pub IsTabStop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsTabStop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub TabIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub SetTabIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Invoked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Invoked: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveInvoked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveInvoked: usize,
    #[cfg(feature = "Foundation")]
    pub GotFocus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GotFocus: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveGotFocus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveGotFocus: usize,
    #[cfg(feature = "Foundation")]
    pub LostFocus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LostFocus: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveLostFocus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveLostFocus: usize,
    pub Focus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::FocusState, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContentLinkInvokedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IContentLinkInvokedEventArgs {
    type Vtable = IContentLinkInvokedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x546717c1_e8df_4593_9639_97595fdf8310);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContentLinkInvokedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "UI_Text")]
    pub ContentLinkInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Text"))]
    ContentLinkInfo: usize,
    pub Handled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContentLinkProvider(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IContentLinkProvider {
    type Vtable = IContentLinkProvider_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x730587fd_bfdc_4cb3_904d_b65ab339bbf5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContentLinkProvider_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContentLinkProviderCollection(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IContentLinkProviderCollection {
    type Vtable = IContentLinkProviderCollection_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf5b84d0c_a9f4_4d1a_a13c_10def1843734);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContentLinkProviderCollection_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContentLinkProviderFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IContentLinkProviderFactory {
    type Vtable = IContentLinkProviderFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x57d60d3b_ef1a_4e8e_839b_d36ef3a503e0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContentLinkProviderFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContentLinkStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IContentLinkStatics {
    type Vtable = IContentLinkStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa34e3063_eb16_484e_a3df_522b9a832e6e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContentLinkStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub BackgroundProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CursorProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub XYFocusLeftProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub XYFocusRightProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub XYFocusUpProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub XYFocusDownProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub ElementSoundModeProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub FocusStateProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub XYFocusUpNavigationStrategyProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub XYFocusDownNavigationStrategyProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub XYFocusLeftNavigationStrategyProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub XYFocusRightNavigationStrategyProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub IsTabStopProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub TabIndexProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGlyphs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGlyphs {
    type Vtable = IGlyphs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd079498b_f2b1_4281_99a2_e4d05932b2b5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGlyphs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub UnicodeString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetUnicodeString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Indices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetIndices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub FontUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FontUri: usize,
    #[cfg(feature = "Foundation")]
    pub SetFontUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetFontUri: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub StyleSimulations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::Media::StyleSimulations) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    StyleSimulations: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetStyleSimulations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::Media::StyleSimulations) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetStyleSimulations: usize,
    pub FontRenderingEmSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetFontRenderingEmSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub OriginX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetOriginX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub OriginY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetOriginY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Media")]
    pub Fill: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    Fill: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetFill: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetFill: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGlyphs2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGlyphs2 {
    type Vtable = IGlyphs2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaa8bfe5c_3754_4bee_bbe1_4403ee9b86f0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGlyphs2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub IsColorFontEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsColorFontEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub ColorFontPaletteIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub SetColorFontPaletteIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGlyphsStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGlyphsStatics {
    type Vtable = IGlyphsStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x225cf4c5_fdf1_43ed_958f_414e86f103f2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGlyphsStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub UnicodeStringProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub IndicesProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub FontUriProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub StyleSimulationsProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub FontRenderingEmSizeProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub OriginXProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub OriginYProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub FillProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGlyphsStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGlyphsStatics2 {
    type Vtable = IGlyphsStatics2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x10489aa7_1615_4a33_aa02_d7ef2aefc739);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGlyphsStatics2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub IsColorFontEnabledProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub ColorFontPaletteIndexProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHyperlink(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHyperlink {
    type Vtable = IHyperlink_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0fe2363b_14e9_4152_9e58_5aea5b21f08d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHyperlink_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub NavigateUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    NavigateUri: usize,
    #[cfg(feature = "Foundation")]
    pub SetNavigateUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetNavigateUri: usize,
    #[cfg(feature = "Foundation")]
    pub Click: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Click: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveClick: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveClick: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHyperlink2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHyperlink2 {
    type Vtable = IHyperlink2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4ce9da5f_7cff_4291_b78f_dfec72490576);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHyperlink2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub UnderlineStyle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut UnderlineStyle) -> ::windows::core::HRESULT,
    pub SetUnderlineStyle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: UnderlineStyle) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHyperlink3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHyperlink3 {
    type Vtable = IHyperlink3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc3f157d9_e5d3_4fb7_8702_4f6d85dd9e0a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHyperlink3_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub XYFocusLeft: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetXYFocusLeft: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub XYFocusRight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetXYFocusRight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub XYFocusUp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetXYFocusUp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub XYFocusDown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetXYFocusDown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub ElementSoundMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::ElementSoundMode) -> ::windows::core::HRESULT,
    pub SetElementSoundMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::ElementSoundMode) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHyperlink4(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHyperlink4 {
    type Vtable = IHyperlink4_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf7d02959_82fb_400a_a407_5a4ee677988a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHyperlink4_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub FocusState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::FocusState) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Input")]
    pub XYFocusUpNavigationStrategy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::Input::XYFocusNavigationStrategy) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    XYFocusUpNavigationStrategy: usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub SetXYFocusUpNavigationStrategy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::Input::XYFocusNavigationStrategy) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    SetXYFocusUpNavigationStrategy: usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub XYFocusDownNavigationStrategy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::Input::XYFocusNavigationStrategy) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    XYFocusDownNavigationStrategy: usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub SetXYFocusDownNavigationStrategy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::Input::XYFocusNavigationStrategy) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    SetXYFocusDownNavigationStrategy: usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub XYFocusLeftNavigationStrategy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::Input::XYFocusNavigationStrategy) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    XYFocusLeftNavigationStrategy: usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub SetXYFocusLeftNavigationStrategy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::Input::XYFocusNavigationStrategy) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    SetXYFocusLeftNavigationStrategy: usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub XYFocusRightNavigationStrategy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::Input::XYFocusNavigationStrategy) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    XYFocusRightNavigationStrategy: usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub SetXYFocusRightNavigationStrategy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::Input::XYFocusNavigationStrategy) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    SetXYFocusRightNavigationStrategy: usize,
    #[cfg(feature = "Foundation")]
    pub GotFocus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GotFocus: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveGotFocus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveGotFocus: usize,
    #[cfg(feature = "Foundation")]
    pub LostFocus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LostFocus: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveLostFocus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveLostFocus: usize,
    pub Focus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::FocusState, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHyperlink5(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHyperlink5 {
    type Vtable = IHyperlink5_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x607dd7d2_0945_4328_91ee_94ccec2ea6c3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHyperlink5_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub IsTabStop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsTabStop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub TabIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub SetTabIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHyperlinkClickEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHyperlinkClickEventArgs {
    type Vtable = IHyperlinkClickEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc755916b_7bdc_4be7_b373_9240a503d870);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHyperlinkClickEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHyperlinkStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHyperlinkStatics {
    type Vtable = IHyperlinkStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3a44d3d4_fd41_41db_8c72_3b790acd9fd3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHyperlinkStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub NavigateUriProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHyperlinkStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHyperlinkStatics2 {
    type Vtable = IHyperlinkStatics2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5028d8b7_7adf_43ee_a4ae_9c925f755716);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHyperlinkStatics2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub UnderlineStyleProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHyperlinkStatics3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHyperlinkStatics3 {
    type Vtable = IHyperlinkStatics3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3e15dea0_205e_4947_99a5_74e757e8e1b4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHyperlinkStatics3_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub XYFocusLeftProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub XYFocusRightProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub XYFocusUpProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub XYFocusDownProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub ElementSoundModeProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHyperlinkStatics4(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHyperlinkStatics4 {
    type Vtable = IHyperlinkStatics4_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0476b378_8faa_4e24_b3b6_e9de4d3c708c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHyperlinkStatics4_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub FocusStateProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub XYFocusUpNavigationStrategyProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub XYFocusDownNavigationStrategyProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub XYFocusLeftNavigationStrategyProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub XYFocusRightNavigationStrategyProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHyperlinkStatics5(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHyperlinkStatics5 {
    type Vtable = IHyperlinkStatics5_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x59308cea_1e49_4921_bd88_a2878d07e30e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHyperlinkStatics5_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub IsTabStopProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub TabIndexProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInline(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInline {
    type Vtable = IInline_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0c92712d_1bc9_4931_8cb1_1aeadf1cc685);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInline_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInlineFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInlineFactory {
    type Vtable = IInlineFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4058acd1_2f90_4b8f_99dd_4218ef5f03de);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInlineFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInlineUIContainer(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInlineUIContainer {
    type Vtable = IInlineUIContainer_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1416ce81_28ee_452e_b121_5fc4f60b86a6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInlineUIContainer_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Child: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetChild: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IItalic(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IItalic {
    type Vtable = IItalic_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x91f4619c_fcbb_4157_802c_76f63b5fb657);
}
#[repr(C)]
#[doc(hidden)]
pub struct IItalic_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILineBreak(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILineBreak {
    type Vtable = ILineBreak_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x645589c4_f769_41ed_895b_8a1b2fb31562);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILineBreak_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IParagraph(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IParagraph {
    type Vtable = IParagraph_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf83ef59a_fa61_4bef_ae33_0b0ad756a84d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IParagraph_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Inlines: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Inlines: usize,
    pub TextIndent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetTextIndent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IParagraphStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IParagraphStatics {
    type Vtable = IParagraphStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xef08889a_535b_4e4c_8d84_283b33e98a37);
}
#[repr(C)]
#[doc(hidden)]
pub struct IParagraphStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub TextIndentProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPlaceContentLinkProvider(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPlaceContentLinkProvider {
    type Vtable = IPlaceContentLinkProvider_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x10348a4c_2366_41be_90c8_3258b53b5483);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlaceContentLinkProvider_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRun(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IRun {
    type Vtable = IRun_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x59553c83_0e14_49bd_b84b_c526f3034349);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRun_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Text: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub FlowDirection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::FlowDirection) -> ::windows::core::HRESULT,
    pub SetFlowDirection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::FlowDirection) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRunStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IRunStatics {
    type Vtable = IRunStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe9303cef_65a0_4b8d_a7f7_8fdb287b46f3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRunStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub FlowDirectionProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpan(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISpan {
    type Vtable = ISpan_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9839d4a9_02af_4811_aa15_6bef3acac97a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpan_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Inlines: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Inlines: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SetInlines: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetInlines: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpanFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISpanFactory {
    type Vtable = ISpanFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5b916f5c_cd2d_40c0_956a_386448322f79);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpanFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITextElement(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITextElement {
    type Vtable = ITextElement_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe83b0062_d776_4f92_baea_40e77d4791d5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextElement_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub FontSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetFontSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Media")]
    pub FontFamily: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    FontFamily: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetFontFamily: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetFontFamily: usize,
    #[cfg(feature = "UI_Text")]
    pub FontWeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Text::FontWeight) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Text"))]
    FontWeight: usize,
    #[cfg(feature = "UI_Text")]
    pub SetFontWeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Text::FontWeight) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Text"))]
    SetFontWeight: usize,
    #[cfg(feature = "UI_Text")]
    pub FontStyle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Text::FontStyle) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Text"))]
    FontStyle: usize,
    #[cfg(feature = "UI_Text")]
    pub SetFontStyle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Text::FontStyle) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Text"))]
    SetFontStyle: usize,
    #[cfg(feature = "UI_Text")]
    pub FontStretch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Text::FontStretch) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Text"))]
    FontStretch: usize,
    #[cfg(feature = "UI_Text")]
    pub SetFontStretch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Text::FontStretch) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Text"))]
    SetFontStretch: usize,
    pub CharacterSpacing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub SetCharacterSpacing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Media")]
    pub Foreground: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    Foreground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetForeground: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetForeground: usize,
    pub Language: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ContentStart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub ContentEnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub ElementStart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub ElementEnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub FindName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITextElement2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITextElement2 {
    type Vtable = ITextElement2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa8076aa8_f892_49f6_8cd2_89addaf06d2d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextElement2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub IsTextScaleFactorEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsTextScaleFactorEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITextElement3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITextElement3 {
    type Vtable = ITextElement3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd1db340f_1bc4_4ca8_bcf7_770bff9b27ab);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextElement3_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub AllowFocusOnInteraction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetAllowFocusOnInteraction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub AccessKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetAccessKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ExitDisplayModeOnAccessKeyInvoked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetExitDisplayModeOnAccessKeyInvoked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITextElement4(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITextElement4 {
    type Vtable = ITextElement4_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb196e222_ca0e_48a9_83bc_36ce50566ac7);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextElement4_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "UI_Text")]
    pub TextDecorations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Text::TextDecorations) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Text"))]
    TextDecorations: usize,
    #[cfg(feature = "UI_Text")]
    pub SetTextDecorations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Text::TextDecorations) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Text"))]
    SetTextDecorations: usize,
    pub IsAccessKeyScope: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsAccessKeyScope: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub AccessKeyScopeOwner: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetAccessKeyScopeOwner: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Input")]
    pub KeyTipPlacementMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::Input::KeyTipPlacementMode) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    KeyTipPlacementMode: usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub SetKeyTipPlacementMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::Input::KeyTipPlacementMode) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    SetKeyTipPlacementMode: usize,
    pub KeyTipHorizontalOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetKeyTipHorizontalOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub KeyTipVerticalOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetKeyTipVerticalOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "UI_Xaml_Input"))]
    pub AccessKeyDisplayRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Xaml_Input")))]
    AccessKeyDisplayRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAccessKeyDisplayRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAccessKeyDisplayRequested: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Xaml_Input"))]
    pub AccessKeyDisplayDismissed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Xaml_Input")))]
    AccessKeyDisplayDismissed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAccessKeyDisplayDismissed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAccessKeyDisplayDismissed: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Xaml_Input"))]
    pub AccessKeyInvoked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Xaml_Input")))]
    AccessKeyInvoked: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAccessKeyInvoked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAccessKeyInvoked: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITextElement5(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITextElement5 {
    type Vtable = ITextElement5_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbd9552f3_540d_58bf_b6a8_07556aeda2ea);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextElement5_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub XamlRoot: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetXamlRoot: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITextElementFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITextElementFactory {
    type Vtable = ITextElementFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x35007285_cf47_4bfe_b1bc_39c93af4ae80);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextElementFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITextElementOverrides(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITextElementOverrides {
    type Vtable = ITextElementOverrides_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0ce21ee7_4f76_4dd9_bf91_163beccf84bc);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextElementOverrides_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub OnDisconnectVisualChildren: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITextElementStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITextElementStatics {
    type Vtable = ITextElementStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0a2f9b98_6c03_4470_a79b_3298a10482ce);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextElementStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub FontSizeProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub FontFamilyProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub FontWeightProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub FontStyleProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub FontStretchProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CharacterSpacingProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub ForegroundProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub LanguageProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITextElementStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITextElementStatics2 {
    type Vtable = ITextElementStatics2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x164297b2_982b_49e1_8c03_ca43bc4d5b6d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextElementStatics2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub IsTextScaleFactorEnabledProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITextElementStatics3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITextElementStatics3 {
    type Vtable = ITextElementStatics3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcfefcfaf_0fa1_45ec_9a4e_9b33664dc8b1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextElementStatics3_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub AllowFocusOnInteractionProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub AccessKeyProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub ExitDisplayModeOnAccessKeyInvokedProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITextElementStatics4(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITextElementStatics4 {
    type Vtable = ITextElementStatics4_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfd8f641e_6b12_40d5_b6ef_d1bd12ac9066);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextElementStatics4_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub TextDecorationsProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub IsAccessKeyScopeProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub AccessKeyScopeOwnerProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub KeyTipPlacementModeProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub KeyTipHorizontalOffsetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub KeyTipVerticalOffsetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITextHighlighter(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITextHighlighter {
    type Vtable = ITextHighlighter_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xba6cb54b_7d75_4535_b30d_a81a00b637a4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextHighlighter_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Ranges: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Ranges: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub Foreground: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    Foreground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetForeground: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetForeground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub Background: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    Background: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetBackground: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetBackground: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITextHighlighterBase(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITextHighlighterBase {
    type Vtable = ITextHighlighterBase_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd957601a_5f0d_4cdf_9758_97e0eb95c8fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextHighlighterBase_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITextHighlighterBaseFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITextHighlighterBaseFactory {
    type Vtable = ITextHighlighterBaseFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9592b2d0_eadc_4c74_92c8_6e896e22506d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextHighlighterBaseFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITextHighlighterFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITextHighlighterFactory {
    type Vtable = ITextHighlighterFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x70125461_9a8f_4fa0_b235_8ffaa507bef2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextHighlighterFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITextHighlighterStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITextHighlighterStatics {
    type Vtable = ITextHighlighterStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb3b009c4_3a7e_49cc_ab84_29c405488765);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextHighlighterStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub ForegroundProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub BackgroundProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITextPointer(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITextPointer {
    type Vtable = ITextPointer_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xac687aa1_6a41_43ff_851e_45348aa2cf7b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextPointer_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Parent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub VisualParent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub LogicalDirection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut LogicalDirection) -> ::windows::core::HRESULT,
    pub Offset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetCharacterRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, direction: LogicalDirection, result__: *mut super::super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetCharacterRect: usize,
    pub GetPositionAtOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offset: i32, direction: LogicalDirection, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITypography(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITypography {
    type Vtable = ITypography_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x866f65d5_ea97_42ab_9288_9c01aebc7a97);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITypography_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITypographyStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITypographyStatics {
    type Vtable = ITypographyStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x67b9ec88_6c57_4ce0_95f1_d4b9ed632fb4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITypographyStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub AnnotationAlternatesProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetAnnotationAlternates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
    pub SetAnnotationAlternates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: i32) -> ::windows::core::HRESULT,
    pub EastAsianExpertFormsProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetEastAsianExpertForms: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetEastAsianExpertForms: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub EastAsianLanguageProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetEastAsianLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut super::FontEastAsianLanguage) -> ::windows::core::HRESULT,
    pub SetEastAsianLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: super::FontEastAsianLanguage) -> ::windows::core::HRESULT,
    pub EastAsianWidthsProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetEastAsianWidths: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut super::FontEastAsianWidths) -> ::windows::core::HRESULT,
    pub SetEastAsianWidths: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: super::FontEastAsianWidths) -> ::windows::core::HRESULT,
    pub StandardLigaturesProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetStandardLigatures: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetStandardLigatures: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub ContextualLigaturesProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetContextualLigatures: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetContextualLigatures: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub DiscretionaryLigaturesProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetDiscretionaryLigatures: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetDiscretionaryLigatures: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub HistoricalLigaturesProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetHistoricalLigatures: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetHistoricalLigatures: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub StandardSwashesProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetStandardSwashes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
    pub SetStandardSwashes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: i32) -> ::windows::core::HRESULT,
    pub ContextualSwashesProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetContextualSwashes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
    pub SetContextualSwashes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: i32) -> ::windows::core::HRESULT,
    pub ContextualAlternatesProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetContextualAlternates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetContextualAlternates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub StylisticAlternatesProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetStylisticAlternates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
    pub SetStylisticAlternates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: i32) -> ::windows::core::HRESULT,
    pub StylisticSet1Property: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetStylisticSet1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetStylisticSet1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub StylisticSet2Property: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetStylisticSet2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetStylisticSet2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub StylisticSet3Property: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetStylisticSet3: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetStylisticSet3: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub StylisticSet4Property: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetStylisticSet4: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetStylisticSet4: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub StylisticSet5Property: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetStylisticSet5: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetStylisticSet5: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub StylisticSet6Property: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetStylisticSet6: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetStylisticSet6: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub StylisticSet7Property: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetStylisticSet7: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetStylisticSet7: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub StylisticSet8Property: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetStylisticSet8: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetStylisticSet8: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub StylisticSet9Property: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetStylisticSet9: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetStylisticSet9: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub StylisticSet10Property: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetStylisticSet10: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetStylisticSet10: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub StylisticSet11Property: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetStylisticSet11: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetStylisticSet11: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub StylisticSet12Property: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetStylisticSet12: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetStylisticSet12: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub StylisticSet13Property: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetStylisticSet13: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetStylisticSet13: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub StylisticSet14Property: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetStylisticSet14: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetStylisticSet14: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub StylisticSet15Property: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetStylisticSet15: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetStylisticSet15: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub StylisticSet16Property: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetStylisticSet16: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetStylisticSet16: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub StylisticSet17Property: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetStylisticSet17: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetStylisticSet17: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub StylisticSet18Property: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetStylisticSet18: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetStylisticSet18: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub StylisticSet19Property: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetStylisticSet19: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetStylisticSet19: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub StylisticSet20Property: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetStylisticSet20: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetStylisticSet20: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub CapitalsProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetCapitals: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut super::FontCapitals) -> ::windows::core::HRESULT,
    pub SetCapitals: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: super::FontCapitals) -> ::windows::core::HRESULT,
    pub CapitalSpacingProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetCapitalSpacing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetCapitalSpacing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub KerningProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetKerning: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetKerning: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub CaseSensitiveFormsProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetCaseSensitiveForms: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetCaseSensitiveForms: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub HistoricalFormsProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetHistoricalForms: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetHistoricalForms: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub FractionProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetFraction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut super::FontFraction) -> ::windows::core::HRESULT,
    pub SetFraction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: super::FontFraction) -> ::windows::core::HRESULT,
    pub NumeralStyleProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetNumeralStyle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut super::FontNumeralStyle) -> ::windows::core::HRESULT,
    pub SetNumeralStyle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: super::FontNumeralStyle) -> ::windows::core::HRESULT,
    pub NumeralAlignmentProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetNumeralAlignment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut super::FontNumeralAlignment) -> ::windows::core::HRESULT,
    pub SetNumeralAlignment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: super::FontNumeralAlignment) -> ::windows::core::HRESULT,
    pub SlashedZeroProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetSlashedZero: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetSlashedZero: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub MathematicalGreekProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetMathematicalGreek: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetMathematicalGreek: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub VariantsProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetVariants: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut super::FontVariants) -> ::windows::core::HRESULT,
    pub SetVariants: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: super::FontVariants) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUnderline(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUnderline {
    type Vtable = IUnderline_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa5fa8202_61c0_47d7_93ef_bc0b577c5f26);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUnderline_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
}
#[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
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
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for Inline {
    type Vtable = IInline_Vtbl;
    const IID: ::windows::core::GUID = <IInline as ::windows::core::Interface>::IID;
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a Inline {
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a Inline {
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
#[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"Foundation_Collections\"`*"]
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
pub struct InlineCollection(::windows::core::IUnknown);
#[cfg(feature = "Foundation_Collections")]
impl InlineCollection {
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterator<Inline>> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IIterable<Inline>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).First)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IIterator<Inline>>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAt(&self, index: u32) -> ::windows::core::Result<Inline> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetAt)(::core::mem::transmute_copy(this), index, &mut result__).from_abi::<Inline>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Size)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetView(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<Inline>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetView)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<Inline>>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn IndexOf<'a, Param0: ::windows::core::IntoParam<'a, Inline>>(&self, value: Param0, index: &mut u32) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IndexOf)(::core::mem::transmute_copy(this), value.into_param().abi(), index, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetAt<'a, Param1: ::windows::core::IntoParam<'a, Inline>>(&self, index: u32, value: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetAt)(::core::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn InsertAt<'a, Param1: ::windows::core::IntoParam<'a, Inline>>(&self, index: u32, value: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).InsertAt)(::core::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveAt)(::core::mem::transmute_copy(this), index).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Append<'a, Param0: ::windows::core::IntoParam<'a, Inline>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Append)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAtEnd(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveAtEnd)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Clear)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, startindex: u32, items: &mut [::core::option::Option<Inline>]) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetMany)(::core::mem::transmute_copy(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReplaceAll(&self, items: &[::core::option::Option<Inline>]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).ReplaceAll)(::core::mem::transmute_copy(this), items.len() as u32, ::core::mem::transmute(items.as_ptr())).ok() }
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
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::Interface for InlineCollection {
    type Vtable = super::super::super::Foundation::Collections::IVector_Vtbl<Inline>;
    const IID: ::windows::core::GUID = <super::super::super::Foundation::Collections::IVector<Inline> as ::windows::core::Interface>::IID;
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a InlineCollection {
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a InlineCollection {
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
#[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
#[repr(transparent)]
pub struct InlineUIContainer(::windows::core::IUnknown);
impl InlineUIContainer {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<InlineUIContainer, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn Child(&self) -> ::windows::core::Result<super::UIElement> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Child)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::UIElement>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetChild<'a, Param0: ::windows::core::IntoParam<'a, super::UIElement>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetChild)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
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
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for InlineUIContainer {
    type Vtable = IInlineUIContainer_Vtbl;
    const IID: ::windows::core::GUID = <IInlineUIContainer as ::windows::core::Interface>::IID;
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a InlineUIContainer {
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a InlineUIContainer {
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
#[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
#[repr(transparent)]
pub struct Italic(::windows::core::IUnknown);
impl Italic {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
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
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for Italic {
    type Vtable = IItalic_Vtbl;
    const IID: ::windows::core::GUID = <IItalic as ::windows::core::Interface>::IID;
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a Italic {
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a Italic {
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
#[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
#[repr(transparent)]
pub struct LineBreak(::windows::core::IUnknown);
impl LineBreak {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
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
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for LineBreak {
    type Vtable = ILineBreak_Vtbl;
    const IID: ::windows::core::GUID = <ILineBreak as ::windows::core::Interface>::IID;
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a LineBreak {
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a LineBreak {
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
#[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
impl ::core::default::Default for LogicalDirection {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for LogicalDirection {
    type Abi = Self;
}
impl ::core::fmt::Debug for LogicalDirection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LogicalDirection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LogicalDirection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Documents.LogicalDirection;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
#[repr(transparent)]
pub struct Paragraph(::windows::core::IUnknown);
impl Paragraph {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Paragraph, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Inlines(&self) -> ::windows::core::Result<InlineCollection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Inlines)(::core::mem::transmute_copy(this), &mut result__).from_abi::<InlineCollection>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn TextIndent(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TextIndent)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetTextIndent(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetTextIndent)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn TextIndentProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IParagraphStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TextIndentProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
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
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for Paragraph {
    type Vtable = IParagraph_Vtbl;
    const IID: ::windows::core::GUID = <IParagraph as ::windows::core::Interface>::IID;
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a Paragraph {
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a Paragraph {
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
#[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
#[repr(transparent)]
pub struct PlaceContentLinkProvider(::windows::core::IUnknown);
impl PlaceContentLinkProvider {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
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
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PlaceContentLinkProvider {
    type Vtable = IPlaceContentLinkProvider_Vtbl;
    const IID: ::windows::core::GUID = <IPlaceContentLinkProvider as ::windows::core::Interface>::IID;
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PlaceContentLinkProvider {
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PlaceContentLinkProvider {
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
#[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
#[repr(transparent)]
pub struct Run(::windows::core::IUnknown);
impl Run {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Run, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Text)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetText<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetText)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn FlowDirection(&self) -> ::windows::core::Result<super::FlowDirection> {
        let this = self;
        unsafe {
            let mut result__: super::FlowDirection = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FlowDirection)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::FlowDirection>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetFlowDirection(&self, value: super::FlowDirection) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetFlowDirection)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn FlowDirectionProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IRunStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FlowDirectionProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
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
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for Run {
    type Vtable = IRun_Vtbl;
    const IID: ::windows::core::GUID = <IRun as ::windows::core::Interface>::IID;
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a Run {
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a Run {
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
#[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
#[repr(transparent)]
pub struct Span(::windows::core::IUnknown);
impl Span {
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Inlines(&self) -> ::windows::core::Result<InlineCollection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Inlines)(::core::mem::transmute_copy(this), &mut result__).from_abi::<InlineCollection>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetInlines<'a, Param0: ::windows::core::IntoParam<'a, InlineCollection>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetInlines)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn new() -> ::windows::core::Result<Span> {
        Self::ISpanFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstance)(::core::mem::transmute_copy(this), ::core::ptr::null_mut(), &mut ::core::option::Option::<::windows::core::IInspectable>::None as *mut _ as _, &mut result__).from_abi::<Span>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn compose<T: ::windows::core::Compose>(compose: T) -> ::windows::core::Result<Span> {
        Self::ISpanFactory(|this| unsafe {
            let (derived__, base__) = ::windows::core::Compose::compose(compose);
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstance)(::core::mem::transmute_copy(this), ::core::mem::transmute_copy(&derived__), base__ as *mut _ as _, &mut result__).from_abi::<Span>(result__)
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
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for Span {
    type Vtable = ISpan_Vtbl;
    const IID: ::windows::core::GUID = <ISpan as ::windows::core::Interface>::IID;
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a Span {
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a Span {
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
#[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
#[repr(transparent)]
pub struct TextElement(::windows::core::IUnknown);
impl TextElement {
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Name)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn FontSize(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FontSize)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetFontSize(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetFontSize)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn FontFamily(&self) -> ::windows::core::Result<super::Media::FontFamily> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FontFamily)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Media::FontFamily>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetFontFamily<'a, Param0: ::windows::core::IntoParam<'a, super::Media::FontFamily>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetFontFamily)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Text\"`*"]
    #[cfg(feature = "UI_Text")]
    pub fn FontWeight(&self) -> ::windows::core::Result<super::super::Text::FontWeight> {
        let this = self;
        unsafe {
            let mut result__: super::super::Text::FontWeight = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FontWeight)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Text::FontWeight>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Text\"`*"]
    #[cfg(feature = "UI_Text")]
    pub fn SetFontWeight<'a, Param0: ::windows::core::IntoParam<'a, super::super::Text::FontWeight>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetFontWeight)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Text\"`*"]
    #[cfg(feature = "UI_Text")]
    pub fn FontStyle(&self) -> ::windows::core::Result<super::super::Text::FontStyle> {
        let this = self;
        unsafe {
            let mut result__: super::super::Text::FontStyle = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FontStyle)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Text::FontStyle>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Text\"`*"]
    #[cfg(feature = "UI_Text")]
    pub fn SetFontStyle(&self, value: super::super::Text::FontStyle) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetFontStyle)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Text\"`*"]
    #[cfg(feature = "UI_Text")]
    pub fn FontStretch(&self) -> ::windows::core::Result<super::super::Text::FontStretch> {
        let this = self;
        unsafe {
            let mut result__: super::super::Text::FontStretch = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FontStretch)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Text::FontStretch>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Text\"`*"]
    #[cfg(feature = "UI_Text")]
    pub fn SetFontStretch(&self, value: super::super::Text::FontStretch) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetFontStretch)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn CharacterSpacing(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CharacterSpacing)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetCharacterSpacing(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetCharacterSpacing)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn Foreground(&self) -> ::windows::core::Result<super::Media::Brush> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Foreground)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Media::Brush>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetForeground<'a, Param0: ::windows::core::IntoParam<'a, super::Media::Brush>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetForeground)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Language)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetLanguage<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetLanguage)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ContentStart(&self) -> ::windows::core::Result<TextPointer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ContentStart)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TextPointer>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ContentEnd(&self) -> ::windows::core::Result<TextPointer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ContentEnd)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TextPointer>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ElementStart(&self) -> ::windows::core::Result<TextPointer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ElementStart)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TextPointer>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ElementEnd(&self) -> ::windows::core::Result<TextPointer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ElementEnd)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TextPointer>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn FindName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FindName)(::core::mem::transmute_copy(this), name.into_param().abi(), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn IsTextScaleFactorEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsTextScaleFactorEnabled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetIsTextScaleFactorEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetIsTextScaleFactorEnabled)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn AllowFocusOnInteraction(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement3>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AllowFocusOnInteraction)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetAllowFocusOnInteraction(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetAllowFocusOnInteraction)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn AccessKey(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ITextElement3>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AccessKey)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetAccessKey<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetAccessKey)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ExitDisplayModeOnAccessKeyInvoked(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement3>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ExitDisplayModeOnAccessKeyInvoked)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetExitDisplayModeOnAccessKeyInvoked(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetExitDisplayModeOnAccessKeyInvoked)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Text\"`*"]
    #[cfg(feature = "UI_Text")]
    pub fn TextDecorations(&self) -> ::windows::core::Result<super::super::Text::TextDecorations> {
        let this = &::windows::core::Interface::cast::<ITextElement4>(self)?;
        unsafe {
            let mut result__: super::super::Text::TextDecorations = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TextDecorations)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Text::TextDecorations>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Text\"`*"]
    #[cfg(feature = "UI_Text")]
    pub fn SetTextDecorations(&self, value: super::super::Text::TextDecorations) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetTextDecorations)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn IsAccessKeyScope(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement4>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsAccessKeyScope)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetIsAccessKeyScope(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetIsAccessKeyScope)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn AccessKeyScopeOwner(&self) -> ::windows::core::Result<super::DependencyObject> {
        let this = &::windows::core::Interface::cast::<ITextElement4>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AccessKeyScopeOwner)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyObject>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetAccessKeyScopeOwner<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetAccessKeyScopeOwner)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn KeyTipPlacementMode(&self) -> ::windows::core::Result<super::Input::KeyTipPlacementMode> {
        let this = &::windows::core::Interface::cast::<ITextElement4>(self)?;
        unsafe {
            let mut result__: super::Input::KeyTipPlacementMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).KeyTipPlacementMode)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Input::KeyTipPlacementMode>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetKeyTipPlacementMode(&self, value: super::Input::KeyTipPlacementMode) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetKeyTipPlacementMode)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn KeyTipHorizontalOffset(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<ITextElement4>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).KeyTipHorizontalOffset)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetKeyTipHorizontalOffset(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetKeyTipHorizontalOffset)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn KeyTipVerticalOffset(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<ITextElement4>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).KeyTipVerticalOffset)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetKeyTipVerticalOffset(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetKeyTipVerticalOffset)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"Foundation\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(all(feature = "Foundation", feature = "UI_Xaml_Input"))]
    pub fn AccessKeyDisplayRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<TextElement, super::Input::AccessKeyDisplayRequestedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ITextElement4>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AccessKeyDisplayRequested)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveAccessKeyDisplayRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveAccessKeyDisplayRequested)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"Foundation\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(all(feature = "Foundation", feature = "UI_Xaml_Input"))]
    pub fn AccessKeyDisplayDismissed<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<TextElement, super::Input::AccessKeyDisplayDismissedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ITextElement4>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AccessKeyDisplayDismissed)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveAccessKeyDisplayDismissed<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveAccessKeyDisplayDismissed)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"Foundation\"`, `\"UI_Xaml_Input\"`*"]
    #[cfg(all(feature = "Foundation", feature = "UI_Xaml_Input"))]
    pub fn AccessKeyInvoked<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<TextElement, super::Input::AccessKeyInvokedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ITextElement4>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AccessKeyInvoked)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveAccessKeyInvoked<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveAccessKeyInvoked)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn XamlRoot(&self) -> ::windows::core::Result<super::XamlRoot> {
        let this = &::windows::core::Interface::cast::<ITextElement5>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).XamlRoot)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::XamlRoot>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetXamlRoot<'a, Param0: ::windows::core::IntoParam<'a, super::XamlRoot>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement5>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetXamlRoot)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn FontSizeProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITextElementStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FontSizeProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn FontFamilyProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITextElementStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FontFamilyProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn FontWeightProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITextElementStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FontWeightProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn FontStyleProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITextElementStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FontStyleProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn FontStretchProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITextElementStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FontStretchProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn CharacterSpacingProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITextElementStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CharacterSpacingProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ForegroundProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITextElementStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ForegroundProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn LanguageProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITextElementStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LanguageProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn IsTextScaleFactorEnabledProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITextElementStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsTextScaleFactorEnabledProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn AllowFocusOnInteractionProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITextElementStatics3(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AllowFocusOnInteractionProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn AccessKeyProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITextElementStatics3(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AccessKeyProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ExitDisplayModeOnAccessKeyInvokedProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITextElementStatics3(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ExitDisplayModeOnAccessKeyInvokedProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn TextDecorationsProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITextElementStatics4(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TextDecorationsProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn IsAccessKeyScopeProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITextElementStatics4(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsAccessKeyScopeProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn AccessKeyScopeOwnerProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITextElementStatics4(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AccessKeyScopeOwnerProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn KeyTipPlacementModeProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITextElementStatics4(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).KeyTipPlacementModeProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn KeyTipHorizontalOffsetProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITextElementStatics4(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).KeyTipHorizontalOffsetProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn KeyTipVerticalOffsetProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITextElementStatics4(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).KeyTipVerticalOffsetProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
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
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for TextElement {
    type Vtable = ITextElement_Vtbl;
    const IID: ::windows::core::GUID = <ITextElement as ::windows::core::Interface>::IID;
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a TextElement {
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a TextElement {
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
#[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
#[repr(transparent)]
pub struct TextHighlighter(::windows::core::IUnknown);
impl TextHighlighter {
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Ranges(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<TextRange>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Ranges)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVector<TextRange>>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn Foreground(&self) -> ::windows::core::Result<super::Media::Brush> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Foreground)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Media::Brush>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetForeground<'a, Param0: ::windows::core::IntoParam<'a, super::Media::Brush>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetForeground)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn Background(&self) -> ::windows::core::Result<super::Media::Brush> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Background)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Media::Brush>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetBackground<'a, Param0: ::windows::core::IntoParam<'a, super::Media::Brush>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetBackground)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn new() -> ::windows::core::Result<TextHighlighter> {
        Self::ITextHighlighterFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstance)(::core::mem::transmute_copy(this), ::core::ptr::null_mut(), &mut ::core::option::Option::<::windows::core::IInspectable>::None as *mut _ as _, &mut result__).from_abi::<TextHighlighter>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn compose<T: ::windows::core::Compose>(compose: T) -> ::windows::core::Result<TextHighlighter> {
        Self::ITextHighlighterFactory(|this| unsafe {
            let (derived__, base__) = ::windows::core::Compose::compose(compose);
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstance)(::core::mem::transmute_copy(this), ::core::mem::transmute_copy(&derived__), base__ as *mut _ as _, &mut result__).from_abi::<TextHighlighter>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ForegroundProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITextHighlighterStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ForegroundProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn BackgroundProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITextHighlighterStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).BackgroundProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
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
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for TextHighlighter {
    type Vtable = ITextHighlighter_Vtbl;
    const IID: ::windows::core::GUID = <ITextHighlighter as ::windows::core::Interface>::IID;
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a TextHighlighter {
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a TextHighlighter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for TextHighlighter {}
unsafe impl ::core::marker::Sync for TextHighlighter {}
#[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
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
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for TextHighlighterBase {
    type Vtable = ITextHighlighterBase_Vtbl;
    const IID: ::windows::core::GUID = <ITextHighlighterBase as ::windows::core::Interface>::IID;
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a TextHighlighterBase {
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a TextHighlighterBase {
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
#[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
#[repr(transparent)]
pub struct TextPointer(::windows::core::IUnknown);
impl TextPointer {
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn Parent(&self) -> ::windows::core::Result<super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Parent)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyObject>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn VisualParent(&self) -> ::windows::core::Result<super::FrameworkElement> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).VisualParent)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::FrameworkElement>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn LogicalDirection(&self) -> ::windows::core::Result<LogicalDirection> {
        let this = self;
        unsafe {
            let mut result__: LogicalDirection = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LogicalDirection)(::core::mem::transmute_copy(this), &mut result__).from_abi::<LogicalDirection>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn Offset(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Offset)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetCharacterRect(&self, direction: LogicalDirection) -> ::windows::core::Result<super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetCharacterRect)(::core::mem::transmute_copy(this), direction, &mut result__).from_abi::<super::super::super::Foundation::Rect>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GetPositionAtOffset(&self, offset: i32, direction: LogicalDirection) -> ::windows::core::Result<TextPointer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetPositionAtOffset)(::core::mem::transmute_copy(this), offset, direction, &mut result__).from_abi::<TextPointer>(result__)
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
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for TextPointer {
    type Vtable = ITextPointer_Vtbl;
    const IID: ::windows::core::GUID = <ITextPointer as ::windows::core::Interface>::IID;
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a TextPointer {
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a TextPointer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for TextPointer {}
unsafe impl ::core::marker::Sync for TextPointer {}
#[repr(C)]
#[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
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
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
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
#[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
#[repr(transparent)]
pub struct Typography(::windows::core::IUnknown);
impl Typography {
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn AnnotationAlternatesProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AnnotationAlternatesProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GetAnnotationAlternates<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<i32> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetAnnotationAlternates)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<i32>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetAnnotationAlternates<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: i32) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe { (::windows::core::Interface::vtable(this).SetAnnotationAlternates)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn EastAsianExpertFormsProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).EastAsianExpertFormsProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GetEastAsianExpertForms<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetEastAsianExpertForms)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetEastAsianExpertForms<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: bool) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe { (::windows::core::Interface::vtable(this).SetEastAsianExpertForms)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn EastAsianLanguageProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).EastAsianLanguageProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GetEastAsianLanguage<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<super::FontEastAsianLanguage> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: super::FontEastAsianLanguage = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetEastAsianLanguage)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<super::FontEastAsianLanguage>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetEastAsianLanguage<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: super::FontEastAsianLanguage) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe { (::windows::core::Interface::vtable(this).SetEastAsianLanguage)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn EastAsianWidthsProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).EastAsianWidthsProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GetEastAsianWidths<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<super::FontEastAsianWidths> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: super::FontEastAsianWidths = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetEastAsianWidths)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<super::FontEastAsianWidths>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetEastAsianWidths<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: super::FontEastAsianWidths) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe { (::windows::core::Interface::vtable(this).SetEastAsianWidths)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn StandardLigaturesProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StandardLigaturesProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GetStandardLigatures<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetStandardLigatures)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetStandardLigatures<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: bool) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe { (::windows::core::Interface::vtable(this).SetStandardLigatures)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ContextualLigaturesProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ContextualLigaturesProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GetContextualLigatures<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetContextualLigatures)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetContextualLigatures<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: bool) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe { (::windows::core::Interface::vtable(this).SetContextualLigatures)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn DiscretionaryLigaturesProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DiscretionaryLigaturesProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GetDiscretionaryLigatures<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDiscretionaryLigatures)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetDiscretionaryLigatures<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: bool) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe { (::windows::core::Interface::vtable(this).SetDiscretionaryLigatures)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn HistoricalLigaturesProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).HistoricalLigaturesProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GetHistoricalLigatures<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetHistoricalLigatures)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetHistoricalLigatures<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: bool) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe { (::windows::core::Interface::vtable(this).SetHistoricalLigatures)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn StandardSwashesProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StandardSwashesProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GetStandardSwashes<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<i32> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetStandardSwashes)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<i32>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetStandardSwashes<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: i32) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe { (::windows::core::Interface::vtable(this).SetStandardSwashes)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ContextualSwashesProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ContextualSwashesProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GetContextualSwashes<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<i32> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetContextualSwashes)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<i32>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetContextualSwashes<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: i32) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe { (::windows::core::Interface::vtable(this).SetContextualSwashes)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn ContextualAlternatesProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ContextualAlternatesProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GetContextualAlternates<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetContextualAlternates)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetContextualAlternates<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: bool) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe { (::windows::core::Interface::vtable(this).SetContextualAlternates)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn StylisticAlternatesProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StylisticAlternatesProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GetStylisticAlternates<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<i32> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetStylisticAlternates)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<i32>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetStylisticAlternates<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: i32) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe { (::windows::core::Interface::vtable(this).SetStylisticAlternates)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn StylisticSet1Property() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StylisticSet1Property)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GetStylisticSet1<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetStylisticSet1)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetStylisticSet1<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: bool) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe { (::windows::core::Interface::vtable(this).SetStylisticSet1)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn StylisticSet2Property() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StylisticSet2Property)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GetStylisticSet2<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetStylisticSet2)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetStylisticSet2<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: bool) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe { (::windows::core::Interface::vtable(this).SetStylisticSet2)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn StylisticSet3Property() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StylisticSet3Property)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GetStylisticSet3<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetStylisticSet3)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetStylisticSet3<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: bool) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe { (::windows::core::Interface::vtable(this).SetStylisticSet3)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn StylisticSet4Property() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StylisticSet4Property)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GetStylisticSet4<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetStylisticSet4)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetStylisticSet4<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: bool) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe { (::windows::core::Interface::vtable(this).SetStylisticSet4)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn StylisticSet5Property() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StylisticSet5Property)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GetStylisticSet5<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetStylisticSet5)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetStylisticSet5<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: bool) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe { (::windows::core::Interface::vtable(this).SetStylisticSet5)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn StylisticSet6Property() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StylisticSet6Property)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GetStylisticSet6<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetStylisticSet6)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetStylisticSet6<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: bool) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe { (::windows::core::Interface::vtable(this).SetStylisticSet6)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn StylisticSet7Property() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StylisticSet7Property)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GetStylisticSet7<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetStylisticSet7)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetStylisticSet7<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: bool) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe { (::windows::core::Interface::vtable(this).SetStylisticSet7)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn StylisticSet8Property() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StylisticSet8Property)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GetStylisticSet8<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetStylisticSet8)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetStylisticSet8<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: bool) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe { (::windows::core::Interface::vtable(this).SetStylisticSet8)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn StylisticSet9Property() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StylisticSet9Property)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GetStylisticSet9<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetStylisticSet9)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetStylisticSet9<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: bool) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe { (::windows::core::Interface::vtable(this).SetStylisticSet9)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn StylisticSet10Property() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StylisticSet10Property)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GetStylisticSet10<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetStylisticSet10)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetStylisticSet10<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: bool) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe { (::windows::core::Interface::vtable(this).SetStylisticSet10)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn StylisticSet11Property() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StylisticSet11Property)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GetStylisticSet11<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetStylisticSet11)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetStylisticSet11<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: bool) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe { (::windows::core::Interface::vtable(this).SetStylisticSet11)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn StylisticSet12Property() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StylisticSet12Property)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GetStylisticSet12<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetStylisticSet12)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetStylisticSet12<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: bool) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe { (::windows::core::Interface::vtable(this).SetStylisticSet12)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn StylisticSet13Property() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StylisticSet13Property)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GetStylisticSet13<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetStylisticSet13)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetStylisticSet13<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: bool) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe { (::windows::core::Interface::vtable(this).SetStylisticSet13)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn StylisticSet14Property() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StylisticSet14Property)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GetStylisticSet14<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetStylisticSet14)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetStylisticSet14<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: bool) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe { (::windows::core::Interface::vtable(this).SetStylisticSet14)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn StylisticSet15Property() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StylisticSet15Property)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GetStylisticSet15<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetStylisticSet15)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetStylisticSet15<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: bool) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe { (::windows::core::Interface::vtable(this).SetStylisticSet15)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn StylisticSet16Property() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StylisticSet16Property)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GetStylisticSet16<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetStylisticSet16)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetStylisticSet16<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: bool) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe { (::windows::core::Interface::vtable(this).SetStylisticSet16)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn StylisticSet17Property() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StylisticSet17Property)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GetStylisticSet17<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetStylisticSet17)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetStylisticSet17<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: bool) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe { (::windows::core::Interface::vtable(this).SetStylisticSet17)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn StylisticSet18Property() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StylisticSet18Property)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GetStylisticSet18<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetStylisticSet18)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetStylisticSet18<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: bool) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe { (::windows::core::Interface::vtable(this).SetStylisticSet18)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn StylisticSet19Property() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StylisticSet19Property)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GetStylisticSet19<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetStylisticSet19)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetStylisticSet19<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: bool) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe { (::windows::core::Interface::vtable(this).SetStylisticSet19)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn StylisticSet20Property() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StylisticSet20Property)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GetStylisticSet20<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetStylisticSet20)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetStylisticSet20<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: bool) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe { (::windows::core::Interface::vtable(this).SetStylisticSet20)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn CapitalsProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CapitalsProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GetCapitals<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<super::FontCapitals> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: super::FontCapitals = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetCapitals)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<super::FontCapitals>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetCapitals<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: super::FontCapitals) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe { (::windows::core::Interface::vtable(this).SetCapitals)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn CapitalSpacingProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CapitalSpacingProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GetCapitalSpacing<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetCapitalSpacing)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetCapitalSpacing<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: bool) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe { (::windows::core::Interface::vtable(this).SetCapitalSpacing)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn KerningProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).KerningProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GetKerning<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetKerning)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetKerning<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: bool) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe { (::windows::core::Interface::vtable(this).SetKerning)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn CaseSensitiveFormsProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CaseSensitiveFormsProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GetCaseSensitiveForms<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetCaseSensitiveForms)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetCaseSensitiveForms<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: bool) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe { (::windows::core::Interface::vtable(this).SetCaseSensitiveForms)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn HistoricalFormsProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).HistoricalFormsProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GetHistoricalForms<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetHistoricalForms)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetHistoricalForms<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: bool) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe { (::windows::core::Interface::vtable(this).SetHistoricalForms)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn FractionProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FractionProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GetFraction<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<super::FontFraction> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: super::FontFraction = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetFraction)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<super::FontFraction>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetFraction<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: super::FontFraction) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe { (::windows::core::Interface::vtable(this).SetFraction)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn NumeralStyleProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).NumeralStyleProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GetNumeralStyle<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<super::FontNumeralStyle> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: super::FontNumeralStyle = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetNumeralStyle)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<super::FontNumeralStyle>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetNumeralStyle<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: super::FontNumeralStyle) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe { (::windows::core::Interface::vtable(this).SetNumeralStyle)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn NumeralAlignmentProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).NumeralAlignmentProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GetNumeralAlignment<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<super::FontNumeralAlignment> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: super::FontNumeralAlignment = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetNumeralAlignment)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<super::FontNumeralAlignment>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetNumeralAlignment<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: super::FontNumeralAlignment) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe { (::windows::core::Interface::vtable(this).SetNumeralAlignment)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SlashedZeroProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SlashedZeroProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GetSlashedZero<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetSlashedZero)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetSlashedZero<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: bool) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe { (::windows::core::Interface::vtable(this).SetSlashedZero)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn MathematicalGreekProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MathematicalGreekProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GetMathematicalGreek<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetMathematicalGreek)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetMathematicalGreek<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: bool) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe { (::windows::core::Interface::vtable(this).SetMathematicalGreek)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn VariantsProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).VariantsProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn GetVariants<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows::core::Result<super::FontVariants> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: super::FontVariants = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetVariants)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<super::FontVariants>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
    pub fn SetVariants<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: super::FontVariants) -> ::windows::core::Result<()> {
        Self::ITypographyStatics(|this| unsafe { (::windows::core::Interface::vtable(this).SetVariants)(::core::mem::transmute_copy(this), element.into_param().abi(), value).ok() })
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
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for Typography {
    type Vtable = ITypography_Vtbl;
    const IID: ::windows::core::GUID = <ITypography as ::windows::core::Interface>::IID;
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a Typography {
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a Typography {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for Typography {}
unsafe impl ::core::marker::Sync for Typography {}
#[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
#[repr(transparent)]
pub struct Underline(::windows::core::IUnknown);
impl Underline {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
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
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for Underline {
    type Vtable = IUnderline_Vtbl;
    const IID: ::windows::core::GUID = <IUnderline as ::windows::core::Interface>::IID;
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a Underline {
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a Underline {
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
#[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
impl ::core::default::Default for UnderlineStyle {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for UnderlineStyle {
    type Abi = Self;
}
impl ::core::fmt::Debug for UnderlineStyle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UnderlineStyle").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UnderlineStyle {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Documents.UnderlineStyle;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
