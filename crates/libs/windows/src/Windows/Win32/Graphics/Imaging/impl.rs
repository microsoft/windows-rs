pub trait IWICBitmapImpl: Sized + IWICBitmapSourceImpl {
    fn Lock();
    fn SetPalette();
    fn SetResolution();
}
pub trait IWICBitmapClipperImpl: Sized + IWICBitmapSourceImpl {
    fn Initialize();
}
pub trait IWICBitmapCodecInfoImpl: Sized + IWICComponentInfoImpl {
    fn GetContainerFormat();
    fn GetPixelFormats();
    fn GetColorManagementVersion();
    fn GetDeviceManufacturer();
    fn GetDeviceModels();
    fn GetMimeTypes();
    fn GetFileExtensions();
    fn DoesSupportAnimation();
    fn DoesSupportChromakey();
    fn DoesSupportLossless();
    fn DoesSupportMultiframe();
    fn MatchesMimeType();
}
pub trait IWICBitmapCodecProgressNotificationImpl: Sized {
    fn RegisterProgressNotification();
}
pub trait IWICBitmapDecoderImpl: Sized {
    fn QueryCapability();
    fn Initialize();
    fn GetContainerFormat();
    fn GetDecoderInfo();
    fn CopyPalette();
    fn GetMetadataQueryReader();
    fn GetPreview();
    fn GetColorContexts();
    fn GetThumbnail();
    fn GetFrameCount();
    fn GetFrame();
}
pub trait IWICBitmapDecoderInfoImpl: Sized + IWICBitmapCodecInfoImpl + IWICComponentInfoImpl {
    fn GetPatterns();
    fn MatchesPattern();
    fn CreateInstance();
}
pub trait IWICBitmapEncoderImpl: Sized {
    fn Initialize();
    fn GetContainerFormat();
    fn GetEncoderInfo();
    fn SetColorContexts();
    fn SetPalette();
    fn SetThumbnail();
    fn SetPreview();
    fn CreateNewFrame();
    fn Commit();
    fn GetMetadataQueryWriter();
}
pub trait IWICBitmapEncoderInfoImpl: Sized + IWICBitmapCodecInfoImpl + IWICComponentInfoImpl {
    fn CreateInstance();
}
pub trait IWICBitmapFlipRotatorImpl: Sized + IWICBitmapSourceImpl {
    fn Initialize();
}
pub trait IWICBitmapFrameDecodeImpl: Sized + IWICBitmapSourceImpl {
    fn GetMetadataQueryReader();
    fn GetColorContexts();
    fn GetThumbnail();
}
pub trait IWICBitmapFrameEncodeImpl: Sized {
    fn Initialize();
    fn SetSize();
    fn SetResolution();
    fn SetPixelFormat();
    fn SetColorContexts();
    fn SetPalette();
    fn SetThumbnail();
    fn WritePixels();
    fn WriteSource();
    fn Commit();
    fn GetMetadataQueryWriter();
}
pub trait IWICBitmapLockImpl: Sized {
    fn GetSize();
    fn GetStride();
    fn GetDataPointer();
    fn GetPixelFormat();
}
pub trait IWICBitmapScalerImpl: Sized + IWICBitmapSourceImpl {
    fn Initialize();
}
pub trait IWICBitmapSourceImpl: Sized {
    fn GetSize();
    fn GetPixelFormat();
    fn GetResolution();
    fn CopyPalette();
    fn CopyPixels();
}
pub trait IWICBitmapSourceTransformImpl: Sized {
    fn CopyPixels();
    fn GetClosestSize();
    fn GetClosestPixelFormat();
    fn DoesSupportTransform();
}
pub trait IWICColorContextImpl: Sized {
    fn InitializeFromFilename();
    fn InitializeFromMemory();
    fn InitializeFromExifColorSpace();
    fn GetType();
    fn GetProfileBytes();
    fn GetExifColorSpace();
}
pub trait IWICColorTransformImpl: Sized + IWICBitmapSourceImpl {
    fn Initialize();
}
pub trait IWICComponentFactoryImpl: Sized + IWICImagingFactoryImpl {
    fn CreateMetadataReader();
    fn CreateMetadataReaderFromContainer();
    fn CreateMetadataWriter();
    fn CreateMetadataWriterFromReader();
    fn CreateQueryReaderFromBlockReader();
    fn CreateQueryWriterFromBlockWriter();
    fn CreateEncoderPropertyBag();
}
pub trait IWICComponentInfoImpl: Sized {
    fn GetComponentType();
    fn GetCLSID();
    fn GetSigningStatus();
    fn GetAuthor();
    fn GetVendorGUID();
    fn GetVersion();
    fn GetSpecVersion();
    fn GetFriendlyName();
}
pub trait IWICDdsDecoderImpl: Sized {
    fn GetParameters();
    fn GetFrame();
}
pub trait IWICDdsEncoderImpl: Sized {
    fn SetParameters();
    fn GetParameters();
    fn CreateNewFrame();
}
pub trait IWICDdsFrameDecodeImpl: Sized {
    fn GetSizeInBlocks();
    fn GetFormatInfo();
    fn CopyBlocks();
}
pub trait IWICDevelopRawImpl: Sized + IWICBitmapFrameDecodeImpl + IWICBitmapSourceImpl {
    fn QueryRawCapabilitiesInfo();
    fn LoadParameterSet();
    fn GetCurrentParameterSet();
    fn SetExposureCompensation();
    fn GetExposureCompensation();
    fn SetWhitePointRGB();
    fn GetWhitePointRGB();
    fn SetNamedWhitePoint();
    fn GetNamedWhitePoint();
    fn SetWhitePointKelvin();
    fn GetWhitePointKelvin();
    fn GetKelvinRangeInfo();
    fn SetContrast();
    fn GetContrast();
    fn SetGamma();
    fn GetGamma();
    fn SetSharpness();
    fn GetSharpness();
    fn SetSaturation();
    fn GetSaturation();
    fn SetTint();
    fn GetTint();
    fn SetNoiseReduction();
    fn GetNoiseReduction();
    fn SetDestinationColorContext();
    fn SetToneCurve();
    fn GetToneCurve();
    fn SetRotation();
    fn GetRotation();
    fn SetRenderMode();
    fn GetRenderMode();
    fn SetNotificationCallback();
}
pub trait IWICDevelopRawNotificationCallbackImpl: Sized {
    fn Notify();
}
pub trait IWICEnumMetadataItemImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
pub trait IWICFastMetadataEncoderImpl: Sized {
    fn Commit();
    fn GetMetadataQueryWriter();
}
pub trait IWICFormatConverterImpl: Sized + IWICBitmapSourceImpl {
    fn Initialize();
    fn CanConvert();
}
pub trait IWICFormatConverterInfoImpl: Sized + IWICComponentInfoImpl {
    fn GetPixelFormats();
    fn CreateInstance();
}
pub trait IWICImagingFactoryImpl: Sized {
    fn CreateDecoderFromFilename();
    fn CreateDecoderFromStream();
    fn CreateDecoderFromFileHandle();
    fn CreateComponentInfo();
    fn CreateDecoder();
    fn CreateEncoder();
    fn CreatePalette();
    fn CreateFormatConverter();
    fn CreateBitmapScaler();
    fn CreateBitmapClipper();
    fn CreateBitmapFlipRotator();
    fn CreateStream();
    fn CreateColorContext();
    fn CreateColorTransformer();
    fn CreateBitmap();
    fn CreateBitmapFromSource();
    fn CreateBitmapFromSourceRect();
    fn CreateBitmapFromMemory();
    fn CreateBitmapFromHBITMAP();
    fn CreateBitmapFromHICON();
    fn CreateComponentEnumerator();
    fn CreateFastMetadataEncoderFromDecoder();
    fn CreateFastMetadataEncoderFromFrameDecode();
    fn CreateQueryWriter();
    fn CreateQueryWriterFromReader();
}
pub trait IWICJpegFrameDecodeImpl: Sized {
    fn DoesSupportIndexing();
    fn SetIndexing();
    fn ClearIndexing();
    fn GetAcHuffmanTable();
    fn GetDcHuffmanTable();
    fn GetQuantizationTable();
    fn GetFrameHeader();
    fn GetScanHeader();
    fn CopyScan();
    fn CopyMinimalStream();
}
pub trait IWICJpegFrameEncodeImpl: Sized {
    fn GetAcHuffmanTable();
    fn GetDcHuffmanTable();
    fn GetQuantizationTable();
    fn WriteScan();
}
pub trait IWICMetadataBlockReaderImpl: Sized {
    fn GetContainerFormat();
    fn GetCount();
    fn GetReaderByIndex();
    fn GetEnumerator();
}
pub trait IWICMetadataBlockWriterImpl: Sized + IWICMetadataBlockReaderImpl {
    fn InitializeFromBlockReader();
    fn GetWriterByIndex();
    fn AddWriter();
    fn SetWriterByIndex();
    fn RemoveWriterByIndex();
}
pub trait IWICMetadataHandlerInfoImpl: Sized + IWICComponentInfoImpl {
    fn GetMetadataFormat();
    fn GetContainerFormats();
    fn GetDeviceManufacturer();
    fn GetDeviceModels();
    fn DoesRequireFullStream();
    fn DoesSupportPadding();
    fn DoesRequireFixedSize();
}
pub trait IWICMetadataQueryReaderImpl: Sized {
    fn GetContainerFormat();
    fn GetLocation();
    fn GetMetadataByName();
    fn GetEnumerator();
}
pub trait IWICMetadataQueryWriterImpl: Sized + IWICMetadataQueryReaderImpl {
    fn SetMetadataByName();
    fn RemoveMetadataByName();
}
pub trait IWICMetadataReaderImpl: Sized {
    fn GetMetadataFormat();
    fn GetMetadataHandlerInfo();
    fn GetCount();
    fn GetValueByIndex();
    fn GetValue();
    fn GetEnumerator();
}
pub trait IWICMetadataReaderInfoImpl: Sized + IWICMetadataHandlerInfoImpl + IWICComponentInfoImpl {
    fn GetPatterns();
    fn MatchesPattern();
    fn CreateInstance();
}
pub trait IWICMetadataWriterImpl: Sized + IWICMetadataReaderImpl {
    fn SetValue();
    fn SetValueByIndex();
    fn RemoveValue();
    fn RemoveValueByIndex();
}
pub trait IWICMetadataWriterInfoImpl: Sized + IWICMetadataHandlerInfoImpl + IWICComponentInfoImpl {
    fn GetHeader();
    fn CreateInstance();
}
pub trait IWICPaletteImpl: Sized {
    fn InitializePredefined();
    fn InitializeCustom();
    fn InitializeFromBitmap();
    fn InitializeFromPalette();
    fn GetType();
    fn GetColorCount();
    fn GetColors();
    fn IsBlackWhite();
    fn IsGrayscale();
    fn HasAlpha();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWICPersistStreamImpl: Sized + IPersistStreamImpl + IPersistImpl {
    fn LoadEx();
    fn SaveEx();
}
pub trait IWICPixelFormatInfoImpl: Sized + IWICComponentInfoImpl {
    fn GetFormatGUID();
    fn GetColorContext();
    fn GetBitsPerPixel();
    fn GetChannelCount();
    fn GetChannelMask();
}
pub trait IWICPixelFormatInfo2Impl: Sized + IWICPixelFormatInfoImpl + IWICComponentInfoImpl {
    fn SupportsTransparency();
    fn GetNumericRepresentation();
}
pub trait IWICPlanarBitmapFrameEncodeImpl: Sized {
    fn WritePixels();
    fn WriteSource();
}
pub trait IWICPlanarBitmapSourceTransformImpl: Sized {
    fn DoesSupportTransform();
    fn CopyPixels();
}
pub trait IWICPlanarFormatConverterImpl: Sized + IWICBitmapSourceImpl {
    fn Initialize();
    fn CanConvert();
}
pub trait IWICProgressCallbackImpl: Sized {
    fn Notify();
}
pub trait IWICProgressiveLevelControlImpl: Sized {
    fn GetLevelCount();
    fn GetCurrentLevel();
    fn SetCurrentLevel();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWICStreamImpl: Sized + IStreamImpl + ISequentialStreamImpl {
    fn InitializeFromIStream();
    fn InitializeFromFilename();
    fn InitializeFromMemory();
    fn InitializeFromIStreamRegion();
}
pub trait IWICStreamProviderImpl: Sized {
    fn GetStream();
    fn GetPersistOptions();
    fn GetPreferredVendorGUID();
    fn RefreshStream();
}
