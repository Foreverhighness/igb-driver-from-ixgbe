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

pub const IGB_RCTL: u32 = 0x00100;
pub const IGB_RCTL_ENABLE: u32 = 1 << 1;
pub const IGB_RCTL_SZ_1024: u32 = 0x0001_0000;

pub const IGB_RXDCTL: u32 = 0x0C028;
pub const IGB_RXDCTL_ENABLE: u32 = 1 << 25;
pub const IGB_RXDCTL_PTHRESH: u32 = 16;
pub const IGB_RXDCTL_HTHRESH: u32 = 8 << 8;
pub const IGB_RXDCTL_WTHRESH: u32 = 1 << 16;

pub const IGB_SRRCTL: u32 = 0x0C00C;

pub const IGB_TCTL: u32 = 0x00400;
pub const IGB_TCTL_ENABLE: u32 = 1 << 1;

pub const IGB_TXDCTL: u32 = 0x0E028;
pub const IGB_TXDCTL_ENABLE: u32 = 1 << 25;
pub const IGB_TXDCTL_WTHRESH: u32 = 1 << 16;

pub const IGB_RDBAL: u32 = 0x0C000;
pub const IGB_RDBAH: u32 = 0x0C004;
pub const IGB_RDLEN: u32 = 0x0C008;
pub const IGB_RDH: u32 = 0x0C010;
pub const IGB_RDT: u32 = 0x0C018;

pub const IGB_TDBAL: u32 = 0x0E000;
pub const IGB_TDBAH: u32 = 0x0E004;
pub const IGB_TDLEN: u32 = 0x0E008;
pub const IGB_TDH: u32 = 0x0E010;
pub const IGB_TDT: u32 = 0x0E018;

pub const IGB_IMS: u32 = 0x1508;
pub const IGB_IMS_RXDW: u32 = 1 << 7;
pub const IGB_IMS_TXDW: u32 = 1;
pub const IGB_IMS_RXDMT0: u32 = 1 << 4;
pub const IGB_IMS_LSC: u32 = 1 << 2;
pub const IGB_IMS_RXSEQ: u32 = 1 << 3;
