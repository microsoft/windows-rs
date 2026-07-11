#[inline]
pub unsafe fn WICConvertBitmapSource<P1>(dstformat: REFWICPixelFormatGUID, pisrc: P1) -> windows_core::Result<IWICBitmapSource>
where
    P1: windows_core::Param<IWICBitmapSource>,
{
    windows_core::link!("windowscodecs.dll" "system" fn WICConvertBitmapSource(dstformat : REFWICPixelFormatGUID, pisrc : *mut core::ffi::c_void, ppidst : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WICConvertBitmapSource(dstformat, pisrc.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn WICCreateBitmapFromSection(width: u32, height: u32, pixelformat: REFWICPixelFormatGUID, hsection: super::winnt::HANDLE, stride: u32, offset: u32) -> windows_core::Result<IWICBitmap> {
    windows_core::link!("windowscodecs.dll" "system" fn WICCreateBitmapFromSection(width : u32, height : u32, pixelformat : REFWICPixelFormatGUID, hsection : super::winnt::HANDLE, stride : u32, offset : u32, ppibitmap : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WICCreateBitmapFromSection(width, height, pixelformat, hsection, stride, offset, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn WICCreateBitmapFromSectionEx(width: u32, height: u32, pixelformat: REFWICPixelFormatGUID, hsection: super::winnt::HANDLE, stride: u32, offset: u32, desiredaccesslevel: WICSectionAccessLevel) -> windows_core::Result<IWICBitmap> {
    windows_core::link!("windowscodecs.dll" "system" fn WICCreateBitmapFromSectionEx(width : u32, height : u32, pixelformat : REFWICPixelFormatGUID, hsection : super::winnt::HANDLE, stride : u32, offset : u32, desiredaccesslevel : WICSectionAccessLevel, ppibitmap : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WICCreateBitmapFromSectionEx(width, height, pixelformat, hsection, stride, offset, desiredaccesslevel, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn WICMapGuidToShortName(guid: *const windows_core::GUID, wzname: Option<&mut [u16]>, pcchactual: *mut u32) -> windows_core::HRESULT {
    windows_core::link!("windowscodecs.dll" "system" fn WICMapGuidToShortName(guid : *const windows_core::GUID, cchname : u32, wzname : *mut u16, pcchactual : *mut u32) -> windows_core::HRESULT);
    unsafe { WICMapGuidToShortName(guid, wzname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(wzname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pcchactual as _) }
}
#[inline]
pub unsafe fn WICMapSchemaToName<P1>(guidmetadataformat: *const windows_core::GUID, pwzschema: P1, wzname: Option<&mut [u16]>, pcchactual: *mut u32) -> windows_core::HRESULT
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("windowscodecs.dll" "system" fn WICMapSchemaToName(guidmetadataformat : *const windows_core::GUID, pwzschema : windows_core::PCWSTR, cchname : u32, wzname : *mut u16, pcchactual : *mut u32) -> windows_core::HRESULT);
    unsafe { WICMapSchemaToName(guidmetadataformat, pwzschema.param().abi(), wzname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(wzname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pcchactual as _) }
}
#[inline]
pub unsafe fn WICMapShortNameToGuid<P0>(wzname: P0) -> windows_core::Result<windows_core::GUID>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("windowscodecs.dll" "system" fn WICMapShortNameToGuid(wzname : windows_core::PCWSTR, pguid : *mut windows_core::GUID) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WICMapShortNameToGuid(wzname.param().abi(), &mut result__).map(|| result__)
    }
}
pub const CATID_WICBitmapDecoders: windows_core::GUID = windows_core::GUID::from_u128(0x7ed96837_96f0_4812_b211_f13c24117ed3);
pub const CATID_WICBitmapEncoders: windows_core::GUID = windows_core::GUID::from_u128(0xac757296_3522_4e11_9862_c17be5a1767e);
pub const CATID_WICFormatConverters: windows_core::GUID = windows_core::GUID::from_u128(0x7835eae8_bf14_49d1_93ce_533a407b2248);
pub const CATID_WICMetadataReader: windows_core::GUID = windows_core::GUID::from_u128(0x05af94d8_7174_4cd2_be4a_4124b80ee4b8);
pub const CATID_WICMetadataWriter: windows_core::GUID = windows_core::GUID::from_u128(0xabe3b9a4_257d_4b97_bd1a_294af496222e);
pub const CATID_WICPixelFormats: windows_core::GUID = windows_core::GUID::from_u128(0x2b46e70f_cda7_473e_89f6_dc9630a2390b);
pub const CLSID_WICAdngDecoder: windows_core::GUID = windows_core::GUID::from_u128(0x981d9411_909e_42a7_8f5d_a747ff052edb);
pub const CLSID_WICBmpDecoder: windows_core::GUID = windows_core::GUID::from_u128(0x6b462062_7cbf_400d_9fdb_813dd10f2778);
pub const CLSID_WICBmpEncoder: windows_core::GUID = windows_core::GUID::from_u128(0x69be8bb4_d66d_47c8_865a_ed1589433782);
pub const CLSID_WICDdsDecoder: windows_core::GUID = windows_core::GUID::from_u128(0x9053699f_a341_429d_9e90_ee437cf80c73);
pub const CLSID_WICDdsEncoder: windows_core::GUID = windows_core::GUID::from_u128(0xa61dde94_66ce_4ac1_881b_71680588895e);
pub const CLSID_WICDefaultFormatConverter: windows_core::GUID = windows_core::GUID::from_u128(0x1a3f11dc_b514_4b17_8c5f_2154513852f1);
pub const CLSID_WICFormatConverterHighColor: windows_core::GUID = windows_core::GUID::from_u128(0xac75d454_9f37_48f8_b972_4e19bc856011);
pub const CLSID_WICFormatConverterNChannel: windows_core::GUID = windows_core::GUID::from_u128(0xc17cabb2_d4a3_47d7_a557_339b2efbd4f1);
pub const CLSID_WICFormatConverterWMPhoto: windows_core::GUID = windows_core::GUID::from_u128(0x9cb5172b_d600_46ba_ab77_77bb7e3a00d9);
pub const CLSID_WICGifDecoder: windows_core::GUID = windows_core::GUID::from_u128(0x381dda3c_9ce9_4834_a23e_1f98f8fc52be);
pub const CLSID_WICGifEncoder: windows_core::GUID = windows_core::GUID::from_u128(0x114f5598_0b22_40a0_86a1_c83ea495adbd);
pub const CLSID_WICHeifDecoder: windows_core::GUID = windows_core::GUID::from_u128(0xe9a4a80a_44fe_4de4_8971_7150b10a5199);
pub const CLSID_WICHeifEncoder: windows_core::GUID = windows_core::GUID::from_u128(0x0dbecec1_9eb3_4860_9c6f_ddbe86634575);
pub const CLSID_WICIcoDecoder: windows_core::GUID = windows_core::GUID::from_u128(0xc61bfcdf_2e0f_4aad_a8d7_e06bafebcdfe);
pub const CLSID_WICImagingCategories: windows_core::GUID = windows_core::GUID::from_u128(0xfae3d380_fea4_4623_8c75_c6b61110b681);
pub const CLSID_WICImagingFactory: windows_core::GUID = windows_core::GUID::from_u128(0xcacaf262_9370_4615_a13b_9f5539da4c0a);
pub const CLSID_WICImagingFactory1: windows_core::GUID = windows_core::GUID::from_u128(0xcacaf262_9370_4615_a13b_9f5539da4c0a);
pub const CLSID_WICImagingFactory2: windows_core::GUID = windows_core::GUID::from_u128(0x317d06e8_5f24_433d_bdf7_79ce68d8abc2);
pub const CLSID_WICJpegDecoder: windows_core::GUID = windows_core::GUID::from_u128(0x9456a480_e88b_43ea_9e73_0b2d9b71b1ca);
pub const CLSID_WICJpegEncoder: windows_core::GUID = windows_core::GUID::from_u128(0x1a34f5c1_4a5a_46dc_b644_1f4567e7a676);
pub const CLSID_WICJpegQualcommPhoneEncoder: windows_core::GUID = windows_core::GUID::from_u128(0x68ed5c62_f534_4979_b2b3_686a12b2b34c);
pub const CLSID_WICJpegXLDecoder: windows_core::GUID = windows_core::GUID::from_u128(0xfc6ceece_aef5_4a23_96ec_5984ffb486d9);
pub const CLSID_WICJpegXLEncoder: windows_core::GUID = windows_core::GUID::from_u128(0x0e4ecd3b_1ba6_4636_8198_56c73040964a);
pub const CLSID_WICPlanarFormatConverter: windows_core::GUID = windows_core::GUID::from_u128(0x184132b8_32f8_4784_9131_dd7224b23438);
pub const CLSID_WICPngDecoder: windows_core::GUID = windows_core::GUID::from_u128(0x389ea17b_5078_4cde_b6ef_25c15175c751);
pub const CLSID_WICPngDecoder1: windows_core::GUID = windows_core::GUID::from_u128(0x389ea17b_5078_4cde_b6ef_25c15175c751);
pub const CLSID_WICPngDecoder2: windows_core::GUID = windows_core::GUID::from_u128(0xe018945b_aa86_4008_9bd4_6777a1e40c11);
pub const CLSID_WICPngEncoder: windows_core::GUID = windows_core::GUID::from_u128(0x27949969_876a_41d7_9447_568f6a35a4dc);
pub const CLSID_WICRAWDecoder: windows_core::GUID = windows_core::GUID::from_u128(0x41945702_8302_44a6_9445_ac98e8afa086);
pub const CLSID_WICTiffDecoder: windows_core::GUID = windows_core::GUID::from_u128(0xb54e85d9_fe23_499f_8b88_6acea713752b);
pub const CLSID_WICTiffEncoder: windows_core::GUID = windows_core::GUID::from_u128(0x0131be10_2001_4c5f_a9b0_cc88fab64ce8);
pub const CLSID_WICWebpDecoder: windows_core::GUID = windows_core::GUID::from_u128(0x7693e886_51c9_4070_8419_9f70738ec8fa);
pub const CLSID_WICWmpDecoder: windows_core::GUID = windows_core::GUID::from_u128(0xa26cec36_234c_4950_ae16_e34aace71d0d);
pub const CLSID_WICWmpEncoder: windows_core::GUID = windows_core::GUID::from_u128(0xac4ce3cb_e1c1_44cd_8215_5a1665509ec2);
pub const FACILITY_WINCODEC_ERR: u32 = 2200;
pub const GUID_ContainerFormatAdng: windows_core::GUID = windows_core::GUID::from_u128(0xf3ff6d0d_38c0_41c4_b1fe_1f3824f17b84);
pub const GUID_ContainerFormatBmp: windows_core::GUID = windows_core::GUID::from_u128(0x0af1d87e_fcfe_4188_bdeb_a7906471cbe3);
pub const GUID_ContainerFormatDds: windows_core::GUID = windows_core::GUID::from_u128(0x9967cb95_2e85_4ac8_8ca2_83d7ccd425c9);
pub const GUID_ContainerFormatGif: windows_core::GUID = windows_core::GUID::from_u128(0x1f8a5601_7d4d_4cbd_9c82_1bc8d4eeb9a5);
pub const GUID_ContainerFormatHeif: windows_core::GUID = windows_core::GUID::from_u128(0xe1e62521_6787_405b_a339_500715b5763f);
pub const GUID_ContainerFormatIco: windows_core::GUID = windows_core::GUID::from_u128(0xa3a860c4_338f_4c17_919a_fba4b5628f21);
pub const GUID_ContainerFormatJpeg: windows_core::GUID = windows_core::GUID::from_u128(0x19e4a5aa_5662_4fc5_a0c0_1758028e1057);
pub const GUID_ContainerFormatJpegXL: windows_core::GUID = windows_core::GUID::from_u128(0xfec14e3f_427a_4736_aae6_27ed84f69322);
pub const GUID_ContainerFormatPng: windows_core::GUID = windows_core::GUID::from_u128(0x1b7cfaf4_713f_473c_bbcd_6137425faeaf);
pub const GUID_ContainerFormatRaw: windows_core::GUID = windows_core::GUID::from_u128(0xfe99ce60_f19c_433c_a3ae_00acefa9ca21);
pub const GUID_ContainerFormatTiff: windows_core::GUID = windows_core::GUID::from_u128(0x163bcc30_e2e9_4f0b_961d_a3e9fdb788a3);
pub const GUID_ContainerFormatWebp: windows_core::GUID = windows_core::GUID::from_u128(0xe094b0e2_67f2_45b3_b0ea_115337ca7cf3);
pub const GUID_ContainerFormatWmp: windows_core::GUID = windows_core::GUID::from_u128(0x57a37caa_367a_4540_916b_f183c5093a4b);
pub const GUID_VendorMicrosoft: windows_core::GUID = windows_core::GUID::from_u128(0xf0e749ca_edef_4589_a73a_ee0e626a2a2b);
pub const GUID_VendorMicrosoftBuiltIn: windows_core::GUID = windows_core::GUID::from_u128(0x257a30fd_06b6_462b_aea4_63f70b86e533);
pub const GUID_WICPixelFormat112bpp6ChannelsAlpha: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc937);
pub const GUID_WICPixelFormat112bpp7Channels: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc92a);
pub const GUID_WICPixelFormat128bpp7ChannelsAlpha: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc938);
pub const GUID_WICPixelFormat128bpp8Channels: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc92b);
pub const GUID_WICPixelFormat128bppPRGBAFloat: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc91a);
pub const GUID_WICPixelFormat128bppRGBAFixedPoint: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc91e);
pub const GUID_WICPixelFormat128bppRGBAFloat: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc919);
pub const GUID_WICPixelFormat128bppRGBFixedPoint: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc941);
pub const GUID_WICPixelFormat128bppRGBFloat: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc91b);
pub const GUID_WICPixelFormat144bpp8ChannelsAlpha: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc939);
pub const GUID_WICPixelFormat16bppBGR555: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc909);
pub const GUID_WICPixelFormat16bppBGR565: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc90a);
pub const GUID_WICPixelFormat16bppBGRA5551: windows_core::GUID = windows_core::GUID::from_u128(0x05ec7c2b_f1e6_4961_ad46_e1cc810a87d2);
pub const GUID_WICPixelFormat16bppCbCr: windows_core::GUID = windows_core::GUID::from_u128(0xff95ba6e_11e0_4263_bb45_01721f3460a4);
pub const GUID_WICPixelFormat16bppCbQuantizedDctCoefficients: windows_core::GUID = windows_core::GUID::from_u128(0xd2c4ff61_56a5_49c2_8b5c_4c1925964837);
pub const GUID_WICPixelFormat16bppCrQuantizedDctCoefficients: windows_core::GUID = windows_core::GUID::from_u128(0x2fe354f0_1680_42d8_9231_e73c0565bfc1);
pub const GUID_WICPixelFormat16bppGray: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc90b);
pub const GUID_WICPixelFormat16bppGrayFixedPoint: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc913);
pub const GUID_WICPixelFormat16bppGrayHalf: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc93e);
pub const GUID_WICPixelFormat16bppYQuantizedDctCoefficients: windows_core::GUID = windows_core::GUID::from_u128(0xa355f433_48e8_4a42_84d8_e2aa26ca80a4);
pub const GUID_WICPixelFormat1bppIndexed: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc901);
pub const GUID_WICPixelFormat24bpp3Channels: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc920);
pub const GUID_WICPixelFormat24bppBGR: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc90c);
pub const GUID_WICPixelFormat24bppRGB: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc90d);
pub const GUID_WICPixelFormat24bppRGBGain: windows_core::GUID = windows_core::GUID::from_u128(0xa5022b24_7109_443b_9948_25b6ed8f39fd);
pub const GUID_WICPixelFormat2bppGray: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc906);
pub const GUID_WICPixelFormat2bppIndexed: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc902);
pub const GUID_WICPixelFormat32bpp3ChannelsAlpha: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc92e);
pub const GUID_WICPixelFormat32bpp4Channels: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc921);
pub const GUID_WICPixelFormat32bppBGR: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc90e);
pub const GUID_WICPixelFormat32bppBGR101010: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc914);
pub const GUID_WICPixelFormat32bppBGRA: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc90f);
pub const GUID_WICPixelFormat32bppBGRGain: windows_core::GUID = windows_core::GUID::from_u128(0x837d6738_208a_43e0_8995_79ab74407402);
pub const GUID_WICPixelFormat32bppCMYK: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc91c);
pub const GUID_WICPixelFormat32bppGrayFixedPoint: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc93f);
pub const GUID_WICPixelFormat32bppGrayFloat: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc911);
pub const GUID_WICPixelFormat32bppPBGRA: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc910);
pub const GUID_WICPixelFormat32bppPRGBA: windows_core::GUID = windows_core::GUID::from_u128(0x3cc4a650_a527_4d37_a916_3142c7ebedba);
pub const GUID_WICPixelFormat32bppR10G10B10A2: windows_core::GUID = windows_core::GUID::from_u128(0x604e1bb5_8a3c_4b65_b11c_bc0b8dd75b7f);
pub const GUID_WICPixelFormat32bppR10G10B10A2HDR10: windows_core::GUID = windows_core::GUID::from_u128(0x9c215c5d_1acc_4f0e_a4bc_70fb3ae8fd28);
pub const GUID_WICPixelFormat32bppRGB: windows_core::GUID = windows_core::GUID::from_u128(0xd98c6b95_3efe_47d6_bb25_eb1748ab0cf1);
pub const GUID_WICPixelFormat32bppRGBA: windows_core::GUID = windows_core::GUID::from_u128(0xf5c7ad2d_6a8d_43dd_a7a8_a29935261ae9);
pub const GUID_WICPixelFormat32bppRGBA1010102: windows_core::GUID = windows_core::GUID::from_u128(0x25238d72_fcf9_4522_b514_5578e5ad55e0);
pub const GUID_WICPixelFormat32bppRGBA1010102XR: windows_core::GUID = windows_core::GUID::from_u128(0x00de6b9a_c101_434b_b502_d0165ee1122c);
pub const GUID_WICPixelFormat32bppRGBE: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc93d);
pub const GUID_WICPixelFormat40bpp4ChannelsAlpha: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc92f);
pub const GUID_WICPixelFormat40bpp5Channels: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc922);
pub const GUID_WICPixelFormat40bppCMYKAlpha: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc92c);
pub const GUID_WICPixelFormat48bpp3Channels: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc926);
pub const GUID_WICPixelFormat48bpp5ChannelsAlpha: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc930);
pub const GUID_WICPixelFormat48bpp6Channels: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc923);
pub const GUID_WICPixelFormat48bppBGR: windows_core::GUID = windows_core::GUID::from_u128(0xe605a384_b468_46ce_bb2e_36f180e64313);
pub const GUID_WICPixelFormat48bppBGRFixedPoint: windows_core::GUID = windows_core::GUID::from_u128(0x49ca140e_cab6_493b_9ddf_60187c37532a);
pub const GUID_WICPixelFormat48bppRGB: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc915);
pub const GUID_WICPixelFormat48bppRGBFixedPoint: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc912);
pub const GUID_WICPixelFormat48bppRGBHalf: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc93b);
pub const GUID_WICPixelFormat4bppGray: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc907);
pub const GUID_WICPixelFormat4bppIndexed: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc903);
pub const GUID_WICPixelFormat56bpp6ChannelsAlpha: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc931);
pub const GUID_WICPixelFormat56bpp7Channels: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc924);
pub const GUID_WICPixelFormat64bpp3ChannelsAlpha: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc934);
pub const GUID_WICPixelFormat64bpp4Channels: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc927);
pub const GUID_WICPixelFormat64bpp7ChannelsAlpha: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc932);
pub const GUID_WICPixelFormat64bpp8Channels: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc925);
pub const GUID_WICPixelFormat64bppBGRA: windows_core::GUID = windows_core::GUID::from_u128(0x1562ff7c_d352_46f9_979e_42976b792246);
pub const GUID_WICPixelFormat64bppBGRAFixedPoint: windows_core::GUID = windows_core::GUID::from_u128(0x356de33c_54d2_4a23_bb04_9b7bf9b1d42d);
pub const GUID_WICPixelFormat64bppCMYK: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc91f);
pub const GUID_WICPixelFormat64bppPBGRA: windows_core::GUID = windows_core::GUID::from_u128(0x8c518e8e_a4ec_468b_ae70_c9a35a9c5530);
pub const GUID_WICPixelFormat64bppPRGBA: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc917);
pub const GUID_WICPixelFormat64bppPRGBAHalf: windows_core::GUID = windows_core::GUID::from_u128(0x58ad26c2_c623_4d9d_b320_387e49f8c442);
pub const GUID_WICPixelFormat64bppRGB: windows_core::GUID = windows_core::GUID::from_u128(0xa1182111_186d_4d42_bc6a_9c8303a8dff9);
pub const GUID_WICPixelFormat64bppRGBA: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc916);
pub const GUID_WICPixelFormat64bppRGBAFixedPoint: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc91d);
pub const GUID_WICPixelFormat64bppRGBAHalf: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc93a);
pub const GUID_WICPixelFormat64bppRGBFixedPoint: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc940);
pub const GUID_WICPixelFormat64bppRGBHalf: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc942);
pub const GUID_WICPixelFormat72bpp8ChannelsAlpha: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc933);
pub const GUID_WICPixelFormat80bpp4ChannelsAlpha: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc935);
pub const GUID_WICPixelFormat80bpp5Channels: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc928);
pub const GUID_WICPixelFormat80bppCMYKAlpha: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc92d);
pub const GUID_WICPixelFormat8bppAlpha: windows_core::GUID = windows_core::GUID::from_u128(0xe6cd0116_eeba_4161_aa85_27dd9fb3a895);
pub const GUID_WICPixelFormat8bppCb: windows_core::GUID = windows_core::GUID::from_u128(0x1339f224_6bfe_4c3e_9302_e4f3a6d0ca2a);
pub const GUID_WICPixelFormat8bppCr: windows_core::GUID = windows_core::GUID::from_u128(0xb8145053_2116_49f0_8835_ed844b205c51);
pub const GUID_WICPixelFormat8bppDepth: windows_core::GUID = windows_core::GUID::from_u128(0x4c9c9f45_1d89_4e31_9bc7_69343a0dca69);
pub const GUID_WICPixelFormat8bppGain: windows_core::GUID = windows_core::GUID::from_u128(0xa884022a_af13_4c16_b746_619bf618b878);
pub const GUID_WICPixelFormat8bppGray: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc908);
pub const GUID_WICPixelFormat8bppIndexed: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc904);
pub const GUID_WICPixelFormat8bppY: windows_core::GUID = windows_core::GUID::from_u128(0x91b4db54_2df9_42f0_b449_2909bb3df88e);
pub const GUID_WICPixelFormat96bpp5ChannelsAlpha: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc936);
pub const GUID_WICPixelFormat96bpp6Channels: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc929);
pub const GUID_WICPixelFormat96bppRGBFixedPoint: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc918);
pub const GUID_WICPixelFormat96bppRGBFloat: windows_core::GUID = windows_core::GUID::from_u128(0xe3fed78f_e8db_4acf_84c1_e97f6136b327);
pub const GUID_WICPixelFormatBlackWhite: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc905);
pub const GUID_WICPixelFormatDontCare: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc900);
windows_core::imp::define_interface!(IWICBitmap, IWICBitmap_Vtbl, 0x00000121_a8f2_4877_ba0a_fd2b6645fb94);
impl core::ops::Deref for IWICBitmap {
    type Target = IWICBitmapSource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWICBitmap, windows_core::IUnknown, IWICBitmapSource);
impl IWICBitmap {
    pub unsafe fn Lock(&self, prclock: *const WICRect, flags: u32) -> windows_core::Result<IWICBitmapLock> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Lock)(windows_core::Interface::as_raw(self), prclock, flags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetPalette<P0>(&self, pipalette: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IWICPalette>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetPalette)(windows_core::Interface::as_raw(self), pipalette.param().abi()) }
    }
    pub unsafe fn SetResolution(&self, dpix: f64, dpiy: f64) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetResolution)(windows_core::Interface::as_raw(self), dpix, dpiy) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICBitmap_Vtbl {
    pub base__: IWICBitmapSource_Vtbl,
    pub Lock: unsafe extern "system" fn(*mut core::ffi::c_void, *const WICRect, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPalette: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetResolution: unsafe extern "system" fn(*mut core::ffi::c_void, f64, f64) -> windows_core::HRESULT,
}
pub trait IWICBitmap_Impl: IWICBitmapSource_Impl {
    fn Lock(&self, prclock: *const WICRect, flags: u32) -> windows_core::Result<IWICBitmapLock>;
    fn SetPalette(&self, pipalette: windows_core::Ref<IWICPalette>) -> windows_core::Result<()>;
    fn SetResolution(&self, dpix: f64, dpiy: f64) -> windows_core::Result<()>;
}
impl IWICBitmap_Vtbl {
    pub const fn new<Identity: IWICBitmap_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Lock<Identity: IWICBitmap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prclock: *const WICRect, flags: u32, ppilock: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICBitmap_Impl::Lock(this, core::mem::transmute_copy(&prclock), core::mem::transmute_copy(&flags)) {
                    Ok(ok__) => {
                        ppilock.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPalette<Identity: IWICBitmap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pipalette: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICBitmap_Impl::SetPalette(this, core::mem::transmute_copy(&pipalette)).into()
            }
        }
        unsafe extern "system" fn SetResolution<Identity: IWICBitmap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dpix: f64, dpiy: f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICBitmap_Impl::SetResolution(this, core::mem::transmute_copy(&dpix), core::mem::transmute_copy(&dpiy)).into()
            }
        }
        Self {
            base__: IWICBitmapSource_Vtbl::new::<Identity, OFFSET>(),
            Lock: Lock::<Identity, OFFSET>,
            SetPalette: SetPalette::<Identity, OFFSET>,
            SetResolution: SetResolution::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWICBitmap as windows_core::Interface>::IID || iid == &<IWICBitmapSource as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWICBitmap {}
windows_core::imp::define_interface!(IWICBitmapClipper, IWICBitmapClipper_Vtbl, 0xe4fbcf03_223d_4e81_9333_d635556dd1b5);
impl core::ops::Deref for IWICBitmapClipper {
    type Target = IWICBitmapSource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWICBitmapClipper, windows_core::IUnknown, IWICBitmapSource);
impl IWICBitmapClipper {
    pub unsafe fn Initialize<P0>(&self, pisource: P0, prc: *const WICRect) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IWICBitmapSource>,
    {
        unsafe { (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), pisource.param().abi(), prc) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICBitmapClipper_Vtbl {
    pub base__: IWICBitmapSource_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const WICRect) -> windows_core::HRESULT,
}
pub trait IWICBitmapClipper_Impl: IWICBitmapSource_Impl {
    fn Initialize(&self, pisource: windows_core::Ref<IWICBitmapSource>, prc: *const WICRect) -> windows_core::Result<()>;
}
impl IWICBitmapClipper_Vtbl {
    pub const fn new<Identity: IWICBitmapClipper_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Initialize<Identity: IWICBitmapClipper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisource: *mut core::ffi::c_void, prc: *const WICRect) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICBitmapClipper_Impl::Initialize(this, core::mem::transmute_copy(&pisource), core::mem::transmute_copy(&prc)).into()
            }
        }
        Self { base__: IWICBitmapSource_Vtbl::new::<Identity, OFFSET>(), Initialize: Initialize::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWICBitmapClipper as windows_core::Interface>::IID || iid == &<IWICBitmapSource as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWICBitmapClipper {}
windows_core::imp::define_interface!(IWICBitmapCodecInfo, IWICBitmapCodecInfo_Vtbl, 0xe87a44c4_b76e_4c47_8b09_298eb12a2714);
impl core::ops::Deref for IWICBitmapCodecInfo {
    type Target = IWICComponentInfo;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWICBitmapCodecInfo, windows_core::IUnknown, IWICComponentInfo);
impl IWICBitmapCodecInfo {
    pub unsafe fn GetContainerFormat(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetContainerFormat)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetPixelFormats(&self, cformats: u32, pguidpixelformats: *mut windows_core::GUID, pcactual: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPixelFormats)(windows_core::Interface::as_raw(self), cformats, pguidpixelformats as _, pcactual as _) }
    }
    pub unsafe fn GetColorManagementVersion(&self, cchcolormanagementversion: u32, wzcolormanagementversion: *mut u16, pcchactual: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetColorManagementVersion)(windows_core::Interface::as_raw(self), cchcolormanagementversion, wzcolormanagementversion as _, pcchactual as _) }
    }
    pub unsafe fn GetDeviceManufacturer(&self, cchdevicemanufacturer: u32, wzdevicemanufacturer: *mut u16, pcchactual: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDeviceManufacturer)(windows_core::Interface::as_raw(self), cchdevicemanufacturer, wzdevicemanufacturer as _, pcchactual as _) }
    }
    pub unsafe fn GetDeviceModels(&self, cchdevicemodels: u32, wzdevicemodels: *mut u16, pcchactual: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDeviceModels)(windows_core::Interface::as_raw(self), cchdevicemodels, wzdevicemodels as _, pcchactual as _) }
    }
    pub unsafe fn GetMimeTypes(&self, cchmimetypes: u32, wzmimetypes: *mut u16, pcchactual: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetMimeTypes)(windows_core::Interface::as_raw(self), cchmimetypes, wzmimetypes as _, pcchactual as _) }
    }
    pub unsafe fn GetFileExtensions(&self, cchfileextensions: u32, wzfileextensions: *mut u16, pcchactual: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetFileExtensions)(windows_core::Interface::as_raw(self), cchfileextensions, wzfileextensions as _, pcchactual as _) }
    }
    pub unsafe fn DoesSupportAnimation(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DoesSupportAnimation)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn DoesSupportChromakey(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DoesSupportChromakey)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn DoesSupportLossless(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DoesSupportLossless)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn DoesSupportMultiframe(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DoesSupportMultiframe)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn MatchesMimeType<P0>(&self, wzmimetype: P0) -> windows_core::Result<windows_core::BOOL>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MatchesMimeType)(windows_core::Interface::as_raw(self), wzmimetype.param().abi(), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICBitmapCodecInfo_Vtbl {
    pub base__: IWICComponentInfo_Vtbl,
    pub GetContainerFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GetPixelFormats: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut windows_core::GUID, *mut u32) -> windows_core::HRESULT,
    pub GetColorManagementVersion: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u16, *mut u32) -> windows_core::HRESULT,
    pub GetDeviceManufacturer: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u16, *mut u32) -> windows_core::HRESULT,
    pub GetDeviceModels: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u16, *mut u32) -> windows_core::HRESULT,
    pub GetMimeTypes: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u16, *mut u32) -> windows_core::HRESULT,
    pub GetFileExtensions: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u16, *mut u32) -> windows_core::HRESULT,
    pub DoesSupportAnimation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub DoesSupportChromakey: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub DoesSupportLossless: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub DoesSupportMultiframe: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub MatchesMimeType: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IWICBitmapCodecInfo_Impl: IWICComponentInfo_Impl {
    fn GetContainerFormat(&self) -> windows_core::Result<windows_core::GUID>;
    fn GetPixelFormats(&self, cformats: u32, pguidpixelformats: *mut windows_core::GUID, pcactual: *mut u32) -> windows_core::Result<()>;
    fn GetColorManagementVersion(&self, cchcolormanagementversion: u32, wzcolormanagementversion: *mut u16, pcchactual: *mut u32) -> windows_core::Result<()>;
    fn GetDeviceManufacturer(&self, cchdevicemanufacturer: u32, wzdevicemanufacturer: *mut u16, pcchactual: *mut u32) -> windows_core::Result<()>;
    fn GetDeviceModels(&self, cchdevicemodels: u32, wzdevicemodels: *mut u16, pcchactual: *mut u32) -> windows_core::Result<()>;
    fn GetMimeTypes(&self, cchmimetypes: u32, wzmimetypes: *mut u16, pcchactual: *mut u32) -> windows_core::Result<()>;
    fn GetFileExtensions(&self, cchfileextensions: u32, wzfileextensions: *mut u16, pcchactual: *mut u32) -> windows_core::Result<()>;
    fn DoesSupportAnimation(&self) -> windows_core::Result<windows_core::BOOL>;
    fn DoesSupportChromakey(&self) -> windows_core::Result<windows_core::BOOL>;
    fn DoesSupportLossless(&self) -> windows_core::Result<windows_core::BOOL>;
    fn DoesSupportMultiframe(&self) -> windows_core::Result<windows_core::BOOL>;
    fn MatchesMimeType(&self, wzmimetype: &windows_core::PCWSTR) -> windows_core::Result<windows_core::BOOL>;
}
impl IWICBitmapCodecInfo_Vtbl {
    pub const fn new<Identity: IWICBitmapCodecInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetContainerFormat<Identity: IWICBitmapCodecInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidcontainerformat: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICBitmapCodecInfo_Impl::GetContainerFormat(this) {
                    Ok(ok__) => {
                        pguidcontainerformat.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPixelFormats<Identity: IWICBitmapCodecInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cformats: u32, pguidpixelformats: *mut windows_core::GUID, pcactual: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICBitmapCodecInfo_Impl::GetPixelFormats(this, core::mem::transmute_copy(&cformats), core::mem::transmute_copy(&pguidpixelformats), core::mem::transmute_copy(&pcactual)).into()
            }
        }
        unsafe extern "system" fn GetColorManagementVersion<Identity: IWICBitmapCodecInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cchcolormanagementversion: u32, wzcolormanagementversion: *mut u16, pcchactual: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICBitmapCodecInfo_Impl::GetColorManagementVersion(this, core::mem::transmute_copy(&cchcolormanagementversion), core::mem::transmute_copy(&wzcolormanagementversion), core::mem::transmute_copy(&pcchactual)).into()
            }
        }
        unsafe extern "system" fn GetDeviceManufacturer<Identity: IWICBitmapCodecInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cchdevicemanufacturer: u32, wzdevicemanufacturer: *mut u16, pcchactual: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICBitmapCodecInfo_Impl::GetDeviceManufacturer(this, core::mem::transmute_copy(&cchdevicemanufacturer), core::mem::transmute_copy(&wzdevicemanufacturer), core::mem::transmute_copy(&pcchactual)).into()
            }
        }
        unsafe extern "system" fn GetDeviceModels<Identity: IWICBitmapCodecInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cchdevicemodels: u32, wzdevicemodels: *mut u16, pcchactual: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICBitmapCodecInfo_Impl::GetDeviceModels(this, core::mem::transmute_copy(&cchdevicemodels), core::mem::transmute_copy(&wzdevicemodels), core::mem::transmute_copy(&pcchactual)).into()
            }
        }
        unsafe extern "system" fn GetMimeTypes<Identity: IWICBitmapCodecInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cchmimetypes: u32, wzmimetypes: *mut u16, pcchactual: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICBitmapCodecInfo_Impl::GetMimeTypes(this, core::mem::transmute_copy(&cchmimetypes), core::mem::transmute_copy(&wzmimetypes), core::mem::transmute_copy(&pcchactual)).into()
            }
        }
        unsafe extern "system" fn GetFileExtensions<Identity: IWICBitmapCodecInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cchfileextensions: u32, wzfileextensions: *mut u16, pcchactual: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICBitmapCodecInfo_Impl::GetFileExtensions(this, core::mem::transmute_copy(&cchfileextensions), core::mem::transmute_copy(&wzfileextensions), core::mem::transmute_copy(&pcchactual)).into()
            }
        }
        unsafe extern "system" fn DoesSupportAnimation<Identity: IWICBitmapCodecInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfsupportanimation: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICBitmapCodecInfo_Impl::DoesSupportAnimation(this) {
                    Ok(ok__) => {
                        pfsupportanimation.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DoesSupportChromakey<Identity: IWICBitmapCodecInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfsupportchromakey: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICBitmapCodecInfo_Impl::DoesSupportChromakey(this) {
                    Ok(ok__) => {
                        pfsupportchromakey.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DoesSupportLossless<Identity: IWICBitmapCodecInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfsupportlossless: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICBitmapCodecInfo_Impl::DoesSupportLossless(this) {
                    Ok(ok__) => {
                        pfsupportlossless.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DoesSupportMultiframe<Identity: IWICBitmapCodecInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfsupportmultiframe: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICBitmapCodecInfo_Impl::DoesSupportMultiframe(this) {
                    Ok(ok__) => {
                        pfsupportmultiframe.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MatchesMimeType<Identity: IWICBitmapCodecInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wzmimetype: windows_core::PCWSTR, pfmatches: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICBitmapCodecInfo_Impl::MatchesMimeType(this, core::mem::transmute(&wzmimetype)) {
                    Ok(ok__) => {
                        pfmatches.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IWICComponentInfo_Vtbl::new::<Identity, OFFSET>(),
            GetContainerFormat: GetContainerFormat::<Identity, OFFSET>,
            GetPixelFormats: GetPixelFormats::<Identity, OFFSET>,
            GetColorManagementVersion: GetColorManagementVersion::<Identity, OFFSET>,
            GetDeviceManufacturer: GetDeviceManufacturer::<Identity, OFFSET>,
            GetDeviceModels: GetDeviceModels::<Identity, OFFSET>,
            GetMimeTypes: GetMimeTypes::<Identity, OFFSET>,
            GetFileExtensions: GetFileExtensions::<Identity, OFFSET>,
            DoesSupportAnimation: DoesSupportAnimation::<Identity, OFFSET>,
            DoesSupportChromakey: DoesSupportChromakey::<Identity, OFFSET>,
            DoesSupportLossless: DoesSupportLossless::<Identity, OFFSET>,
            DoesSupportMultiframe: DoesSupportMultiframe::<Identity, OFFSET>,
            MatchesMimeType: MatchesMimeType::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWICBitmapCodecInfo as windows_core::Interface>::IID || iid == &<IWICComponentInfo as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWICBitmapCodecInfo {}
windows_core::imp::define_interface!(IWICBitmapCodecProgressNotification, IWICBitmapCodecProgressNotification_Vtbl, 0x64c1024e_c3cf_4462_8078_88c2b11c46d9);
windows_core::imp::interface_hierarchy!(IWICBitmapCodecProgressNotification, windows_core::IUnknown);
impl IWICBitmapCodecProgressNotification {
    pub unsafe fn RegisterProgressNotification(&self, pfnprogressnotification: PFNProgressNotification, pvdata: Option<*const core::ffi::c_void>, dwprogressflags: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RegisterProgressNotification)(windows_core::Interface::as_raw(self), pfnprogressnotification, pvdata.unwrap_or(core::mem::zeroed()) as _, dwprogressflags) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICBitmapCodecProgressNotification_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub RegisterProgressNotification: unsafe extern "system" fn(*mut core::ffi::c_void, PFNProgressNotification, *const core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait IWICBitmapCodecProgressNotification_Impl: windows_core::IUnknownImpl {
    fn RegisterProgressNotification(&self, pfnprogressnotification: PFNProgressNotification, pvdata: *const core::ffi::c_void, dwprogressflags: u32) -> windows_core::Result<()>;
}
impl IWICBitmapCodecProgressNotification_Vtbl {
    pub const fn new<Identity: IWICBitmapCodecProgressNotification_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RegisterProgressNotification<Identity: IWICBitmapCodecProgressNotification_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfnprogressnotification: PFNProgressNotification, pvdata: *const core::ffi::c_void, dwprogressflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICBitmapCodecProgressNotification_Impl::RegisterProgressNotification(this, core::mem::transmute_copy(&pfnprogressnotification), core::mem::transmute_copy(&pvdata), core::mem::transmute_copy(&dwprogressflags)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), RegisterProgressNotification: RegisterProgressNotification::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWICBitmapCodecProgressNotification as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWICBitmapCodecProgressNotification {}
windows_core::imp::define_interface!(IWICBitmapDecoder, IWICBitmapDecoder_Vtbl, 0x9edde9e7_8dee_47ea_99df_e6faf2ed44bf);
windows_core::imp::interface_hierarchy!(IWICBitmapDecoder, windows_core::IUnknown);
impl IWICBitmapDecoder {
    #[cfg(feature = "objidlbase")]
    pub unsafe fn QueryCapability<P0>(&self, pistream: P0) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<super::objidlbase::IStream>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).QueryCapability)(windows_core::Interface::as_raw(self), pistream.param().abi(), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "objidlbase")]
    pub unsafe fn Initialize<P0>(&self, pistream: P0, cacheoptions: WICDecodeOptions) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::objidlbase::IStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), pistream.param().abi(), cacheoptions) }
    }
    pub unsafe fn GetContainerFormat(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetContainerFormat)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetDecoderInfo(&self) -> windows_core::Result<IWICBitmapDecoderInfo> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDecoderInfo)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CopyPalette<P0>(&self, pipalette: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IWICPalette>,
    {
        unsafe { (windows_core::Interface::vtable(self).CopyPalette)(windows_core::Interface::as_raw(self), pipalette.param().abi()) }
    }
    pub unsafe fn GetMetadataQueryReader(&self) -> windows_core::Result<IWICMetadataQueryReader> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMetadataQueryReader)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetPreview(&self) -> windows_core::Result<IWICBitmapSource> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPreview)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetColorContexts(&self, ccount: u32, ppicolorcontexts: *mut Option<IWICColorContext>, pcactualcount: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetColorContexts)(windows_core::Interface::as_raw(self), ccount, core::mem::transmute(ppicolorcontexts), pcactualcount as _) }
    }
    pub unsafe fn GetThumbnail(&self) -> windows_core::Result<IWICBitmapSource> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetThumbnail)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetFrameCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFrameCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetFrame(&self, index: u32) -> windows_core::Result<IWICBitmapFrameDecode> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFrame)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICBitmapDecoder_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "objidlbase")]
    pub QueryCapability: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidlbase"))]
    QueryCapability: usize,
    #[cfg(feature = "objidlbase")]
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, WICDecodeOptions) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidlbase"))]
    Initialize: usize,
    pub GetContainerFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GetDecoderInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CopyPalette: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetMetadataQueryReader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPreview: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetColorContexts: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetThumbnail: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFrameCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetFrame: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "objidlbase")]
pub trait IWICBitmapDecoder_Impl: windows_core::IUnknownImpl {
    fn QueryCapability(&self, pistream: windows_core::Ref<super::objidlbase::IStream>) -> windows_core::Result<u32>;
    fn Initialize(&self, pistream: windows_core::Ref<super::objidlbase::IStream>, cacheoptions: WICDecodeOptions) -> windows_core::Result<()>;
    fn GetContainerFormat(&self) -> windows_core::Result<windows_core::GUID>;
    fn GetDecoderInfo(&self) -> windows_core::Result<IWICBitmapDecoderInfo>;
    fn CopyPalette(&self, pipalette: windows_core::Ref<IWICPalette>) -> windows_core::Result<()>;
    fn GetMetadataQueryReader(&self) -> windows_core::Result<IWICMetadataQueryReader>;
    fn GetPreview(&self) -> windows_core::Result<IWICBitmapSource>;
    fn GetColorContexts(&self, ccount: u32, ppicolorcontexts: windows_core::OutRef<IWICColorContext>, pcactualcount: *mut u32) -> windows_core::Result<()>;
    fn GetThumbnail(&self) -> windows_core::Result<IWICBitmapSource>;
    fn GetFrameCount(&self) -> windows_core::Result<u32>;
    fn GetFrame(&self, index: u32) -> windows_core::Result<IWICBitmapFrameDecode>;
}
#[cfg(feature = "objidlbase")]
impl IWICBitmapDecoder_Vtbl {
    pub const fn new<Identity: IWICBitmapDecoder_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn QueryCapability<Identity: IWICBitmapDecoder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pistream: *mut core::ffi::c_void, pdwcapability: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICBitmapDecoder_Impl::QueryCapability(this, core::mem::transmute_copy(&pistream)) {
                    Ok(ok__) => {
                        pdwcapability.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Initialize<Identity: IWICBitmapDecoder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pistream: *mut core::ffi::c_void, cacheoptions: WICDecodeOptions) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICBitmapDecoder_Impl::Initialize(this, core::mem::transmute_copy(&pistream), core::mem::transmute_copy(&cacheoptions)).into()
            }
        }
        unsafe extern "system" fn GetContainerFormat<Identity: IWICBitmapDecoder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidcontainerformat: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICBitmapDecoder_Impl::GetContainerFormat(this) {
                    Ok(ok__) => {
                        pguidcontainerformat.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDecoderInfo<Identity: IWICBitmapDecoder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppidecoderinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICBitmapDecoder_Impl::GetDecoderInfo(this) {
                    Ok(ok__) => {
                        ppidecoderinfo.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CopyPalette<Identity: IWICBitmapDecoder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pipalette: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICBitmapDecoder_Impl::CopyPalette(this, core::mem::transmute_copy(&pipalette)).into()
            }
        }
        unsafe extern "system" fn GetMetadataQueryReader<Identity: IWICBitmapDecoder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppimetadataqueryreader: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICBitmapDecoder_Impl::GetMetadataQueryReader(this) {
                    Ok(ok__) => {
                        ppimetadataqueryreader.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPreview<Identity: IWICBitmapDecoder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppibitmapsource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICBitmapDecoder_Impl::GetPreview(this) {
                    Ok(ok__) => {
                        ppibitmapsource.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetColorContexts<Identity: IWICBitmapDecoder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ccount: u32, ppicolorcontexts: *mut *mut core::ffi::c_void, pcactualcount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICBitmapDecoder_Impl::GetColorContexts(this, core::mem::transmute_copy(&ccount), core::mem::transmute_copy(&ppicolorcontexts), core::mem::transmute_copy(&pcactualcount)).into()
            }
        }
        unsafe extern "system" fn GetThumbnail<Identity: IWICBitmapDecoder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppithumbnail: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICBitmapDecoder_Impl::GetThumbnail(this) {
                    Ok(ok__) => {
                        ppithumbnail.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFrameCount<Identity: IWICBitmapDecoder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICBitmapDecoder_Impl::GetFrameCount(this) {
                    Ok(ok__) => {
                        pcount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFrame<Identity: IWICBitmapDecoder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, ppibitmapframe: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICBitmapDecoder_Impl::GetFrame(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        ppibitmapframe.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            QueryCapability: QueryCapability::<Identity, OFFSET>,
            Initialize: Initialize::<Identity, OFFSET>,
            GetContainerFormat: GetContainerFormat::<Identity, OFFSET>,
            GetDecoderInfo: GetDecoderInfo::<Identity, OFFSET>,
            CopyPalette: CopyPalette::<Identity, OFFSET>,
            GetMetadataQueryReader: GetMetadataQueryReader::<Identity, OFFSET>,
            GetPreview: GetPreview::<Identity, OFFSET>,
            GetColorContexts: GetColorContexts::<Identity, OFFSET>,
            GetThumbnail: GetThumbnail::<Identity, OFFSET>,
            GetFrameCount: GetFrameCount::<Identity, OFFSET>,
            GetFrame: GetFrame::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWICBitmapDecoder as windows_core::Interface>::IID
    }
}
#[cfg(feature = "objidlbase")]
impl windows_core::RuntimeName for IWICBitmapDecoder {}
windows_core::imp::define_interface!(IWICBitmapDecoderInfo, IWICBitmapDecoderInfo_Vtbl, 0xd8cd007f_d08f_4191_9bfc_236ea7f0e4b5);
impl core::ops::Deref for IWICBitmapDecoderInfo {
    type Target = IWICBitmapCodecInfo;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWICBitmapDecoderInfo, windows_core::IUnknown, IWICComponentInfo, IWICBitmapCodecInfo);
impl IWICBitmapDecoderInfo {
    pub unsafe fn GetPatterns(&self, cbsizepatterns: u32, ppatterns: Option<*mut WICBitmapPattern>, pcpatterns: Option<*mut u32>, pcbpatternsactual: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPatterns)(windows_core::Interface::as_raw(self), cbsizepatterns, ppatterns.unwrap_or(core::mem::zeroed()) as _, pcpatterns.unwrap_or(core::mem::zeroed()) as _, pcbpatternsactual as _) }
    }
    #[cfg(feature = "objidlbase")]
    pub unsafe fn MatchesPattern<P0>(&self, pistream: P0) -> windows_core::Result<windows_core::BOOL>
    where
        P0: windows_core::Param<super::objidlbase::IStream>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MatchesPattern)(windows_core::Interface::as_raw(self), pistream.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CreateInstance(&self) -> windows_core::Result<IWICBitmapDecoder> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateInstance)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICBitmapDecoderInfo_Vtbl {
    pub base__: IWICBitmapCodecInfo_Vtbl,
    pub GetPatterns: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut WICBitmapPattern, *mut u32, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "objidlbase")]
    pub MatchesPattern: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidlbase"))]
    MatchesPattern: usize,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "objidlbase")]
pub trait IWICBitmapDecoderInfo_Impl: IWICBitmapCodecInfo_Impl {
    fn GetPatterns(&self, cbsizepatterns: u32, ppatterns: *mut WICBitmapPattern, pcpatterns: *mut u32, pcbpatternsactual: *mut u32) -> windows_core::Result<()>;
    fn MatchesPattern(&self, pistream: windows_core::Ref<super::objidlbase::IStream>) -> windows_core::Result<windows_core::BOOL>;
    fn CreateInstance(&self) -> windows_core::Result<IWICBitmapDecoder>;
}
#[cfg(feature = "objidlbase")]
impl IWICBitmapDecoderInfo_Vtbl {
    pub const fn new<Identity: IWICBitmapDecoderInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetPatterns<Identity: IWICBitmapDecoderInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbsizepatterns: u32, ppatterns: *mut WICBitmapPattern, pcpatterns: *mut u32, pcbpatternsactual: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICBitmapDecoderInfo_Impl::GetPatterns(this, core::mem::transmute_copy(&cbsizepatterns), core::mem::transmute_copy(&ppatterns), core::mem::transmute_copy(&pcpatterns), core::mem::transmute_copy(&pcbpatternsactual)).into()
            }
        }
        unsafe extern "system" fn MatchesPattern<Identity: IWICBitmapDecoderInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pistream: *mut core::ffi::c_void, pfmatches: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICBitmapDecoderInfo_Impl::MatchesPattern(this, core::mem::transmute_copy(&pistream)) {
                    Ok(ok__) => {
                        pfmatches.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateInstance<Identity: IWICBitmapDecoderInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppibitmapdecoder: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICBitmapDecoderInfo_Impl::CreateInstance(this) {
                    Ok(ok__) => {
                        ppibitmapdecoder.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IWICBitmapCodecInfo_Vtbl::new::<Identity, OFFSET>(),
            GetPatterns: GetPatterns::<Identity, OFFSET>,
            MatchesPattern: MatchesPattern::<Identity, OFFSET>,
            CreateInstance: CreateInstance::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWICBitmapDecoderInfo as windows_core::Interface>::IID || iid == &<IWICComponentInfo as windows_core::Interface>::IID || iid == &<IWICBitmapCodecInfo as windows_core::Interface>::IID
    }
}
#[cfg(feature = "objidlbase")]
impl windows_core::RuntimeName for IWICBitmapDecoderInfo {}
windows_core::imp::define_interface!(IWICBitmapEncoder, IWICBitmapEncoder_Vtbl, 0x00000103_a8f2_4877_ba0a_fd2b6645fb94);
windows_core::imp::interface_hierarchy!(IWICBitmapEncoder, windows_core::IUnknown);
impl IWICBitmapEncoder {
    #[cfg(feature = "objidlbase")]
    pub unsafe fn Initialize<P0>(&self, pistream: P0, cacheoption: WICBitmapEncoderCacheOption) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::objidlbase::IStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), pistream.param().abi(), cacheoption) }
    }
    pub unsafe fn GetContainerFormat(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetContainerFormat)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetEncoderInfo(&self) -> windows_core::Result<IWICBitmapEncoderInfo> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetEncoderInfo)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetColorContexts(&self, ccount: u32, ppicolorcontext: *const Option<IWICColorContext>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetColorContexts)(windows_core::Interface::as_raw(self), ccount, core::mem::transmute(ppicolorcontext)) }
    }
    pub unsafe fn SetPalette<P0>(&self, pipalette: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IWICPalette>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetPalette)(windows_core::Interface::as_raw(self), pipalette.param().abi()) }
    }
    pub unsafe fn SetThumbnail<P0>(&self, pithumbnail: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IWICBitmapSource>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetThumbnail)(windows_core::Interface::as_raw(self), pithumbnail.param().abi()) }
    }
    pub unsafe fn SetPreview<P0>(&self, pipreview: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IWICBitmapSource>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetPreview)(windows_core::Interface::as_raw(self), pipreview.param().abi()) }
    }
    #[cfg(feature = "ocidl")]
    pub unsafe fn CreateNewFrame(&self, ppiframeencode: *mut Option<IWICBitmapFrameEncode>, ppiencoderoptions: *mut Option<super::ocidl::IPropertyBag2>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CreateNewFrame)(windows_core::Interface::as_raw(self), core::mem::transmute(ppiframeencode), core::mem::transmute(ppiencoderoptions)) }
    }
    pub unsafe fn Commit(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Commit)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetMetadataQueryWriter(&self) -> windows_core::Result<IWICMetadataQueryWriter> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMetadataQueryWriter)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICBitmapEncoder_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "objidlbase")]
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, WICBitmapEncoderCacheOption) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidlbase"))]
    Initialize: usize,
    pub GetContainerFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GetEncoderInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetColorContexts: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPalette: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetThumbnail: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPreview: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "ocidl")]
    pub CreateNewFrame: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "ocidl"))]
    CreateNewFrame: usize,
    pub Commit: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetMetadataQueryWriter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "objidlbase", feature = "ocidl"))]
pub trait IWICBitmapEncoder_Impl: windows_core::IUnknownImpl {
    fn Initialize(&self, pistream: windows_core::Ref<super::objidlbase::IStream>, cacheoption: WICBitmapEncoderCacheOption) -> windows_core::Result<()>;
    fn GetContainerFormat(&self) -> windows_core::Result<windows_core::GUID>;
    fn GetEncoderInfo(&self) -> windows_core::Result<IWICBitmapEncoderInfo>;
    fn SetColorContexts(&self, ccount: u32, ppicolorcontext: *const Option<IWICColorContext>) -> windows_core::Result<()>;
    fn SetPalette(&self, pipalette: windows_core::Ref<IWICPalette>) -> windows_core::Result<()>;
    fn SetThumbnail(&self, pithumbnail: windows_core::Ref<IWICBitmapSource>) -> windows_core::Result<()>;
    fn SetPreview(&self, pipreview: windows_core::Ref<IWICBitmapSource>) -> windows_core::Result<()>;
    fn CreateNewFrame(&self, ppiframeencode: windows_core::OutRef<IWICBitmapFrameEncode>, ppiencoderoptions: windows_core::OutRef<super::ocidl::IPropertyBag2>) -> windows_core::Result<()>;
    fn Commit(&self) -> windows_core::Result<()>;
    fn GetMetadataQueryWriter(&self) -> windows_core::Result<IWICMetadataQueryWriter>;
}
#[cfg(all(feature = "objidlbase", feature = "ocidl"))]
impl IWICBitmapEncoder_Vtbl {
    pub const fn new<Identity: IWICBitmapEncoder_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Initialize<Identity: IWICBitmapEncoder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pistream: *mut core::ffi::c_void, cacheoption: WICBitmapEncoderCacheOption) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICBitmapEncoder_Impl::Initialize(this, core::mem::transmute_copy(&pistream), core::mem::transmute_copy(&cacheoption)).into()
            }
        }
        unsafe extern "system" fn GetContainerFormat<Identity: IWICBitmapEncoder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidcontainerformat: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICBitmapEncoder_Impl::GetContainerFormat(this) {
                    Ok(ok__) => {
                        pguidcontainerformat.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetEncoderInfo<Identity: IWICBitmapEncoder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppiencoderinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICBitmapEncoder_Impl::GetEncoderInfo(this) {
                    Ok(ok__) => {
                        ppiencoderinfo.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetColorContexts<Identity: IWICBitmapEncoder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ccount: u32, ppicolorcontext: *const *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICBitmapEncoder_Impl::SetColorContexts(this, core::mem::transmute_copy(&ccount), core::mem::transmute_copy(&ppicolorcontext)).into()
            }
        }
        unsafe extern "system" fn SetPalette<Identity: IWICBitmapEncoder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pipalette: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICBitmapEncoder_Impl::SetPalette(this, core::mem::transmute_copy(&pipalette)).into()
            }
        }
        unsafe extern "system" fn SetThumbnail<Identity: IWICBitmapEncoder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pithumbnail: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICBitmapEncoder_Impl::SetThumbnail(this, core::mem::transmute_copy(&pithumbnail)).into()
            }
        }
        unsafe extern "system" fn SetPreview<Identity: IWICBitmapEncoder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pipreview: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICBitmapEncoder_Impl::SetPreview(this, core::mem::transmute_copy(&pipreview)).into()
            }
        }
        unsafe extern "system" fn CreateNewFrame<Identity: IWICBitmapEncoder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppiframeencode: *mut *mut core::ffi::c_void, ppiencoderoptions: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICBitmapEncoder_Impl::CreateNewFrame(this, core::mem::transmute_copy(&ppiframeencode), core::mem::transmute_copy(&ppiencoderoptions)).into()
            }
        }
        unsafe extern "system" fn Commit<Identity: IWICBitmapEncoder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICBitmapEncoder_Impl::Commit(this).into()
            }
        }
        unsafe extern "system" fn GetMetadataQueryWriter<Identity: IWICBitmapEncoder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppimetadataquerywriter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICBitmapEncoder_Impl::GetMetadataQueryWriter(this) {
                    Ok(ok__) => {
                        ppimetadataquerywriter.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            GetContainerFormat: GetContainerFormat::<Identity, OFFSET>,
            GetEncoderInfo: GetEncoderInfo::<Identity, OFFSET>,
            SetColorContexts: SetColorContexts::<Identity, OFFSET>,
            SetPalette: SetPalette::<Identity, OFFSET>,
            SetThumbnail: SetThumbnail::<Identity, OFFSET>,
            SetPreview: SetPreview::<Identity, OFFSET>,
            CreateNewFrame: CreateNewFrame::<Identity, OFFSET>,
            Commit: Commit::<Identity, OFFSET>,
            GetMetadataQueryWriter: GetMetadataQueryWriter::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWICBitmapEncoder as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "objidlbase", feature = "ocidl"))]
impl windows_core::RuntimeName for IWICBitmapEncoder {}
windows_core::imp::define_interface!(IWICBitmapEncoderInfo, IWICBitmapEncoderInfo_Vtbl, 0x94c9b4ee_a09f_4f92_8a1e_4a9bce7e76fb);
impl core::ops::Deref for IWICBitmapEncoderInfo {
    type Target = IWICBitmapCodecInfo;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWICBitmapEncoderInfo, windows_core::IUnknown, IWICComponentInfo, IWICBitmapCodecInfo);
impl IWICBitmapEncoderInfo {
    pub unsafe fn CreateInstance(&self) -> windows_core::Result<IWICBitmapEncoder> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateInstance)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICBitmapEncoderInfo_Vtbl {
    pub base__: IWICBitmapCodecInfo_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWICBitmapEncoderInfo_Impl: IWICBitmapCodecInfo_Impl {
    fn CreateInstance(&self) -> windows_core::Result<IWICBitmapEncoder>;
}
impl IWICBitmapEncoderInfo_Vtbl {
    pub const fn new<Identity: IWICBitmapEncoderInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateInstance<Identity: IWICBitmapEncoderInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppibitmapencoder: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICBitmapEncoderInfo_Impl::CreateInstance(this) {
                    Ok(ok__) => {
                        ppibitmapencoder.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: IWICBitmapCodecInfo_Vtbl::new::<Identity, OFFSET>(), CreateInstance: CreateInstance::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWICBitmapEncoderInfo as windows_core::Interface>::IID || iid == &<IWICComponentInfo as windows_core::Interface>::IID || iid == &<IWICBitmapCodecInfo as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWICBitmapEncoderInfo {}
windows_core::imp::define_interface!(IWICBitmapFlipRotator, IWICBitmapFlipRotator_Vtbl, 0x5009834f_2d6a_41ce_9e1b_17c5aff7a782);
impl core::ops::Deref for IWICBitmapFlipRotator {
    type Target = IWICBitmapSource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWICBitmapFlipRotator, windows_core::IUnknown, IWICBitmapSource);
impl IWICBitmapFlipRotator {
    pub unsafe fn Initialize<P0>(&self, pisource: P0, options: WICBitmapTransformOptions) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IWICBitmapSource>,
    {
        unsafe { (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), pisource.param().abi(), options) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICBitmapFlipRotator_Vtbl {
    pub base__: IWICBitmapSource_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, WICBitmapTransformOptions) -> windows_core::HRESULT,
}
pub trait IWICBitmapFlipRotator_Impl: IWICBitmapSource_Impl {
    fn Initialize(&self, pisource: windows_core::Ref<IWICBitmapSource>, options: WICBitmapTransformOptions) -> windows_core::Result<()>;
}
impl IWICBitmapFlipRotator_Vtbl {
    pub const fn new<Identity: IWICBitmapFlipRotator_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Initialize<Identity: IWICBitmapFlipRotator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisource: *mut core::ffi::c_void, options: WICBitmapTransformOptions) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICBitmapFlipRotator_Impl::Initialize(this, core::mem::transmute_copy(&pisource), core::mem::transmute_copy(&options)).into()
            }
        }
        Self { base__: IWICBitmapSource_Vtbl::new::<Identity, OFFSET>(), Initialize: Initialize::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWICBitmapFlipRotator as windows_core::Interface>::IID || iid == &<IWICBitmapSource as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWICBitmapFlipRotator {}
windows_core::imp::define_interface!(IWICBitmapFrameChainReader, IWICBitmapFrameChainReader_Vtbl, 0x0c599495_a120_4222_9130_a8c29410bd0b);
windows_core::imp::interface_hierarchy!(IWICBitmapFrameChainReader, windows_core::IUnknown);
impl IWICBitmapFrameChainReader {
    pub unsafe fn GetChainedFrameCount(&self, chaintype: WICBitmapChainType) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetChainedFrameCount)(windows_core::Interface::as_raw(self), chaintype, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetChainedFrame(&self, chaintype: WICBitmapChainType, index: u32) -> windows_core::Result<IWICBitmapFrameDecode> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetChainedFrame)(windows_core::Interface::as_raw(self), chaintype, index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICBitmapFrameChainReader_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetChainedFrameCount: unsafe extern "system" fn(*mut core::ffi::c_void, WICBitmapChainType, *mut u32) -> windows_core::HRESULT,
    pub GetChainedFrame: unsafe extern "system" fn(*mut core::ffi::c_void, WICBitmapChainType, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWICBitmapFrameChainReader_Impl: windows_core::IUnknownImpl {
    fn GetChainedFrameCount(&self, chaintype: WICBitmapChainType) -> windows_core::Result<u32>;
    fn GetChainedFrame(&self, chaintype: WICBitmapChainType, index: u32) -> windows_core::Result<IWICBitmapFrameDecode>;
}
impl IWICBitmapFrameChainReader_Vtbl {
    pub const fn new<Identity: IWICBitmapFrameChainReader_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetChainedFrameCount<Identity: IWICBitmapFrameChainReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, chaintype: WICBitmapChainType, pcount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICBitmapFrameChainReader_Impl::GetChainedFrameCount(this, core::mem::transmute_copy(&chaintype)) {
                    Ok(ok__) => {
                        pcount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetChainedFrame<Identity: IWICBitmapFrameChainReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, chaintype: WICBitmapChainType, index: u32, ppibitmapframe: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICBitmapFrameChainReader_Impl::GetChainedFrame(this, core::mem::transmute_copy(&chaintype), core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        ppibitmapframe.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetChainedFrameCount: GetChainedFrameCount::<Identity, OFFSET>,
            GetChainedFrame: GetChainedFrame::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWICBitmapFrameChainReader as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWICBitmapFrameChainReader {}
windows_core::imp::define_interface!(IWICBitmapFrameChainWriter, IWICBitmapFrameChainWriter_Vtbl, 0x40d9ea28_4768_47b3_8c12_558a48e98e38);
windows_core::imp::interface_hierarchy!(IWICBitmapFrameChainWriter, windows_core::IUnknown);
impl IWICBitmapFrameChainWriter {
    #[cfg(feature = "ocidl")]
    pub unsafe fn AppendFrameToChain(&self, chaintype: WICBitmapChainType, ppiframeencode: *mut Option<IWICBitmapFrameEncode>, ppiencoderoptions: *mut Option<super::ocidl::IPropertyBag2>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AppendFrameToChain)(windows_core::Interface::as_raw(self), chaintype, core::mem::transmute(ppiframeencode), core::mem::transmute(ppiencoderoptions)) }
    }
    pub unsafe fn DoesSupportChainType(&self, chaintype: WICBitmapChainType) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DoesSupportChainType)(windows_core::Interface::as_raw(self), chaintype, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICBitmapFrameChainWriter_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "ocidl")]
    pub AppendFrameToChain: unsafe extern "system" fn(*mut core::ffi::c_void, WICBitmapChainType, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "ocidl"))]
    AppendFrameToChain: usize,
    pub DoesSupportChainType: unsafe extern "system" fn(*mut core::ffi::c_void, WICBitmapChainType, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
#[cfg(feature = "ocidl")]
pub trait IWICBitmapFrameChainWriter_Impl: windows_core::IUnknownImpl {
    fn AppendFrameToChain(&self, chaintype: WICBitmapChainType, ppiframeencode: windows_core::OutRef<IWICBitmapFrameEncode>, ppiencoderoptions: windows_core::OutRef<super::ocidl::IPropertyBag2>) -> windows_core::Result<()>;
    fn DoesSupportChainType(&self, chaintype: WICBitmapChainType) -> windows_core::Result<windows_core::BOOL>;
}
#[cfg(feature = "ocidl")]
impl IWICBitmapFrameChainWriter_Vtbl {
    pub const fn new<Identity: IWICBitmapFrameChainWriter_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AppendFrameToChain<Identity: IWICBitmapFrameChainWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, chaintype: WICBitmapChainType, ppiframeencode: *mut *mut core::ffi::c_void, ppiencoderoptions: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICBitmapFrameChainWriter_Impl::AppendFrameToChain(this, core::mem::transmute_copy(&chaintype), core::mem::transmute_copy(&ppiframeencode), core::mem::transmute_copy(&ppiencoderoptions)).into()
            }
        }
        unsafe extern "system" fn DoesSupportChainType<Identity: IWICBitmapFrameChainWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, chaintype: WICBitmapChainType, pfissupported: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICBitmapFrameChainWriter_Impl::DoesSupportChainType(this, core::mem::transmute_copy(&chaintype)) {
                    Ok(ok__) => {
                        pfissupported.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AppendFrameToChain: AppendFrameToChain::<Identity, OFFSET>,
            DoesSupportChainType: DoesSupportChainType::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWICBitmapFrameChainWriter as windows_core::Interface>::IID
    }
}
#[cfg(feature = "ocidl")]
impl windows_core::RuntimeName for IWICBitmapFrameChainWriter {}
windows_core::imp::define_interface!(IWICBitmapFrameDecode, IWICBitmapFrameDecode_Vtbl, 0x3b16811b_6a43_4ec9_a813_3d930c13b940);
impl core::ops::Deref for IWICBitmapFrameDecode {
    type Target = IWICBitmapSource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWICBitmapFrameDecode, windows_core::IUnknown, IWICBitmapSource);
impl IWICBitmapFrameDecode {
    pub unsafe fn GetMetadataQueryReader(&self) -> windows_core::Result<IWICMetadataQueryReader> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMetadataQueryReader)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetColorContexts(&self, ccount: u32, ppicolorcontexts: *mut Option<IWICColorContext>, pcactualcount: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetColorContexts)(windows_core::Interface::as_raw(self), ccount, core::mem::transmute(ppicolorcontexts), pcactualcount as _) }
    }
    pub unsafe fn GetThumbnail(&self) -> windows_core::Result<IWICBitmapSource> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetThumbnail)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICBitmapFrameDecode_Vtbl {
    pub base__: IWICBitmapSource_Vtbl,
    pub GetMetadataQueryReader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetColorContexts: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetThumbnail: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWICBitmapFrameDecode_Impl: IWICBitmapSource_Impl {
    fn GetMetadataQueryReader(&self) -> windows_core::Result<IWICMetadataQueryReader>;
    fn GetColorContexts(&self, ccount: u32, ppicolorcontexts: windows_core::OutRef<IWICColorContext>, pcactualcount: *mut u32) -> windows_core::Result<()>;
    fn GetThumbnail(&self) -> windows_core::Result<IWICBitmapSource>;
}
impl IWICBitmapFrameDecode_Vtbl {
    pub const fn new<Identity: IWICBitmapFrameDecode_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetMetadataQueryReader<Identity: IWICBitmapFrameDecode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppimetadataqueryreader: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICBitmapFrameDecode_Impl::GetMetadataQueryReader(this) {
                    Ok(ok__) => {
                        ppimetadataqueryreader.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetColorContexts<Identity: IWICBitmapFrameDecode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ccount: u32, ppicolorcontexts: *mut *mut core::ffi::c_void, pcactualcount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICBitmapFrameDecode_Impl::GetColorContexts(this, core::mem::transmute_copy(&ccount), core::mem::transmute_copy(&ppicolorcontexts), core::mem::transmute_copy(&pcactualcount)).into()
            }
        }
        unsafe extern "system" fn GetThumbnail<Identity: IWICBitmapFrameDecode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppithumbnail: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICBitmapFrameDecode_Impl::GetThumbnail(this) {
                    Ok(ok__) => {
                        ppithumbnail.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IWICBitmapSource_Vtbl::new::<Identity, OFFSET>(),
            GetMetadataQueryReader: GetMetadataQueryReader::<Identity, OFFSET>,
            GetColorContexts: GetColorContexts::<Identity, OFFSET>,
            GetThumbnail: GetThumbnail::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWICBitmapFrameDecode as windows_core::Interface>::IID || iid == &<IWICBitmapSource as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWICBitmapFrameDecode {}
windows_core::imp::define_interface!(IWICBitmapFrameEncode, IWICBitmapFrameEncode_Vtbl, 0x00000105_a8f2_4877_ba0a_fd2b6645fb94);
windows_core::imp::interface_hierarchy!(IWICBitmapFrameEncode, windows_core::IUnknown);
impl IWICBitmapFrameEncode {
    #[cfg(feature = "ocidl")]
    pub unsafe fn Initialize<P0>(&self, piencoderoptions: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::ocidl::IPropertyBag2>,
    {
        unsafe { (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), piencoderoptions.param().abi()) }
    }
    pub unsafe fn SetSize(&self, uiwidth: u32, uiheight: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSize)(windows_core::Interface::as_raw(self), uiwidth, uiheight) }
    }
    pub unsafe fn SetResolution(&self, dpix: f64, dpiy: f64) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetResolution)(windows_core::Interface::as_raw(self), dpix, dpiy) }
    }
    pub unsafe fn SetPixelFormat(&self, ppixelformat: *mut WICPixelFormatGUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPixelFormat)(windows_core::Interface::as_raw(self), ppixelformat as _) }
    }
    pub unsafe fn SetColorContexts(&self, ccount: u32, ppicolorcontext: *const Option<IWICColorContext>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetColorContexts)(windows_core::Interface::as_raw(self), ccount, core::mem::transmute(ppicolorcontext)) }
    }
    pub unsafe fn SetPalette<P0>(&self, pipalette: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IWICPalette>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetPalette)(windows_core::Interface::as_raw(self), pipalette.param().abi()) }
    }
    pub unsafe fn SetThumbnail<P0>(&self, pithumbnail: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IWICBitmapSource>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetThumbnail)(windows_core::Interface::as_raw(self), pithumbnail.param().abi()) }
    }
    pub unsafe fn WritePixels(&self, linecount: u32, cbstride: u32, cbbuffersize: u32, pbpixels: *const u8) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).WritePixels)(windows_core::Interface::as_raw(self), linecount, cbstride, cbbuffersize, pbpixels) }
    }
    pub unsafe fn WriteSource<P0>(&self, pibitmapsource: P0, prc: *const WICRect) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IWICBitmapSource>,
    {
        unsafe { (windows_core::Interface::vtable(self).WriteSource)(windows_core::Interface::as_raw(self), pibitmapsource.param().abi(), prc) }
    }
    pub unsafe fn Commit(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Commit)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetMetadataQueryWriter(&self) -> windows_core::Result<IWICMetadataQueryWriter> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMetadataQueryWriter)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICBitmapFrameEncode_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "ocidl")]
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "ocidl"))]
    Initialize: usize,
    pub SetSize: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub SetResolution: unsafe extern "system" fn(*mut core::ffi::c_void, f64, f64) -> windows_core::HRESULT,
    pub SetPixelFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WICPixelFormatGUID) -> windows_core::HRESULT,
    pub SetColorContexts: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPalette: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetThumbnail: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub WritePixels: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32, *const u8) -> windows_core::HRESULT,
    pub WriteSource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const WICRect) -> windows_core::HRESULT,
    pub Commit: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetMetadataQueryWriter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "ocidl")]
pub trait IWICBitmapFrameEncode_Impl: windows_core::IUnknownImpl {
    fn Initialize(&self, piencoderoptions: windows_core::Ref<super::ocidl::IPropertyBag2>) -> windows_core::Result<()>;
    fn SetSize(&self, uiwidth: u32, uiheight: u32) -> windows_core::Result<()>;
    fn SetResolution(&self, dpix: f64, dpiy: f64) -> windows_core::Result<()>;
    fn SetPixelFormat(&self, ppixelformat: *mut WICPixelFormatGUID) -> windows_core::Result<()>;
    fn SetColorContexts(&self, ccount: u32, ppicolorcontext: *const Option<IWICColorContext>) -> windows_core::Result<()>;
    fn SetPalette(&self, pipalette: windows_core::Ref<IWICPalette>) -> windows_core::Result<()>;
    fn SetThumbnail(&self, pithumbnail: windows_core::Ref<IWICBitmapSource>) -> windows_core::Result<()>;
    fn WritePixels(&self, linecount: u32, cbstride: u32, cbbuffersize: u32, pbpixels: *const u8) -> windows_core::Result<()>;
    fn WriteSource(&self, pibitmapsource: windows_core::Ref<IWICBitmapSource>, prc: *const WICRect) -> windows_core::Result<()>;
    fn Commit(&self) -> windows_core::Result<()>;
    fn GetMetadataQueryWriter(&self) -> windows_core::Result<IWICMetadataQueryWriter>;
}
#[cfg(feature = "ocidl")]
impl IWICBitmapFrameEncode_Vtbl {
    pub const fn new<Identity: IWICBitmapFrameEncode_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Initialize<Identity: IWICBitmapFrameEncode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, piencoderoptions: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICBitmapFrameEncode_Impl::Initialize(this, core::mem::transmute_copy(&piencoderoptions)).into()
            }
        }
        unsafe extern "system" fn SetSize<Identity: IWICBitmapFrameEncode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uiwidth: u32, uiheight: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICBitmapFrameEncode_Impl::SetSize(this, core::mem::transmute_copy(&uiwidth), core::mem::transmute_copy(&uiheight)).into()
            }
        }
        unsafe extern "system" fn SetResolution<Identity: IWICBitmapFrameEncode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dpix: f64, dpiy: f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICBitmapFrameEncode_Impl::SetResolution(this, core::mem::transmute_copy(&dpix), core::mem::transmute_copy(&dpiy)).into()
            }
        }
        unsafe extern "system" fn SetPixelFormat<Identity: IWICBitmapFrameEncode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppixelformat: *mut WICPixelFormatGUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICBitmapFrameEncode_Impl::SetPixelFormat(this, core::mem::transmute_copy(&ppixelformat)).into()
            }
        }
        unsafe extern "system" fn SetColorContexts<Identity: IWICBitmapFrameEncode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ccount: u32, ppicolorcontext: *const *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICBitmapFrameEncode_Impl::SetColorContexts(this, core::mem::transmute_copy(&ccount), core::mem::transmute_copy(&ppicolorcontext)).into()
            }
        }
        unsafe extern "system" fn SetPalette<Identity: IWICBitmapFrameEncode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pipalette: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICBitmapFrameEncode_Impl::SetPalette(this, core::mem::transmute_copy(&pipalette)).into()
            }
        }
        unsafe extern "system" fn SetThumbnail<Identity: IWICBitmapFrameEncode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pithumbnail: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICBitmapFrameEncode_Impl::SetThumbnail(this, core::mem::transmute_copy(&pithumbnail)).into()
            }
        }
        unsafe extern "system" fn WritePixels<Identity: IWICBitmapFrameEncode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, linecount: u32, cbstride: u32, cbbuffersize: u32, pbpixels: *const u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICBitmapFrameEncode_Impl::WritePixels(this, core::mem::transmute_copy(&linecount), core::mem::transmute_copy(&cbstride), core::mem::transmute_copy(&cbbuffersize), core::mem::transmute_copy(&pbpixels)).into()
            }
        }
        unsafe extern "system" fn WriteSource<Identity: IWICBitmapFrameEncode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pibitmapsource: *mut core::ffi::c_void, prc: *const WICRect) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICBitmapFrameEncode_Impl::WriteSource(this, core::mem::transmute_copy(&pibitmapsource), core::mem::transmute_copy(&prc)).into()
            }
        }
        unsafe extern "system" fn Commit<Identity: IWICBitmapFrameEncode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICBitmapFrameEncode_Impl::Commit(this).into()
            }
        }
        unsafe extern "system" fn GetMetadataQueryWriter<Identity: IWICBitmapFrameEncode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppimetadataquerywriter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICBitmapFrameEncode_Impl::GetMetadataQueryWriter(this) {
                    Ok(ok__) => {
                        ppimetadataquerywriter.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            SetSize: SetSize::<Identity, OFFSET>,
            SetResolution: SetResolution::<Identity, OFFSET>,
            SetPixelFormat: SetPixelFormat::<Identity, OFFSET>,
            SetColorContexts: SetColorContexts::<Identity, OFFSET>,
            SetPalette: SetPalette::<Identity, OFFSET>,
            SetThumbnail: SetThumbnail::<Identity, OFFSET>,
            WritePixels: WritePixels::<Identity, OFFSET>,
            WriteSource: WriteSource::<Identity, OFFSET>,
            Commit: Commit::<Identity, OFFSET>,
            GetMetadataQueryWriter: GetMetadataQueryWriter::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWICBitmapFrameEncode as windows_core::Interface>::IID
    }
}
#[cfg(feature = "ocidl")]
impl windows_core::RuntimeName for IWICBitmapFrameEncode {}
windows_core::imp::define_interface!(IWICBitmapLock, IWICBitmapLock_Vtbl, 0x00000123_a8f2_4877_ba0a_fd2b6645fb94);
windows_core::imp::interface_hierarchy!(IWICBitmapLock, windows_core::IUnknown);
impl IWICBitmapLock {
    pub unsafe fn GetSize(&self, puiwidth: *mut u32, puiheight: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetSize)(windows_core::Interface::as_raw(self), puiwidth as _, puiheight as _) }
    }
    pub unsafe fn GetStride(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStride)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetDataPointer(&self, pcbbuffersize: *mut u32, ppbdata: *mut WICInProcPointer) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDataPointer)(windows_core::Interface::as_raw(self), pcbbuffersize as _, ppbdata as _) }
    }
    pub unsafe fn GetPixelFormat(&self) -> windows_core::Result<WICPixelFormatGUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPixelFormat)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICBitmapLock_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub GetStride: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetDataPointer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut WICInProcPointer) -> windows_core::HRESULT,
    pub GetPixelFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WICPixelFormatGUID) -> windows_core::HRESULT,
}
pub trait IWICBitmapLock_Impl: windows_core::IUnknownImpl {
    fn GetSize(&self, puiwidth: *mut u32, puiheight: *mut u32) -> windows_core::Result<()>;
    fn GetStride(&self) -> windows_core::Result<u32>;
    fn GetDataPointer(&self, pcbbuffersize: *mut u32, ppbdata: *mut WICInProcPointer) -> windows_core::Result<()>;
    fn GetPixelFormat(&self) -> windows_core::Result<WICPixelFormatGUID>;
}
impl IWICBitmapLock_Vtbl {
    pub const fn new<Identity: IWICBitmapLock_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetSize<Identity: IWICBitmapLock_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, puiwidth: *mut u32, puiheight: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICBitmapLock_Impl::GetSize(this, core::mem::transmute_copy(&puiwidth), core::mem::transmute_copy(&puiheight)).into()
            }
        }
        unsafe extern "system" fn GetStride<Identity: IWICBitmapLock_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcbstride: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICBitmapLock_Impl::GetStride(this) {
                    Ok(ok__) => {
                        pcbstride.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDataPointer<Identity: IWICBitmapLock_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcbbuffersize: *mut u32, ppbdata: *mut WICInProcPointer) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICBitmapLock_Impl::GetDataPointer(this, core::mem::transmute_copy(&pcbbuffersize), core::mem::transmute_copy(&ppbdata)).into()
            }
        }
        unsafe extern "system" fn GetPixelFormat<Identity: IWICBitmapLock_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppixelformat: *mut WICPixelFormatGUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICBitmapLock_Impl::GetPixelFormat(this) {
                    Ok(ok__) => {
                        ppixelformat.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSize: GetSize::<Identity, OFFSET>,
            GetStride: GetStride::<Identity, OFFSET>,
            GetDataPointer: GetDataPointer::<Identity, OFFSET>,
            GetPixelFormat: GetPixelFormat::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWICBitmapLock as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWICBitmapLock {}
windows_core::imp::define_interface!(IWICBitmapScaler, IWICBitmapScaler_Vtbl, 0x00000302_a8f2_4877_ba0a_fd2b6645fb94);
impl core::ops::Deref for IWICBitmapScaler {
    type Target = IWICBitmapSource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWICBitmapScaler, windows_core::IUnknown, IWICBitmapSource);
impl IWICBitmapScaler {
    pub unsafe fn Initialize<P0>(&self, pisource: P0, uiwidth: u32, uiheight: u32, mode: WICBitmapInterpolationMode) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IWICBitmapSource>,
    {
        unsafe { (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), pisource.param().abi(), uiwidth, uiheight, mode) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICBitmapScaler_Vtbl {
    pub base__: IWICBitmapSource_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32, WICBitmapInterpolationMode) -> windows_core::HRESULT,
}
pub trait IWICBitmapScaler_Impl: IWICBitmapSource_Impl {
    fn Initialize(&self, pisource: windows_core::Ref<IWICBitmapSource>, uiwidth: u32, uiheight: u32, mode: WICBitmapInterpolationMode) -> windows_core::Result<()>;
}
impl IWICBitmapScaler_Vtbl {
    pub const fn new<Identity: IWICBitmapScaler_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Initialize<Identity: IWICBitmapScaler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisource: *mut core::ffi::c_void, uiwidth: u32, uiheight: u32, mode: WICBitmapInterpolationMode) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICBitmapScaler_Impl::Initialize(this, core::mem::transmute_copy(&pisource), core::mem::transmute_copy(&uiwidth), core::mem::transmute_copy(&uiheight), core::mem::transmute_copy(&mode)).into()
            }
        }
        Self { base__: IWICBitmapSource_Vtbl::new::<Identity, OFFSET>(), Initialize: Initialize::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWICBitmapScaler as windows_core::Interface>::IID || iid == &<IWICBitmapSource as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWICBitmapScaler {}
windows_core::imp::define_interface!(IWICBitmapSource, IWICBitmapSource_Vtbl, 0x00000120_a8f2_4877_ba0a_fd2b6645fb94);
windows_core::imp::interface_hierarchy!(IWICBitmapSource, windows_core::IUnknown);
impl IWICBitmapSource {
    pub unsafe fn GetSize(&self, puiwidth: *mut u32, puiheight: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetSize)(windows_core::Interface::as_raw(self), puiwidth as _, puiheight as _) }
    }
    pub unsafe fn GetPixelFormat(&self) -> windows_core::Result<WICPixelFormatGUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPixelFormat)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetResolution(&self, pdpix: *mut f64, pdpiy: *mut f64) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetResolution)(windows_core::Interface::as_raw(self), pdpix as _, pdpiy as _) }
    }
    pub unsafe fn CopyPalette<P0>(&self, pipalette: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IWICPalette>,
    {
        unsafe { (windows_core::Interface::vtable(self).CopyPalette)(windows_core::Interface::as_raw(self), pipalette.param().abi()) }
    }
    pub unsafe fn CopyPixels(&self, prc: *const WICRect, cbstride: u32, cbbuffersize: u32) -> windows_core::Result<u8> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CopyPixels)(windows_core::Interface::as_raw(self), prc, cbstride, cbbuffersize, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICBitmapSource_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub GetPixelFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WICPixelFormatGUID) -> windows_core::HRESULT,
    pub GetResolution: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64, *mut f64) -> windows_core::HRESULT,
    pub CopyPalette: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CopyPixels: unsafe extern "system" fn(*mut core::ffi::c_void, *const WICRect, u32, u32, *mut u8) -> windows_core::HRESULT,
}
pub trait IWICBitmapSource_Impl: windows_core::IUnknownImpl {
    fn GetSize(&self, puiwidth: *mut u32, puiheight: *mut u32) -> windows_core::Result<()>;
    fn GetPixelFormat(&self) -> windows_core::Result<WICPixelFormatGUID>;
    fn GetResolution(&self, pdpix: *mut f64, pdpiy: *mut f64) -> windows_core::Result<()>;
    fn CopyPalette(&self, pipalette: windows_core::Ref<IWICPalette>) -> windows_core::Result<()>;
    fn CopyPixels(&self, prc: *const WICRect, cbstride: u32, cbbuffersize: u32) -> windows_core::Result<u8>;
}
impl IWICBitmapSource_Vtbl {
    pub const fn new<Identity: IWICBitmapSource_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetSize<Identity: IWICBitmapSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, puiwidth: *mut u32, puiheight: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICBitmapSource_Impl::GetSize(this, core::mem::transmute_copy(&puiwidth), core::mem::transmute_copy(&puiheight)).into()
            }
        }
        unsafe extern "system" fn GetPixelFormat<Identity: IWICBitmapSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppixelformat: *mut WICPixelFormatGUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICBitmapSource_Impl::GetPixelFormat(this) {
                    Ok(ok__) => {
                        ppixelformat.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetResolution<Identity: IWICBitmapSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdpix: *mut f64, pdpiy: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICBitmapSource_Impl::GetResolution(this, core::mem::transmute_copy(&pdpix), core::mem::transmute_copy(&pdpiy)).into()
            }
        }
        unsafe extern "system" fn CopyPalette<Identity: IWICBitmapSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pipalette: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICBitmapSource_Impl::CopyPalette(this, core::mem::transmute_copy(&pipalette)).into()
            }
        }
        unsafe extern "system" fn CopyPixels<Identity: IWICBitmapSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prc: *const WICRect, cbstride: u32, cbbuffersize: u32, pbbuffer: *mut u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICBitmapSource_Impl::CopyPixels(this, core::mem::transmute_copy(&prc), core::mem::transmute_copy(&cbstride), core::mem::transmute_copy(&cbbuffersize)) {
                    Ok(ok__) => {
                        pbbuffer.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSize: GetSize::<Identity, OFFSET>,
            GetPixelFormat: GetPixelFormat::<Identity, OFFSET>,
            GetResolution: GetResolution::<Identity, OFFSET>,
            CopyPalette: CopyPalette::<Identity, OFFSET>,
            CopyPixels: CopyPixels::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWICBitmapSource as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWICBitmapSource {}
windows_core::imp::define_interface!(IWICBitmapSourceTransform, IWICBitmapSourceTransform_Vtbl, 0x3b16811b_6a43_4ec9_b713_3d5a0c13b940);
windows_core::imp::interface_hierarchy!(IWICBitmapSourceTransform, windows_core::IUnknown);
impl IWICBitmapSourceTransform {
    pub unsafe fn CopyPixels(&self, prc: *const WICRect, uiwidth: u32, uiheight: u32, pguiddstformat: *const WICPixelFormatGUID, dsttransform: WICBitmapTransformOptions, nstride: u32, cbbuffersize: u32) -> windows_core::Result<u8> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CopyPixels)(windows_core::Interface::as_raw(self), prc, uiwidth, uiheight, pguiddstformat, dsttransform, nstride, cbbuffersize, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetClosestSize(&self, puiwidth: *mut u32, puiheight: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetClosestSize)(windows_core::Interface::as_raw(self), puiwidth as _, puiheight as _) }
    }
    pub unsafe fn GetClosestPixelFormat(&self, pguiddstformat: *mut WICPixelFormatGUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetClosestPixelFormat)(windows_core::Interface::as_raw(self), pguiddstformat as _) }
    }
    pub unsafe fn DoesSupportTransform(&self, dsttransform: WICBitmapTransformOptions) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DoesSupportTransform)(windows_core::Interface::as_raw(self), dsttransform, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICBitmapSourceTransform_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CopyPixels: unsafe extern "system" fn(*mut core::ffi::c_void, *const WICRect, u32, u32, *const WICPixelFormatGUID, WICBitmapTransformOptions, u32, u32, *mut u8) -> windows_core::HRESULT,
    pub GetClosestSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub GetClosestPixelFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WICPixelFormatGUID) -> windows_core::HRESULT,
    pub DoesSupportTransform: unsafe extern "system" fn(*mut core::ffi::c_void, WICBitmapTransformOptions, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IWICBitmapSourceTransform_Impl: windows_core::IUnknownImpl {
    fn CopyPixels(&self, prc: *const WICRect, uiwidth: u32, uiheight: u32, pguiddstformat: *const WICPixelFormatGUID, dsttransform: WICBitmapTransformOptions, nstride: u32, cbbuffersize: u32) -> windows_core::Result<u8>;
    fn GetClosestSize(&self, puiwidth: *mut u32, puiheight: *mut u32) -> windows_core::Result<()>;
    fn GetClosestPixelFormat(&self, pguiddstformat: *mut WICPixelFormatGUID) -> windows_core::Result<()>;
    fn DoesSupportTransform(&self, dsttransform: WICBitmapTransformOptions) -> windows_core::Result<windows_core::BOOL>;
}
impl IWICBitmapSourceTransform_Vtbl {
    pub const fn new<Identity: IWICBitmapSourceTransform_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CopyPixels<Identity: IWICBitmapSourceTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prc: *const WICRect, uiwidth: u32, uiheight: u32, pguiddstformat: *const WICPixelFormatGUID, dsttransform: WICBitmapTransformOptions, nstride: u32, cbbuffersize: u32, pbbuffer: *mut u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICBitmapSourceTransform_Impl::CopyPixels(this, core::mem::transmute_copy(&prc), core::mem::transmute_copy(&uiwidth), core::mem::transmute_copy(&uiheight), core::mem::transmute_copy(&pguiddstformat), core::mem::transmute_copy(&dsttransform), core::mem::transmute_copy(&nstride), core::mem::transmute_copy(&cbbuffersize)) {
                    Ok(ok__) => {
                        pbbuffer.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetClosestSize<Identity: IWICBitmapSourceTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, puiwidth: *mut u32, puiheight: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICBitmapSourceTransform_Impl::GetClosestSize(this, core::mem::transmute_copy(&puiwidth), core::mem::transmute_copy(&puiheight)).into()
            }
        }
        unsafe extern "system" fn GetClosestPixelFormat<Identity: IWICBitmapSourceTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguiddstformat: *mut WICPixelFormatGUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICBitmapSourceTransform_Impl::GetClosestPixelFormat(this, core::mem::transmute_copy(&pguiddstformat)).into()
            }
        }
        unsafe extern "system" fn DoesSupportTransform<Identity: IWICBitmapSourceTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dsttransform: WICBitmapTransformOptions, pfissupported: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICBitmapSourceTransform_Impl::DoesSupportTransform(this, core::mem::transmute_copy(&dsttransform)) {
                    Ok(ok__) => {
                        pfissupported.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CopyPixels: CopyPixels::<Identity, OFFSET>,
            GetClosestSize: GetClosestSize::<Identity, OFFSET>,
            GetClosestPixelFormat: GetClosestPixelFormat::<Identity, OFFSET>,
            DoesSupportTransform: DoesSupportTransform::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWICBitmapSourceTransform as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWICBitmapSourceTransform {}
windows_core::imp::define_interface!(IWICBitmapSourceTransform2, IWICBitmapSourceTransform2_Vtbl, 0xc3373fdf_6d39_4e5f_8e79_bf40c0b7ed77);
impl core::ops::Deref for IWICBitmapSourceTransform2 {
    type Target = IWICBitmapSourceTransform;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWICBitmapSourceTransform2, windows_core::IUnknown, IWICBitmapSourceTransform);
impl IWICBitmapSourceTransform2 {
    pub unsafe fn GetColorContextsForPixelFormat(&self, ppixelformat: *const WICPixelFormatGUID, ccount: u32, ppicolorcontexts: *const Option<IWICColorContext>) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetColorContextsForPixelFormat)(windows_core::Interface::as_raw(self), ppixelformat, ccount, core::mem::transmute(ppicolorcontexts), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICBitmapSourceTransform2_Vtbl {
    pub base__: IWICBitmapSourceTransform_Vtbl,
    pub GetColorContextsForPixelFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *const WICPixelFormatGUID, u32, *const *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
pub trait IWICBitmapSourceTransform2_Impl: IWICBitmapSourceTransform_Impl {
    fn GetColorContextsForPixelFormat(&self, ppixelformat: *const WICPixelFormatGUID, ccount: u32, ppicolorcontexts: *const Option<IWICColorContext>) -> windows_core::Result<u32>;
}
impl IWICBitmapSourceTransform2_Vtbl {
    pub const fn new<Identity: IWICBitmapSourceTransform2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetColorContextsForPixelFormat<Identity: IWICBitmapSourceTransform2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppixelformat: *const WICPixelFormatGUID, ccount: u32, ppicolorcontexts: *const *mut core::ffi::c_void, pcactualcount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICBitmapSourceTransform2_Impl::GetColorContextsForPixelFormat(this, core::mem::transmute_copy(&ppixelformat), core::mem::transmute_copy(&ccount), core::mem::transmute_copy(&ppicolorcontexts)) {
                    Ok(ok__) => {
                        pcactualcount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IWICBitmapSourceTransform_Vtbl::new::<Identity, OFFSET>(),
            GetColorContextsForPixelFormat: GetColorContextsForPixelFormat::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWICBitmapSourceTransform2 as windows_core::Interface>::IID || iid == &<IWICBitmapSourceTransform as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWICBitmapSourceTransform2 {}
windows_core::imp::define_interface!(IWICBitmapToneMapper, IWICBitmapToneMapper_Vtbl, 0x44728ded_1edf_4fe9_b50b_c89a264c9439);
impl core::ops::Deref for IWICBitmapToneMapper {
    type Target = IWICBitmapSource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWICBitmapToneMapper, windows_core::IUnknown, IWICBitmapSource);
impl IWICBitmapToneMapper {
    pub unsafe fn InitializeForHdrTarget<P0>(&self, pisource: P0, guiddstformat: REFWICPixelFormatGUID, fluminanceinnits: f32, fwhitelevelinnits: f32, mode: WICBitmapToneMappingMode) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IWICBitmapSource>,
    {
        unsafe { (windows_core::Interface::vtable(self).InitializeForHdrTarget)(windows_core::Interface::as_raw(self), pisource.param().abi(), guiddstformat, fluminanceinnits, fwhitelevelinnits, mode) }
    }
    pub unsafe fn InitializeForSdrTarget<P0>(&self, pisource: P0, guiddstformat: REFWICPixelFormatGUID, mode: WICBitmapToneMappingMode) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IWICBitmapSource>,
    {
        unsafe { (windows_core::Interface::vtable(self).InitializeForSdrTarget)(windows_core::Interface::as_raw(self), pisource.param().abi(), guiddstformat, mode) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICBitmapToneMapper_Vtbl {
    pub base__: IWICBitmapSource_Vtbl,
    pub InitializeForHdrTarget: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, REFWICPixelFormatGUID, f32, f32, WICBitmapToneMappingMode) -> windows_core::HRESULT,
    pub InitializeForSdrTarget: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, REFWICPixelFormatGUID, WICBitmapToneMappingMode) -> windows_core::HRESULT,
}
pub trait IWICBitmapToneMapper_Impl: IWICBitmapSource_Impl {
    fn InitializeForHdrTarget(&self, pisource: windows_core::Ref<IWICBitmapSource>, guiddstformat: REFWICPixelFormatGUID, fluminanceinnits: f32, fwhitelevelinnits: f32, mode: WICBitmapToneMappingMode) -> windows_core::Result<()>;
    fn InitializeForSdrTarget(&self, pisource: windows_core::Ref<IWICBitmapSource>, guiddstformat: REFWICPixelFormatGUID, mode: WICBitmapToneMappingMode) -> windows_core::Result<()>;
}
impl IWICBitmapToneMapper_Vtbl {
    pub const fn new<Identity: IWICBitmapToneMapper_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn InitializeForHdrTarget<Identity: IWICBitmapToneMapper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisource: *mut core::ffi::c_void, guiddstformat: REFWICPixelFormatGUID, fluminanceinnits: f32, fwhitelevelinnits: f32, mode: WICBitmapToneMappingMode) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICBitmapToneMapper_Impl::InitializeForHdrTarget(this, core::mem::transmute_copy(&pisource), core::mem::transmute_copy(&guiddstformat), core::mem::transmute_copy(&fluminanceinnits), core::mem::transmute_copy(&fwhitelevelinnits), core::mem::transmute_copy(&mode)).into()
            }
        }
        unsafe extern "system" fn InitializeForSdrTarget<Identity: IWICBitmapToneMapper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisource: *mut core::ffi::c_void, guiddstformat: REFWICPixelFormatGUID, mode: WICBitmapToneMappingMode) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICBitmapToneMapper_Impl::InitializeForSdrTarget(this, core::mem::transmute_copy(&pisource), core::mem::transmute_copy(&guiddstformat), core::mem::transmute_copy(&mode)).into()
            }
        }
        Self {
            base__: IWICBitmapSource_Vtbl::new::<Identity, OFFSET>(),
            InitializeForHdrTarget: InitializeForHdrTarget::<Identity, OFFSET>,
            InitializeForSdrTarget: InitializeForSdrTarget::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWICBitmapToneMapper as windows_core::Interface>::IID || iid == &<IWICBitmapSource as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWICBitmapToneMapper {}
windows_core::imp::define_interface!(IWICColorContext, IWICColorContext_Vtbl, 0x3c613a02_34b2_44ea_9a7c_45aea9c6fd6d);
windows_core::imp::interface_hierarchy!(IWICColorContext, windows_core::IUnknown);
impl IWICColorContext {
    pub unsafe fn InitializeFromFilename<P0>(&self, wzfilename: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).InitializeFromFilename)(windows_core::Interface::as_raw(self), wzfilename.param().abi()) }
    }
    pub unsafe fn InitializeFromMemory(&self, pbbuffer: *const u8, cbbuffersize: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).InitializeFromMemory)(windows_core::Interface::as_raw(self), pbbuffer, cbbuffersize) }
    }
    pub unsafe fn InitializeFromExifColorSpace(&self, value: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).InitializeFromExifColorSpace)(windows_core::Interface::as_raw(self), value) }
    }
    pub unsafe fn GetType(&self) -> windows_core::Result<WICColorContextType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetProfileBytes(&self, cbbuffer: u32, pbbuffer: *mut u8, pcbactual: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetProfileBytes)(windows_core::Interface::as_raw(self), cbbuffer, pbbuffer as _, pcbactual as _) }
    }
    pub unsafe fn GetExifColorSpace(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetExifColorSpace)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICColorContext_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub InitializeFromFilename: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub InitializeFromMemory: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, u32) -> windows_core::HRESULT,
    pub InitializeFromExifColorSpace: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WICColorContextType) -> windows_core::HRESULT,
    pub GetProfileBytes: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u8, *mut u32) -> windows_core::HRESULT,
    pub GetExifColorSpace: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
pub trait IWICColorContext_Impl: windows_core::IUnknownImpl {
    fn InitializeFromFilename(&self, wzfilename: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn InitializeFromMemory(&self, pbbuffer: *const u8, cbbuffersize: u32) -> windows_core::Result<()>;
    fn InitializeFromExifColorSpace(&self, value: u32) -> windows_core::Result<()>;
    fn GetType(&self) -> windows_core::Result<WICColorContextType>;
    fn GetProfileBytes(&self, cbbuffer: u32, pbbuffer: *mut u8, pcbactual: *mut u32) -> windows_core::Result<()>;
    fn GetExifColorSpace(&self) -> windows_core::Result<u32>;
}
impl IWICColorContext_Vtbl {
    pub const fn new<Identity: IWICColorContext_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn InitializeFromFilename<Identity: IWICColorContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wzfilename: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICColorContext_Impl::InitializeFromFilename(this, core::mem::transmute(&wzfilename)).into()
            }
        }
        unsafe extern "system" fn InitializeFromMemory<Identity: IWICColorContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbbuffer: *const u8, cbbuffersize: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICColorContext_Impl::InitializeFromMemory(this, core::mem::transmute_copy(&pbbuffer), core::mem::transmute_copy(&cbbuffersize)).into()
            }
        }
        unsafe extern "system" fn InitializeFromExifColorSpace<Identity: IWICColorContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICColorContext_Impl::InitializeFromExifColorSpace(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetType<Identity: IWICColorContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptype: *mut WICColorContextType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICColorContext_Impl::GetType(this) {
                    Ok(ok__) => {
                        ptype.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetProfileBytes<Identity: IWICColorContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbbuffer: u32, pbbuffer: *mut u8, pcbactual: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICColorContext_Impl::GetProfileBytes(this, core::mem::transmute_copy(&cbbuffer), core::mem::transmute_copy(&pbbuffer), core::mem::transmute_copy(&pcbactual)).into()
            }
        }
        unsafe extern "system" fn GetExifColorSpace<Identity: IWICColorContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICColorContext_Impl::GetExifColorSpace(this) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            InitializeFromFilename: InitializeFromFilename::<Identity, OFFSET>,
            InitializeFromMemory: InitializeFromMemory::<Identity, OFFSET>,
            InitializeFromExifColorSpace: InitializeFromExifColorSpace::<Identity, OFFSET>,
            GetType: GetType::<Identity, OFFSET>,
            GetProfileBytes: GetProfileBytes::<Identity, OFFSET>,
            GetExifColorSpace: GetExifColorSpace::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWICColorContext as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWICColorContext {}
windows_core::imp::define_interface!(IWICColorTransform, IWICColorTransform_Vtbl, 0xb66f034f_d0e2_40ab_b436_6de39e321a94);
impl core::ops::Deref for IWICColorTransform {
    type Target = IWICBitmapSource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWICColorTransform, windows_core::IUnknown, IWICBitmapSource);
impl IWICColorTransform {
    pub unsafe fn Initialize<P0, P1, P2>(&self, pibitmapsource: P0, picontextsource: P1, picontextdest: P2, pixelfmtdest: REFWICPixelFormatGUID) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IWICBitmapSource>,
        P1: windows_core::Param<IWICColorContext>,
        P2: windows_core::Param<IWICColorContext>,
    {
        unsafe { (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), pibitmapsource.param().abi(), picontextsource.param().abi(), picontextdest.param().abi(), pixelfmtdest) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICColorTransform_Vtbl {
    pub base__: IWICBitmapSource_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, REFWICPixelFormatGUID) -> windows_core::HRESULT,
}
pub trait IWICColorTransform_Impl: IWICBitmapSource_Impl {
    fn Initialize(&self, pibitmapsource: windows_core::Ref<IWICBitmapSource>, picontextsource: windows_core::Ref<IWICColorContext>, picontextdest: windows_core::Ref<IWICColorContext>, pixelfmtdest: REFWICPixelFormatGUID) -> windows_core::Result<()>;
}
impl IWICColorTransform_Vtbl {
    pub const fn new<Identity: IWICColorTransform_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Initialize<Identity: IWICColorTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pibitmapsource: *mut core::ffi::c_void, picontextsource: *mut core::ffi::c_void, picontextdest: *mut core::ffi::c_void, pixelfmtdest: REFWICPixelFormatGUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICColorTransform_Impl::Initialize(this, core::mem::transmute_copy(&pibitmapsource), core::mem::transmute_copy(&picontextsource), core::mem::transmute_copy(&picontextdest), core::mem::transmute_copy(&pixelfmtdest)).into()
            }
        }
        Self { base__: IWICBitmapSource_Vtbl::new::<Identity, OFFSET>(), Initialize: Initialize::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWICColorTransform as windows_core::Interface>::IID || iid == &<IWICBitmapSource as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWICColorTransform {}
windows_core::imp::define_interface!(IWICComponentInfo, IWICComponentInfo_Vtbl, 0x23bc3f0a_698b_4357_886b_f24d50671334);
windows_core::imp::interface_hierarchy!(IWICComponentInfo, windows_core::IUnknown);
impl IWICComponentInfo {
    pub unsafe fn GetComponentType(&self) -> windows_core::Result<WICComponentType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetComponentType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetCLSID(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCLSID)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetSigningStatus(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSigningStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetAuthor(&self, cchauthor: u32, wzauthor: *mut u16, pcchactual: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetAuthor)(windows_core::Interface::as_raw(self), cchauthor, wzauthor as _, pcchactual as _) }
    }
    pub unsafe fn GetVendorGUID(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetVendorGUID)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetVersion(&self, cchversion: u32, wzversion: *mut u16, pcchactual: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetVersion)(windows_core::Interface::as_raw(self), cchversion, wzversion as _, pcchactual as _) }
    }
    pub unsafe fn GetSpecVersion(&self, cchspecversion: u32, wzspecversion: *mut u16, pcchactual: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetSpecVersion)(windows_core::Interface::as_raw(self), cchspecversion, wzspecversion as _, pcchactual as _) }
    }
    pub unsafe fn GetFriendlyName(&self, cchfriendlyname: u32, wzfriendlyname: *mut u16, pcchactual: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetFriendlyName)(windows_core::Interface::as_raw(self), cchfriendlyname, wzfriendlyname as _, pcchactual as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICComponentInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetComponentType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WICComponentType) -> windows_core::HRESULT,
    pub GetCLSID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GetSigningStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetAuthor: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u16, *mut u32) -> windows_core::HRESULT,
    pub GetVendorGUID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GetVersion: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u16, *mut u32) -> windows_core::HRESULT,
    pub GetSpecVersion: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u16, *mut u32) -> windows_core::HRESULT,
    pub GetFriendlyName: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u16, *mut u32) -> windows_core::HRESULT,
}
pub trait IWICComponentInfo_Impl: windows_core::IUnknownImpl {
    fn GetComponentType(&self) -> windows_core::Result<WICComponentType>;
    fn GetCLSID(&self) -> windows_core::Result<windows_core::GUID>;
    fn GetSigningStatus(&self) -> windows_core::Result<u32>;
    fn GetAuthor(&self, cchauthor: u32, wzauthor: *mut u16, pcchactual: *mut u32) -> windows_core::Result<()>;
    fn GetVendorGUID(&self) -> windows_core::Result<windows_core::GUID>;
    fn GetVersion(&self, cchversion: u32, wzversion: *mut u16, pcchactual: *mut u32) -> windows_core::Result<()>;
    fn GetSpecVersion(&self, cchspecversion: u32, wzspecversion: *mut u16, pcchactual: *mut u32) -> windows_core::Result<()>;
    fn GetFriendlyName(&self, cchfriendlyname: u32, wzfriendlyname: *mut u16, pcchactual: *mut u32) -> windows_core::Result<()>;
}
impl IWICComponentInfo_Vtbl {
    pub const fn new<Identity: IWICComponentInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetComponentType<Identity: IWICComponentInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptype: *mut WICComponentType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICComponentInfo_Impl::GetComponentType(this) {
                    Ok(ok__) => {
                        ptype.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCLSID<Identity: IWICComponentInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pclsid: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICComponentInfo_Impl::GetCLSID(this) {
                    Ok(ok__) => {
                        pclsid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSigningStatus<Identity: IWICComponentInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstatus: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICComponentInfo_Impl::GetSigningStatus(this) {
                    Ok(ok__) => {
                        pstatus.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetAuthor<Identity: IWICComponentInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cchauthor: u32, wzauthor: *mut u16, pcchactual: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICComponentInfo_Impl::GetAuthor(this, core::mem::transmute_copy(&cchauthor), core::mem::transmute_copy(&wzauthor), core::mem::transmute_copy(&pcchactual)).into()
            }
        }
        unsafe extern "system" fn GetVendorGUID<Identity: IWICComponentInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidvendor: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICComponentInfo_Impl::GetVendorGUID(this) {
                    Ok(ok__) => {
                        pguidvendor.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetVersion<Identity: IWICComponentInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cchversion: u32, wzversion: *mut u16, pcchactual: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICComponentInfo_Impl::GetVersion(this, core::mem::transmute_copy(&cchversion), core::mem::transmute_copy(&wzversion), core::mem::transmute_copy(&pcchactual)).into()
            }
        }
        unsafe extern "system" fn GetSpecVersion<Identity: IWICComponentInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cchspecversion: u32, wzspecversion: *mut u16, pcchactual: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICComponentInfo_Impl::GetSpecVersion(this, core::mem::transmute_copy(&cchspecversion), core::mem::transmute_copy(&wzspecversion), core::mem::transmute_copy(&pcchactual)).into()
            }
        }
        unsafe extern "system" fn GetFriendlyName<Identity: IWICComponentInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cchfriendlyname: u32, wzfriendlyname: *mut u16, pcchactual: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICComponentInfo_Impl::GetFriendlyName(this, core::mem::transmute_copy(&cchfriendlyname), core::mem::transmute_copy(&wzfriendlyname), core::mem::transmute_copy(&pcchactual)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetComponentType: GetComponentType::<Identity, OFFSET>,
            GetCLSID: GetCLSID::<Identity, OFFSET>,
            GetSigningStatus: GetSigningStatus::<Identity, OFFSET>,
            GetAuthor: GetAuthor::<Identity, OFFSET>,
            GetVendorGUID: GetVendorGUID::<Identity, OFFSET>,
            GetVersion: GetVersion::<Identity, OFFSET>,
            GetSpecVersion: GetSpecVersion::<Identity, OFFSET>,
            GetFriendlyName: GetFriendlyName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWICComponentInfo as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWICComponentInfo {}
windows_core::imp::define_interface!(IWICD3DTextureSource, IWICD3DTextureSource_Vtbl, 0xcaf65cc4_8ebe_4718_a21f_8dbf40bb7e25);
windows_core::imp::interface_hierarchy!(IWICD3DTextureSource, windows_core::IUnknown);
impl IWICD3DTextureSource {
    #[cfg(feature = "ocidl")]
    pub unsafe fn GetTexture<P0, P1, T>(&self, pd3ddevice: P0, pid3dtextureoptions: P1) -> windows_core::Result<T>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        P1: windows_core::Param<super::ocidl::IPropertyBag2>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetTexture)(windows_core::Interface::as_raw(self), pd3ddevice.param().abi(), pid3dtextureoptions.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    #[cfg(feature = "ocidl")]
    pub unsafe fn GetTransformedTexture<P5, P6, T>(&self, prc: Option<*const WICRect>, uiwidth: u32, uiheight: u32, pguiddstformat: Option<*const WICPixelFormatGUID>, dsttransform: WICBitmapTransformOptions, pd3ddevice: P5, pid3dtextureoptions: P6) -> windows_core::Result<T>
    where
        P5: windows_core::Param<windows_core::IUnknown>,
        P6: windows_core::Param<super::ocidl::IPropertyBag2>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetTransformedTexture)(windows_core::Interface::as_raw(self), prc.unwrap_or(core::mem::zeroed()) as _, uiwidth, uiheight, pguiddstformat.unwrap_or(core::mem::zeroed()) as _, dsttransform, pd3ddevice.param().abi(), pid3dtextureoptions.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    pub unsafe fn DoesSupportD3DDeviceType(&self, riid: *const windows_core::GUID) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DoesSupportD3DDeviceType)(windows_core::Interface::as_raw(self), riid, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "ocidl")]
    pub unsafe fn GetD3DTextureOptions(&self) -> windows_core::Result<super::ocidl::IPropertyBag2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetD3DTextureOptions)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICD3DTextureSource_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "ocidl")]
    pub GetTexture: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "ocidl"))]
    GetTexture: usize,
    #[cfg(feature = "ocidl")]
    pub GetTransformedTexture: unsafe extern "system" fn(*mut core::ffi::c_void, *const WICRect, u32, u32, *const WICPixelFormatGUID, WICBitmapTransformOptions, *mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "ocidl"))]
    GetTransformedTexture: usize,
    pub DoesSupportD3DDeviceType: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "ocidl")]
    pub GetD3DTextureOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "ocidl"))]
    GetD3DTextureOptions: usize,
}
#[cfg(feature = "ocidl")]
pub trait IWICD3DTextureSource_Impl: windows_core::IUnknownImpl {
    fn GetTexture(&self, pd3ddevice: windows_core::Ref<windows_core::IUnknown>, pid3dtextureoptions: windows_core::Ref<super::ocidl::IPropertyBag2>, riid: *const windows_core::GUID, pptexture: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetTransformedTexture(&self, prc: *const WICRect, uiwidth: u32, uiheight: u32, pguiddstformat: *const WICPixelFormatGUID, dsttransform: WICBitmapTransformOptions, pd3ddevice: windows_core::Ref<windows_core::IUnknown>, pid3dtextureoptions: windows_core::Ref<super::ocidl::IPropertyBag2>, riid: *const windows_core::GUID, pptexture: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn DoesSupportD3DDeviceType(&self, riid: *const windows_core::GUID) -> windows_core::Result<windows_core::BOOL>;
    fn GetD3DTextureOptions(&self) -> windows_core::Result<super::ocidl::IPropertyBag2>;
}
#[cfg(feature = "ocidl")]
impl IWICD3DTextureSource_Vtbl {
    pub const fn new<Identity: IWICD3DTextureSource_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetTexture<Identity: IWICD3DTextureSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pd3ddevice: *mut core::ffi::c_void, pid3dtextureoptions: *mut core::ffi::c_void, riid: *const windows_core::GUID, pptexture: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICD3DTextureSource_Impl::GetTexture(this, core::mem::transmute_copy(&pd3ddevice), core::mem::transmute_copy(&pid3dtextureoptions), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&pptexture)).into()
            }
        }
        unsafe extern "system" fn GetTransformedTexture<Identity: IWICD3DTextureSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prc: *const WICRect, uiwidth: u32, uiheight: u32, pguiddstformat: *const WICPixelFormatGUID, dsttransform: WICBitmapTransformOptions, pd3ddevice: *mut core::ffi::c_void, pid3dtextureoptions: *mut core::ffi::c_void, riid: *const windows_core::GUID, pptexture: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICD3DTextureSource_Impl::GetTransformedTexture(this, core::mem::transmute_copy(&prc), core::mem::transmute_copy(&uiwidth), core::mem::transmute_copy(&uiheight), core::mem::transmute_copy(&pguiddstformat), core::mem::transmute_copy(&dsttransform), core::mem::transmute_copy(&pd3ddevice), core::mem::transmute_copy(&pid3dtextureoptions), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&pptexture)).into()
            }
        }
        unsafe extern "system" fn DoesSupportD3DDeviceType<Identity: IWICD3DTextureSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, pfissupported: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICD3DTextureSource_Impl::DoesSupportD3DDeviceType(this, core::mem::transmute_copy(&riid)) {
                    Ok(ok__) => {
                        pfissupported.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetD3DTextureOptions<Identity: IWICD3DTextureSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppid3dtextureoptions: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICD3DTextureSource_Impl::GetD3DTextureOptions(this) {
                    Ok(ok__) => {
                        ppid3dtextureoptions.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetTexture: GetTexture::<Identity, OFFSET>,
            GetTransformedTexture: GetTransformedTexture::<Identity, OFFSET>,
            DoesSupportD3DDeviceType: DoesSupportD3DDeviceType::<Identity, OFFSET>,
            GetD3DTextureOptions: GetD3DTextureOptions::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWICD3DTextureSource as windows_core::Interface>::IID
    }
}
#[cfg(feature = "ocidl")]
impl windows_core::RuntimeName for IWICD3DTextureSource {}
windows_core::imp::define_interface!(IWICDdsDecoder, IWICDdsDecoder_Vtbl, 0x409cd537_8532_40cb_9774_e2feb2df4e9c);
windows_core::imp::interface_hierarchy!(IWICDdsDecoder, windows_core::IUnknown);
impl IWICDdsDecoder {
    #[cfg(feature = "dxgi")]
    pub unsafe fn GetParameters(&self, pparameters: *mut WICDdsParameters) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetParameters)(windows_core::Interface::as_raw(self), pparameters as _) }
    }
    pub unsafe fn GetFrame(&self, arrayindex: u32, miplevel: u32, sliceindex: u32) -> windows_core::Result<IWICBitmapFrameDecode> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFrame)(windows_core::Interface::as_raw(self), arrayindex, miplevel, sliceindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICDdsDecoder_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "dxgi")]
    pub GetParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WICDdsParameters) -> windows_core::HRESULT,
    #[cfg(not(feature = "dxgi"))]
    GetParameters: usize,
    pub GetFrame: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "dxgi")]
pub trait IWICDdsDecoder_Impl: windows_core::IUnknownImpl {
    fn GetParameters(&self, pparameters: *mut WICDdsParameters) -> windows_core::Result<()>;
    fn GetFrame(&self, arrayindex: u32, miplevel: u32, sliceindex: u32) -> windows_core::Result<IWICBitmapFrameDecode>;
}
#[cfg(feature = "dxgi")]
impl IWICDdsDecoder_Vtbl {
    pub const fn new<Identity: IWICDdsDecoder_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetParameters<Identity: IWICDdsDecoder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pparameters: *mut WICDdsParameters) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICDdsDecoder_Impl::GetParameters(this, core::mem::transmute_copy(&pparameters)).into()
            }
        }
        unsafe extern "system" fn GetFrame<Identity: IWICDdsDecoder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, arrayindex: u32, miplevel: u32, sliceindex: u32, ppibitmapframe: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICDdsDecoder_Impl::GetFrame(this, core::mem::transmute_copy(&arrayindex), core::mem::transmute_copy(&miplevel), core::mem::transmute_copy(&sliceindex)) {
                    Ok(ok__) => {
                        ppibitmapframe.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetParameters: GetParameters::<Identity, OFFSET>,
            GetFrame: GetFrame::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWICDdsDecoder as windows_core::Interface>::IID
    }
}
#[cfg(feature = "dxgi")]
impl windows_core::RuntimeName for IWICDdsDecoder {}
windows_core::imp::define_interface!(IWICDdsEncoder, IWICDdsEncoder_Vtbl, 0x5cacdb4c_407e_41b3_b936_d0f010cd6732);
windows_core::imp::interface_hierarchy!(IWICDdsEncoder, windows_core::IUnknown);
impl IWICDdsEncoder {
    #[cfg(feature = "dxgi")]
    pub unsafe fn SetParameters(&self, pparameters: *const WICDdsParameters) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetParameters)(windows_core::Interface::as_raw(self), pparameters) }
    }
    #[cfg(feature = "dxgi")]
    pub unsafe fn GetParameters(&self, pparameters: *mut WICDdsParameters) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetParameters)(windows_core::Interface::as_raw(self), pparameters as _) }
    }
    pub unsafe fn CreateNewFrame(&self, ppiframeencode: *mut Option<IWICBitmapFrameEncode>, parrayindex: Option<*mut u32>, pmiplevel: Option<*mut u32>, psliceindex: Option<*mut u32>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CreateNewFrame)(windows_core::Interface::as_raw(self), core::mem::transmute(ppiframeencode), parrayindex.unwrap_or(core::mem::zeroed()) as _, pmiplevel.unwrap_or(core::mem::zeroed()) as _, psliceindex.unwrap_or(core::mem::zeroed()) as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICDdsEncoder_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "dxgi")]
    pub SetParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *const WICDdsParameters) -> windows_core::HRESULT,
    #[cfg(not(feature = "dxgi"))]
    SetParameters: usize,
    #[cfg(feature = "dxgi")]
    pub GetParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WICDdsParameters) -> windows_core::HRESULT,
    #[cfg(not(feature = "dxgi"))]
    GetParameters: usize,
    pub CreateNewFrame: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut u32, *mut u32, *mut u32) -> windows_core::HRESULT,
}
#[cfg(feature = "dxgi")]
pub trait IWICDdsEncoder_Impl: windows_core::IUnknownImpl {
    fn SetParameters(&self, pparameters: *const WICDdsParameters) -> windows_core::Result<()>;
    fn GetParameters(&self, pparameters: *mut WICDdsParameters) -> windows_core::Result<()>;
    fn CreateNewFrame(&self, ppiframeencode: windows_core::OutRef<IWICBitmapFrameEncode>, parrayindex: *mut u32, pmiplevel: *mut u32, psliceindex: *mut u32) -> windows_core::Result<()>;
}
#[cfg(feature = "dxgi")]
impl IWICDdsEncoder_Vtbl {
    pub const fn new<Identity: IWICDdsEncoder_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetParameters<Identity: IWICDdsEncoder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pparameters: *const WICDdsParameters) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICDdsEncoder_Impl::SetParameters(this, core::mem::transmute_copy(&pparameters)).into()
            }
        }
        unsafe extern "system" fn GetParameters<Identity: IWICDdsEncoder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pparameters: *mut WICDdsParameters) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICDdsEncoder_Impl::GetParameters(this, core::mem::transmute_copy(&pparameters)).into()
            }
        }
        unsafe extern "system" fn CreateNewFrame<Identity: IWICDdsEncoder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppiframeencode: *mut *mut core::ffi::c_void, parrayindex: *mut u32, pmiplevel: *mut u32, psliceindex: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICDdsEncoder_Impl::CreateNewFrame(this, core::mem::transmute_copy(&ppiframeencode), core::mem::transmute_copy(&parrayindex), core::mem::transmute_copy(&pmiplevel), core::mem::transmute_copy(&psliceindex)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetParameters: SetParameters::<Identity, OFFSET>,
            GetParameters: GetParameters::<Identity, OFFSET>,
            CreateNewFrame: CreateNewFrame::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWICDdsEncoder as windows_core::Interface>::IID
    }
}
#[cfg(feature = "dxgi")]
impl windows_core::RuntimeName for IWICDdsEncoder {}
windows_core::imp::define_interface!(IWICDdsFrameDecode, IWICDdsFrameDecode_Vtbl, 0x3d4c0c61_18a4_41e4_bd80_481a4fc9f464);
windows_core::imp::interface_hierarchy!(IWICDdsFrameDecode, windows_core::IUnknown);
impl IWICDdsFrameDecode {
    pub unsafe fn GetSizeInBlocks(&self, pwidthinblocks: *mut u32, pheightinblocks: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetSizeInBlocks)(windows_core::Interface::as_raw(self), pwidthinblocks as _, pheightinblocks as _) }
    }
    #[cfg(feature = "dxgi")]
    pub unsafe fn GetFormatInfo(&self) -> windows_core::Result<WICDdsFormatInfo> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFormatInfo)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CopyBlocks(&self, prcboundsinblocks: *const WICRect, cbstride: u32, cbbuffersize: u32) -> windows_core::Result<u8> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CopyBlocks)(windows_core::Interface::as_raw(self), prcboundsinblocks, cbstride, cbbuffersize, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICDdsFrameDecode_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetSizeInBlocks: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "dxgi")]
    pub GetFormatInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WICDdsFormatInfo) -> windows_core::HRESULT,
    #[cfg(not(feature = "dxgi"))]
    GetFormatInfo: usize,
    pub CopyBlocks: unsafe extern "system" fn(*mut core::ffi::c_void, *const WICRect, u32, u32, *mut u8) -> windows_core::HRESULT,
}
#[cfg(feature = "dxgi")]
pub trait IWICDdsFrameDecode_Impl: windows_core::IUnknownImpl {
    fn GetSizeInBlocks(&self, pwidthinblocks: *mut u32, pheightinblocks: *mut u32) -> windows_core::Result<()>;
    fn GetFormatInfo(&self) -> windows_core::Result<WICDdsFormatInfo>;
    fn CopyBlocks(&self, prcboundsinblocks: *const WICRect, cbstride: u32, cbbuffersize: u32) -> windows_core::Result<u8>;
}
#[cfg(feature = "dxgi")]
impl IWICDdsFrameDecode_Vtbl {
    pub const fn new<Identity: IWICDdsFrameDecode_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetSizeInBlocks<Identity: IWICDdsFrameDecode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwidthinblocks: *mut u32, pheightinblocks: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICDdsFrameDecode_Impl::GetSizeInBlocks(this, core::mem::transmute_copy(&pwidthinblocks), core::mem::transmute_copy(&pheightinblocks)).into()
            }
        }
        unsafe extern "system" fn GetFormatInfo<Identity: IWICDdsFrameDecode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pformatinfo: *mut WICDdsFormatInfo) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICDdsFrameDecode_Impl::GetFormatInfo(this) {
                    Ok(ok__) => {
                        pformatinfo.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CopyBlocks<Identity: IWICDdsFrameDecode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prcboundsinblocks: *const WICRect, cbstride: u32, cbbuffersize: u32, pbbuffer: *mut u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICDdsFrameDecode_Impl::CopyBlocks(this, core::mem::transmute_copy(&prcboundsinblocks), core::mem::transmute_copy(&cbstride), core::mem::transmute_copy(&cbbuffersize)) {
                    Ok(ok__) => {
                        pbbuffer.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSizeInBlocks: GetSizeInBlocks::<Identity, OFFSET>,
            GetFormatInfo: GetFormatInfo::<Identity, OFFSET>,
            CopyBlocks: CopyBlocks::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWICDdsFrameDecode as windows_core::Interface>::IID
    }
}
#[cfg(feature = "dxgi")]
impl windows_core::RuntimeName for IWICDdsFrameDecode {}
windows_core::imp::define_interface!(IWICDevelopRaw, IWICDevelopRaw_Vtbl, 0xfbec5e44_f7be_4b65_b7f8_c0c81fef026d);
impl core::ops::Deref for IWICDevelopRaw {
    type Target = IWICBitmapFrameDecode;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWICDevelopRaw, windows_core::IUnknown, IWICBitmapSource, IWICBitmapFrameDecode);
impl IWICDevelopRaw {
    pub unsafe fn QueryRawCapabilitiesInfo(&self, pinfo: *mut WICRawCapabilitiesInfo) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).QueryRawCapabilitiesInfo)(windows_core::Interface::as_raw(self), pinfo as _) }
    }
    pub unsafe fn LoadParameterSet(&self, parameterset: WICRawParameterSet) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).LoadParameterSet)(windows_core::Interface::as_raw(self), parameterset) }
    }
    #[cfg(feature = "ocidl")]
    pub unsafe fn GetCurrentParameterSet(&self) -> windows_core::Result<super::ocidl::IPropertyBag2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCurrentParameterSet)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetExposureCompensation(&self, ev: f64) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetExposureCompensation)(windows_core::Interface::as_raw(self), ev) }
    }
    pub unsafe fn GetExposureCompensation(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetExposureCompensation)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetWhitePointRGB(&self, red: u32, green: u32, blue: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetWhitePointRGB)(windows_core::Interface::as_raw(self), red, green, blue) }
    }
    pub unsafe fn GetWhitePointRGB(&self, pred: *mut u32, pgreen: *mut u32, pblue: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetWhitePointRGB)(windows_core::Interface::as_raw(self), pred as _, pgreen as _, pblue as _) }
    }
    pub unsafe fn SetNamedWhitePoint(&self, whitepoint: WICNamedWhitePoint) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetNamedWhitePoint)(windows_core::Interface::as_raw(self), whitepoint) }
    }
    pub unsafe fn GetNamedWhitePoint(&self) -> windows_core::Result<WICNamedWhitePoint> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNamedWhitePoint)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetWhitePointKelvin(&self, whitepointkelvin: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetWhitePointKelvin)(windows_core::Interface::as_raw(self), whitepointkelvin) }
    }
    pub unsafe fn GetWhitePointKelvin(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetWhitePointKelvin)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetKelvinRangeInfo(&self, pminkelvintemp: *mut u32, pmaxkelvintemp: *mut u32, pkelvintempstepvalue: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetKelvinRangeInfo)(windows_core::Interface::as_raw(self), pminkelvintemp as _, pmaxkelvintemp as _, pkelvintempstepvalue as _) }
    }
    pub unsafe fn SetContrast(&self, contrast: f64) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetContrast)(windows_core::Interface::as_raw(self), contrast) }
    }
    pub unsafe fn GetContrast(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetContrast)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetGamma(&self, gamma: f64) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetGamma)(windows_core::Interface::as_raw(self), gamma) }
    }
    pub unsafe fn GetGamma(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetGamma)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetSharpness(&self, sharpness: f64) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSharpness)(windows_core::Interface::as_raw(self), sharpness) }
    }
    pub unsafe fn GetSharpness(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSharpness)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetSaturation(&self, saturation: f64) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSaturation)(windows_core::Interface::as_raw(self), saturation) }
    }
    pub unsafe fn GetSaturation(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSaturation)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetTint(&self, tint: f64) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetTint)(windows_core::Interface::as_raw(self), tint) }
    }
    pub unsafe fn GetTint(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTint)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetNoiseReduction(&self, noisereduction: f64) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetNoiseReduction)(windows_core::Interface::as_raw(self), noisereduction) }
    }
    pub unsafe fn GetNoiseReduction(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNoiseReduction)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetDestinationColorContext<P0>(&self, pcolorcontext: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IWICColorContext>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetDestinationColorContext)(windows_core::Interface::as_raw(self), pcolorcontext.param().abi()) }
    }
    pub unsafe fn SetToneCurve(&self, cbtonecurvesize: u32, ptonecurve: *const WICRawToneCurve) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetToneCurve)(windows_core::Interface::as_raw(self), cbtonecurvesize, ptonecurve) }
    }
    pub unsafe fn GetToneCurve(&self, cbtonecurvebuffersize: u32, ptonecurve: Option<*mut WICRawToneCurve>, pcbactualtonecurvebuffersize: Option<*mut u32>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetToneCurve)(windows_core::Interface::as_raw(self), cbtonecurvebuffersize, ptonecurve.unwrap_or(core::mem::zeroed()) as _, pcbactualtonecurvebuffersize.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn SetRotation(&self, rotation: f64) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetRotation)(windows_core::Interface::as_raw(self), rotation) }
    }
    pub unsafe fn GetRotation(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRotation)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetRenderMode(&self, rendermode: WICRawRenderMode) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetRenderMode)(windows_core::Interface::as_raw(self), rendermode) }
    }
    pub unsafe fn GetRenderMode(&self) -> windows_core::Result<WICRawRenderMode> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRenderMode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetNotificationCallback<P0>(&self, pcallback: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IWICDevelopRawNotificationCallback>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetNotificationCallback)(windows_core::Interface::as_raw(self), pcallback.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICDevelopRaw_Vtbl {
    pub base__: IWICBitmapFrameDecode_Vtbl,
    pub QueryRawCapabilitiesInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WICRawCapabilitiesInfo) -> windows_core::HRESULT,
    pub LoadParameterSet: unsafe extern "system" fn(*mut core::ffi::c_void, WICRawParameterSet) -> windows_core::HRESULT,
    #[cfg(feature = "ocidl")]
    pub GetCurrentParameterSet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "ocidl"))]
    GetCurrentParameterSet: usize,
    pub SetExposureCompensation: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub GetExposureCompensation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetWhitePointRGB: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32) -> windows_core::HRESULT,
    pub GetWhitePointRGB: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub SetNamedWhitePoint: unsafe extern "system" fn(*mut core::ffi::c_void, WICNamedWhitePoint) -> windows_core::HRESULT,
    pub GetNamedWhitePoint: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WICNamedWhitePoint) -> windows_core::HRESULT,
    pub SetWhitePointKelvin: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetWhitePointKelvin: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetKelvinRangeInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub SetContrast: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub GetContrast: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetGamma: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub GetGamma: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetSharpness: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub GetSharpness: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetSaturation: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub GetSaturation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetTint: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub GetTint: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetNoiseReduction: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub GetNoiseReduction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetDestinationColorContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetToneCurve: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const WICRawToneCurve) -> windows_core::HRESULT,
    pub GetToneCurve: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut WICRawToneCurve, *mut u32) -> windows_core::HRESULT,
    pub SetRotation: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub GetRotation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetRenderMode: unsafe extern "system" fn(*mut core::ffi::c_void, WICRawRenderMode) -> windows_core::HRESULT,
    pub GetRenderMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WICRawRenderMode) -> windows_core::HRESULT,
    pub SetNotificationCallback: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "ocidl")]
pub trait IWICDevelopRaw_Impl: IWICBitmapFrameDecode_Impl {
    fn QueryRawCapabilitiesInfo(&self, pinfo: *mut WICRawCapabilitiesInfo) -> windows_core::Result<()>;
    fn LoadParameterSet(&self, parameterset: WICRawParameterSet) -> windows_core::Result<()>;
    fn GetCurrentParameterSet(&self) -> windows_core::Result<super::ocidl::IPropertyBag2>;
    fn SetExposureCompensation(&self, ev: f64) -> windows_core::Result<()>;
    fn GetExposureCompensation(&self) -> windows_core::Result<f64>;
    fn SetWhitePointRGB(&self, red: u32, green: u32, blue: u32) -> windows_core::Result<()>;
    fn GetWhitePointRGB(&self, pred: *mut u32, pgreen: *mut u32, pblue: *mut u32) -> windows_core::Result<()>;
    fn SetNamedWhitePoint(&self, whitepoint: WICNamedWhitePoint) -> windows_core::Result<()>;
    fn GetNamedWhitePoint(&self) -> windows_core::Result<WICNamedWhitePoint>;
    fn SetWhitePointKelvin(&self, whitepointkelvin: u32) -> windows_core::Result<()>;
    fn GetWhitePointKelvin(&self) -> windows_core::Result<u32>;
    fn GetKelvinRangeInfo(&self, pminkelvintemp: *mut u32, pmaxkelvintemp: *mut u32, pkelvintempstepvalue: *mut u32) -> windows_core::Result<()>;
    fn SetContrast(&self, contrast: f64) -> windows_core::Result<()>;
    fn GetContrast(&self) -> windows_core::Result<f64>;
    fn SetGamma(&self, gamma: f64) -> windows_core::Result<()>;
    fn GetGamma(&self) -> windows_core::Result<f64>;
    fn SetSharpness(&self, sharpness: f64) -> windows_core::Result<()>;
    fn GetSharpness(&self) -> windows_core::Result<f64>;
    fn SetSaturation(&self, saturation: f64) -> windows_core::Result<()>;
    fn GetSaturation(&self) -> windows_core::Result<f64>;
    fn SetTint(&self, tint: f64) -> windows_core::Result<()>;
    fn GetTint(&self) -> windows_core::Result<f64>;
    fn SetNoiseReduction(&self, noisereduction: f64) -> windows_core::Result<()>;
    fn GetNoiseReduction(&self) -> windows_core::Result<f64>;
    fn SetDestinationColorContext(&self, pcolorcontext: windows_core::Ref<IWICColorContext>) -> windows_core::Result<()>;
    fn SetToneCurve(&self, cbtonecurvesize: u32, ptonecurve: *const WICRawToneCurve) -> windows_core::Result<()>;
    fn GetToneCurve(&self, cbtonecurvebuffersize: u32, ptonecurve: *mut WICRawToneCurve, pcbactualtonecurvebuffersize: *mut u32) -> windows_core::Result<()>;
    fn SetRotation(&self, rotation: f64) -> windows_core::Result<()>;
    fn GetRotation(&self) -> windows_core::Result<f64>;
    fn SetRenderMode(&self, rendermode: WICRawRenderMode) -> windows_core::Result<()>;
    fn GetRenderMode(&self) -> windows_core::Result<WICRawRenderMode>;
    fn SetNotificationCallback(&self, pcallback: windows_core::Ref<IWICDevelopRawNotificationCallback>) -> windows_core::Result<()>;
}
#[cfg(feature = "ocidl")]
impl IWICDevelopRaw_Vtbl {
    pub const fn new<Identity: IWICDevelopRaw_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn QueryRawCapabilitiesInfo<Identity: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *mut WICRawCapabilitiesInfo) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICDevelopRaw_Impl::QueryRawCapabilitiesInfo(this, core::mem::transmute_copy(&pinfo)).into()
            }
        }
        unsafe extern "system" fn LoadParameterSet<Identity: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, parameterset: WICRawParameterSet) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICDevelopRaw_Impl::LoadParameterSet(this, core::mem::transmute_copy(&parameterset)).into()
            }
        }
        unsafe extern "system" fn GetCurrentParameterSet<Identity: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcurrentparameterset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICDevelopRaw_Impl::GetCurrentParameterSet(this) {
                    Ok(ok__) => {
                        ppcurrentparameterset.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetExposureCompensation<Identity: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ev: f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICDevelopRaw_Impl::SetExposureCompensation(this, core::mem::transmute_copy(&ev)).into()
            }
        }
        unsafe extern "system" fn GetExposureCompensation<Identity: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pev: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICDevelopRaw_Impl::GetExposureCompensation(this) {
                    Ok(ok__) => {
                        pev.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetWhitePointRGB<Identity: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, red: u32, green: u32, blue: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICDevelopRaw_Impl::SetWhitePointRGB(this, core::mem::transmute_copy(&red), core::mem::transmute_copy(&green), core::mem::transmute_copy(&blue)).into()
            }
        }
        unsafe extern "system" fn GetWhitePointRGB<Identity: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pred: *mut u32, pgreen: *mut u32, pblue: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICDevelopRaw_Impl::GetWhitePointRGB(this, core::mem::transmute_copy(&pred), core::mem::transmute_copy(&pgreen), core::mem::transmute_copy(&pblue)).into()
            }
        }
        unsafe extern "system" fn SetNamedWhitePoint<Identity: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, whitepoint: WICNamedWhitePoint) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICDevelopRaw_Impl::SetNamedWhitePoint(this, core::mem::transmute_copy(&whitepoint)).into()
            }
        }
        unsafe extern "system" fn GetNamedWhitePoint<Identity: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwhitepoint: *mut WICNamedWhitePoint) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICDevelopRaw_Impl::GetNamedWhitePoint(this) {
                    Ok(ok__) => {
                        pwhitepoint.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetWhitePointKelvin<Identity: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, whitepointkelvin: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICDevelopRaw_Impl::SetWhitePointKelvin(this, core::mem::transmute_copy(&whitepointkelvin)).into()
            }
        }
        unsafe extern "system" fn GetWhitePointKelvin<Identity: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwhitepointkelvin: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICDevelopRaw_Impl::GetWhitePointKelvin(this) {
                    Ok(ok__) => {
                        pwhitepointkelvin.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetKelvinRangeInfo<Identity: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pminkelvintemp: *mut u32, pmaxkelvintemp: *mut u32, pkelvintempstepvalue: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICDevelopRaw_Impl::GetKelvinRangeInfo(this, core::mem::transmute_copy(&pminkelvintemp), core::mem::transmute_copy(&pmaxkelvintemp), core::mem::transmute_copy(&pkelvintempstepvalue)).into()
            }
        }
        unsafe extern "system" fn SetContrast<Identity: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, contrast: f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICDevelopRaw_Impl::SetContrast(this, core::mem::transmute_copy(&contrast)).into()
            }
        }
        unsafe extern "system" fn GetContrast<Identity: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcontrast: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICDevelopRaw_Impl::GetContrast(this) {
                    Ok(ok__) => {
                        pcontrast.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetGamma<Identity: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, gamma: f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICDevelopRaw_Impl::SetGamma(this, core::mem::transmute_copy(&gamma)).into()
            }
        }
        unsafe extern "system" fn GetGamma<Identity: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pgamma: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICDevelopRaw_Impl::GetGamma(this) {
                    Ok(ok__) => {
                        pgamma.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSharpness<Identity: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sharpness: f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICDevelopRaw_Impl::SetSharpness(this, core::mem::transmute_copy(&sharpness)).into()
            }
        }
        unsafe extern "system" fn GetSharpness<Identity: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psharpness: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICDevelopRaw_Impl::GetSharpness(this) {
                    Ok(ok__) => {
                        psharpness.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSaturation<Identity: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, saturation: f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICDevelopRaw_Impl::SetSaturation(this, core::mem::transmute_copy(&saturation)).into()
            }
        }
        unsafe extern "system" fn GetSaturation<Identity: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psaturation: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICDevelopRaw_Impl::GetSaturation(this) {
                    Ok(ok__) => {
                        psaturation.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetTint<Identity: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, tint: f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICDevelopRaw_Impl::SetTint(this, core::mem::transmute_copy(&tint)).into()
            }
        }
        unsafe extern "system" fn GetTint<Identity: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptint: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICDevelopRaw_Impl::GetTint(this) {
                    Ok(ok__) => {
                        ptint.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetNoiseReduction<Identity: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, noisereduction: f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICDevelopRaw_Impl::SetNoiseReduction(this, core::mem::transmute_copy(&noisereduction)).into()
            }
        }
        unsafe extern "system" fn GetNoiseReduction<Identity: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnoisereduction: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICDevelopRaw_Impl::GetNoiseReduction(this) {
                    Ok(ok__) => {
                        pnoisereduction.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDestinationColorContext<Identity: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcolorcontext: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICDevelopRaw_Impl::SetDestinationColorContext(this, core::mem::transmute_copy(&pcolorcontext)).into()
            }
        }
        unsafe extern "system" fn SetToneCurve<Identity: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbtonecurvesize: u32, ptonecurve: *const WICRawToneCurve) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICDevelopRaw_Impl::SetToneCurve(this, core::mem::transmute_copy(&cbtonecurvesize), core::mem::transmute_copy(&ptonecurve)).into()
            }
        }
        unsafe extern "system" fn GetToneCurve<Identity: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbtonecurvebuffersize: u32, ptonecurve: *mut WICRawToneCurve, pcbactualtonecurvebuffersize: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICDevelopRaw_Impl::GetToneCurve(this, core::mem::transmute_copy(&cbtonecurvebuffersize), core::mem::transmute_copy(&ptonecurve), core::mem::transmute_copy(&pcbactualtonecurvebuffersize)).into()
            }
        }
        unsafe extern "system" fn SetRotation<Identity: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rotation: f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICDevelopRaw_Impl::SetRotation(this, core::mem::transmute_copy(&rotation)).into()
            }
        }
        unsafe extern "system" fn GetRotation<Identity: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, protation: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICDevelopRaw_Impl::GetRotation(this) {
                    Ok(ok__) => {
                        protation.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetRenderMode<Identity: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rendermode: WICRawRenderMode) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICDevelopRaw_Impl::SetRenderMode(this, core::mem::transmute_copy(&rendermode)).into()
            }
        }
        unsafe extern "system" fn GetRenderMode<Identity: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prendermode: *mut WICRawRenderMode) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICDevelopRaw_Impl::GetRenderMode(this) {
                    Ok(ok__) => {
                        prendermode.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetNotificationCallback<Identity: IWICDevelopRaw_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcallback: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICDevelopRaw_Impl::SetNotificationCallback(this, core::mem::transmute_copy(&pcallback)).into()
            }
        }
        Self {
            base__: IWICBitmapFrameDecode_Vtbl::new::<Identity, OFFSET>(),
            QueryRawCapabilitiesInfo: QueryRawCapabilitiesInfo::<Identity, OFFSET>,
            LoadParameterSet: LoadParameterSet::<Identity, OFFSET>,
            GetCurrentParameterSet: GetCurrentParameterSet::<Identity, OFFSET>,
            SetExposureCompensation: SetExposureCompensation::<Identity, OFFSET>,
            GetExposureCompensation: GetExposureCompensation::<Identity, OFFSET>,
            SetWhitePointRGB: SetWhitePointRGB::<Identity, OFFSET>,
            GetWhitePointRGB: GetWhitePointRGB::<Identity, OFFSET>,
            SetNamedWhitePoint: SetNamedWhitePoint::<Identity, OFFSET>,
            GetNamedWhitePoint: GetNamedWhitePoint::<Identity, OFFSET>,
            SetWhitePointKelvin: SetWhitePointKelvin::<Identity, OFFSET>,
            GetWhitePointKelvin: GetWhitePointKelvin::<Identity, OFFSET>,
            GetKelvinRangeInfo: GetKelvinRangeInfo::<Identity, OFFSET>,
            SetContrast: SetContrast::<Identity, OFFSET>,
            GetContrast: GetContrast::<Identity, OFFSET>,
            SetGamma: SetGamma::<Identity, OFFSET>,
            GetGamma: GetGamma::<Identity, OFFSET>,
            SetSharpness: SetSharpness::<Identity, OFFSET>,
            GetSharpness: GetSharpness::<Identity, OFFSET>,
            SetSaturation: SetSaturation::<Identity, OFFSET>,
            GetSaturation: GetSaturation::<Identity, OFFSET>,
            SetTint: SetTint::<Identity, OFFSET>,
            GetTint: GetTint::<Identity, OFFSET>,
            SetNoiseReduction: SetNoiseReduction::<Identity, OFFSET>,
            GetNoiseReduction: GetNoiseReduction::<Identity, OFFSET>,
            SetDestinationColorContext: SetDestinationColorContext::<Identity, OFFSET>,
            SetToneCurve: SetToneCurve::<Identity, OFFSET>,
            GetToneCurve: GetToneCurve::<Identity, OFFSET>,
            SetRotation: SetRotation::<Identity, OFFSET>,
            GetRotation: GetRotation::<Identity, OFFSET>,
            SetRenderMode: SetRenderMode::<Identity, OFFSET>,
            GetRenderMode: GetRenderMode::<Identity, OFFSET>,
            SetNotificationCallback: SetNotificationCallback::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWICDevelopRaw as windows_core::Interface>::IID || iid == &<IWICBitmapSource as windows_core::Interface>::IID || iid == &<IWICBitmapFrameDecode as windows_core::Interface>::IID
    }
}
#[cfg(feature = "ocidl")]
impl windows_core::RuntimeName for IWICDevelopRaw {}
windows_core::imp::define_interface!(IWICDevelopRawNotificationCallback, IWICDevelopRawNotificationCallback_Vtbl, 0x95c75a6e_3e8c_4ec2_85a8_aebcc551e59b);
windows_core::imp::interface_hierarchy!(IWICDevelopRawNotificationCallback, windows_core::IUnknown);
impl IWICDevelopRawNotificationCallback {
    pub unsafe fn Notify(&self, notificationmask: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Notify)(windows_core::Interface::as_raw(self), notificationmask) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICDevelopRawNotificationCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Notify: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait IWICDevelopRawNotificationCallback_Impl: windows_core::IUnknownImpl {
    fn Notify(&self, notificationmask: u32) -> windows_core::Result<()>;
}
impl IWICDevelopRawNotificationCallback_Vtbl {
    pub const fn new<Identity: IWICDevelopRawNotificationCallback_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Notify<Identity: IWICDevelopRawNotificationCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, notificationmask: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICDevelopRawNotificationCallback_Impl::Notify(this, core::mem::transmute_copy(&notificationmask)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Notify: Notify::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWICDevelopRawNotificationCallback as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWICDevelopRawNotificationCallback {}
windows_core::imp::define_interface!(IWICDisplayAdaptationControl, IWICDisplayAdaptationControl_Vtbl, 0xde9d91d2_70b4_4f41_836c_25fcd39626d3);
windows_core::imp::interface_hierarchy!(IWICDisplayAdaptationControl, windows_core::IUnknown);
impl IWICDisplayAdaptationControl {
    pub unsafe fn DoesSupportChangingMaxLuminance(&self, pguiddstformat: *const WICPixelFormatGUID) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DoesSupportChangingMaxLuminance)(windows_core::Interface::as_raw(self), pguiddstformat, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetDisplayMaxLuminance(&self, fluminanceinnits: f32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetDisplayMaxLuminance)(windows_core::Interface::as_raw(self), fluminanceinnits) }
    }
    pub unsafe fn GetDisplayMaxLuminance(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDisplayMaxLuminance)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICDisplayAdaptationControl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub DoesSupportChangingMaxLuminance: unsafe extern "system" fn(*mut core::ffi::c_void, *const WICPixelFormatGUID, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub SetDisplayMaxLuminance: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub GetDisplayMaxLuminance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
}
pub trait IWICDisplayAdaptationControl_Impl: windows_core::IUnknownImpl {
    fn DoesSupportChangingMaxLuminance(&self, pguiddstformat: *const WICPixelFormatGUID) -> windows_core::Result<windows_core::BOOL>;
    fn SetDisplayMaxLuminance(&self, fluminanceinnits: f32) -> windows_core::Result<()>;
    fn GetDisplayMaxLuminance(&self) -> windows_core::Result<f32>;
}
impl IWICDisplayAdaptationControl_Vtbl {
    pub const fn new<Identity: IWICDisplayAdaptationControl_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn DoesSupportChangingMaxLuminance<Identity: IWICDisplayAdaptationControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguiddstformat: *const WICPixelFormatGUID, pfissupported: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICDisplayAdaptationControl_Impl::DoesSupportChangingMaxLuminance(this, core::mem::transmute_copy(&pguiddstformat)) {
                    Ok(ok__) => {
                        pfissupported.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDisplayMaxLuminance<Identity: IWICDisplayAdaptationControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fluminanceinnits: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICDisplayAdaptationControl_Impl::SetDisplayMaxLuminance(this, core::mem::transmute_copy(&fluminanceinnits)).into()
            }
        }
        unsafe extern "system" fn GetDisplayMaxLuminance<Identity: IWICDisplayAdaptationControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfluminanceinnits: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICDisplayAdaptationControl_Impl::GetDisplayMaxLuminance(this) {
                    Ok(ok__) => {
                        pfluminanceinnits.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            DoesSupportChangingMaxLuminance: DoesSupportChangingMaxLuminance::<Identity, OFFSET>,
            SetDisplayMaxLuminance: SetDisplayMaxLuminance::<Identity, OFFSET>,
            GetDisplayMaxLuminance: GetDisplayMaxLuminance::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWICDisplayAdaptationControl as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWICDisplayAdaptationControl {}
windows_core::imp::define_interface!(IWICDisplayAdaptationControl2, IWICDisplayAdaptationControl2_Vtbl, 0xd7508d29_3ab7_447e_a676_4d80d7de726b);
impl core::ops::Deref for IWICDisplayAdaptationControl2 {
    type Target = IWICDisplayAdaptationControl;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWICDisplayAdaptationControl2, windows_core::IUnknown, IWICDisplayAdaptationControl);
impl IWICDisplayAdaptationControl2 {
    pub unsafe fn SetSdrWhiteLevel(&self, fwhitelevelinnits: f32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSdrWhiteLevel)(windows_core::Interface::as_raw(self), fwhitelevelinnits) }
    }
    pub unsafe fn GetSdrWhiteLevel(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSdrWhiteLevel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetToneMappingMode(&self, mode: WICBitmapToneMappingMode) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetToneMappingMode)(windows_core::Interface::as_raw(self), mode) }
    }
    pub unsafe fn GetToneMappingMode(&self) -> windows_core::Result<WICBitmapToneMappingMode> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetToneMappingMode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn DoesSupportToneMappingMode(&self, mode: WICBitmapToneMappingMode) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DoesSupportToneMappingMode)(windows_core::Interface::as_raw(self), mode, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICDisplayAdaptationControl2_Vtbl {
    pub base__: IWICDisplayAdaptationControl_Vtbl,
    pub SetSdrWhiteLevel: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub GetSdrWhiteLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetToneMappingMode: unsafe extern "system" fn(*mut core::ffi::c_void, WICBitmapToneMappingMode) -> windows_core::HRESULT,
    pub GetToneMappingMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WICBitmapToneMappingMode) -> windows_core::HRESULT,
    pub DoesSupportToneMappingMode: unsafe extern "system" fn(*mut core::ffi::c_void, WICBitmapToneMappingMode, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IWICDisplayAdaptationControl2_Impl: IWICDisplayAdaptationControl_Impl {
    fn SetSdrWhiteLevel(&self, fwhitelevelinnits: f32) -> windows_core::Result<()>;
    fn GetSdrWhiteLevel(&self) -> windows_core::Result<f32>;
    fn SetToneMappingMode(&self, mode: WICBitmapToneMappingMode) -> windows_core::Result<()>;
    fn GetToneMappingMode(&self) -> windows_core::Result<WICBitmapToneMappingMode>;
    fn DoesSupportToneMappingMode(&self, mode: WICBitmapToneMappingMode) -> windows_core::Result<windows_core::BOOL>;
}
impl IWICDisplayAdaptationControl2_Vtbl {
    pub const fn new<Identity: IWICDisplayAdaptationControl2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetSdrWhiteLevel<Identity: IWICDisplayAdaptationControl2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fwhitelevelinnits: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICDisplayAdaptationControl2_Impl::SetSdrWhiteLevel(this, core::mem::transmute_copy(&fwhitelevelinnits)).into()
            }
        }
        unsafe extern "system" fn GetSdrWhiteLevel<Identity: IWICDisplayAdaptationControl2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfwhitelevelinnits: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICDisplayAdaptationControl2_Impl::GetSdrWhiteLevel(this) {
                    Ok(ok__) => {
                        pfwhitelevelinnits.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetToneMappingMode<Identity: IWICDisplayAdaptationControl2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mode: WICBitmapToneMappingMode) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICDisplayAdaptationControl2_Impl::SetToneMappingMode(this, core::mem::transmute_copy(&mode)).into()
            }
        }
        unsafe extern "system" fn GetToneMappingMode<Identity: IWICDisplayAdaptationControl2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mode: *mut WICBitmapToneMappingMode) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICDisplayAdaptationControl2_Impl::GetToneMappingMode(this) {
                    Ok(ok__) => {
                        mode.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DoesSupportToneMappingMode<Identity: IWICDisplayAdaptationControl2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mode: WICBitmapToneMappingMode, pfissupported: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICDisplayAdaptationControl2_Impl::DoesSupportToneMappingMode(this, core::mem::transmute_copy(&mode)) {
                    Ok(ok__) => {
                        pfissupported.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IWICDisplayAdaptationControl_Vtbl::new::<Identity, OFFSET>(),
            SetSdrWhiteLevel: SetSdrWhiteLevel::<Identity, OFFSET>,
            GetSdrWhiteLevel: GetSdrWhiteLevel::<Identity, OFFSET>,
            SetToneMappingMode: SetToneMappingMode::<Identity, OFFSET>,
            GetToneMappingMode: GetToneMappingMode::<Identity, OFFSET>,
            DoesSupportToneMappingMode: DoesSupportToneMappingMode::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWICDisplayAdaptationControl2 as windows_core::Interface>::IID || iid == &<IWICDisplayAdaptationControl as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWICDisplayAdaptationControl2 {}
windows_core::imp::define_interface!(IWICEnumMetadataItem, IWICEnumMetadataItem_Vtbl, 0xdc2bb46d_3f07_481e_8625_220c4aedbb33);
windows_core::imp::interface_hierarchy!(IWICEnumMetadataItem, windows_core::IUnknown);
impl IWICEnumMetadataItem {
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn Next(&self, celt: u32, rgeltschema: *mut super::propidlbase::PROPVARIANT, rgeltid: *mut super::propidlbase::PROPVARIANT, rgeltvalue: Option<*mut super::propidlbase::PROPVARIANT>, pceltfetched: Option<*mut u32>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), celt, core::mem::transmute(rgeltschema), core::mem::transmute(rgeltid), rgeltvalue.unwrap_or(core::mem::zeroed()) as _, pceltfetched.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt) }
    }
    pub unsafe fn Reset(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICEnumMetadataItem_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut super::propidlbase::PROPVARIANT, *mut super::propidlbase::PROPVARIANT, *mut super::propidlbase::PROPVARIANT, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase")))]
    Next: usize,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
pub trait IWICEnumMetadataItem_Impl: windows_core::IUnknownImpl {
    fn Next(&self, celt: u32, rgeltschema: *mut super::propidlbase::PROPVARIANT, rgeltid: *mut super::propidlbase::PROPVARIANT, rgeltvalue: *mut super::propidlbase::PROPVARIANT, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IWICEnumMetadataItem>;
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
impl IWICEnumMetadataItem_Vtbl {
    pub const fn new<Identity: IWICEnumMetadataItem_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IWICEnumMetadataItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgeltschema: *mut super::propidlbase::PROPVARIANT, rgeltid: *mut super::propidlbase::PROPVARIANT, rgeltvalue: *mut super::propidlbase::PROPVARIANT, pceltfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICEnumMetadataItem_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgeltschema), core::mem::transmute_copy(&rgeltid), core::mem::transmute_copy(&rgeltvalue), core::mem::transmute_copy(&pceltfetched)).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IWICEnumMetadataItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICEnumMetadataItem_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IWICEnumMetadataItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICEnumMetadataItem_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IWICEnumMetadataItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppienummetadataitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICEnumMetadataItem_Impl::Clone(this) {
                    Ok(ok__) => {
                        ppienummetadataitem.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWICEnumMetadataItem as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IWICEnumMetadataItem {}
windows_core::imp::define_interface!(IWICFastMetadataEncoder, IWICFastMetadataEncoder_Vtbl, 0xb84e2c09_78c9_4ac4_8bd3_524ae1663a2f);
windows_core::imp::interface_hierarchy!(IWICFastMetadataEncoder, windows_core::IUnknown);
impl IWICFastMetadataEncoder {
    pub unsafe fn Commit(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Commit)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetMetadataQueryWriter(&self) -> windows_core::Result<IWICMetadataQueryWriter> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMetadataQueryWriter)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICFastMetadataEncoder_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Commit: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetMetadataQueryWriter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWICFastMetadataEncoder_Impl: windows_core::IUnknownImpl {
    fn Commit(&self) -> windows_core::Result<()>;
    fn GetMetadataQueryWriter(&self) -> windows_core::Result<IWICMetadataQueryWriter>;
}
impl IWICFastMetadataEncoder_Vtbl {
    pub const fn new<Identity: IWICFastMetadataEncoder_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Commit<Identity: IWICFastMetadataEncoder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICFastMetadataEncoder_Impl::Commit(this).into()
            }
        }
        unsafe extern "system" fn GetMetadataQueryWriter<Identity: IWICFastMetadataEncoder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppimetadataquerywriter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICFastMetadataEncoder_Impl::GetMetadataQueryWriter(this) {
                    Ok(ok__) => {
                        ppimetadataquerywriter.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Commit: Commit::<Identity, OFFSET>,
            GetMetadataQueryWriter: GetMetadataQueryWriter::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWICFastMetadataEncoder as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWICFastMetadataEncoder {}
windows_core::imp::define_interface!(IWICFormatConverter, IWICFormatConverter_Vtbl, 0x00000301_a8f2_4877_ba0a_fd2b6645fb94);
impl core::ops::Deref for IWICFormatConverter {
    type Target = IWICBitmapSource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWICFormatConverter, windows_core::IUnknown, IWICBitmapSource);
impl IWICFormatConverter {
    pub unsafe fn Initialize<P0, P3>(&self, pisource: P0, dstformat: REFWICPixelFormatGUID, dither: WICBitmapDitherType, pipalette: P3, alphathresholdpercent: f64, palettetranslate: WICBitmapPaletteType) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IWICBitmapSource>,
        P3: windows_core::Param<IWICPalette>,
    {
        unsafe { (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), pisource.param().abi(), dstformat, dither, pipalette.param().abi(), alphathresholdpercent, palettetranslate) }
    }
    pub unsafe fn CanConvert(&self, srcpixelformat: REFWICPixelFormatGUID, dstpixelformat: REFWICPixelFormatGUID) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CanConvert)(windows_core::Interface::as_raw(self), srcpixelformat, dstpixelformat, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICFormatConverter_Vtbl {
    pub base__: IWICBitmapSource_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, REFWICPixelFormatGUID, WICBitmapDitherType, *mut core::ffi::c_void, f64, WICBitmapPaletteType) -> windows_core::HRESULT,
    pub CanConvert: unsafe extern "system" fn(*mut core::ffi::c_void, REFWICPixelFormatGUID, REFWICPixelFormatGUID, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IWICFormatConverter_Impl: IWICBitmapSource_Impl {
    fn Initialize(&self, pisource: windows_core::Ref<IWICBitmapSource>, dstformat: REFWICPixelFormatGUID, dither: WICBitmapDitherType, pipalette: windows_core::Ref<IWICPalette>, alphathresholdpercent: f64, palettetranslate: WICBitmapPaletteType) -> windows_core::Result<()>;
    fn CanConvert(&self, srcpixelformat: REFWICPixelFormatGUID, dstpixelformat: REFWICPixelFormatGUID) -> windows_core::Result<windows_core::BOOL>;
}
impl IWICFormatConverter_Vtbl {
    pub const fn new<Identity: IWICFormatConverter_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Initialize<Identity: IWICFormatConverter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisource: *mut core::ffi::c_void, dstformat: REFWICPixelFormatGUID, dither: WICBitmapDitherType, pipalette: *mut core::ffi::c_void, alphathresholdpercent: f64, palettetranslate: WICBitmapPaletteType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICFormatConverter_Impl::Initialize(this, core::mem::transmute_copy(&pisource), core::mem::transmute_copy(&dstformat), core::mem::transmute_copy(&dither), core::mem::transmute_copy(&pipalette), core::mem::transmute_copy(&alphathresholdpercent), core::mem::transmute_copy(&palettetranslate)).into()
            }
        }
        unsafe extern "system" fn CanConvert<Identity: IWICFormatConverter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, srcpixelformat: REFWICPixelFormatGUID, dstpixelformat: REFWICPixelFormatGUID, pfcanconvert: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICFormatConverter_Impl::CanConvert(this, core::mem::transmute_copy(&srcpixelformat), core::mem::transmute_copy(&dstpixelformat)) {
                    Ok(ok__) => {
                        pfcanconvert.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: IWICBitmapSource_Vtbl::new::<Identity, OFFSET>(), Initialize: Initialize::<Identity, OFFSET>, CanConvert: CanConvert::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWICFormatConverter as windows_core::Interface>::IID || iid == &<IWICBitmapSource as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWICFormatConverter {}
windows_core::imp::define_interface!(IWICFormatConverterInfo, IWICFormatConverterInfo_Vtbl, 0x9f34fb65_13f4_4f15_bc57_3726b5e53d9f);
impl core::ops::Deref for IWICFormatConverterInfo {
    type Target = IWICComponentInfo;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWICFormatConverterInfo, windows_core::IUnknown, IWICComponentInfo);
impl IWICFormatConverterInfo {
    pub unsafe fn GetPixelFormats(&self, cformats: u32, ppixelformatguids: *mut WICPixelFormatGUID, pcactual: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPixelFormats)(windows_core::Interface::as_raw(self), cformats, ppixelformatguids as _, pcactual as _) }
    }
    pub unsafe fn CreateInstance(&self) -> windows_core::Result<IWICFormatConverter> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateInstance)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICFormatConverterInfo_Vtbl {
    pub base__: IWICComponentInfo_Vtbl,
    pub GetPixelFormats: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut WICPixelFormatGUID, *mut u32) -> windows_core::HRESULT,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWICFormatConverterInfo_Impl: IWICComponentInfo_Impl {
    fn GetPixelFormats(&self, cformats: u32, ppixelformatguids: *mut WICPixelFormatGUID, pcactual: *mut u32) -> windows_core::Result<()>;
    fn CreateInstance(&self) -> windows_core::Result<IWICFormatConverter>;
}
impl IWICFormatConverterInfo_Vtbl {
    pub const fn new<Identity: IWICFormatConverterInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetPixelFormats<Identity: IWICFormatConverterInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cformats: u32, ppixelformatguids: *mut WICPixelFormatGUID, pcactual: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICFormatConverterInfo_Impl::GetPixelFormats(this, core::mem::transmute_copy(&cformats), core::mem::transmute_copy(&ppixelformatguids), core::mem::transmute_copy(&pcactual)).into()
            }
        }
        unsafe extern "system" fn CreateInstance<Identity: IWICFormatConverterInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppiconverter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICFormatConverterInfo_Impl::CreateInstance(this) {
                    Ok(ok__) => {
                        ppiconverter.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IWICComponentInfo_Vtbl::new::<Identity, OFFSET>(),
            GetPixelFormats: GetPixelFormats::<Identity, OFFSET>,
            CreateInstance: CreateInstance::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWICFormatConverterInfo as windows_core::Interface>::IID || iid == &<IWICComponentInfo as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWICFormatConverterInfo {}
windows_core::imp::define_interface!(IWICImageEncoder, IWICImageEncoder_Vtbl, 0x04c75bf8_3ce1_473b_acc5_3cc4f5e94999);
windows_core::imp::interface_hierarchy!(IWICImageEncoder, windows_core::IUnknown);
impl IWICImageEncoder {
    #[cfg(all(feature = "d2d", feature = "dcommon", feature = "dxgi"))]
    pub unsafe fn WriteFrame<P0, P1>(&self, pimage: P0, pframeencode: P1, pimageparameters: *const WICImageParameters) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::d2d::ID2D1Image>,
        P1: windows_core::Param<IWICBitmapFrameEncode>,
    {
        unsafe { (windows_core::Interface::vtable(self).WriteFrame)(windows_core::Interface::as_raw(self), pimage.param().abi(), pframeencode.param().abi(), pimageparameters) }
    }
    #[cfg(all(feature = "d2d", feature = "dcommon", feature = "dxgi"))]
    pub unsafe fn WriteFrameThumbnail<P0, P1>(&self, pimage: P0, pframeencode: P1, pimageparameters: *const WICImageParameters) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::d2d::ID2D1Image>,
        P1: windows_core::Param<IWICBitmapFrameEncode>,
    {
        unsafe { (windows_core::Interface::vtable(self).WriteFrameThumbnail)(windows_core::Interface::as_raw(self), pimage.param().abi(), pframeencode.param().abi(), pimageparameters) }
    }
    #[cfg(all(feature = "d2d", feature = "dcommon", feature = "dxgi"))]
    pub unsafe fn WriteThumbnail<P0, P1>(&self, pimage: P0, pencoder: P1, pimageparameters: *const WICImageParameters) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::d2d::ID2D1Image>,
        P1: windows_core::Param<IWICBitmapEncoder>,
    {
        unsafe { (windows_core::Interface::vtable(self).WriteThumbnail)(windows_core::Interface::as_raw(self), pimage.param().abi(), pencoder.param().abi(), pimageparameters) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICImageEncoder_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "d2d", feature = "dcommon", feature = "dxgi"))]
    pub WriteFrame: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *const WICImageParameters) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "d2d", feature = "dcommon", feature = "dxgi")))]
    WriteFrame: usize,
    #[cfg(all(feature = "d2d", feature = "dcommon", feature = "dxgi"))]
    pub WriteFrameThumbnail: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *const WICImageParameters) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "d2d", feature = "dcommon", feature = "dxgi")))]
    WriteFrameThumbnail: usize,
    #[cfg(all(feature = "d2d", feature = "dcommon", feature = "dxgi"))]
    pub WriteThumbnail: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *const WICImageParameters) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "d2d", feature = "dcommon", feature = "dxgi")))]
    WriteThumbnail: usize,
}
#[cfg(all(feature = "d2d", feature = "dcommon", feature = "dxgi"))]
pub trait IWICImageEncoder_Impl: windows_core::IUnknownImpl {
    fn WriteFrame(&self, pimage: windows_core::Ref<super::d2d::ID2D1Image>, pframeencode: windows_core::Ref<IWICBitmapFrameEncode>, pimageparameters: *const WICImageParameters) -> windows_core::Result<()>;
    fn WriteFrameThumbnail(&self, pimage: windows_core::Ref<super::d2d::ID2D1Image>, pframeencode: windows_core::Ref<IWICBitmapFrameEncode>, pimageparameters: *const WICImageParameters) -> windows_core::Result<()>;
    fn WriteThumbnail(&self, pimage: windows_core::Ref<super::d2d::ID2D1Image>, pencoder: windows_core::Ref<IWICBitmapEncoder>, pimageparameters: *const WICImageParameters) -> windows_core::Result<()>;
}
#[cfg(all(feature = "d2d", feature = "dcommon", feature = "dxgi"))]
impl IWICImageEncoder_Vtbl {
    pub const fn new<Identity: IWICImageEncoder_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn WriteFrame<Identity: IWICImageEncoder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pimage: *mut core::ffi::c_void, pframeencode: *mut core::ffi::c_void, pimageparameters: *const WICImageParameters) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICImageEncoder_Impl::WriteFrame(this, core::mem::transmute_copy(&pimage), core::mem::transmute_copy(&pframeencode), core::mem::transmute_copy(&pimageparameters)).into()
            }
        }
        unsafe extern "system" fn WriteFrameThumbnail<Identity: IWICImageEncoder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pimage: *mut core::ffi::c_void, pframeencode: *mut core::ffi::c_void, pimageparameters: *const WICImageParameters) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICImageEncoder_Impl::WriteFrameThumbnail(this, core::mem::transmute_copy(&pimage), core::mem::transmute_copy(&pframeencode), core::mem::transmute_copy(&pimageparameters)).into()
            }
        }
        unsafe extern "system" fn WriteThumbnail<Identity: IWICImageEncoder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pimage: *mut core::ffi::c_void, pencoder: *mut core::ffi::c_void, pimageparameters: *const WICImageParameters) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICImageEncoder_Impl::WriteThumbnail(this, core::mem::transmute_copy(&pimage), core::mem::transmute_copy(&pencoder), core::mem::transmute_copy(&pimageparameters)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            WriteFrame: WriteFrame::<Identity, OFFSET>,
            WriteFrameThumbnail: WriteFrameThumbnail::<Identity, OFFSET>,
            WriteThumbnail: WriteThumbnail::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWICImageEncoder as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "d2d", feature = "dcommon", feature = "dxgi"))]
impl windows_core::RuntimeName for IWICImageEncoder {}
windows_core::imp::define_interface!(IWICImagingFactory, IWICImagingFactory_Vtbl, 0xec5ec8a9_c395_4314_9c77_54d7a935ff70);
windows_core::imp::interface_hierarchy!(IWICImagingFactory, windows_core::IUnknown);
impl IWICImagingFactory {
    pub unsafe fn CreateDecoderFromFilename<P0>(&self, wzfilename: P0, pguidvendor: *const windows_core::GUID, dwdesiredaccess: u32, metadataoptions: WICDecodeOptions) -> windows_core::Result<IWICBitmapDecoder>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateDecoderFromFilename)(windows_core::Interface::as_raw(self), wzfilename.param().abi(), pguidvendor, dwdesiredaccess, metadataoptions, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "objidlbase")]
    pub unsafe fn CreateDecoderFromStream<P0>(&self, pistream: P0, pguidvendor: *const windows_core::GUID, metadataoptions: WICDecodeOptions) -> windows_core::Result<IWICBitmapDecoder>
    where
        P0: windows_core::Param<super::objidlbase::IStream>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateDecoderFromStream)(windows_core::Interface::as_raw(self), pistream.param().abi(), pguidvendor, metadataoptions, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateDecoderFromFileHandle(&self, hfile: usize, pguidvendor: *const windows_core::GUID, metadataoptions: WICDecodeOptions) -> windows_core::Result<IWICBitmapDecoder> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateDecoderFromFileHandle)(windows_core::Interface::as_raw(self), hfile, pguidvendor, metadataoptions, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateComponentInfo(&self, clsidcomponent: *const windows_core::GUID) -> windows_core::Result<IWICComponentInfo> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateComponentInfo)(windows_core::Interface::as_raw(self), clsidcomponent, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateDecoder(&self, guidcontainerformat: *const windows_core::GUID, pguidvendor: *const windows_core::GUID) -> windows_core::Result<IWICBitmapDecoder> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateDecoder)(windows_core::Interface::as_raw(self), guidcontainerformat, pguidvendor, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateEncoder(&self, guidcontainerformat: *const windows_core::GUID, pguidvendor: *const windows_core::GUID) -> windows_core::Result<IWICBitmapEncoder> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateEncoder)(windows_core::Interface::as_raw(self), guidcontainerformat, pguidvendor, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreatePalette(&self) -> windows_core::Result<IWICPalette> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreatePalette)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateFormatConverter(&self) -> windows_core::Result<IWICFormatConverter> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateFormatConverter)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateBitmapScaler(&self) -> windows_core::Result<IWICBitmapScaler> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateBitmapScaler)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateBitmapClipper(&self) -> windows_core::Result<IWICBitmapClipper> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateBitmapClipper)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateBitmapFlipRotator(&self) -> windows_core::Result<IWICBitmapFlipRotator> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateBitmapFlipRotator)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "objidlbase")]
    pub unsafe fn CreateStream(&self) -> windows_core::Result<IWICStream> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateStream)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateColorContext(&self) -> windows_core::Result<IWICColorContext> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateColorContext)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateColorTransformer(&self) -> windows_core::Result<IWICColorTransform> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateColorTransformer)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateBitmap(&self, uiwidth: u32, uiheight: u32, pixelformat: REFWICPixelFormatGUID, option: WICBitmapCreateCacheOption) -> windows_core::Result<IWICBitmap> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateBitmap)(windows_core::Interface::as_raw(self), uiwidth, uiheight, pixelformat, option, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateBitmapFromSource<P0>(&self, pibitmapsource: P0, option: WICBitmapCreateCacheOption) -> windows_core::Result<IWICBitmap>
    where
        P0: windows_core::Param<IWICBitmapSource>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateBitmapFromSource)(windows_core::Interface::as_raw(self), pibitmapsource.param().abi(), option, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateBitmapFromSourceRect<P0>(&self, pibitmapsource: P0, x: u32, y: u32, width: u32, height: u32) -> windows_core::Result<IWICBitmap>
    where
        P0: windows_core::Param<IWICBitmapSource>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateBitmapFromSourceRect)(windows_core::Interface::as_raw(self), pibitmapsource.param().abi(), x, y, width, height, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateBitmapFromMemory(&self, uiwidth: u32, uiheight: u32, pixelformat: REFWICPixelFormatGUID, cbstride: u32, cbbuffersize: u32, pbbuffer: *const u8) -> windows_core::Result<IWICBitmap> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateBitmapFromMemory)(windows_core::Interface::as_raw(self), uiwidth, uiheight, pixelformat, cbstride, cbbuffersize, pbbuffer, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn CreateBitmapFromHBITMAP(&self, hbitmap: super::windef::HBITMAP, hpalette: super::windef::HPALETTE, options: WICBitmapAlphaChannelOption) -> windows_core::Result<IWICBitmap> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateBitmapFromHBITMAP)(windows_core::Interface::as_raw(self), hbitmap, hpalette, options, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn CreateBitmapFromHICON(&self, hicon: super::windef::HICON) -> windows_core::Result<IWICBitmap> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateBitmapFromHICON)(windows_core::Interface::as_raw(self), hicon, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "objidlbase")]
    pub unsafe fn CreateComponentEnumerator(&self, componenttypes: u32, options: u32) -> windows_core::Result<super::objidlbase::IEnumUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateComponentEnumerator)(windows_core::Interface::as_raw(self), componenttypes, options, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateFastMetadataEncoderFromDecoder<P0>(&self, pidecoder: P0) -> windows_core::Result<IWICFastMetadataEncoder>
    where
        P0: windows_core::Param<IWICBitmapDecoder>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateFastMetadataEncoderFromDecoder)(windows_core::Interface::as_raw(self), pidecoder.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateFastMetadataEncoderFromFrameDecode<P0>(&self, piframedecoder: P0) -> windows_core::Result<IWICFastMetadataEncoder>
    where
        P0: windows_core::Param<IWICBitmapFrameDecode>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateFastMetadataEncoderFromFrameDecode)(windows_core::Interface::as_raw(self), piframedecoder.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateQueryWriter(&self, guidmetadataformat: *const windows_core::GUID, pguidvendor: *const windows_core::GUID) -> windows_core::Result<IWICMetadataQueryWriter> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateQueryWriter)(windows_core::Interface::as_raw(self), guidmetadataformat, pguidvendor, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateQueryWriterFromReader<P0>(&self, piqueryreader: P0, pguidvendor: *const windows_core::GUID) -> windows_core::Result<IWICMetadataQueryWriter>
    where
        P0: windows_core::Param<IWICMetadataQueryReader>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateQueryWriterFromReader)(windows_core::Interface::as_raw(self), piqueryreader.param().abi(), pguidvendor, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICImagingFactory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateDecoderFromFilename: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const windows_core::GUID, u32, WICDecodeOptions, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "objidlbase")]
    pub CreateDecoderFromStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_core::GUID, WICDecodeOptions, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidlbase"))]
    CreateDecoderFromStream: usize,
    pub CreateDecoderFromFileHandle: unsafe extern "system" fn(*mut core::ffi::c_void, usize, *const windows_core::GUID, WICDecodeOptions, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateComponentInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateDecoder: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateEncoder: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreatePalette: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateFormatConverter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateBitmapScaler: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateBitmapClipper: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateBitmapFlipRotator: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "objidlbase")]
    pub CreateStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidlbase"))]
    CreateStream: usize,
    pub CreateColorContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateColorTransformer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateBitmap: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, REFWICPixelFormatGUID, WICBitmapCreateCacheOption, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateBitmapFromSource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, WICBitmapCreateCacheOption, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateBitmapFromSourceRect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32, u32, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateBitmapFromMemory: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, REFWICPixelFormatGUID, u32, u32, *const u8, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "windef")]
    pub CreateBitmapFromHBITMAP: unsafe extern "system" fn(*mut core::ffi::c_void, super::windef::HBITMAP, super::windef::HPALETTE, WICBitmapAlphaChannelOption, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    CreateBitmapFromHBITMAP: usize,
    #[cfg(feature = "windef")]
    pub CreateBitmapFromHICON: unsafe extern "system" fn(*mut core::ffi::c_void, super::windef::HICON, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    CreateBitmapFromHICON: usize,
    #[cfg(feature = "objidlbase")]
    pub CreateComponentEnumerator: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidlbase"))]
    CreateComponentEnumerator: usize,
    pub CreateFastMetadataEncoderFromDecoder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateFastMetadataEncoderFromFrameDecode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateQueryWriter: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateQueryWriterFromReader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "objidlbase", feature = "windef"))]
pub trait IWICImagingFactory_Impl: windows_core::IUnknownImpl {
    fn CreateDecoderFromFilename(&self, wzfilename: &windows_core::PCWSTR, pguidvendor: *const windows_core::GUID, dwdesiredaccess: u32, metadataoptions: WICDecodeOptions) -> windows_core::Result<IWICBitmapDecoder>;
    fn CreateDecoderFromStream(&self, pistream: windows_core::Ref<super::objidlbase::IStream>, pguidvendor: *const windows_core::GUID, metadataoptions: WICDecodeOptions) -> windows_core::Result<IWICBitmapDecoder>;
    fn CreateDecoderFromFileHandle(&self, hfile: usize, pguidvendor: *const windows_core::GUID, metadataoptions: WICDecodeOptions) -> windows_core::Result<IWICBitmapDecoder>;
    fn CreateComponentInfo(&self, clsidcomponent: *const windows_core::GUID) -> windows_core::Result<IWICComponentInfo>;
    fn CreateDecoder(&self, guidcontainerformat: *const windows_core::GUID, pguidvendor: *const windows_core::GUID) -> windows_core::Result<IWICBitmapDecoder>;
    fn CreateEncoder(&self, guidcontainerformat: *const windows_core::GUID, pguidvendor: *const windows_core::GUID) -> windows_core::Result<IWICBitmapEncoder>;
    fn CreatePalette(&self) -> windows_core::Result<IWICPalette>;
    fn CreateFormatConverter(&self) -> windows_core::Result<IWICFormatConverter>;
    fn CreateBitmapScaler(&self) -> windows_core::Result<IWICBitmapScaler>;
    fn CreateBitmapClipper(&self) -> windows_core::Result<IWICBitmapClipper>;
    fn CreateBitmapFlipRotator(&self) -> windows_core::Result<IWICBitmapFlipRotator>;
    fn CreateStream(&self) -> windows_core::Result<IWICStream>;
    fn CreateColorContext(&self) -> windows_core::Result<IWICColorContext>;
    fn CreateColorTransformer(&self) -> windows_core::Result<IWICColorTransform>;
    fn CreateBitmap(&self, uiwidth: u32, uiheight: u32, pixelformat: REFWICPixelFormatGUID, option: WICBitmapCreateCacheOption) -> windows_core::Result<IWICBitmap>;
    fn CreateBitmapFromSource(&self, pibitmapsource: windows_core::Ref<IWICBitmapSource>, option: WICBitmapCreateCacheOption) -> windows_core::Result<IWICBitmap>;
    fn CreateBitmapFromSourceRect(&self, pibitmapsource: windows_core::Ref<IWICBitmapSource>, x: u32, y: u32, width: u32, height: u32) -> windows_core::Result<IWICBitmap>;
    fn CreateBitmapFromMemory(&self, uiwidth: u32, uiheight: u32, pixelformat: REFWICPixelFormatGUID, cbstride: u32, cbbuffersize: u32, pbbuffer: *const u8) -> windows_core::Result<IWICBitmap>;
    fn CreateBitmapFromHBITMAP(&self, hbitmap: super::windef::HBITMAP, hpalette: super::windef::HPALETTE, options: WICBitmapAlphaChannelOption) -> windows_core::Result<IWICBitmap>;
    fn CreateBitmapFromHICON(&self, hicon: super::windef::HICON) -> windows_core::Result<IWICBitmap>;
    fn CreateComponentEnumerator(&self, componenttypes: u32, options: u32) -> windows_core::Result<super::objidlbase::IEnumUnknown>;
    fn CreateFastMetadataEncoderFromDecoder(&self, pidecoder: windows_core::Ref<IWICBitmapDecoder>) -> windows_core::Result<IWICFastMetadataEncoder>;
    fn CreateFastMetadataEncoderFromFrameDecode(&self, piframedecoder: windows_core::Ref<IWICBitmapFrameDecode>) -> windows_core::Result<IWICFastMetadataEncoder>;
    fn CreateQueryWriter(&self, guidmetadataformat: *const windows_core::GUID, pguidvendor: *const windows_core::GUID) -> windows_core::Result<IWICMetadataQueryWriter>;
    fn CreateQueryWriterFromReader(&self, piqueryreader: windows_core::Ref<IWICMetadataQueryReader>, pguidvendor: *const windows_core::GUID) -> windows_core::Result<IWICMetadataQueryWriter>;
}
#[cfg(all(feature = "objidlbase", feature = "windef"))]
impl IWICImagingFactory_Vtbl {
    pub const fn new<Identity: IWICImagingFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateDecoderFromFilename<Identity: IWICImagingFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wzfilename: windows_core::PCWSTR, pguidvendor: *const windows_core::GUID, dwdesiredaccess: u32, metadataoptions: WICDecodeOptions, ppidecoder: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICImagingFactory_Impl::CreateDecoderFromFilename(this, core::mem::transmute(&wzfilename), core::mem::transmute_copy(&pguidvendor), core::mem::transmute_copy(&dwdesiredaccess), core::mem::transmute_copy(&metadataoptions)) {
                    Ok(ok__) => {
                        ppidecoder.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateDecoderFromStream<Identity: IWICImagingFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pistream: *mut core::ffi::c_void, pguidvendor: *const windows_core::GUID, metadataoptions: WICDecodeOptions, ppidecoder: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICImagingFactory_Impl::CreateDecoderFromStream(this, core::mem::transmute_copy(&pistream), core::mem::transmute_copy(&pguidvendor), core::mem::transmute_copy(&metadataoptions)) {
                    Ok(ok__) => {
                        ppidecoder.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateDecoderFromFileHandle<Identity: IWICImagingFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hfile: usize, pguidvendor: *const windows_core::GUID, metadataoptions: WICDecodeOptions, ppidecoder: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICImagingFactory_Impl::CreateDecoderFromFileHandle(this, core::mem::transmute_copy(&hfile), core::mem::transmute_copy(&pguidvendor), core::mem::transmute_copy(&metadataoptions)) {
                    Ok(ok__) => {
                        ppidecoder.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateComponentInfo<Identity: IWICImagingFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, clsidcomponent: *const windows_core::GUID, ppiinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICImagingFactory_Impl::CreateComponentInfo(this, core::mem::transmute_copy(&clsidcomponent)) {
                    Ok(ok__) => {
                        ppiinfo.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateDecoder<Identity: IWICImagingFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidcontainerformat: *const windows_core::GUID, pguidvendor: *const windows_core::GUID, ppidecoder: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICImagingFactory_Impl::CreateDecoder(this, core::mem::transmute_copy(&guidcontainerformat), core::mem::transmute_copy(&pguidvendor)) {
                    Ok(ok__) => {
                        ppidecoder.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateEncoder<Identity: IWICImagingFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidcontainerformat: *const windows_core::GUID, pguidvendor: *const windows_core::GUID, ppiencoder: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICImagingFactory_Impl::CreateEncoder(this, core::mem::transmute_copy(&guidcontainerformat), core::mem::transmute_copy(&pguidvendor)) {
                    Ok(ok__) => {
                        ppiencoder.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreatePalette<Identity: IWICImagingFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppipalette: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICImagingFactory_Impl::CreatePalette(this) {
                    Ok(ok__) => {
                        ppipalette.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateFormatConverter<Identity: IWICImagingFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppiformatconverter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICImagingFactory_Impl::CreateFormatConverter(this) {
                    Ok(ok__) => {
                        ppiformatconverter.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateBitmapScaler<Identity: IWICImagingFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppibitmapscaler: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICImagingFactory_Impl::CreateBitmapScaler(this) {
                    Ok(ok__) => {
                        ppibitmapscaler.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateBitmapClipper<Identity: IWICImagingFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppibitmapclipper: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICImagingFactory_Impl::CreateBitmapClipper(this) {
                    Ok(ok__) => {
                        ppibitmapclipper.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateBitmapFlipRotator<Identity: IWICImagingFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppibitmapfliprotator: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICImagingFactory_Impl::CreateBitmapFlipRotator(this) {
                    Ok(ok__) => {
                        ppibitmapfliprotator.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateStream<Identity: IWICImagingFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppiwicstream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICImagingFactory_Impl::CreateStream(this) {
                    Ok(ok__) => {
                        ppiwicstream.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateColorContext<Identity: IWICImagingFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppiwiccolorcontext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICImagingFactory_Impl::CreateColorContext(this) {
                    Ok(ok__) => {
                        ppiwiccolorcontext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateColorTransformer<Identity: IWICImagingFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppiwiccolortransform: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICImagingFactory_Impl::CreateColorTransformer(this) {
                    Ok(ok__) => {
                        ppiwiccolortransform.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateBitmap<Identity: IWICImagingFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uiwidth: u32, uiheight: u32, pixelformat: REFWICPixelFormatGUID, option: WICBitmapCreateCacheOption, ppibitmap: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICImagingFactory_Impl::CreateBitmap(this, core::mem::transmute_copy(&uiwidth), core::mem::transmute_copy(&uiheight), core::mem::transmute_copy(&pixelformat), core::mem::transmute_copy(&option)) {
                    Ok(ok__) => {
                        ppibitmap.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateBitmapFromSource<Identity: IWICImagingFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pibitmapsource: *mut core::ffi::c_void, option: WICBitmapCreateCacheOption, ppibitmap: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICImagingFactory_Impl::CreateBitmapFromSource(this, core::mem::transmute_copy(&pibitmapsource), core::mem::transmute_copy(&option)) {
                    Ok(ok__) => {
                        ppibitmap.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateBitmapFromSourceRect<Identity: IWICImagingFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pibitmapsource: *mut core::ffi::c_void, x: u32, y: u32, width: u32, height: u32, ppibitmap: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICImagingFactory_Impl::CreateBitmapFromSourceRect(this, core::mem::transmute_copy(&pibitmapsource), core::mem::transmute_copy(&x), core::mem::transmute_copy(&y), core::mem::transmute_copy(&width), core::mem::transmute_copy(&height)) {
                    Ok(ok__) => {
                        ppibitmap.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateBitmapFromMemory<Identity: IWICImagingFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uiwidth: u32, uiheight: u32, pixelformat: REFWICPixelFormatGUID, cbstride: u32, cbbuffersize: u32, pbbuffer: *const u8, ppibitmap: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICImagingFactory_Impl::CreateBitmapFromMemory(this, core::mem::transmute_copy(&uiwidth), core::mem::transmute_copy(&uiheight), core::mem::transmute_copy(&pixelformat), core::mem::transmute_copy(&cbstride), core::mem::transmute_copy(&cbbuffersize), core::mem::transmute_copy(&pbbuffer)) {
                    Ok(ok__) => {
                        ppibitmap.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateBitmapFromHBITMAP<Identity: IWICImagingFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hbitmap: super::windef::HBITMAP, hpalette: super::windef::HPALETTE, options: WICBitmapAlphaChannelOption, ppibitmap: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICImagingFactory_Impl::CreateBitmapFromHBITMAP(this, core::mem::transmute_copy(&hbitmap), core::mem::transmute_copy(&hpalette), core::mem::transmute_copy(&options)) {
                    Ok(ok__) => {
                        ppibitmap.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateBitmapFromHICON<Identity: IWICImagingFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hicon: super::windef::HICON, ppibitmap: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICImagingFactory_Impl::CreateBitmapFromHICON(this, core::mem::transmute_copy(&hicon)) {
                    Ok(ok__) => {
                        ppibitmap.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateComponentEnumerator<Identity: IWICImagingFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, componenttypes: u32, options: u32, ppienumunknown: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICImagingFactory_Impl::CreateComponentEnumerator(this, core::mem::transmute_copy(&componenttypes), core::mem::transmute_copy(&options)) {
                    Ok(ok__) => {
                        ppienumunknown.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateFastMetadataEncoderFromDecoder<Identity: IWICImagingFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pidecoder: *mut core::ffi::c_void, ppifastencoder: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICImagingFactory_Impl::CreateFastMetadataEncoderFromDecoder(this, core::mem::transmute_copy(&pidecoder)) {
                    Ok(ok__) => {
                        ppifastencoder.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateFastMetadataEncoderFromFrameDecode<Identity: IWICImagingFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, piframedecoder: *mut core::ffi::c_void, ppifastencoder: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICImagingFactory_Impl::CreateFastMetadataEncoderFromFrameDecode(this, core::mem::transmute_copy(&piframedecoder)) {
                    Ok(ok__) => {
                        ppifastencoder.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateQueryWriter<Identity: IWICImagingFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidmetadataformat: *const windows_core::GUID, pguidvendor: *const windows_core::GUID, ppiquerywriter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICImagingFactory_Impl::CreateQueryWriter(this, core::mem::transmute_copy(&guidmetadataformat), core::mem::transmute_copy(&pguidvendor)) {
                    Ok(ok__) => {
                        ppiquerywriter.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateQueryWriterFromReader<Identity: IWICImagingFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, piqueryreader: *mut core::ffi::c_void, pguidvendor: *const windows_core::GUID, ppiquerywriter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICImagingFactory_Impl::CreateQueryWriterFromReader(this, core::mem::transmute_copy(&piqueryreader), core::mem::transmute_copy(&pguidvendor)) {
                    Ok(ok__) => {
                        ppiquerywriter.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateDecoderFromFilename: CreateDecoderFromFilename::<Identity, OFFSET>,
            CreateDecoderFromStream: CreateDecoderFromStream::<Identity, OFFSET>,
            CreateDecoderFromFileHandle: CreateDecoderFromFileHandle::<Identity, OFFSET>,
            CreateComponentInfo: CreateComponentInfo::<Identity, OFFSET>,
            CreateDecoder: CreateDecoder::<Identity, OFFSET>,
            CreateEncoder: CreateEncoder::<Identity, OFFSET>,
            CreatePalette: CreatePalette::<Identity, OFFSET>,
            CreateFormatConverter: CreateFormatConverter::<Identity, OFFSET>,
            CreateBitmapScaler: CreateBitmapScaler::<Identity, OFFSET>,
            CreateBitmapClipper: CreateBitmapClipper::<Identity, OFFSET>,
            CreateBitmapFlipRotator: CreateBitmapFlipRotator::<Identity, OFFSET>,
            CreateStream: CreateStream::<Identity, OFFSET>,
            CreateColorContext: CreateColorContext::<Identity, OFFSET>,
            CreateColorTransformer: CreateColorTransformer::<Identity, OFFSET>,
            CreateBitmap: CreateBitmap::<Identity, OFFSET>,
            CreateBitmapFromSource: CreateBitmapFromSource::<Identity, OFFSET>,
            CreateBitmapFromSourceRect: CreateBitmapFromSourceRect::<Identity, OFFSET>,
            CreateBitmapFromMemory: CreateBitmapFromMemory::<Identity, OFFSET>,
            CreateBitmapFromHBITMAP: CreateBitmapFromHBITMAP::<Identity, OFFSET>,
            CreateBitmapFromHICON: CreateBitmapFromHICON::<Identity, OFFSET>,
            CreateComponentEnumerator: CreateComponentEnumerator::<Identity, OFFSET>,
            CreateFastMetadataEncoderFromDecoder: CreateFastMetadataEncoderFromDecoder::<Identity, OFFSET>,
            CreateFastMetadataEncoderFromFrameDecode: CreateFastMetadataEncoderFromFrameDecode::<Identity, OFFSET>,
            CreateQueryWriter: CreateQueryWriter::<Identity, OFFSET>,
            CreateQueryWriterFromReader: CreateQueryWriterFromReader::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWICImagingFactory as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "objidlbase", feature = "windef"))]
impl windows_core::RuntimeName for IWICImagingFactory {}
windows_core::imp::define_interface!(IWICImagingFactory2, IWICImagingFactory2_Vtbl, 0x7b816b45_1996_4476_b132_de9e247c8af0);
impl core::ops::Deref for IWICImagingFactory2 {
    type Target = IWICImagingFactory;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWICImagingFactory2, windows_core::IUnknown, IWICImagingFactory);
impl IWICImagingFactory2 {
    #[cfg(feature = "d2d")]
    pub unsafe fn CreateImageEncoder<P0>(&self, pd2ddevice: P0) -> windows_core::Result<IWICImageEncoder>
    where
        P0: windows_core::Param<super::d2d::ID2D1Device>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateImageEncoder)(windows_core::Interface::as_raw(self), pd2ddevice.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICImagingFactory2_Vtbl {
    pub base__: IWICImagingFactory_Vtbl,
    #[cfg(feature = "d2d")]
    pub CreateImageEncoder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "d2d"))]
    CreateImageEncoder: usize,
}
#[cfg(all(feature = "d2d", feature = "objidlbase", feature = "windef"))]
pub trait IWICImagingFactory2_Impl: IWICImagingFactory_Impl {
    fn CreateImageEncoder(&self, pd2ddevice: windows_core::Ref<super::d2d::ID2D1Device>) -> windows_core::Result<IWICImageEncoder>;
}
#[cfg(all(feature = "d2d", feature = "objidlbase", feature = "windef"))]
impl IWICImagingFactory2_Vtbl {
    pub const fn new<Identity: IWICImagingFactory2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateImageEncoder<Identity: IWICImagingFactory2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pd2ddevice: *mut core::ffi::c_void, ppwicimageencoder: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICImagingFactory2_Impl::CreateImageEncoder(this, core::mem::transmute_copy(&pd2ddevice)) {
                    Ok(ok__) => {
                        ppwicimageencoder.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: IWICImagingFactory_Vtbl::new::<Identity, OFFSET>(), CreateImageEncoder: CreateImageEncoder::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWICImagingFactory2 as windows_core::Interface>::IID || iid == &<IWICImagingFactory as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "d2d", feature = "objidlbase", feature = "windef"))]
impl windows_core::RuntimeName for IWICImagingFactory2 {}
windows_core::imp::define_interface!(IWICImagingFactory3, IWICImagingFactory3_Vtbl, 0x489b3d8b_624a_4258_b678_7eece70f299d);
impl core::ops::Deref for IWICImagingFactory3 {
    type Target = IWICImagingFactory2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWICImagingFactory3, windows_core::IUnknown, IWICImagingFactory, IWICImagingFactory2);
impl IWICImagingFactory3 {
    pub unsafe fn CreateBitmapToneMapper(&self) -> windows_core::Result<IWICBitmapToneMapper> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateBitmapToneMapper)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICImagingFactory3_Vtbl {
    pub base__: IWICImagingFactory2_Vtbl,
    pub CreateBitmapToneMapper: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "d2d", feature = "objidlbase", feature = "windef"))]
pub trait IWICImagingFactory3_Impl: IWICImagingFactory2_Impl {
    fn CreateBitmapToneMapper(&self) -> windows_core::Result<IWICBitmapToneMapper>;
}
#[cfg(all(feature = "d2d", feature = "objidlbase", feature = "windef"))]
impl IWICImagingFactory3_Vtbl {
    pub const fn new<Identity: IWICImagingFactory3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateBitmapToneMapper<Identity: IWICImagingFactory3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pptonemapper: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICImagingFactory3_Impl::CreateBitmapToneMapper(this) {
                    Ok(ok__) => {
                        pptonemapper.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: IWICImagingFactory2_Vtbl::new::<Identity, OFFSET>(), CreateBitmapToneMapper: CreateBitmapToneMapper::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWICImagingFactory3 as windows_core::Interface>::IID || iid == &<IWICImagingFactory as windows_core::Interface>::IID || iid == &<IWICImagingFactory2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "d2d", feature = "objidlbase", feature = "windef"))]
impl windows_core::RuntimeName for IWICImagingFactory3 {}
windows_core::imp::define_interface!(IWICJpegFrameDecode, IWICJpegFrameDecode_Vtbl, 0x8939f66e_c46a_4c21_a9d1_98b327ce1679);
windows_core::imp::interface_hierarchy!(IWICJpegFrameDecode, windows_core::IUnknown);
impl IWICJpegFrameDecode {
    pub unsafe fn DoesSupportIndexing(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DoesSupportIndexing)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetIndexing(&self, options: WICJpegIndexingOptions, horizontalintervalsize: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetIndexing)(windows_core::Interface::as_raw(self), options, horizontalintervalsize) }
    }
    pub unsafe fn ClearIndexing(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ClearIndexing)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "dxgi")]
    pub unsafe fn GetAcHuffmanTable(&self, scanindex: u32, tableindex: u32, pachuffmantable: *mut super::dxgi::DXGI_JPEG_AC_HUFFMAN_TABLE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetAcHuffmanTable)(windows_core::Interface::as_raw(self), scanindex, tableindex, pachuffmantable as _) }
    }
    #[cfg(feature = "dxgi")]
    pub unsafe fn GetDcHuffmanTable(&self, scanindex: u32, tableindex: u32, pdchuffmantable: *mut super::dxgi::DXGI_JPEG_DC_HUFFMAN_TABLE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDcHuffmanTable)(windows_core::Interface::as_raw(self), scanindex, tableindex, pdchuffmantable as _) }
    }
    #[cfg(feature = "dxgi")]
    pub unsafe fn GetQuantizationTable(&self, scanindex: u32, tableindex: u32, pquantizationtable: *mut super::dxgi::DXGI_JPEG_QUANTIZATION_TABLE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetQuantizationTable)(windows_core::Interface::as_raw(self), scanindex, tableindex, pquantizationtable as _) }
    }
    pub unsafe fn GetFrameHeader(&self, pframeheader: *mut WICJpegFrameHeader) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetFrameHeader)(windows_core::Interface::as_raw(self), pframeheader as _) }
    }
    pub unsafe fn GetScanHeader(&self, scanindex: u32, pscanheader: *mut WICJpegScanHeader) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetScanHeader)(windows_core::Interface::as_raw(self), scanindex, pscanheader as _) }
    }
    pub unsafe fn CopyScan(&self, scanindex: u32, scanoffset: u32, cbscandata: u32, pbscandata: *mut u8, pcbscandataactual: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CopyScan)(windows_core::Interface::as_raw(self), scanindex, scanoffset, cbscandata, pbscandata as _, pcbscandataactual as _) }
    }
    pub unsafe fn CopyMinimalStream(&self, streamoffset: u32, cbstreamdata: u32, pbstreamdata: *mut u8, pcbstreamdataactual: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CopyMinimalStream)(windows_core::Interface::as_raw(self), streamoffset, cbstreamdata, pbstreamdata as _, pcbstreamdataactual as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICJpegFrameDecode_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub DoesSupportIndexing: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub SetIndexing: unsafe extern "system" fn(*mut core::ffi::c_void, WICJpegIndexingOptions, u32) -> windows_core::HRESULT,
    pub ClearIndexing: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "dxgi")]
    pub GetAcHuffmanTable: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut super::dxgi::DXGI_JPEG_AC_HUFFMAN_TABLE) -> windows_core::HRESULT,
    #[cfg(not(feature = "dxgi"))]
    GetAcHuffmanTable: usize,
    #[cfg(feature = "dxgi")]
    pub GetDcHuffmanTable: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut super::dxgi::DXGI_JPEG_DC_HUFFMAN_TABLE) -> windows_core::HRESULT,
    #[cfg(not(feature = "dxgi"))]
    GetDcHuffmanTable: usize,
    #[cfg(feature = "dxgi")]
    pub GetQuantizationTable: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut super::dxgi::DXGI_JPEG_QUANTIZATION_TABLE) -> windows_core::HRESULT,
    #[cfg(not(feature = "dxgi"))]
    GetQuantizationTable: usize,
    pub GetFrameHeader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WICJpegFrameHeader) -> windows_core::HRESULT,
    pub GetScanHeader: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut WICJpegScanHeader) -> windows_core::HRESULT,
    pub CopyScan: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32, *mut u8, *mut u32) -> windows_core::HRESULT,
    pub CopyMinimalStream: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut u8, *mut u32) -> windows_core::HRESULT,
}
#[cfg(feature = "dxgi")]
pub trait IWICJpegFrameDecode_Impl: windows_core::IUnknownImpl {
    fn DoesSupportIndexing(&self) -> windows_core::Result<windows_core::BOOL>;
    fn SetIndexing(&self, options: WICJpegIndexingOptions, horizontalintervalsize: u32) -> windows_core::Result<()>;
    fn ClearIndexing(&self) -> windows_core::Result<()>;
    fn GetAcHuffmanTable(&self, scanindex: u32, tableindex: u32, pachuffmantable: *mut super::dxgi::DXGI_JPEG_AC_HUFFMAN_TABLE) -> windows_core::Result<()>;
    fn GetDcHuffmanTable(&self, scanindex: u32, tableindex: u32, pdchuffmantable: *mut super::dxgi::DXGI_JPEG_DC_HUFFMAN_TABLE) -> windows_core::Result<()>;
    fn GetQuantizationTable(&self, scanindex: u32, tableindex: u32, pquantizationtable: *mut super::dxgi::DXGI_JPEG_QUANTIZATION_TABLE) -> windows_core::Result<()>;
    fn GetFrameHeader(&self, pframeheader: *mut WICJpegFrameHeader) -> windows_core::Result<()>;
    fn GetScanHeader(&self, scanindex: u32, pscanheader: *mut WICJpegScanHeader) -> windows_core::Result<()>;
    fn CopyScan(&self, scanindex: u32, scanoffset: u32, cbscandata: u32, pbscandata: *mut u8, pcbscandataactual: *mut u32) -> windows_core::Result<()>;
    fn CopyMinimalStream(&self, streamoffset: u32, cbstreamdata: u32, pbstreamdata: *mut u8, pcbstreamdataactual: *mut u32) -> windows_core::Result<()>;
}
#[cfg(feature = "dxgi")]
impl IWICJpegFrameDecode_Vtbl {
    pub const fn new<Identity: IWICJpegFrameDecode_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn DoesSupportIndexing<Identity: IWICJpegFrameDecode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfindexingsupported: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICJpegFrameDecode_Impl::DoesSupportIndexing(this) {
                    Ok(ok__) => {
                        pfindexingsupported.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetIndexing<Identity: IWICJpegFrameDecode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, options: WICJpegIndexingOptions, horizontalintervalsize: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICJpegFrameDecode_Impl::SetIndexing(this, core::mem::transmute_copy(&options), core::mem::transmute_copy(&horizontalintervalsize)).into()
            }
        }
        unsafe extern "system" fn ClearIndexing<Identity: IWICJpegFrameDecode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICJpegFrameDecode_Impl::ClearIndexing(this).into()
            }
        }
        unsafe extern "system" fn GetAcHuffmanTable<Identity: IWICJpegFrameDecode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, scanindex: u32, tableindex: u32, pachuffmantable: *mut super::dxgi::DXGI_JPEG_AC_HUFFMAN_TABLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICJpegFrameDecode_Impl::GetAcHuffmanTable(this, core::mem::transmute_copy(&scanindex), core::mem::transmute_copy(&tableindex), core::mem::transmute_copy(&pachuffmantable)).into()
            }
        }
        unsafe extern "system" fn GetDcHuffmanTable<Identity: IWICJpegFrameDecode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, scanindex: u32, tableindex: u32, pdchuffmantable: *mut super::dxgi::DXGI_JPEG_DC_HUFFMAN_TABLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICJpegFrameDecode_Impl::GetDcHuffmanTable(this, core::mem::transmute_copy(&scanindex), core::mem::transmute_copy(&tableindex), core::mem::transmute_copy(&pdchuffmantable)).into()
            }
        }
        unsafe extern "system" fn GetQuantizationTable<Identity: IWICJpegFrameDecode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, scanindex: u32, tableindex: u32, pquantizationtable: *mut super::dxgi::DXGI_JPEG_QUANTIZATION_TABLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICJpegFrameDecode_Impl::GetQuantizationTable(this, core::mem::transmute_copy(&scanindex), core::mem::transmute_copy(&tableindex), core::mem::transmute_copy(&pquantizationtable)).into()
            }
        }
        unsafe extern "system" fn GetFrameHeader<Identity: IWICJpegFrameDecode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pframeheader: *mut WICJpegFrameHeader) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICJpegFrameDecode_Impl::GetFrameHeader(this, core::mem::transmute_copy(&pframeheader)).into()
            }
        }
        unsafe extern "system" fn GetScanHeader<Identity: IWICJpegFrameDecode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, scanindex: u32, pscanheader: *mut WICJpegScanHeader) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICJpegFrameDecode_Impl::GetScanHeader(this, core::mem::transmute_copy(&scanindex), core::mem::transmute_copy(&pscanheader)).into()
            }
        }
        unsafe extern "system" fn CopyScan<Identity: IWICJpegFrameDecode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, scanindex: u32, scanoffset: u32, cbscandata: u32, pbscandata: *mut u8, pcbscandataactual: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICJpegFrameDecode_Impl::CopyScan(this, core::mem::transmute_copy(&scanindex), core::mem::transmute_copy(&scanoffset), core::mem::transmute_copy(&cbscandata), core::mem::transmute_copy(&pbscandata), core::mem::transmute_copy(&pcbscandataactual)).into()
            }
        }
        unsafe extern "system" fn CopyMinimalStream<Identity: IWICJpegFrameDecode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, streamoffset: u32, cbstreamdata: u32, pbstreamdata: *mut u8, pcbstreamdataactual: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICJpegFrameDecode_Impl::CopyMinimalStream(this, core::mem::transmute_copy(&streamoffset), core::mem::transmute_copy(&cbstreamdata), core::mem::transmute_copy(&pbstreamdata), core::mem::transmute_copy(&pcbstreamdataactual)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            DoesSupportIndexing: DoesSupportIndexing::<Identity, OFFSET>,
            SetIndexing: SetIndexing::<Identity, OFFSET>,
            ClearIndexing: ClearIndexing::<Identity, OFFSET>,
            GetAcHuffmanTable: GetAcHuffmanTable::<Identity, OFFSET>,
            GetDcHuffmanTable: GetDcHuffmanTable::<Identity, OFFSET>,
            GetQuantizationTable: GetQuantizationTable::<Identity, OFFSET>,
            GetFrameHeader: GetFrameHeader::<Identity, OFFSET>,
            GetScanHeader: GetScanHeader::<Identity, OFFSET>,
            CopyScan: CopyScan::<Identity, OFFSET>,
            CopyMinimalStream: CopyMinimalStream::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWICJpegFrameDecode as windows_core::Interface>::IID
    }
}
#[cfg(feature = "dxgi")]
impl windows_core::RuntimeName for IWICJpegFrameDecode {}
windows_core::imp::define_interface!(IWICJpegFrameEncode, IWICJpegFrameEncode_Vtbl, 0x2f0c601f_d2c6_468c_abfa_49495d983ed1);
windows_core::imp::interface_hierarchy!(IWICJpegFrameEncode, windows_core::IUnknown);
impl IWICJpegFrameEncode {
    #[cfg(feature = "dxgi")]
    pub unsafe fn GetAcHuffmanTable(&self, scanindex: u32, tableindex: u32, pachuffmantable: *mut super::dxgi::DXGI_JPEG_AC_HUFFMAN_TABLE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetAcHuffmanTable)(windows_core::Interface::as_raw(self), scanindex, tableindex, pachuffmantable as _) }
    }
    #[cfg(feature = "dxgi")]
    pub unsafe fn GetDcHuffmanTable(&self, scanindex: u32, tableindex: u32, pdchuffmantable: *mut super::dxgi::DXGI_JPEG_DC_HUFFMAN_TABLE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDcHuffmanTable)(windows_core::Interface::as_raw(self), scanindex, tableindex, pdchuffmantable as _) }
    }
    #[cfg(feature = "dxgi")]
    pub unsafe fn GetQuantizationTable(&self, scanindex: u32, tableindex: u32, pquantizationtable: *mut super::dxgi::DXGI_JPEG_QUANTIZATION_TABLE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetQuantizationTable)(windows_core::Interface::as_raw(self), scanindex, tableindex, pquantizationtable as _) }
    }
    pub unsafe fn WriteScan(&self, cbscandata: u32, pbscandata: *const u8) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).WriteScan)(windows_core::Interface::as_raw(self), cbscandata, pbscandata) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICJpegFrameEncode_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "dxgi")]
    pub GetAcHuffmanTable: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut super::dxgi::DXGI_JPEG_AC_HUFFMAN_TABLE) -> windows_core::HRESULT,
    #[cfg(not(feature = "dxgi"))]
    GetAcHuffmanTable: usize,
    #[cfg(feature = "dxgi")]
    pub GetDcHuffmanTable: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut super::dxgi::DXGI_JPEG_DC_HUFFMAN_TABLE) -> windows_core::HRESULT,
    #[cfg(not(feature = "dxgi"))]
    GetDcHuffmanTable: usize,
    #[cfg(feature = "dxgi")]
    pub GetQuantizationTable: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut super::dxgi::DXGI_JPEG_QUANTIZATION_TABLE) -> windows_core::HRESULT,
    #[cfg(not(feature = "dxgi"))]
    GetQuantizationTable: usize,
    pub WriteScan: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8) -> windows_core::HRESULT,
}
#[cfg(feature = "dxgi")]
pub trait IWICJpegFrameEncode_Impl: windows_core::IUnknownImpl {
    fn GetAcHuffmanTable(&self, scanindex: u32, tableindex: u32, pachuffmantable: *mut super::dxgi::DXGI_JPEG_AC_HUFFMAN_TABLE) -> windows_core::Result<()>;
    fn GetDcHuffmanTable(&self, scanindex: u32, tableindex: u32, pdchuffmantable: *mut super::dxgi::DXGI_JPEG_DC_HUFFMAN_TABLE) -> windows_core::Result<()>;
    fn GetQuantizationTable(&self, scanindex: u32, tableindex: u32, pquantizationtable: *mut super::dxgi::DXGI_JPEG_QUANTIZATION_TABLE) -> windows_core::Result<()>;
    fn WriteScan(&self, cbscandata: u32, pbscandata: *const u8) -> windows_core::Result<()>;
}
#[cfg(feature = "dxgi")]
impl IWICJpegFrameEncode_Vtbl {
    pub const fn new<Identity: IWICJpegFrameEncode_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetAcHuffmanTable<Identity: IWICJpegFrameEncode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, scanindex: u32, tableindex: u32, pachuffmantable: *mut super::dxgi::DXGI_JPEG_AC_HUFFMAN_TABLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICJpegFrameEncode_Impl::GetAcHuffmanTable(this, core::mem::transmute_copy(&scanindex), core::mem::transmute_copy(&tableindex), core::mem::transmute_copy(&pachuffmantable)).into()
            }
        }
        unsafe extern "system" fn GetDcHuffmanTable<Identity: IWICJpegFrameEncode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, scanindex: u32, tableindex: u32, pdchuffmantable: *mut super::dxgi::DXGI_JPEG_DC_HUFFMAN_TABLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICJpegFrameEncode_Impl::GetDcHuffmanTable(this, core::mem::transmute_copy(&scanindex), core::mem::transmute_copy(&tableindex), core::mem::transmute_copy(&pdchuffmantable)).into()
            }
        }
        unsafe extern "system" fn GetQuantizationTable<Identity: IWICJpegFrameEncode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, scanindex: u32, tableindex: u32, pquantizationtable: *mut super::dxgi::DXGI_JPEG_QUANTIZATION_TABLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICJpegFrameEncode_Impl::GetQuantizationTable(this, core::mem::transmute_copy(&scanindex), core::mem::transmute_copy(&tableindex), core::mem::transmute_copy(&pquantizationtable)).into()
            }
        }
        unsafe extern "system" fn WriteScan<Identity: IWICJpegFrameEncode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbscandata: u32, pbscandata: *const u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICJpegFrameEncode_Impl::WriteScan(this, core::mem::transmute_copy(&cbscandata), core::mem::transmute_copy(&pbscandata)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetAcHuffmanTable: GetAcHuffmanTable::<Identity, OFFSET>,
            GetDcHuffmanTable: GetDcHuffmanTable::<Identity, OFFSET>,
            GetQuantizationTable: GetQuantizationTable::<Identity, OFFSET>,
            WriteScan: WriteScan::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWICJpegFrameEncode as windows_core::Interface>::IID
    }
}
#[cfg(feature = "dxgi")]
impl windows_core::RuntimeName for IWICJpegFrameEncode {}
windows_core::imp::define_interface!(IWICMetadataQueryReader, IWICMetadataQueryReader_Vtbl, 0x30989668_e1c9_4597_b395_458eedb808df);
windows_core::imp::interface_hierarchy!(IWICMetadataQueryReader, windows_core::IUnknown);
impl IWICMetadataQueryReader {
    pub unsafe fn GetContainerFormat(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetContainerFormat)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetLocation(&self, cchmaxlength: u32, wznamespace: *mut u16, pcchactuallength: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetLocation)(windows_core::Interface::as_raw(self), cchmaxlength, wznamespace as _, pcchactuallength as _) }
    }
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetMetadataByName<P0>(&self, wzname: P0, pvarvalue: *mut super::propidlbase::PROPVARIANT) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetMetadataByName)(windows_core::Interface::as_raw(self), wzname.param().abi(), core::mem::transmute(pvarvalue)) }
    }
    #[cfg(feature = "objidlbase")]
    pub unsafe fn GetEnumerator(&self) -> windows_core::Result<super::objidlbase::IEnumString> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetEnumerator)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICMetadataQueryReader_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetContainerFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GetLocation: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u16, *mut u32) -> windows_core::HRESULT,
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub GetMetadataByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut super::propidlbase::PROPVARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase")))]
    GetMetadataByName: usize,
    #[cfg(feature = "objidlbase")]
    pub GetEnumerator: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidlbase"))]
    GetEnumerator: usize,
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
pub trait IWICMetadataQueryReader_Impl: windows_core::IUnknownImpl {
    fn GetContainerFormat(&self) -> windows_core::Result<windows_core::GUID>;
    fn GetLocation(&self, cchmaxlength: u32, wznamespace: *mut u16, pcchactuallength: *mut u32) -> windows_core::Result<()>;
    fn GetMetadataByName(&self, wzname: &windows_core::PCWSTR, pvarvalue: *mut super::propidlbase::PROPVARIANT) -> windows_core::Result<()>;
    fn GetEnumerator(&self) -> windows_core::Result<super::objidlbase::IEnumString>;
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
impl IWICMetadataQueryReader_Vtbl {
    pub const fn new<Identity: IWICMetadataQueryReader_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetContainerFormat<Identity: IWICMetadataQueryReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidcontainerformat: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICMetadataQueryReader_Impl::GetContainerFormat(this) {
                    Ok(ok__) => {
                        pguidcontainerformat.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetLocation<Identity: IWICMetadataQueryReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cchmaxlength: u32, wznamespace: *mut u16, pcchactuallength: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICMetadataQueryReader_Impl::GetLocation(this, core::mem::transmute_copy(&cchmaxlength), core::mem::transmute_copy(&wznamespace), core::mem::transmute_copy(&pcchactuallength)).into()
            }
        }
        unsafe extern "system" fn GetMetadataByName<Identity: IWICMetadataQueryReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wzname: windows_core::PCWSTR, pvarvalue: *mut super::propidlbase::PROPVARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICMetadataQueryReader_Impl::GetMetadataByName(this, core::mem::transmute(&wzname), core::mem::transmute_copy(&pvarvalue)).into()
            }
        }
        unsafe extern "system" fn GetEnumerator<Identity: IWICMetadataQueryReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppienumstring: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICMetadataQueryReader_Impl::GetEnumerator(this) {
                    Ok(ok__) => {
                        ppienumstring.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetContainerFormat: GetContainerFormat::<Identity, OFFSET>,
            GetLocation: GetLocation::<Identity, OFFSET>,
            GetMetadataByName: GetMetadataByName::<Identity, OFFSET>,
            GetEnumerator: GetEnumerator::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWICMetadataQueryReader as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IWICMetadataQueryReader {}
windows_core::imp::define_interface!(IWICMetadataQueryWriter, IWICMetadataQueryWriter_Vtbl, 0xa721791a_0def_4d06_bd91_2118bf1db10b);
impl core::ops::Deref for IWICMetadataQueryWriter {
    type Target = IWICMetadataQueryReader;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWICMetadataQueryWriter, windows_core::IUnknown, IWICMetadataQueryReader);
impl IWICMetadataQueryWriter {
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn SetMetadataByName<P0>(&self, wzname: P0, pvarvalue: *const super::propidlbase::PROPVARIANT) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetMetadataByName)(windows_core::Interface::as_raw(self), wzname.param().abi(), core::mem::transmute(pvarvalue)) }
    }
    pub unsafe fn RemoveMetadataByName<P0>(&self, wzname: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).RemoveMetadataByName)(windows_core::Interface::as_raw(self), wzname.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICMetadataQueryWriter_Vtbl {
    pub base__: IWICMetadataQueryReader_Vtbl,
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub SetMetadataByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const super::propidlbase::PROPVARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase")))]
    SetMetadataByName: usize,
    pub RemoveMetadataByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
pub trait IWICMetadataQueryWriter_Impl: IWICMetadataQueryReader_Impl {
    fn SetMetadataByName(&self, wzname: &windows_core::PCWSTR, pvarvalue: *const super::propidlbase::PROPVARIANT) -> windows_core::Result<()>;
    fn RemoveMetadataByName(&self, wzname: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
impl IWICMetadataQueryWriter_Vtbl {
    pub const fn new<Identity: IWICMetadataQueryWriter_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetMetadataByName<Identity: IWICMetadataQueryWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wzname: windows_core::PCWSTR, pvarvalue: *const super::propidlbase::PROPVARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICMetadataQueryWriter_Impl::SetMetadataByName(this, core::mem::transmute(&wzname), core::mem::transmute_copy(&pvarvalue)).into()
            }
        }
        unsafe extern "system" fn RemoveMetadataByName<Identity: IWICMetadataQueryWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wzname: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICMetadataQueryWriter_Impl::RemoveMetadataByName(this, core::mem::transmute(&wzname)).into()
            }
        }
        Self {
            base__: IWICMetadataQueryReader_Vtbl::new::<Identity, OFFSET>(),
            SetMetadataByName: SetMetadataByName::<Identity, OFFSET>,
            RemoveMetadataByName: RemoveMetadataByName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWICMetadataQueryWriter as windows_core::Interface>::IID || iid == &<IWICMetadataQueryReader as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IWICMetadataQueryWriter {}
windows_core::imp::define_interface!(IWICPalette, IWICPalette_Vtbl, 0x00000040_a8f2_4877_ba0a_fd2b6645fb94);
windows_core::imp::interface_hierarchy!(IWICPalette, windows_core::IUnknown);
impl IWICPalette {
    pub unsafe fn InitializePredefined(&self, epalettetype: WICBitmapPaletteType, faddtransparentcolor: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).InitializePredefined)(windows_core::Interface::as_raw(self), epalettetype, faddtransparentcolor.into()) }
    }
    pub unsafe fn InitializeCustom(&self, pcolors: *const WICColor, ccount: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).InitializeCustom)(windows_core::Interface::as_raw(self), pcolors, ccount) }
    }
    pub unsafe fn InitializeFromBitmap<P0>(&self, pisurface: P0, ccount: u32, faddtransparentcolor: bool) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IWICBitmapSource>,
    {
        unsafe { (windows_core::Interface::vtable(self).InitializeFromBitmap)(windows_core::Interface::as_raw(self), pisurface.param().abi(), ccount, faddtransparentcolor.into()) }
    }
    pub unsafe fn InitializeFromPalette<P0>(&self, pipalette: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<Self>,
    {
        unsafe { (windows_core::Interface::vtable(self).InitializeFromPalette)(windows_core::Interface::as_raw(self), pipalette.param().abi()) }
    }
    pub unsafe fn GetType(&self) -> windows_core::Result<WICBitmapPaletteType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetColorCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetColorCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetColors(&self, ccount: u32, pcolors: *mut WICColor, pcactualcolors: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetColors)(windows_core::Interface::as_raw(self), ccount, pcolors as _, pcactualcolors as _) }
    }
    pub unsafe fn IsBlackWhite(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsBlackWhite)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsGrayscale(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsGrayscale)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn HasAlpha(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).HasAlpha)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICPalette_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub InitializePredefined: unsafe extern "system" fn(*mut core::ffi::c_void, WICBitmapPaletteType, windows_core::BOOL) -> windows_core::HRESULT,
    pub InitializeCustom: unsafe extern "system" fn(*mut core::ffi::c_void, *const WICColor, u32) -> windows_core::HRESULT,
    pub InitializeFromBitmap: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, windows_core::BOOL) -> windows_core::HRESULT,
    pub InitializeFromPalette: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WICBitmapPaletteType) -> windows_core::HRESULT,
    pub GetColorCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetColors: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut WICColor, *mut u32) -> windows_core::HRESULT,
    pub IsBlackWhite: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub IsGrayscale: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub HasAlpha: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IWICPalette_Impl: windows_core::IUnknownImpl {
    fn InitializePredefined(&self, epalettetype: WICBitmapPaletteType, faddtransparentcolor: windows_core::BOOL) -> windows_core::Result<()>;
    fn InitializeCustom(&self, pcolors: *const WICColor, ccount: u32) -> windows_core::Result<()>;
    fn InitializeFromBitmap(&self, pisurface: windows_core::Ref<IWICBitmapSource>, ccount: u32, faddtransparentcolor: windows_core::BOOL) -> windows_core::Result<()>;
    fn InitializeFromPalette(&self, pipalette: windows_core::Ref<IWICPalette>) -> windows_core::Result<()>;
    fn GetType(&self) -> windows_core::Result<WICBitmapPaletteType>;
    fn GetColorCount(&self) -> windows_core::Result<u32>;
    fn GetColors(&self, ccount: u32, pcolors: *mut WICColor, pcactualcolors: *mut u32) -> windows_core::Result<()>;
    fn IsBlackWhite(&self) -> windows_core::Result<windows_core::BOOL>;
    fn IsGrayscale(&self) -> windows_core::Result<windows_core::BOOL>;
    fn HasAlpha(&self) -> windows_core::Result<windows_core::BOOL>;
}
impl IWICPalette_Vtbl {
    pub const fn new<Identity: IWICPalette_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn InitializePredefined<Identity: IWICPalette_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, epalettetype: WICBitmapPaletteType, faddtransparentcolor: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICPalette_Impl::InitializePredefined(this, core::mem::transmute_copy(&epalettetype), core::mem::transmute_copy(&faddtransparentcolor)).into()
            }
        }
        unsafe extern "system" fn InitializeCustom<Identity: IWICPalette_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcolors: *const WICColor, ccount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICPalette_Impl::InitializeCustom(this, core::mem::transmute_copy(&pcolors), core::mem::transmute_copy(&ccount)).into()
            }
        }
        unsafe extern "system" fn InitializeFromBitmap<Identity: IWICPalette_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisurface: *mut core::ffi::c_void, ccount: u32, faddtransparentcolor: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICPalette_Impl::InitializeFromBitmap(this, core::mem::transmute_copy(&pisurface), core::mem::transmute_copy(&ccount), core::mem::transmute_copy(&faddtransparentcolor)).into()
            }
        }
        unsafe extern "system" fn InitializeFromPalette<Identity: IWICPalette_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pipalette: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICPalette_Impl::InitializeFromPalette(this, core::mem::transmute_copy(&pipalette)).into()
            }
        }
        unsafe extern "system" fn GetType<Identity: IWICPalette_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pepalettetype: *mut WICBitmapPaletteType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICPalette_Impl::GetType(this) {
                    Ok(ok__) => {
                        pepalettetype.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetColorCount<Identity: IWICPalette_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pccount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICPalette_Impl::GetColorCount(this) {
                    Ok(ok__) => {
                        pccount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetColors<Identity: IWICPalette_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ccount: u32, pcolors: *mut WICColor, pcactualcolors: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICPalette_Impl::GetColors(this, core::mem::transmute_copy(&ccount), core::mem::transmute_copy(&pcolors), core::mem::transmute_copy(&pcactualcolors)).into()
            }
        }
        unsafe extern "system" fn IsBlackWhite<Identity: IWICPalette_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfisblackwhite: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICPalette_Impl::IsBlackWhite(this) {
                    Ok(ok__) => {
                        pfisblackwhite.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsGrayscale<Identity: IWICPalette_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfisgrayscale: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICPalette_Impl::IsGrayscale(this) {
                    Ok(ok__) => {
                        pfisgrayscale.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn HasAlpha<Identity: IWICPalette_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfhasalpha: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICPalette_Impl::HasAlpha(this) {
                    Ok(ok__) => {
                        pfhasalpha.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            InitializePredefined: InitializePredefined::<Identity, OFFSET>,
            InitializeCustom: InitializeCustom::<Identity, OFFSET>,
            InitializeFromBitmap: InitializeFromBitmap::<Identity, OFFSET>,
            InitializeFromPalette: InitializeFromPalette::<Identity, OFFSET>,
            GetType: GetType::<Identity, OFFSET>,
            GetColorCount: GetColorCount::<Identity, OFFSET>,
            GetColors: GetColors::<Identity, OFFSET>,
            IsBlackWhite: IsBlackWhite::<Identity, OFFSET>,
            IsGrayscale: IsGrayscale::<Identity, OFFSET>,
            HasAlpha: HasAlpha::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWICPalette as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWICPalette {}
windows_core::imp::define_interface!(IWICPixelFormatInfo, IWICPixelFormatInfo_Vtbl, 0xe8eda601_3d48_431a_ab44_69059be88bbe);
impl core::ops::Deref for IWICPixelFormatInfo {
    type Target = IWICComponentInfo;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWICPixelFormatInfo, windows_core::IUnknown, IWICComponentInfo);
impl IWICPixelFormatInfo {
    pub unsafe fn GetFormatGUID(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFormatGUID)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetColorContext(&self) -> windows_core::Result<IWICColorContext> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetColorContext)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetBitsPerPixel(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBitsPerPixel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetChannelCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetChannelCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetChannelMask(&self, uichannelindex: u32, cbmaskbuffer: u32, pbmaskbuffer: *mut u8, pcbactual: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetChannelMask)(windows_core::Interface::as_raw(self), uichannelindex, cbmaskbuffer, pbmaskbuffer as _, pcbactual as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICPixelFormatInfo_Vtbl {
    pub base__: IWICComponentInfo_Vtbl,
    pub GetFormatGUID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GetColorContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetBitsPerPixel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetChannelCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetChannelMask: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut u8, *mut u32) -> windows_core::HRESULT,
}
pub trait IWICPixelFormatInfo_Impl: IWICComponentInfo_Impl {
    fn GetFormatGUID(&self) -> windows_core::Result<windows_core::GUID>;
    fn GetColorContext(&self) -> windows_core::Result<IWICColorContext>;
    fn GetBitsPerPixel(&self) -> windows_core::Result<u32>;
    fn GetChannelCount(&self) -> windows_core::Result<u32>;
    fn GetChannelMask(&self, uichannelindex: u32, cbmaskbuffer: u32, pbmaskbuffer: *mut u8, pcbactual: *mut u32) -> windows_core::Result<()>;
}
impl IWICPixelFormatInfo_Vtbl {
    pub const fn new<Identity: IWICPixelFormatInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetFormatGUID<Identity: IWICPixelFormatInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pformat: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICPixelFormatInfo_Impl::GetFormatGUID(this) {
                    Ok(ok__) => {
                        pformat.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetColorContext<Identity: IWICPixelFormatInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppicolorcontext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICPixelFormatInfo_Impl::GetColorContext(this) {
                    Ok(ok__) => {
                        ppicolorcontext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetBitsPerPixel<Identity: IWICPixelFormatInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, puibitsperpixel: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICPixelFormatInfo_Impl::GetBitsPerPixel(this) {
                    Ok(ok__) => {
                        puibitsperpixel.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetChannelCount<Identity: IWICPixelFormatInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, puichannelcount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICPixelFormatInfo_Impl::GetChannelCount(this) {
                    Ok(ok__) => {
                        puichannelcount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetChannelMask<Identity: IWICPixelFormatInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uichannelindex: u32, cbmaskbuffer: u32, pbmaskbuffer: *mut u8, pcbactual: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICPixelFormatInfo_Impl::GetChannelMask(this, core::mem::transmute_copy(&uichannelindex), core::mem::transmute_copy(&cbmaskbuffer), core::mem::transmute_copy(&pbmaskbuffer), core::mem::transmute_copy(&pcbactual)).into()
            }
        }
        Self {
            base__: IWICComponentInfo_Vtbl::new::<Identity, OFFSET>(),
            GetFormatGUID: GetFormatGUID::<Identity, OFFSET>,
            GetColorContext: GetColorContext::<Identity, OFFSET>,
            GetBitsPerPixel: GetBitsPerPixel::<Identity, OFFSET>,
            GetChannelCount: GetChannelCount::<Identity, OFFSET>,
            GetChannelMask: GetChannelMask::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWICPixelFormatInfo as windows_core::Interface>::IID || iid == &<IWICComponentInfo as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWICPixelFormatInfo {}
windows_core::imp::define_interface!(IWICPixelFormatInfo2, IWICPixelFormatInfo2_Vtbl, 0xa9db33a2_af5f_43c7_b679_74f5984b5aa4);
impl core::ops::Deref for IWICPixelFormatInfo2 {
    type Target = IWICPixelFormatInfo;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWICPixelFormatInfo2, windows_core::IUnknown, IWICComponentInfo, IWICPixelFormatInfo);
impl IWICPixelFormatInfo2 {
    pub unsafe fn SupportsTransparency(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SupportsTransparency)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetNumericRepresentation(&self) -> windows_core::Result<WICPixelFormatNumericRepresentation> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNumericRepresentation)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICPixelFormatInfo2_Vtbl {
    pub base__: IWICPixelFormatInfo_Vtbl,
    pub SupportsTransparency: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub GetNumericRepresentation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WICPixelFormatNumericRepresentation) -> windows_core::HRESULT,
}
pub trait IWICPixelFormatInfo2_Impl: IWICPixelFormatInfo_Impl {
    fn SupportsTransparency(&self) -> windows_core::Result<windows_core::BOOL>;
    fn GetNumericRepresentation(&self) -> windows_core::Result<WICPixelFormatNumericRepresentation>;
}
impl IWICPixelFormatInfo2_Vtbl {
    pub const fn new<Identity: IWICPixelFormatInfo2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SupportsTransparency<Identity: IWICPixelFormatInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfsupportstransparency: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICPixelFormatInfo2_Impl::SupportsTransparency(this) {
                    Ok(ok__) => {
                        pfsupportstransparency.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetNumericRepresentation<Identity: IWICPixelFormatInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnumericrepresentation: *mut WICPixelFormatNumericRepresentation) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICPixelFormatInfo2_Impl::GetNumericRepresentation(this) {
                    Ok(ok__) => {
                        pnumericrepresentation.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IWICPixelFormatInfo_Vtbl::new::<Identity, OFFSET>(),
            SupportsTransparency: SupportsTransparency::<Identity, OFFSET>,
            GetNumericRepresentation: GetNumericRepresentation::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWICPixelFormatInfo2 as windows_core::Interface>::IID || iid == &<IWICComponentInfo as windows_core::Interface>::IID || iid == &<IWICPixelFormatInfo as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWICPixelFormatInfo2 {}
windows_core::imp::define_interface!(IWICPlanarBitmapFrameEncode, IWICPlanarBitmapFrameEncode_Vtbl, 0xf928b7b8_2221_40c1_b72e_7e82f1974d1a);
windows_core::imp::interface_hierarchy!(IWICPlanarBitmapFrameEncode, windows_core::IUnknown);
impl IWICPlanarBitmapFrameEncode {
    pub unsafe fn WritePixels(&self, linecount: u32, pplanes: *const WICBitmapPlane, cplanes: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).WritePixels)(windows_core::Interface::as_raw(self), linecount, pplanes, cplanes) }
    }
    pub unsafe fn WriteSource(&self, ppplanes: *const Option<IWICBitmapSource>, cplanes: u32, prcsource: *const WICRect) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).WriteSource)(windows_core::Interface::as_raw(self), core::mem::transmute(ppplanes), cplanes, prcsource) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICPlanarBitmapFrameEncode_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub WritePixels: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const WICBitmapPlane, u32) -> windows_core::HRESULT,
    pub WriteSource: unsafe extern "system" fn(*mut core::ffi::c_void, *const *mut core::ffi::c_void, u32, *const WICRect) -> windows_core::HRESULT,
}
pub trait IWICPlanarBitmapFrameEncode_Impl: windows_core::IUnknownImpl {
    fn WritePixels(&self, linecount: u32, pplanes: *const WICBitmapPlane, cplanes: u32) -> windows_core::Result<()>;
    fn WriteSource(&self, ppplanes: *const Option<IWICBitmapSource>, cplanes: u32, prcsource: *const WICRect) -> windows_core::Result<()>;
}
impl IWICPlanarBitmapFrameEncode_Vtbl {
    pub const fn new<Identity: IWICPlanarBitmapFrameEncode_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn WritePixels<Identity: IWICPlanarBitmapFrameEncode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, linecount: u32, pplanes: *const WICBitmapPlane, cplanes: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICPlanarBitmapFrameEncode_Impl::WritePixels(this, core::mem::transmute_copy(&linecount), core::mem::transmute_copy(&pplanes), core::mem::transmute_copy(&cplanes)).into()
            }
        }
        unsafe extern "system" fn WriteSource<Identity: IWICPlanarBitmapFrameEncode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppplanes: *const *mut core::ffi::c_void, cplanes: u32, prcsource: *const WICRect) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICPlanarBitmapFrameEncode_Impl::WriteSource(this, core::mem::transmute_copy(&ppplanes), core::mem::transmute_copy(&cplanes), core::mem::transmute_copy(&prcsource)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            WritePixels: WritePixels::<Identity, OFFSET>,
            WriteSource: WriteSource::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWICPlanarBitmapFrameEncode as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWICPlanarBitmapFrameEncode {}
windows_core::imp::define_interface!(IWICPlanarBitmapSourceTransform, IWICPlanarBitmapSourceTransform_Vtbl, 0x3aff9cce_be95_4303_b927_e7d16ff4a613);
windows_core::imp::interface_hierarchy!(IWICPlanarBitmapSourceTransform, windows_core::IUnknown);
impl IWICPlanarBitmapSourceTransform {
    pub unsafe fn DoesSupportTransform(&self, puiwidth: *mut u32, puiheight: *mut u32, dsttransform: WICBitmapTransformOptions, dstplanaroptions: WICPlanarOptions, pguiddstformats: *const WICPixelFormatGUID, pplanedescriptions: *mut WICBitmapPlaneDescription, cplanes: u32, pfissupported: *mut windows_core::BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DoesSupportTransform)(windows_core::Interface::as_raw(self), puiwidth as _, puiheight as _, dsttransform, dstplanaroptions, pguiddstformats, pplanedescriptions as _, cplanes, pfissupported as _) }
    }
    pub unsafe fn CopyPixels(&self, prcsource: *const WICRect, uiwidth: u32, uiheight: u32, dsttransform: WICBitmapTransformOptions, dstplanaroptions: WICPlanarOptions, pdstplanes: *const WICBitmapPlane, cplanes: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CopyPixels)(windows_core::Interface::as_raw(self), prcsource, uiwidth, uiheight, dsttransform, dstplanaroptions, pdstplanes, cplanes) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICPlanarBitmapSourceTransform_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub DoesSupportTransform: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32, WICBitmapTransformOptions, WICPlanarOptions, *const WICPixelFormatGUID, *mut WICBitmapPlaneDescription, u32, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub CopyPixels: unsafe extern "system" fn(*mut core::ffi::c_void, *const WICRect, u32, u32, WICBitmapTransformOptions, WICPlanarOptions, *const WICBitmapPlane, u32) -> windows_core::HRESULT,
}
pub trait IWICPlanarBitmapSourceTransform_Impl: windows_core::IUnknownImpl {
    fn DoesSupportTransform(&self, puiwidth: *mut u32, puiheight: *mut u32, dsttransform: WICBitmapTransformOptions, dstplanaroptions: WICPlanarOptions, pguiddstformats: *const WICPixelFormatGUID, pplanedescriptions: *mut WICBitmapPlaneDescription, cplanes: u32, pfissupported: *mut windows_core::BOOL) -> windows_core::Result<()>;
    fn CopyPixels(&self, prcsource: *const WICRect, uiwidth: u32, uiheight: u32, dsttransform: WICBitmapTransformOptions, dstplanaroptions: WICPlanarOptions, pdstplanes: *const WICBitmapPlane, cplanes: u32) -> windows_core::Result<()>;
}
impl IWICPlanarBitmapSourceTransform_Vtbl {
    pub const fn new<Identity: IWICPlanarBitmapSourceTransform_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn DoesSupportTransform<Identity: IWICPlanarBitmapSourceTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, puiwidth: *mut u32, puiheight: *mut u32, dsttransform: WICBitmapTransformOptions, dstplanaroptions: WICPlanarOptions, pguiddstformats: *const WICPixelFormatGUID, pplanedescriptions: *mut WICBitmapPlaneDescription, cplanes: u32, pfissupported: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICPlanarBitmapSourceTransform_Impl::DoesSupportTransform(this, core::mem::transmute_copy(&puiwidth), core::mem::transmute_copy(&puiheight), core::mem::transmute_copy(&dsttransform), core::mem::transmute_copy(&dstplanaroptions), core::mem::transmute_copy(&pguiddstformats), core::mem::transmute_copy(&pplanedescriptions), core::mem::transmute_copy(&cplanes), core::mem::transmute_copy(&pfissupported)).into()
            }
        }
        unsafe extern "system" fn CopyPixels<Identity: IWICPlanarBitmapSourceTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prcsource: *const WICRect, uiwidth: u32, uiheight: u32, dsttransform: WICBitmapTransformOptions, dstplanaroptions: WICPlanarOptions, pdstplanes: *const WICBitmapPlane, cplanes: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICPlanarBitmapSourceTransform_Impl::CopyPixels(this, core::mem::transmute_copy(&prcsource), core::mem::transmute_copy(&uiwidth), core::mem::transmute_copy(&uiheight), core::mem::transmute_copy(&dsttransform), core::mem::transmute_copy(&dstplanaroptions), core::mem::transmute_copy(&pdstplanes), core::mem::transmute_copy(&cplanes)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            DoesSupportTransform: DoesSupportTransform::<Identity, OFFSET>,
            CopyPixels: CopyPixels::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWICPlanarBitmapSourceTransform as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWICPlanarBitmapSourceTransform {}
windows_core::imp::define_interface!(IWICPlanarFormatConverter, IWICPlanarFormatConverter_Vtbl, 0xbebee9cb_83b0_4dcc_8132_b0aaa55eac96);
impl core::ops::Deref for IWICPlanarFormatConverter {
    type Target = IWICBitmapSource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWICPlanarFormatConverter, windows_core::IUnknown, IWICBitmapSource);
impl IWICPlanarFormatConverter {
    pub unsafe fn Initialize<P4>(&self, ppplanes: *const Option<IWICBitmapSource>, cplanes: u32, dstformat: REFWICPixelFormatGUID, dither: WICBitmapDitherType, pipalette: P4, alphathresholdpercent: f64, palettetranslate: WICBitmapPaletteType) -> windows_core::HRESULT
    where
        P4: windows_core::Param<IWICPalette>,
    {
        unsafe { (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), core::mem::transmute(ppplanes), cplanes, dstformat, dither, pipalette.param().abi(), alphathresholdpercent, palettetranslate) }
    }
    pub unsafe fn CanConvert(&self, psrcpixelformats: *const WICPixelFormatGUID, csrcplanes: u32, dstpixelformat: REFWICPixelFormatGUID) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CanConvert)(windows_core::Interface::as_raw(self), psrcpixelformats, csrcplanes, dstpixelformat, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICPlanarFormatConverter_Vtbl {
    pub base__: IWICBitmapSource_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *const *mut core::ffi::c_void, u32, REFWICPixelFormatGUID, WICBitmapDitherType, *mut core::ffi::c_void, f64, WICBitmapPaletteType) -> windows_core::HRESULT,
    pub CanConvert: unsafe extern "system" fn(*mut core::ffi::c_void, *const WICPixelFormatGUID, u32, REFWICPixelFormatGUID, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IWICPlanarFormatConverter_Impl: IWICBitmapSource_Impl {
    fn Initialize(&self, ppplanes: *const Option<IWICBitmapSource>, cplanes: u32, dstformat: REFWICPixelFormatGUID, dither: WICBitmapDitherType, pipalette: windows_core::Ref<IWICPalette>, alphathresholdpercent: f64, palettetranslate: WICBitmapPaletteType) -> windows_core::Result<()>;
    fn CanConvert(&self, psrcpixelformats: *const WICPixelFormatGUID, csrcplanes: u32, dstpixelformat: REFWICPixelFormatGUID) -> windows_core::Result<windows_core::BOOL>;
}
impl IWICPlanarFormatConverter_Vtbl {
    pub const fn new<Identity: IWICPlanarFormatConverter_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Initialize<Identity: IWICPlanarFormatConverter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppplanes: *const *mut core::ffi::c_void, cplanes: u32, dstformat: REFWICPixelFormatGUID, dither: WICBitmapDitherType, pipalette: *mut core::ffi::c_void, alphathresholdpercent: f64, palettetranslate: WICBitmapPaletteType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICPlanarFormatConverter_Impl::Initialize(this, core::mem::transmute_copy(&ppplanes), core::mem::transmute_copy(&cplanes), core::mem::transmute_copy(&dstformat), core::mem::transmute_copy(&dither), core::mem::transmute_copy(&pipalette), core::mem::transmute_copy(&alphathresholdpercent), core::mem::transmute_copy(&palettetranslate)).into()
            }
        }
        unsafe extern "system" fn CanConvert<Identity: IWICPlanarFormatConverter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psrcpixelformats: *const WICPixelFormatGUID, csrcplanes: u32, dstpixelformat: REFWICPixelFormatGUID, pfcanconvert: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICPlanarFormatConverter_Impl::CanConvert(this, core::mem::transmute_copy(&psrcpixelformats), core::mem::transmute_copy(&csrcplanes), core::mem::transmute_copy(&dstpixelformat)) {
                    Ok(ok__) => {
                        pfcanconvert.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: IWICBitmapSource_Vtbl::new::<Identity, OFFSET>(), Initialize: Initialize::<Identity, OFFSET>, CanConvert: CanConvert::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWICPlanarFormatConverter as windows_core::Interface>::IID || iid == &<IWICBitmapSource as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWICPlanarFormatConverter {}
windows_core::imp::define_interface!(IWICProgressCallback, IWICProgressCallback_Vtbl, 0x4776f9cd_9517_45fa_bf24_e89c5ec5c60c);
windows_core::imp::interface_hierarchy!(IWICProgressCallback, windows_core::IUnknown);
impl IWICProgressCallback {
    pub unsafe fn Notify(&self, uframenum: u32, operation: WICProgressOperation, dblprogress: f64) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Notify)(windows_core::Interface::as_raw(self), uframenum, operation, dblprogress) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICProgressCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Notify: unsafe extern "system" fn(*mut core::ffi::c_void, u32, WICProgressOperation, f64) -> windows_core::HRESULT,
}
pub trait IWICProgressCallback_Impl: windows_core::IUnknownImpl {
    fn Notify(&self, uframenum: u32, operation: WICProgressOperation, dblprogress: f64) -> windows_core::Result<()>;
}
impl IWICProgressCallback_Vtbl {
    pub const fn new<Identity: IWICProgressCallback_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Notify<Identity: IWICProgressCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uframenum: u32, operation: WICProgressOperation, dblprogress: f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICProgressCallback_Impl::Notify(this, core::mem::transmute_copy(&uframenum), core::mem::transmute_copy(&operation), core::mem::transmute_copy(&dblprogress)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Notify: Notify::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWICProgressCallback as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWICProgressCallback {}
windows_core::imp::define_interface!(IWICProgressiveLevelControl, IWICProgressiveLevelControl_Vtbl, 0xdaac296f_7aa5_4dbf_8d15_225c5976f891);
windows_core::imp::interface_hierarchy!(IWICProgressiveLevelControl, windows_core::IUnknown);
impl IWICProgressiveLevelControl {
    pub unsafe fn GetLevelCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLevelCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetCurrentLevel(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCurrentLevel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetCurrentLevel(&self, nlevel: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetCurrentLevel)(windows_core::Interface::as_raw(self), nlevel) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICProgressiveLevelControl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetLevelCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetCurrentLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetCurrentLevel: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait IWICProgressiveLevelControl_Impl: windows_core::IUnknownImpl {
    fn GetLevelCount(&self) -> windows_core::Result<u32>;
    fn GetCurrentLevel(&self) -> windows_core::Result<u32>;
    fn SetCurrentLevel(&self, nlevel: u32) -> windows_core::Result<()>;
}
impl IWICProgressiveLevelControl_Vtbl {
    pub const fn new<Identity: IWICProgressiveLevelControl_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetLevelCount<Identity: IWICProgressiveLevelControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pclevels: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICProgressiveLevelControl_Impl::GetLevelCount(this) {
                    Ok(ok__) => {
                        pclevels.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCurrentLevel<Identity: IWICProgressiveLevelControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnlevel: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICProgressiveLevelControl_Impl::GetCurrentLevel(this) {
                    Ok(ok__) => {
                        pnlevel.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCurrentLevel<Identity: IWICProgressiveLevelControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nlevel: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICProgressiveLevelControl_Impl::SetCurrentLevel(this, core::mem::transmute_copy(&nlevel)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetLevelCount: GetLevelCount::<Identity, OFFSET>,
            GetCurrentLevel: GetCurrentLevel::<Identity, OFFSET>,
            SetCurrentLevel: SetCurrentLevel::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWICProgressiveLevelControl as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWICProgressiveLevelControl {}
#[cfg(feature = "objidlbase")]
windows_core::imp::define_interface!(IWICStream, IWICStream_Vtbl, 0x135ff860_22b7_4ddf_b0f6_218f4f299a43);
#[cfg(feature = "objidlbase")]
impl core::ops::Deref for IWICStream {
    type Target = super::objidlbase::IStream;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "objidlbase")]
windows_core::imp::interface_hierarchy!(IWICStream, windows_core::IUnknown, super::objidlbase::ISequentialStream, super::objidlbase::IStream);
#[cfg(feature = "objidlbase")]
impl IWICStream {
    pub unsafe fn InitializeFromIStream<P0>(&self, pistream: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::objidlbase::IStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).InitializeFromIStream)(windows_core::Interface::as_raw(self), pistream.param().abi()) }
    }
    pub unsafe fn InitializeFromFilename<P0>(&self, wzfilename: P0, dwdesiredaccess: u32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).InitializeFromFilename)(windows_core::Interface::as_raw(self), wzfilename.param().abi(), dwdesiredaccess) }
    }
    pub unsafe fn InitializeFromMemory(&self, pbbuffer: *const u8, cbbuffersize: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).InitializeFromMemory)(windows_core::Interface::as_raw(self), pbbuffer, cbbuffersize) }
    }
    pub unsafe fn InitializeFromIStreamRegion<P0>(&self, pistream: P0, uloffset: u64, ulmaxsize: u64) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::objidlbase::IStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).InitializeFromIStreamRegion)(windows_core::Interface::as_raw(self), pistream.param().abi(), uloffset, ulmaxsize) }
    }
}
#[cfg(feature = "objidlbase")]
#[repr(C)]
#[doc(hidden)]
pub struct IWICStream_Vtbl {
    pub base__: super::objidlbase::IStream_Vtbl,
    pub InitializeFromIStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InitializeFromFilename: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
    pub InitializeFromMemory: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, u32) -> windows_core::HRESULT,
    pub InitializeFromIStreamRegion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u64, u64) -> windows_core::HRESULT,
}
#[cfg(all(feature = "minwindef", feature = "objidlbase"))]
pub trait IWICStream_Impl: super::objidlbase::IStream_Impl {
    fn InitializeFromIStream(&self, pistream: windows_core::Ref<super::objidlbase::IStream>) -> windows_core::Result<()>;
    fn InitializeFromFilename(&self, wzfilename: &windows_core::PCWSTR, dwdesiredaccess: u32) -> windows_core::Result<()>;
    fn InitializeFromMemory(&self, pbbuffer: *const u8, cbbuffersize: u32) -> windows_core::Result<()>;
    fn InitializeFromIStreamRegion(&self, pistream: windows_core::Ref<super::objidlbase::IStream>, uloffset: u64, ulmaxsize: u64) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "objidlbase"))]
impl IWICStream_Vtbl {
    pub const fn new<Identity: IWICStream_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn InitializeFromIStream<Identity: IWICStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pistream: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICStream_Impl::InitializeFromIStream(this, core::mem::transmute_copy(&pistream)).into()
            }
        }
        unsafe extern "system" fn InitializeFromFilename<Identity: IWICStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wzfilename: windows_core::PCWSTR, dwdesiredaccess: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICStream_Impl::InitializeFromFilename(this, core::mem::transmute(&wzfilename), core::mem::transmute_copy(&dwdesiredaccess)).into()
            }
        }
        unsafe extern "system" fn InitializeFromMemory<Identity: IWICStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbbuffer: *const u8, cbbuffersize: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICStream_Impl::InitializeFromMemory(this, core::mem::transmute_copy(&pbbuffer), core::mem::transmute_copy(&cbbuffersize)).into()
            }
        }
        unsafe extern "system" fn InitializeFromIStreamRegion<Identity: IWICStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pistream: *mut core::ffi::c_void, uloffset: u64, ulmaxsize: u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICStream_Impl::InitializeFromIStreamRegion(this, core::mem::transmute_copy(&pistream), core::mem::transmute_copy(&uloffset), core::mem::transmute_copy(&ulmaxsize)).into()
            }
        }
        Self {
            base__: super::objidlbase::IStream_Vtbl::new::<Identity, OFFSET>(),
            InitializeFromIStream: InitializeFromIStream::<Identity, OFFSET>,
            InitializeFromFilename: InitializeFromFilename::<Identity, OFFSET>,
            InitializeFromMemory: InitializeFromMemory::<Identity, OFFSET>,
            InitializeFromIStreamRegion: InitializeFromIStreamRegion::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWICStream as windows_core::Interface>::IID || iid == &<super::objidlbase::ISequentialStream as windows_core::Interface>::IID || iid == &<super::objidlbase::IStream as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "objidlbase"))]
impl windows_core::RuntimeName for IWICStream {}
pub type PFNProgressNotification = Option<unsafe extern "system" fn(pvdata: *mut core::ffi::c_void, uframenum: u32, operation: WICProgressOperation, dblprogress: f64) -> windows_core::HRESULT>;
pub type REFWICPixelFormatGUID = *const windows_core::GUID;
pub const WIC8BIMIptcDigestIptcDigest: WIC8BIMIptcDigestProperties = 2;
pub const WIC8BIMIptcDigestPString: WIC8BIMIptcDigestProperties = 1;
pub type WIC8BIMIptcDigestProperties = i32;
pub const WIC8BIMIptcDigestProperties_FORCE_DWORD: WIC8BIMIptcDigestProperties = 2147483647;
pub const WIC8BIMIptcEmbeddedIPTC: WIC8BIMIptcProperties = 1;
pub const WIC8BIMIptcPString: WIC8BIMIptcProperties = 0;
pub type WIC8BIMIptcProperties = i32;
pub const WIC8BIMIptcProperties_FORCE_DWORD: WIC8BIMIptcProperties = 2147483647;
pub const WIC8BIMResolutionInfoHResolution: WIC8BIMResolutionInfoProperties = 2;
pub const WIC8BIMResolutionInfoHResolutionUnit: WIC8BIMResolutionInfoProperties = 3;
pub const WIC8BIMResolutionInfoHeightUnit: WIC8BIMResolutionInfoProperties = 7;
pub const WIC8BIMResolutionInfoPString: WIC8BIMResolutionInfoProperties = 1;
pub type WIC8BIMResolutionInfoProperties = i32;
pub const WIC8BIMResolutionInfoProperties_FORCE_DWORD: WIC8BIMResolutionInfoProperties = 2147483647;
pub const WIC8BIMResolutionInfoVResolution: WIC8BIMResolutionInfoProperties = 5;
pub const WIC8BIMResolutionInfoVResolutionUnit: WIC8BIMResolutionInfoProperties = 6;
pub const WIC8BIMResolutionInfoWidthUnit: WIC8BIMResolutionInfoProperties = 4;
pub const WICAllComponents: WICComponentType = 63;
pub const WICAsShotParameterSet: WICRawParameterSet = 1;
pub const WICAutoAdjustedParameterSet: WICRawParameterSet = 3;
pub const WICBITMAPALPHACHANNELOPTIONS_FORCE_DWORD: WICBitmapAlphaChannelOption = 2147483647;
pub const WICBITMAPCHAINTYPE_FORCE_DWORD: WICBitmapChainType = 2147483647;
pub const WICBITMAPCREATECACHEOPTION_FORCE_DWORD: WICBitmapCreateCacheOption = 2147483647;
pub const WICBITMAPDECODERCAPABILITIES_FORCE_DWORD: WICBitmapDecoderCapabilities = 2147483647;
pub const WICBITMAPDITHERTYPE_FORCE_DWORD: WICBitmapDitherType = 2147483647;
pub const WICBITMAPENCODERCACHEOPTION_FORCE_DWORD: WICBitmapEncoderCacheOption = 2147483647;
pub const WICBITMAPINTERPOLATIONMODE_FORCE_DWORD: WICBitmapInterpolationMode = 2147483647;
pub const WICBITMAPLOCKFLAGS_FORCE_DWORD: WICBitmapLockFlags = 2147483647;
pub const WICBITMAPPALETTETYPE_FORCE_DWORD: WICBitmapPaletteType = 2147483647;
pub const WICBITMAPTONEMAPPINGMODE_FORCE_DWORD: WICBitmapToneMappingMode = 2147483647;
pub const WICBITMAPTRANSFORMOPTIONS_FORCE_DWORD: WICBitmapTransformOptions = 2147483647;
pub type WICBitmapAlphaChannelOption = i32;
pub const WICBitmapCacheOnDemand: WICBitmapCreateCacheOption = 1;
pub const WICBitmapCacheOnLoad: WICBitmapCreateCacheOption = 2;
pub type WICBitmapChainType = i32;
pub const WICBitmapChainType_AlphaMap: WICBitmapChainType = 5;
pub const WICBitmapChainType_Alternate: WICBitmapChainType = 1;
pub const WICBitmapChainType_DepthMap: WICBitmapChainType = 6;
pub const WICBitmapChainType_GainMap: WICBitmapChainType = 7;
pub const WICBitmapChainType_Layer: WICBitmapChainType = 2;
pub const WICBitmapChainType_Preview: WICBitmapChainType = 3;
pub const WICBitmapChainType_Thumbnail: WICBitmapChainType = 4;
pub type WICBitmapCreateCacheOption = i32;
pub type WICBitmapDecoderCapabilities = i32;
pub const WICBitmapDecoderCapabilityCanDecodeAllImages: WICBitmapDecoderCapabilities = 2;
pub const WICBitmapDecoderCapabilityCanDecodeSomeImages: WICBitmapDecoderCapabilities = 4;
pub const WICBitmapDecoderCapabilityCanDecodeThumbnail: WICBitmapDecoderCapabilities = 16;
pub const WICBitmapDecoderCapabilityCanEnumerateMetadata: WICBitmapDecoderCapabilities = 8;
pub const WICBitmapDecoderCapabilitySameEncoder: WICBitmapDecoderCapabilities = 1;
pub type WICBitmapDitherType = i32;
pub const WICBitmapDitherTypeDualSpiral4x4: WICBitmapDitherType = 6;
pub const WICBitmapDitherTypeDualSpiral8x8: WICBitmapDitherType = 7;
pub const WICBitmapDitherTypeErrorDiffusion: WICBitmapDitherType = 8;
pub const WICBitmapDitherTypeNone: WICBitmapDitherType = 0;
pub const WICBitmapDitherTypeOrdered16x16: WICBitmapDitherType = 3;
pub const WICBitmapDitherTypeOrdered4x4: WICBitmapDitherType = 1;
pub const WICBitmapDitherTypeOrdered8x8: WICBitmapDitherType = 2;
pub const WICBitmapDitherTypeSolid: WICBitmapDitherType = 0;
pub const WICBitmapDitherTypeSpiral4x4: WICBitmapDitherType = 4;
pub const WICBitmapDitherTypeSpiral8x8: WICBitmapDitherType = 5;
pub const WICBitmapEncoderCacheInMemory: WICBitmapEncoderCacheOption = 0;
pub type WICBitmapEncoderCacheOption = i32;
pub const WICBitmapEncoderCacheTempFile: WICBitmapEncoderCacheOption = 1;
pub const WICBitmapEncoderNoCache: WICBitmapEncoderCacheOption = 2;
pub const WICBitmapIgnoreAlpha: WICBitmapAlphaChannelOption = 2;
pub type WICBitmapInterpolationMode = i32;
pub const WICBitmapInterpolationModeCubic: WICBitmapInterpolationMode = 2;
pub const WICBitmapInterpolationModeFant: WICBitmapInterpolationMode = 3;
pub const WICBitmapInterpolationModeHighQualityCubic: WICBitmapInterpolationMode = 4;
pub const WICBitmapInterpolationModeLinear: WICBitmapInterpolationMode = 1;
pub const WICBitmapInterpolationModeNearestNeighbor: WICBitmapInterpolationMode = 0;
pub type WICBitmapLockFlags = i32;
pub const WICBitmapLockRead: WICBitmapLockFlags = 1;
pub const WICBitmapLockWrite: WICBitmapLockFlags = 2;
pub const WICBitmapNoCache: WICBitmapCreateCacheOption = 0;
pub type WICBitmapPaletteType = i32;
pub const WICBitmapPaletteTypeCustom: WICBitmapPaletteType = 0;
pub const WICBitmapPaletteTypeFixedBW: WICBitmapPaletteType = 2;
pub const WICBitmapPaletteTypeFixedGray16: WICBitmapPaletteType = 11;
pub const WICBitmapPaletteTypeFixedGray256: WICBitmapPaletteType = 12;
pub const WICBitmapPaletteTypeFixedGray4: WICBitmapPaletteType = 10;
pub const WICBitmapPaletteTypeFixedHalftone125: WICBitmapPaletteType = 6;
pub const WICBitmapPaletteTypeFixedHalftone216: WICBitmapPaletteType = 7;
pub const WICBitmapPaletteTypeFixedHalftone252: WICBitmapPaletteType = 8;
pub const WICBitmapPaletteTypeFixedHalftone256: WICBitmapPaletteType = 9;
pub const WICBitmapPaletteTypeFixedHalftone27: WICBitmapPaletteType = 4;
pub const WICBitmapPaletteTypeFixedHalftone64: WICBitmapPaletteType = 5;
pub const WICBitmapPaletteTypeFixedHalftone8: WICBitmapPaletteType = 3;
pub const WICBitmapPaletteTypeFixedWebPalette: WICBitmapPaletteType = 7;
pub const WICBitmapPaletteTypeMedianCut: WICBitmapPaletteType = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WICBitmapPattern {
    pub Position: u64,
    pub Length: u32,
    pub Pattern: *mut u8,
    pub Mask: *mut u8,
    pub EndOfStream: windows_core::BOOL,
}
impl Default for WICBitmapPattern {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WICBitmapPlane {
    pub Format: WICPixelFormatGUID,
    pub pbBuffer: *mut u8,
    pub cbStride: u32,
    pub cbBufferSize: u32,
}
impl Default for WICBitmapPlane {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WICBitmapPlaneDescription {
    pub Format: WICPixelFormatGUID,
    pub Width: u32,
    pub Height: u32,
}
pub type WICBitmapToneMappingMode = i32;
pub const WICBitmapToneMappingMode_D2D: WICBitmapToneMappingMode = 2;
pub const WICBitmapToneMappingMode_Default: WICBitmapToneMappingMode = 1;
pub const WICBitmapToneMappingMode_GainMap: WICBitmapToneMappingMode = 3;
pub const WICBitmapToneMappingMode_None: WICBitmapToneMappingMode = 0;
pub const WICBitmapTransformFlipHorizontal: WICBitmapTransformOptions = 8;
pub const WICBitmapTransformFlipVertical: WICBitmapTransformOptions = 16;
pub type WICBitmapTransformOptions = i32;
pub const WICBitmapTransformRotate0: WICBitmapTransformOptions = 0;
pub const WICBitmapTransformRotate180: WICBitmapTransformOptions = 2;
pub const WICBitmapTransformRotate270: WICBitmapTransformOptions = 3;
pub const WICBitmapTransformRotate90: WICBitmapTransformOptions = 1;
pub const WICBitmapUseAlpha: WICBitmapAlphaChannelOption = 0;
pub const WICBitmapUsePremultipliedAlpha: WICBitmapAlphaChannelOption = 1;
pub const WICCOMPONENTENUMERATEOPTIONS_FORCE_DWORD: WICComponentEnumerateOptions = 2147483647;
pub const WICCOMPONENTSIGNING_FORCE_DWORD: WICComponentSigning = 2147483647;
pub const WICCOMPONENTTYPE_FORCE_DWORD: WICComponentType = 2147483647;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct WICColor(pub u32);
pub const WICColorContextExifColorSpace: WICColorContextType = 2;
pub const WICColorContextProfile: WICColorContextType = 1;
pub type WICColorContextType = i32;
pub const WICColorContextUninitialized: WICColorContextType = 0;
pub const WICComponentDisabled: WICComponentSigning = -2147483648;
pub const WICComponentEnumerateBuiltInOnly: WICComponentEnumerateOptions = 536870912;
pub const WICComponentEnumerateDefault: WICComponentEnumerateOptions = 0;
pub const WICComponentEnumerateDisabled: WICComponentEnumerateOptions = -2147483648;
pub type WICComponentEnumerateOptions = i32;
pub const WICComponentEnumerateRefresh: WICComponentEnumerateOptions = 1;
pub const WICComponentEnumerateUnsigned: WICComponentEnumerateOptions = 1073741824;
pub const WICComponentSafe: WICComponentSigning = 4;
pub const WICComponentSigned: WICComponentSigning = 1;
pub type WICComponentSigning = i32;
pub type WICComponentType = i32;
pub const WICComponentUnsigned: WICComponentSigning = 2;
pub const WICDDSALPHAMODE_FORCE_DWORD: WICDdsAlphaMode = 2147483647;
pub const WICDDSTEXTURE_FORCE_DWORD: WICDdsDimension = 2147483647;
pub type WICDdsAlphaMode = i32;
pub const WICDdsAlphaModeCustom: WICDdsAlphaMode = 4;
pub const WICDdsAlphaModeOpaque: WICDdsAlphaMode = 3;
pub const WICDdsAlphaModePremultiplied: WICDdsAlphaMode = 2;
pub const WICDdsAlphaModeStraight: WICDdsAlphaMode = 1;
pub const WICDdsAlphaModeUnknown: WICDdsAlphaMode = 0;
pub type WICDdsDimension = i32;
#[repr(C)]
#[cfg(feature = "dxgi")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WICDdsFormatInfo {
    pub DxgiFormat: super::dxgi::DXGI_FORMAT,
    pub BytesPerBlock: u32,
    pub BlockWidth: u32,
    pub BlockHeight: u32,
}
#[repr(C)]
#[cfg(feature = "dxgi")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WICDdsParameters {
    pub Width: u32,
    pub Height: u32,
    pub Depth: u32,
    pub MipLevels: u32,
    pub ArraySize: u32,
    pub DxgiFormat: super::dxgi::DXGI_FORMAT,
    pub Dimension: WICDdsDimension,
    pub AlphaMode: WICDdsAlphaMode,
}
pub const WICDdsTexture1D: WICDdsDimension = 0;
pub const WICDdsTexture2D: WICDdsDimension = 1;
pub const WICDdsTexture3D: WICDdsDimension = 2;
pub const WICDdsTextureCube: WICDdsDimension = 3;
pub const WICDecodeMetadataCacheOnDemand: WICDecodeOptions = 0;
pub const WICDecodeMetadataCacheOnLoad: WICDecodeOptions = 1;
pub type WICDecodeOptions = i32;
pub const WICDecoder: WICComponentType = 1;
pub const WICEncoder: WICComponentType = 2;
pub const WICGainMapMetadata: WICGainMapProperties = 1;
pub type WICGainMapProperties = i32;
pub const WICGainMapProperties_FORCE_DWORD: WICGainMapProperties = 2147483647;
pub const WICGifApplicationExtensionApplication: WICGifApplicationExtensionProperties = 1;
pub const WICGifApplicationExtensionData: WICGifApplicationExtensionProperties = 2;
pub type WICGifApplicationExtensionProperties = i32;
pub const WICGifApplicationExtensionProperties_FORCE_DWORD: WICGifApplicationExtensionProperties = 2147483647;
pub type WICGifCommentExtensionProperties = i32;
pub const WICGifCommentExtensionProperties_FORCE_DWORD: WICGifCommentExtensionProperties = 2147483647;
pub const WICGifCommentExtensionText: WICGifCommentExtensionProperties = 1;
pub const WICGifGraphicControlExtensionDelay: WICGifGraphicControlExtensionProperties = 4;
pub const WICGifGraphicControlExtensionDisposal: WICGifGraphicControlExtensionProperties = 1;
pub type WICGifGraphicControlExtensionProperties = i32;
pub const WICGifGraphicControlExtensionProperties_FORCE_DWORD: WICGifGraphicControlExtensionProperties = 2147483647;
pub const WICGifGraphicControlExtensionTransparencyFlag: WICGifGraphicControlExtensionProperties = 3;
pub const WICGifGraphicControlExtensionTransparentColorIndex: WICGifGraphicControlExtensionProperties = 5;
pub const WICGifGraphicControlExtensionUserInputFlag: WICGifGraphicControlExtensionProperties = 2;
pub const WICGifImageDescriptorHeight: WICGifImageDescriptorProperties = 4;
pub const WICGifImageDescriptorInterlaceFlag: WICGifImageDescriptorProperties = 6;
pub const WICGifImageDescriptorLeft: WICGifImageDescriptorProperties = 1;
pub const WICGifImageDescriptorLocalColorTableFlag: WICGifImageDescriptorProperties = 5;
pub const WICGifImageDescriptorLocalColorTableSize: WICGifImageDescriptorProperties = 8;
pub type WICGifImageDescriptorProperties = i32;
pub const WICGifImageDescriptorProperties_FORCE_DWORD: WICGifImageDescriptorProperties = 2147483647;
pub const WICGifImageDescriptorSortFlag: WICGifImageDescriptorProperties = 7;
pub const WICGifImageDescriptorTop: WICGifImageDescriptorProperties = 2;
pub const WICGifImageDescriptorWidth: WICGifImageDescriptorProperties = 3;
pub const WICGifLogicalScreenDescriptorBackgroundColorIndex: WICGifLogicalScreenDescriptorProperties = 8;
pub const WICGifLogicalScreenDescriptorColorResolution: WICGifLogicalScreenDescriptorProperties = 5;
pub const WICGifLogicalScreenDescriptorGlobalColorTableFlag: WICGifLogicalScreenDescriptorProperties = 4;
pub const WICGifLogicalScreenDescriptorGlobalColorTableSize: WICGifLogicalScreenDescriptorProperties = 7;
pub const WICGifLogicalScreenDescriptorHeight: WICGifLogicalScreenDescriptorProperties = 3;
pub const WICGifLogicalScreenDescriptorPixelAspectRatio: WICGifLogicalScreenDescriptorProperties = 9;
pub type WICGifLogicalScreenDescriptorProperties = i32;
pub const WICGifLogicalScreenDescriptorProperties_FORCE_DWORD: WICGifLogicalScreenDescriptorProperties = 2147483647;
pub const WICGifLogicalScreenDescriptorSortFlag: WICGifLogicalScreenDescriptorProperties = 6;
pub const WICGifLogicalScreenDescriptorWidth: WICGifLogicalScreenDescriptorProperties = 2;
pub const WICGifLogicalScreenSignature: WICGifLogicalScreenDescriptorProperties = 1;
pub const WICHEIFCOMPRESSIONOPTION_FORCE_DWORD: WICHeifCompressionOption = 2147483647;
pub const WICHeifCompressionAV1: WICHeifCompressionOption = 3;
pub const WICHeifCompressionBrotli: WICHeifCompressionOption = 5;
pub const WICHeifCompressionDeflate: WICHeifCompressionOption = 6;
pub const WICHeifCompressionDontCare: WICHeifCompressionOption = 0;
pub const WICHeifCompressionHEVC: WICHeifCompressionOption = 2;
pub const WICHeifCompressionJpegXL: WICHeifCompressionOption = 4;
pub const WICHeifCompressionNone: WICHeifCompressionOption = 1;
pub type WICHeifCompressionOption = i32;
pub const WICHeifHdrCustomVideoPrimaries: WICHeifHdrProperties = 5;
pub const WICHeifHdrMaximumFrameAverageLuminanceLevel: WICHeifHdrProperties = 2;
pub const WICHeifHdrMaximumLuminanceLevel: WICHeifHdrProperties = 1;
pub const WICHeifHdrMaximumMasteringDisplayLuminanceLevel: WICHeifHdrProperties = 4;
pub const WICHeifHdrMinimumMasteringDisplayLuminanceLevel: WICHeifHdrProperties = 3;
pub type WICHeifHdrProperties = i32;
pub const WICHeifHdrProperties_FORCE_DWORD: WICHeifHdrProperties = 2147483647;
pub const WICHeifLayeredImageCanvasColor: WICHeifProperties = 2;
pub const WICHeifLayeredImageLayerPositions: WICHeifProperties = 3;
pub const WICHeifOrientation: WICHeifProperties = 1;
pub type WICHeifProperties = i32;
pub const WICHeifProperties_FORCE_DWORD: WICHeifProperties = 2147483647;
#[repr(C)]
#[cfg(all(feature = "dcommon", feature = "dxgi"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WICImageParameters {
    pub PixelFormat: super::dcommon::D2D1_PIXEL_FORMAT,
    pub DpiX: f32,
    pub DpiY: f32,
    pub Top: f32,
    pub Left: f32,
    pub PixelWidth: u32,
    pub PixelHeight: u32,
}
pub type WICInProcPointer = *mut u8;
pub const WICJPEGYCRCBSUBSAMPLING_FORCE_DWORD: WICJpegYCrCbSubsamplingOption = 2147483647;
pub type WICJpegChrominanceProperties = i32;
pub const WICJpegChrominanceProperties_FORCE_DWORD: WICJpegChrominanceProperties = 2147483647;
pub const WICJpegChrominanceTable: WICJpegChrominanceProperties = 1;
pub type WICJpegCommentProperties = i32;
pub const WICJpegCommentProperties_FORCE_DWORD: WICJpegCommentProperties = 2147483647;
pub const WICJpegCommentText: WICJpegCommentProperties = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WICJpegFrameHeader {
    pub Width: u32,
    pub Height: u32,
    pub TransferMatrix: WICJpegTransferMatrix,
    pub ScanType: WICJpegScanType,
    pub cComponents: u32,
    pub ComponentIdentifiers: u32,
    pub SampleFactors: u32,
    pub QuantizationTableIndices: u32,
}
pub type WICJpegIndexingOptions = i32;
pub const WICJpegIndexingOptionsGenerateOnDemand: WICJpegIndexingOptions = 0;
pub const WICJpegIndexingOptionsGenerateOnLoad: WICJpegIndexingOptions = 1;
pub const WICJpegIndexingOptions_FORCE_DWORD: WICJpegIndexingOptions = 2147483647;
pub type WICJpegLuminanceProperties = i32;
pub const WICJpegLuminanceProperties_FORCE_DWORD: WICJpegLuminanceProperties = 2147483647;
pub const WICJpegLuminanceTable: WICJpegLuminanceProperties = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WICJpegScanHeader {
    pub cComponents: u32,
    pub RestartInterval: u32,
    pub ComponentSelectors: u32,
    pub HuffmanTableIndices: u32,
    pub StartSpectralSelection: u8,
    pub EndSpectralSelection: u8,
    pub SuccessiveApproximationHigh: u8,
    pub SuccessiveApproximationLow: u8,
}
pub type WICJpegScanType = i32;
pub const WICJpegScanTypeInterleaved: WICJpegScanType = 0;
pub const WICJpegScanTypePlanarComponents: WICJpegScanType = 1;
pub const WICJpegScanTypeProgressive: WICJpegScanType = 2;
pub const WICJpegScanType_FORCE_DWORD: WICJpegScanType = 2147483647;
pub type WICJpegTransferMatrix = i32;
pub const WICJpegTransferMatrixBT601: WICJpegTransferMatrix = 1;
pub const WICJpegTransferMatrixIdentity: WICJpegTransferMatrix = 0;
pub const WICJpegTransferMatrix_FORCE_DWORD: WICJpegTransferMatrix = 2147483647;
pub const WICJpegXLAnimFrameDurationInTicks: WICJpegXLAnimFrameProperties = 1;
pub const WICJpegXLAnimFrameName: WICJpegXLAnimFrameProperties = 2;
pub type WICJpegXLAnimFrameProperties = i32;
pub const WICJpegXLAnimFrameProperties_FORCE_DWORD: WICJpegXLAnimFrameProperties = 2147483647;
pub const WICJpegXLAnimFrameTicksPerSecondDenominator: WICJpegXLAnimProperties = 3;
pub const WICJpegXLAnimFrameTicksPerSecondNumerator: WICJpegXLAnimProperties = 2;
pub const WICJpegXLAnimLoopCount: WICJpegXLAnimProperties = 1;
pub type WICJpegXLAnimProperties = i32;
pub const WICJpegXLAnimProperties_FORCE_DWORD: WICJpegXLAnimProperties = 2147483647;
pub const WICJpegYCrCbSubsampling420: WICJpegYCrCbSubsamplingOption = 1;
pub const WICJpegYCrCbSubsampling422: WICJpegYCrCbSubsamplingOption = 2;
pub const WICJpegYCrCbSubsampling440: WICJpegYCrCbSubsamplingOption = 4;
pub const WICJpegYCrCbSubsampling444: WICJpegYCrCbSubsamplingOption = 3;
pub const WICJpegYCrCbSubsamplingDefault: WICJpegYCrCbSubsamplingOption = 0;
pub type WICJpegYCrCbSubsamplingOption = i32;
pub const WICMETADATACACHEOPTION_FORCE_DWORD: WICDecodeOptions = 2147483647;
pub const WICMetadataReader: WICComponentType = 8;
pub const WICMetadataWriter: WICComponentType = 16;
pub const WICNAMEDWHITEPOINT_FORCE_DWORD: WICNamedWhitePoint = 2147483647;
pub type WICNamedWhitePoint = i32;
pub const WICPLANAROPTIONS_FORCE_DWORD: WICPlanarOptions = 2147483647;
pub const WICPNGFILTEROPTION_FORCE_DWORD: WICPngFilterOption = 2147483647;
pub const WICPROGRESSNOTIFICATION_FORCE_DWORD: WICProgressNotification = 2147483647;
pub const WICPROGRESSOPERATION_FORCE_DWORD: WICProgressOperation = 2147483647;
pub const WICPixelFormat: WICComponentType = 32;
pub const WICPixelFormatConverter: WICComponentType = 4;
pub type WICPixelFormatGUID = windows_core::GUID;
pub type WICPixelFormatNumericRepresentation = i32;
pub const WICPixelFormatNumericRepresentationFixed: WICPixelFormatNumericRepresentation = 4;
pub const WICPixelFormatNumericRepresentationFloat: WICPixelFormatNumericRepresentation = 5;
pub const WICPixelFormatNumericRepresentationIndexed: WICPixelFormatNumericRepresentation = 1;
pub const WICPixelFormatNumericRepresentationSignedInteger: WICPixelFormatNumericRepresentation = 3;
pub const WICPixelFormatNumericRepresentationUnsignedInteger: WICPixelFormatNumericRepresentation = 2;
pub const WICPixelFormatNumericRepresentationUnspecified: WICPixelFormatNumericRepresentation = 0;
pub const WICPixelFormatNumericRepresentation_FORCE_DWORD: WICPixelFormatNumericRepresentation = 2147483647;
pub type WICPlanarOptions = i32;
pub const WICPlanarOptionsDefault: WICPlanarOptions = 0;
pub const WICPlanarOptionsPreserveSubsampling: WICPlanarOptions = 1;
pub const WICPngBkgdBackgroundColor: WICPngBkgdProperties = 1;
pub type WICPngBkgdProperties = i32;
pub const WICPngBkgdProperties_FORCE_DWORD: WICPngBkgdProperties = 2147483647;
pub const WICPngChrmBlueX: WICPngChrmProperties = 7;
pub const WICPngChrmBlueY: WICPngChrmProperties = 8;
pub const WICPngChrmGreenX: WICPngChrmProperties = 5;
pub const WICPngChrmGreenY: WICPngChrmProperties = 6;
pub type WICPngChrmProperties = i32;
pub const WICPngChrmProperties_FORCE_DWORD: WICPngChrmProperties = 2147483647;
pub const WICPngChrmRedX: WICPngChrmProperties = 3;
pub const WICPngChrmRedY: WICPngChrmProperties = 4;
pub const WICPngChrmWhitePointX: WICPngChrmProperties = 1;
pub const WICPngChrmWhitePointY: WICPngChrmProperties = 2;
pub const WICPngFilterAdaptive: WICPngFilterOption = 6;
pub const WICPngFilterAverage: WICPngFilterOption = 4;
pub const WICPngFilterNone: WICPngFilterOption = 1;
pub type WICPngFilterOption = i32;
pub const WICPngFilterPaeth: WICPngFilterOption = 5;
pub const WICPngFilterSub: WICPngFilterOption = 2;
pub const WICPngFilterUnspecified: WICPngFilterOption = 0;
pub const WICPngFilterUp: WICPngFilterOption = 3;
pub const WICPngGamaGamma: WICPngGamaProperties = 1;
pub type WICPngGamaProperties = i32;
pub const WICPngGamaProperties_FORCE_DWORD: WICPngGamaProperties = 2147483647;
pub const WICPngHistFrequencies: WICPngHistProperties = 1;
pub type WICPngHistProperties = i32;
pub const WICPngHistProperties_FORCE_DWORD: WICPngHistProperties = 2147483647;
pub const WICPngIccpProfileData: WICPngIccpProperties = 2;
pub const WICPngIccpProfileName: WICPngIccpProperties = 1;
pub type WICPngIccpProperties = i32;
pub const WICPngIccpProperties_FORCE_DWORD: WICPngIccpProperties = 2147483647;
pub const WICPngItxtCompressionFlag: WICPngItxtProperties = 2;
pub const WICPngItxtKeyword: WICPngItxtProperties = 1;
pub const WICPngItxtLanguageTag: WICPngItxtProperties = 3;
pub type WICPngItxtProperties = i32;
pub const WICPngItxtProperties_FORCE_DWORD: WICPngItxtProperties = 2147483647;
pub const WICPngItxtText: WICPngItxtProperties = 5;
pub const WICPngItxtTranslatedKeyword: WICPngItxtProperties = 4;
pub type WICPngSrgbProperties = i32;
pub const WICPngSrgbProperties_FORCE_DWORD: WICPngSrgbProperties = 2147483647;
pub const WICPngSrgbRenderingIntent: WICPngSrgbProperties = 1;
pub const WICPngTimeDay: WICPngTimeProperties = 3;
pub const WICPngTimeHour: WICPngTimeProperties = 4;
pub const WICPngTimeMinute: WICPngTimeProperties = 5;
pub const WICPngTimeMonth: WICPngTimeProperties = 2;
pub type WICPngTimeProperties = i32;
pub const WICPngTimeProperties_FORCE_DWORD: WICPngTimeProperties = 2147483647;
pub const WICPngTimeSecond: WICPngTimeProperties = 6;
pub const WICPngTimeYear: WICPngTimeProperties = 1;
pub type WICProgressNotification = i32;
pub const WICProgressNotificationAll: WICProgressNotification = -65536;
pub const WICProgressNotificationBegin: WICProgressNotification = 65536;
pub const WICProgressNotificationEnd: WICProgressNotification = 131072;
pub const WICProgressNotificationFrequent: WICProgressNotification = 262144;
pub type WICProgressOperation = i32;
pub const WICProgressOperationAll: WICProgressOperation = 65535;
pub const WICProgressOperationCopyPixels: WICProgressOperation = 1;
pub const WICProgressOperationWritePixels: WICProgressOperation = 2;
pub const WICRAWCAPABILITIES_FORCE_DWORD: WICRawCapabilities = 2147483647;
pub const WICRAWPARAMETERSET_FORCE_DWORD: WICRawParameterSet = 2147483647;
pub const WICRAWRENDERMODE_FORCE_DWORD: WICRawRenderMode = 2147483647;
pub const WICRAWROTATIONCAPABILITIES_FORCE_DWORD: WICRawRotationCapabilities = 2147483647;
pub type WICRawCapabilities = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WICRawCapabilitiesInfo {
    pub cbSize: u32,
    pub CodecMajorVersion: u32,
    pub CodecMinorVersion: u32,
    pub ExposureCompensationSupport: WICRawCapabilities,
    pub ContrastSupport: WICRawCapabilities,
    pub RGBWhitePointSupport: WICRawCapabilities,
    pub NamedWhitePointSupport: WICRawCapabilities,
    pub NamedWhitePointSupportMask: u32,
    pub KelvinWhitePointSupport: WICRawCapabilities,
    pub GammaSupport: WICRawCapabilities,
    pub TintSupport: WICRawCapabilities,
    pub SaturationSupport: WICRawCapabilities,
    pub SharpnessSupport: WICRawCapabilities,
    pub NoiseReductionSupport: WICRawCapabilities,
    pub DestinationColorProfileSupport: WICRawCapabilities,
    pub ToneCurveSupport: WICRawCapabilities,
    pub RotationSupport: WICRawRotationCapabilities,
    pub RenderModeSupport: WICRawCapabilities,
}
pub const WICRawCapabilityFullySupported: WICRawCapabilities = 2;
pub const WICRawCapabilityGetSupported: WICRawCapabilities = 1;
pub const WICRawCapabilityNotSupported: WICRawCapabilities = 0;
pub const WICRawChangeNotification_Contrast: u32 = 16;
pub const WICRawChangeNotification_DestinationColorContext: u32 = 1024;
pub const WICRawChangeNotification_ExposureCompensation: u32 = 1;
pub const WICRawChangeNotification_Gamma: u32 = 32;
pub const WICRawChangeNotification_KelvinWhitePoint: u32 = 4;
pub const WICRawChangeNotification_NamedWhitePoint: u32 = 2;
pub const WICRawChangeNotification_NoiseReduction: u32 = 512;
pub const WICRawChangeNotification_RGBWhitePoint: u32 = 8;
pub const WICRawChangeNotification_RenderMode: u32 = 8192;
pub const WICRawChangeNotification_Rotation: u32 = 4096;
pub const WICRawChangeNotification_Saturation: u32 = 128;
pub const WICRawChangeNotification_Sharpness: u32 = 64;
pub const WICRawChangeNotification_Tint: u32 = 256;
pub const WICRawChangeNotification_ToneCurve: u32 = 2048;
pub type WICRawParameterSet = i32;
pub type WICRawRenderMode = i32;
pub const WICRawRenderModeBestQuality: WICRawRenderMode = 3;
pub const WICRawRenderModeDraft: WICRawRenderMode = 1;
pub const WICRawRenderModeNormal: WICRawRenderMode = 2;
pub type WICRawRotationCapabilities = i32;
pub const WICRawRotationCapabilityFullySupported: WICRawRotationCapabilities = 3;
pub const WICRawRotationCapabilityGetSupported: WICRawRotationCapabilities = 1;
pub const WICRawRotationCapabilityNinetyDegreesSupported: WICRawRotationCapabilities = 2;
pub const WICRawRotationCapabilityNotSupported: WICRawRotationCapabilities = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WICRawToneCurve {
    pub cPoints: u32,
    pub aPoints: [WICRawToneCurvePoint; 1],
}
impl Default for WICRawToneCurve {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WICRawToneCurvePoint {
    pub Input: f64,
    pub Output: f64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WICRect {
    pub X: i32,
    pub Y: i32,
    pub Width: i32,
    pub Height: i32,
}
pub type WICSectionAccessLevel = i32;
pub const WICSectionAccessLevelRead: WICSectionAccessLevel = 1;
pub const WICSectionAccessLevelReadWrite: WICSectionAccessLevel = 3;
pub const WICSectionAccessLevel_FORCE_DWORD: WICSectionAccessLevel = 2147483647;
pub const WICTIFFCOMPRESSIONOPTION_FORCE_DWORD: WICTiffCompressionOption = 2147483647;
pub const WICTiffCompressionCCITT3: WICTiffCompressionOption = 2;
pub const WICTiffCompressionCCITT4: WICTiffCompressionOption = 3;
pub const WICTiffCompressionDontCare: WICTiffCompressionOption = 0;
pub const WICTiffCompressionLZW: WICTiffCompressionOption = 4;
pub const WICTiffCompressionLZWHDifferencing: WICTiffCompressionOption = 7;
pub const WICTiffCompressionNone: WICTiffCompressionOption = 1;
pub type WICTiffCompressionOption = i32;
pub const WICTiffCompressionRLE: WICTiffCompressionOption = 5;
pub const WICTiffCompressionZIP: WICTiffCompressionOption = 6;
pub const WICUserAdjustedParameterSet: WICRawParameterSet = 2;
pub const WICWebpAnimLoopCount: WICWebpAnimProperties = 1;
pub type WICWebpAnimProperties = i32;
pub const WICWebpAnimProperties_FORCE_DWORD: WICWebpAnimProperties = 2147483647;
pub const WICWebpAnmfFrameDuration: WICWebpAnmfProperties = 1;
pub type WICWebpAnmfProperties = i32;
pub const WICWebpAnmfProperties_FORCE_DWORD: WICWebpAnmfProperties = 2147483647;
pub const WICWhitePointAsShot: WICNamedWhitePoint = 1;
pub const WICWhitePointAutoWhiteBalance: WICNamedWhitePoint = 512;
pub const WICWhitePointCloudy: WICNamedWhitePoint = 4;
pub const WICWhitePointCustom: WICNamedWhitePoint = 256;
pub const WICWhitePointDaylight: WICNamedWhitePoint = 2;
pub const WICWhitePointDefault: WICNamedWhitePoint = 1;
pub const WICWhitePointFlash: WICNamedWhitePoint = 64;
pub const WICWhitePointFluorescent: WICNamedWhitePoint = 32;
pub const WICWhitePointShade: WICNamedWhitePoint = 8;
pub const WICWhitePointTungsten: WICNamedWhitePoint = 16;
pub const WICWhitePointUnderwater: WICNamedWhitePoint = 128;
pub const WIC_JPEG_HUFFMAN_BASELINE_ONE: u32 = 0;
pub const WIC_JPEG_HUFFMAN_BASELINE_THREE: u32 = 1118464;
pub const WIC_JPEG_MAX_COMPONENT_COUNT: u32 = 4;
pub const WIC_JPEG_MAX_TABLE_INDEX: u32 = 3;
pub const WIC_JPEG_QUANTIZATION_BASELINE_ONE: u32 = 0;
pub const WIC_JPEG_QUANTIZATION_BASELINE_THREE: u32 = 65792;
pub const WIC_JPEG_SAMPLE_FACTORS_ONE: u32 = 17;
pub const WIC_JPEG_SAMPLE_FACTORS_THREE_420: u32 = 1118498;
pub const WIC_JPEG_SAMPLE_FACTORS_THREE_422: u32 = 1118497;
pub const WIC_JPEG_SAMPLE_FACTORS_THREE_440: u32 = 1118482;
pub const WIC_JPEG_SAMPLE_FACTORS_THREE_444: u32 = 1118481;
pub const WINCODEC_ERR_ABORTED: i32 = -2147467260;
pub const WINCODEC_ERR_ACCESSDENIED: i32 = -2147024891;
pub const WINCODEC_ERR_BASE: u32 = 8192;
pub const WINCODEC_ERR_GENERIC_ERROR: i32 = -2147467259;
pub const WINCODEC_ERR_INVALIDPARAMETER: i32 = -2147024809;
pub const WINCODEC_ERR_NOTIMPLEMENTED: i32 = -2147467263;
pub const WINCODEC_ERR_OUTOFMEMORY: i32 = -2147024882;
pub const WINCODEC_ERR_VALUEOVERFLOW: i32 = -2147024362;
pub const WINCODEC_SDK_VERSION: u32 = 567;
pub const WINCODEC_SDK_VERSION1: u32 = 566;
pub const WINCODEC_SDK_VERSION2: u32 = 567;
