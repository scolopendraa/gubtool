use {
    crate::{
        chr_ctrl::ResolvedChrPtr,
        offsets::{ChainReadExt, chr_ctrl::stats_offsets},
        player::player,
    },
    gubtool_core::sys::sys_error::ProcResult,
    shared::command::EmptyCommand,
    std::ptr,
    strum::Display,
};

#[repr(C, packed)]
pub struct CovenantData {
    current_covenant: u8,
    found_flags:      [u8; 10],
    rank:             [u8; 10],
    progress:         [u16; 10],
}

impl CovenantData {
    pub fn read() -> Self {
        let bytes = player()
            .chr_ctrl()
            .and_then(|chr| {
                chr.get_ptr(ResolvedChrPtr::Stats)
                    .add_offset(stats_offsets::COVENANT)
                    .read::<[u8; std::mem::size_of::<Self>()]>()
            })
            .unwrap_or([0x0; std::mem::size_of::<Self>()]);
        unsafe { ptr::read_unaligned(bytes.as_ptr() as *const Self) }
    }

    pub fn assemble_covenant_info(&self, covenant: CovenantKind) -> CovenantInfo {
        if covenant == CovenantKind::None {
            panic!("can not assemble covenant info for none")
        }
        CovenantInfo {
            covenant,
            progress: Some(self.progress[(covenant as u8) as usize]),
            rank: Some(self.rank[(covenant as u8) as usize]),
            found: Some(self.found_flags[(covenant as u8) as usize] != 0x0),
        }
    }
}

pub struct Covenant;

impl EmptyCommand for Covenant {}

impl std::fmt::Display for Covenant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Covenant: {}", Self.get().unwrap_or_default())
    }
}

impl Covenant {
    pub fn get(&self) -> ProcResult<CovenantKind> {
        player()
            .chr_ctrl()?
            .get_ptr(ResolvedChrPtr::Stats)
            .add_offset(stats_offsets::COVENANT)
            .read::<u8>()
            .map(|val| CovenantKind::try_from(val).unwrap_or_default())
    }

    pub fn set(&self, covenant: CovenantKind) -> ProcResult {
        player()
            .chr_ctrl()?
            .get_ptr(ResolvedChrPtr::Stats)
            .add_offset(stats_offsets::COVENANT)
            .write::<u8>(covenant as u8)
    }
}

#[repr(u8)]
#[derive(Clone, Copy, Default, PartialEq, Display)]
#[strum(serialize_all = "title_case")]
pub enum CovenantKind {
    #[default]
    None               = 0x0,
    HeirsOfTheSun      = 0x1,
    BlueSentinels      = 0x2,
    BrotherhoodOfBlood = 0x3,
    WayOfBlue          = 0x4,
    RatKing            = 0x5,
    BellKeepers        = 0x6,
    DragonRemnants     = 0x7,
    CompanyOfChampions = 0x8,
    PilgrimsOfDark     = 0x9,
}

impl TryFrom<u8> for CovenantKind {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::None),
            1 => Ok(Self::HeirsOfTheSun),
            2 => Ok(Self::BlueSentinels),
            3 => Ok(Self::BrotherhoodOfBlood),
            4 => Ok(Self::WayOfBlue),
            5 => Ok(Self::RatKing),
            6 => Ok(Self::BellKeepers),
            7 => Ok(Self::DragonRemnants),
            8 => Ok(Self::CompanyOfChampions),
            9 => Ok(Self::PilgrimsOfDark),
            _ => Err(()),
        }
    }
}

pub struct CovenantInfo {
    pub covenant: CovenantKind,
    pub progress: Option<u16>,
    pub rank:     Option<u8>,
    pub found:    Option<bool>,
}

pub fn covenants_with_progress() -> [CovenantInfo; 10] {
    let data = CovenantData::read();
    [
        CovenantInfo {
            covenant: CovenantKind::None,
            progress: None,
            rank:     None,
            found:    None,
        },
        data.assemble_covenant_info(CovenantKind::HeirsOfTheSun),
        data.assemble_covenant_info(CovenantKind::BlueSentinels),
        data.assemble_covenant_info(CovenantKind::BrotherhoodOfBlood),
        data.assemble_covenant_info(CovenantKind::WayOfBlue),
        data.assemble_covenant_info(CovenantKind::RatKing),
        data.assemble_covenant_info(CovenantKind::BellKeepers),
        data.assemble_covenant_info(CovenantKind::DragonRemnants),
        data.assemble_covenant_info(CovenantKind::CompanyOfChampions),
        data.assemble_covenant_info(CovenantKind::PilgrimsOfDark),
    ]
}
