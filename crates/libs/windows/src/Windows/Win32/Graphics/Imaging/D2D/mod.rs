windows_core::imp::define_interface!(IWICImageEncoder, IWICImageEncoder_Vtbl, 0x04c75bf8_3ce1_473b_acc5_3cc4f5e94999);
windows_core::imp::interface_hierarchy!(IWICImageEncoder, windows_core::IUnknown);
impl IWICImageEncoder {
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn WriteFrame<P0, P1>(&self, pimage: P0, pframeencode: P1, pimageparameters: *const super::WICImageParameters) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Direct2D::ID2D1Image>,
        P1: windows_core::Param<super::IWICBitmapFrameEncode>,
    {
        (windows_core::Interface::vtable(self).WriteFrame)(windows_core::Interface::as_raw(self), pimage.param().abi(), pframeencode.param().abi(), pimageparameters).ok()
    }
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn WriteFrameThumbnail<P0, P1>(&self, pimage: P0, pframeencode: P1, pimageparameters: *const super::WICImageParameters) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Direct2D::ID2D1Image>,
        P1: windows_core::Param<super::IWICBitmapFrameEncode>,
    {
        (windows_core::Interface::vtable(self).WriteFrameThumbnail)(windows_core::Interface::as_raw(self), pimage.param().abi(), pframeencode.param().abi(), pimageparameters).ok()
    }
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn WriteThumbnail<P0, P1>(&self, pimage: P0, pencoder: P1, pimageparameters: *const super::WICImageParameters) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Direct2D::ID2D1Image>,
        P1: windows_core::Param<super::IWICBitmapEncoder>,
    {
        (windows_core::Interface::vtable(self).WriteThumbnail)(windows_core::Interface::as_raw(self), pimage.param().abi(), pencoder.param().abi(), pimageparameters).ok()
    }
}
#[repr(C)]
pub struct IWICImageEncoder_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub WriteFrame: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *const super::WICImageParameters) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common")))]
    WriteFrame: usize,
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub WriteFrameThumbnail: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *const super::WICImageParameters) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common")))]
    WriteFrameThumbnail: usize,
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub WriteThumbnail: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *const super::WICImageParameters) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common")))]
    WriteThumbnail: usize,
}
windows_core::imp::define_interface!(IWICImagingFactory2, IWICImagingFactory2_Vtbl, 0x7b816b45_1996_4476_b132_de9e247c8af0);
windows_core::imp::interface_hierarchy!(IWICImagingFactory2, windows_core::IUnknown, super::IWICImagingFactory);
impl IWICImagingFactory2 {
    pub unsafe fn CreateDecoderFromFilename<P0>(&self, wzfilename: P0, pguidvendor: Option<*const windows_core::GUID>, dwdesiredaccess: super::super::super::Foundation::GENERIC_ACCESS_RIGHTS, metadataoptions: super::WICDecodeOptions) -> windows_core::Result<super::IWICBitmapDecoder>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = std::mem::zeroed();
        (windows_core::Interface::vtable(self).base__.CreateDecoderFromFilename)(windows_core::Interface::as_raw(self), wzfilename.param().abi(), core::mem::transmute(pguidvendor.unwrap_or(std::ptr::null())), dwdesiredaccess, metadataoptions, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateDecoderFromStream<P0>(&self, pistream: P0, pguidvendor: *const windows_core::GUID, metadataoptions: super::WICDecodeOptions) -> windows_core::Result<super::IWICBitmapDecoder>
    where
        P0: windows_core::Param<super::super::super::System::Com::IStream>,
    {
        let mut result__ = std::mem::zeroed();
        (windows_core::Interface::vtable(self).base__.CreateDecoderFromStream)(windows_core::Interface::as_raw(self), pistream.param().abi(), pguidvendor, metadataoptions, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateDecoderFromFileHandle(&self, hfile: usize, pguidvendor: *const windows_core::GUID, metadataoptions: super::WICDecodeOptions) -> windows_core::Result<super::IWICBitmapDecoder> {
        let mut result__ = std::mem::zeroed();
        (windows_core::Interface::vtable(self).base__.CreateDecoderFromFileHandle)(windows_core::Interface::as_raw(self), hfile, pguidvendor, metadataoptions, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateComponentInfo(&self, clsidcomponent: *const windows_core::GUID) -> windows_core::Result<super::IWICComponentInfo> {
        let mut result__ = std::mem::zeroed();
        (windows_core::Interface::vtable(self).base__.CreateComponentInfo)(windows_core::Interface::as_raw(self), clsidcomponent, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateDecoder(&self, guidcontainerformat: *const windows_core::GUID, pguidvendor: *const windows_core::GUID) -> windows_core::Result<super::IWICBitmapDecoder> {
        let mut result__ = std::mem::zeroed();
        (windows_core::Interface::vtable(self).base__.CreateDecoder)(windows_core::Interface::as_raw(self), guidcontainerformat, pguidvendor, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateEncoder(&self, guidcontainerformat: *const windows_core::GUID, pguidvendor: *const windows_core::GUID) -> windows_core::Result<super::IWICBitmapEncoder> {
        let mut result__ = std::mem::zeroed();
        (windows_core::Interface::vtable(self).base__.CreateEncoder)(windows_core::Interface::as_raw(self), guidcontainerformat, pguidvendor, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreatePalette(&self) -> windows_core::Result<super::IWICPalette> {
        let mut result__ = std::mem::zeroed();
        (windows_core::Interface::vtable(self).base__.CreatePalette)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateFormatConverter(&self) -> windows_core::Result<super::IWICFormatConverter> {
        let mut result__ = std::mem::zeroed();
        (windows_core::Interface::vtable(self).base__.CreateFormatConverter)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateBitmapScaler(&self) -> windows_core::Result<super::IWICBitmapScaler> {
        let mut result__ = std::mem::zeroed();
        (windows_core::Interface::vtable(self).base__.CreateBitmapScaler)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateBitmapClipper(&self) -> windows_core::Result<super::IWICBitmapClipper> {
        let mut result__ = std::mem::zeroed();
        (windows_core::Interface::vtable(self).base__.CreateBitmapClipper)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateBitmapFlipRotator(&self) -> windows_core::Result<super::IWICBitmapFlipRotator> {
        let mut result__ = std::mem::zeroed();
        (windows_core::Interface::vtable(self).base__.CreateBitmapFlipRotator)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateStream(&self) -> windows_core::Result<super::IWICStream> {
        let mut result__ = std::mem::zeroed();
        (windows_core::Interface::vtable(self).base__.CreateStream)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateColorContext(&self) -> windows_core::Result<super::IWICColorContext> {
        let mut result__ = std::mem::zeroed();
        (windows_core::Interface::vtable(self).base__.CreateColorContext)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateColorTransformer(&self) -> windows_core::Result<super::IWICColorTransform> {
        let mut result__ = std::mem::zeroed();
        (windows_core::Interface::vtable(self).base__.CreateColorTransformer)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateBitmap(&self, uiwidth: u32, uiheight: u32, pixelformat: *const windows_core::GUID, option: super::WICBitmapCreateCacheOption) -> windows_core::Result<super::IWICBitmap> {
        let mut result__ = std::mem::zeroed();
        (windows_core::Interface::vtable(self).base__.CreateBitmap)(windows_core::Interface::as_raw(self), uiwidth, uiheight, pixelformat, option, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateBitmapFromSource<P0>(&self, pibitmapsource: P0, option: super::WICBitmapCreateCacheOption) -> windows_core::Result<super::IWICBitmap>
    where
        P0: windows_core::Param<super::IWICBitmapSource>,
    {
        let mut result__ = std::mem::zeroed();
        (windows_core::Interface::vtable(self).base__.CreateBitmapFromSource)(windows_core::Interface::as_raw(self), pibitmapsource.param().abi(), option, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateBitmapFromSourceRect<P0>(&self, pibitmapsource: P0, x: u32, y: u32, width: u32, height: u32) -> windows_core::Result<super::IWICBitmap>
    where
        P0: windows_core::Param<super::IWICBitmapSource>,
    {
        let mut result__ = std::mem::zeroed();
        (windows_core::Interface::vtable(self).base__.CreateBitmapFromSourceRect)(windows_core::Interface::as_raw(self), pibitmapsource.param().abi(), x, y, width, height, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateBitmapFromMemory(&self, uiwidth: u32, uiheight: u32, pixelformat: *const windows_core::GUID, cbstride: u32, pbbuffer: &[u8]) -> windows_core::Result<super::IWICBitmap> {
        let mut result__ = std::mem::zeroed();
        (windows_core::Interface::vtable(self).base__.CreateBitmapFromMemory)(windows_core::Interface::as_raw(self), uiwidth, uiheight, pixelformat, cbstride, pbbuffer.len().try_into().unwrap(), core::mem::transmute(pbbuffer.as_ptr()), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn CreateBitmapFromHBITMAP<P0, P1>(&self, hbitmap: P0, hpalette: P1, options: super::WICBitmapAlphaChannelOption) -> windows_core::Result<super::IWICBitmap>
    where
        P0: windows_core::Param<super::super::Gdi::HBITMAP>,
        P1: windows_core::Param<super::super::Gdi::HPALETTE>,
    {
        let mut result__ = std::mem::zeroed();
        (windows_core::Interface::vtable(self).base__.CreateBitmapFromHBITMAP)(windows_core::Interface::as_raw(self), hbitmap.param().abi(), hpalette.param().abi(), options, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub unsafe fn CreateBitmapFromHICON<P0>(&self, hicon: P0) -> windows_core::Result<super::IWICBitmap>
    where
        P0: windows_core::Param<super::super::super::UI::WindowsAndMessaging::HICON>,
    {
        let mut result__ = std::mem::zeroed();
        (windows_core::Interface::vtable(self).base__.CreateBitmapFromHICON)(windows_core::Interface::as_raw(self), hicon.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateComponentEnumerator(&self, componenttypes: u32, options: u32) -> windows_core::Result<super::super::super::System::Com::IEnumUnknown> {
        let mut result__ = std::mem::zeroed();
        (windows_core::Interface::vtable(self).base__.CreateComponentEnumerator)(windows_core::Interface::as_raw(self), componenttypes, options, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateFastMetadataEncoderFromDecoder<P0>(&self, pidecoder: P0) -> windows_core::Result<super::IWICFastMetadataEncoder>
    where
        P0: windows_core::Param<super::IWICBitmapDecoder>,
    {
        let mut result__ = std::mem::zeroed();
        (windows_core::Interface::vtable(self).base__.CreateFastMetadataEncoderFromDecoder)(windows_core::Interface::as_raw(self), pidecoder.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateFastMetadataEncoderFromFrameDecode<P0>(&self, piframedecoder: P0) -> windows_core::Result<super::IWICFastMetadataEncoder>
    where
        P0: windows_core::Param<super::IWICBitmapFrameDecode>,
    {
        let mut result__ = std::mem::zeroed();
        (windows_core::Interface::vtable(self).base__.CreateFastMetadataEncoderFromFrameDecode)(windows_core::Interface::as_raw(self), piframedecoder.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateQueryWriter(&self, guidmetadataformat: *const windows_core::GUID, pguidvendor: *const windows_core::GUID) -> windows_core::Result<super::IWICMetadataQueryWriter> {
        let mut result__ = std::mem::zeroed();
        (windows_core::Interface::vtable(self).base__.CreateQueryWriter)(windows_core::Interface::as_raw(self), guidmetadataformat, pguidvendor, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateQueryWriterFromReader<P0>(&self, piqueryreader: P0, pguidvendor: *const windows_core::GUID) -> windows_core::Result<super::IWICMetadataQueryWriter>
    where
        P0: windows_core::Param<super::IWICMetadataQueryReader>,
    {
        let mut result__ = std::mem::zeroed();
        (windows_core::Interface::vtable(self).base__.CreateQueryWriterFromReader)(windows_core::Interface::as_raw(self), piqueryreader.param().abi(), pguidvendor, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_Graphics_Direct2D")]
    pub unsafe fn CreateImageEncoder<P0>(&self, pd2ddevice: P0) -> windows_core::Result<IWICImageEncoder>
    where
        P0: windows_core::Param<super::super::Direct2D::ID2D1Device>,
    {
        let mut result__ = std::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateImageEncoder)(windows_core::Interface::as_raw(self), pd2ddevice.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IWICImagingFactory2_Vtbl {
    pub base__: super::IWICImagingFactory_Vtbl,
    #[cfg(feature = "Win32_Graphics_Direct2D")]
    pub CreateImageEncoder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D"))]
    CreateImageEncoder: usize,
}
#[cfg(feature = "implement")]
core::include!("impl.rs");
