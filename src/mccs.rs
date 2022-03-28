// Full list of MCCS v2.2a VCP codes.

#[repr(u8)]
pub enum PresetOperation {
    CodePage = 0x00,
    RestoreFactoryColorDefaults = 0x08,
    RestoreFactoryDefaults = 0x04,
    RestoreFactoryGeometryDefaults = 0x06,
    RestoreFactoryLuminanceContrastDefaults = 0x05,
    RestoreFactoryTvDefaults = 0x0a,
    SaveRestoreSettings = 0xb0,
}

#[repr(u8)]
pub enum ImageAdjustment {
    SixAxisHueControlBlue = 0x9f,
    SixAxisHueControlCyan = 0x9e,
    SixAxisHueControlGreen = 0x9d,
    SixAxisHueControlMagenta = 0xa0,
    SixAxisHueControlRed = 0x9b,
    SixAxisHueControlYellow = 0x9c,
    SixAxisSaturationControlBlue = 0x5d,
    SixAxisSaturationControlCyan = 0x5c,
    SixAxisSaturationControlGreen = 0x5b,
    SixAxisSaturationControlMagenta = 0x5e,
    SixAxisSaturationControlRed = 0x59,
    SixAxisSaturationControlYellow = 0x5a,
    AdjustZoom = 0x7c,
    AutoColorSetup = 0x1f,
    AutoSetup = 0x1e,
    AutoSetupOnOff = 0xa2,
    BacklightControl = 0x13,
    BacklightLevelBlue = 0x71,
    BacklightLevelGreen = 0x6f,
    BacklightLevelRed = 0x6d,
    BacklightLevelWhite = 0x6b,
    BlockLutOperation = 0x75,
    Clock = 0x0e,
    ClockPhase = 0x3e,
    ColorSaturation = 0x8a,
    ColorTemperatureIncrement = 0x0b,
    ColorTemperatureRequest = 0x0c,
    Contrast = 0x12,
    DisplayApplication = 0xdc,
    FleshToneEnhancement = 0x11,
    Focus = 0x1c,
    Gamma = 0x72,
    GrayScaleExpansion = 0x2e,
    HorizontalMoire = 0x56,
    Hue = 0x90,
    Luminance = 0x10,
    LutSize = 0x73,
    ScreenOrientation = 0xaa,
    SelectColorPreset = 0x14,
    Sharpness = 0x87,
    SinglePointLutOperation = 0x74,
    StereoVideoMode = 0xd4,
    TvBlackLevelLuminance = 0x92,
    TvContrast = 0x8e,
    TvSharpness = 0x8c,
    UserColorVisionCompensation = 0x17,
    VelocityScanModulation = 0x88,
    VerticalMoire = 0x58,
    VideoBlackLevelBlue = 0x70,
    VideoBlackLevelGreen = 0x6e,
    VideoBlackLevelRed = 0x6c,
    VideoGainBlue = 0x1a,
    VideoGainGreen = 0x18,
    VideoGainRed = 0x16,
    WindowBackground = 0x9a,
    WindowControlOnOff = 0xa4,
    WindowSelect = 0xa5,
    WindowSize = 0xa6,
    WindowTransparency = 0xa7,
}

#[repr(u8)]
pub enum DisplayControl {
    DisplayControllerId = 0xc8,
    DisplayFirmwareLevel = 0xc9,
    DisplayUsageTime = 0xc6,
    HorizontalFrequency = 0xac,
    ImageMode = 0xdb,
    OsdButtonLevelControl = 0xca,
    OsdLanguage = 0xcc,
    PowerMode = 0xd6,
    SourceColorCoding = 0xb5,
    SourceTimingMode = 0xb4,
    Version = 0xdf,
    VerticalFrequency = 0xae,
}

#[repr(u8)]
pub enum Geometry {
    BottomCornerFlare = 0x4a,
    BottomCornerHook = 0x4c,
    DisplayScaling = 0x86,
    HorizontalConvergenceMG = 0x29,
    HorizontalConvergenceRB = 0x28,
    HorizontalKeystone = 0x42,
    HorizontalLinearity = 0x2a,
    HorizontalLinearityBalance = 0x2c,
    HorizontalMirror = 0x82,
    HorizontalParallelogram = 0x40,
    HorizontalPincushion = 0x24,
    HorizontalPincushionBalance = 0x26,
    HorizontalPosition = 0x20,
    HorizontalSize = 0x22,
    Rotation = 0x44,
    ScanMode = 0xda,
    TopCornerFlare = 0x46,
    TopCornerHook = 0x48,
    VerticalConvergenceMG = 0x39,
    VerticalConvergenceRB = 0x38,
    VerticalKeystone = 0x43,
    VerticalLinearity = 0x3a,
    VerticalLinearityBalance = 0x3c,
    VerticalMirror = 0x84,
    VerticalParallelogram = 0x41,
    VerticalPincushion = 0x34,
    VerticalPincushionBalance = 0x36,
    VerticalPosition = 0x30,
    VerticalSize = 0x32,
    WindowPositionBrX = 0x97,
    WindowPositionBrY = 0x98,
    WindowPositionTlX = 0x95,
    WindowPositionTlY = 0x96,
}

#[repr(u8)]
pub enum Miscellaneous {
    ActiveControl = 0x52,
    AmbientLightSensor = 0x66,
    ApplicationEnableKey = 0xc6,
    AssetTag = 0xd2,
    AuxiliaryDisplayData = 0xcf,
    AuxiliaryDisplaySize = 0xce,
    AuxiliaryPowerOutput = 0xd7,
    Degauss = 0x01,
    DisplayDescriptorLength = 0xc2,
    DisplayIdentificationDataOperation = 0x87,
    DisplayTechnologyType = 0xb6,
    EnableDisplayOfDisplayDescriptor = 0xc4,
    FlatPanelSubPixelLayout = 0xb2,
    InputSource = 0x60,
    NewControlValue = 0x02,
    OutputSelect = 0xd0,
    PerformancePreservation = 0x54,
    RemoteProcedureCall = 0x76,
    ScratchPad = 0xde,
    SoftControls = 0x03,
    StatusIndicators = 0xcd,
    TransmitDisplayDescriptor = 0xc3,
    TvChannelUpDown = 0x8b,
}

#[repr(u8)]
pub enum Audio {
    AudioBalanceLR = 0x93,
    AudioBass = 0x91,
    AudioJackConnectionStatus = 0x65,
    AudioMicrophoneVolume = 0x64,
    AudioMute = 0x8d,
    AudioProcessorMode = 0x94,
    AudioSpeakerSelect = 0x63,
    AudioSpeakerVolume = 0x62,
    AudioTreble = 0x8f,
}

#[repr(u8)]
pub enum DPVL {
    BodyCrcErrorCount = 0xbc,
    ClientId = 0xbd,
    HeaderErrorCount = 0xbb,
    LinkControl = 0xbe,
    MonitorStatus = 0xb7,
    MonitorXOrigin = 0xb9,
    MonitorYOrigin = 0xba,
    PacketCount = 0xb8,
}

impl From<PresetOperation> for u8 {
    fn from(code: PresetOperation) -> Self {
        code.into()
    }
}

impl From<ImageAdjustment> for u8 {
    fn from(code: ImageAdjustment) -> Self {
        code.into()
    }
}

impl From<DisplayControl> for u8 {
    fn from(code: DisplayControl) -> Self {
        code.into()
    }
}

impl From<Geometry> for u8 {
    fn from(code: Geometry) -> Self {
        code.into()
    }
}

impl From<Miscellaneous> for u8 {
    fn from(code: Miscellaneous) -> Self {
        code.into()
    }
}

impl From<Audio> for u8 {
    fn from(code: Audio) -> Self {
        code.into()
    }
}

impl From<DPVL> for u8 {
    fn from(code: DPVL) -> Self {
        code.into()
    }
}