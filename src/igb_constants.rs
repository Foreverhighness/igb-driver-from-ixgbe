pub const IGB_EIMS: u32 = 0x01524;
pub const IGB_EIMC: u32 = 0x01528;
pub const IGB_EICR: u32 = 0x01580;

pub const IGB_RXPBS: u32 = 0x02404;
pub const IGB_TXPBS: u32 = 0x03404;
pub const IGB_SWPBS: u32 = 0x03004;

/* General Registers */
pub const IGB_CTRL: u32 = 0x00000;
pub const IGB_STATUS: u32 = 0x00008;
pub const IGB_CTRL_EXT: u32 = 0x00018;

pub const IGB_CTRL_RST_MASK: u32 = 0x0400_0000;

pub const IGB_MDIC: u32 = 0x00020;
pub const IGB_MDIC_DATA: u32 = 0x0000_FFFF;
pub const IGB_MDIC_WRITE: u32 = 0x0400_0000;
pub const IGB_MDIC_READ: u32 = 0x0800_0000;
pub const IGB_MDIC_READY: u32 = 1 << 28;

pub const IGB_MII_CR_RESTART_AUTO_NEGOTIATION: u32 = 1 << 9;
