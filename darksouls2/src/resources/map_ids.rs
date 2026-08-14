#[repr(u32)]
#[derive(Clone, Copy)]
pub enum MapId {
    ThingsBetwixed          = 0xa020000,  // m10_02_00_00
    Majula                  = 0xa040000,  // m10_04_00_00
    ForestOfFallenGiants    = 0xa0a0000,  // m10_10_00_00
    BrightstoneCoveTseldora = 0xa0e0000,  // m10_14_00_00
    AldiasKeep              = 0xa0f0000,  // m10_15_00_00
    TheLostBastille         = 0xa100000,  // m10_16_00_00
    HarvestValley           = 0xa110000,  // m10_17_00_00
    NoMansWharf             = 0xa120000,  // m10_18_00_00
    IronKeep                = 0xa130000,  // m10_19_00_00
    HuntsmansCorpse         = 0xa170000,  // m10_23_00_00
    TheGutter               = 0xa190000,  // m10_25_00_00
    DragonAerie             = 0xa1b0000,  // m10_27_00_00
    PathToShadedWoods       = 0xa1d0000,  // m10_29_00_00
    PathToNoMansWharf       = 0xa1e0000,  // m10_30_00_00
    HeidesTowerOfFlame      = 0xa1f0000,  // m10_31_00_00
    ShadedWoods             = 0xa200000,  // m10_32_00_00
    DoorsOfPharros          = 0xa210000,  // m10_33_00_00
    GraveOfSaints           = 0xa220000,  // m10_34_00_00
    GiantsMemory            = 0x140a0000, // m20_10_00_00
    ShrineOfAmana           = 0x140b0000, // m20_11_00_00
    DrangleicCastle         = 0x14150000, // m20_21_00_00
    UndeadCrypt             = 0x14180000, // m20_24_00_00
    DragonsMemory           = 0x141a0000, // m20_26_00_00
    DarkChasmOfOld          = 0x28030000, // m40_03_00_00
    ShulvaSanctumCity       = 0x32230000, // m50_35_00_00
    BrumeTower              = 0x32240000, // m50_36_00_00
    FrozenEleumLoyce        = 0x32250000, // m50_37_00_00
    KingsMemory             = 0x32260000, // m50_38_00_00
}
