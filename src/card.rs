use arrayvec::{Array, ArrayString, ArrayVec};
#[cfg(feature = "serialize")]
use bincode;
use core::fmt::Write;
use core::i32;
use core::ops::Deref;
#[cfg(feature = "serialize")]
use core::ops::DerefMut;
use core::result;
#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

const SAVE_SIZE: usize = 0x2000;
const CARD_READ_SIZE: usize = 512;
const CARD_FILENAME_MAX: usize = 32;
const CARD_ICON_MAX: usize = 8;
#[allow(dead_code)]
const CARD_ICON_WIDTH: usize = 32;
#[allow(dead_code)]
const CARD_ICON_HEIGHT: usize = 32;
#[allow(dead_code)]
const CARD_BANNER_WIDTH: usize = 96;
#[allow(dead_code)]
const CARD_BANNER_HEIGHT: usize = 32;

macro_rules! card_try {
    ($e:expr) => {
        let result: CardError = unsafe { $e.into() };
        if result != CardError::Ready {
            return Err(result);
        }
    };
}

#[derive(PartialEq, Debug)]
pub enum CardError {
    Ready = 0,
    Busy = -1,
    WrongDevice = -2,
    NoCard = -3,
    NoFile = -4,
    IoError = -5,
    Broken = -6,
    Exist = -7,
    NoEnt = -8,
    InsSpace = -9,
    NoPerm = -10,
    Limit = -11,
    NameTooLong = -12,
    Encoding = -13,
    Canceled = -14,
    FatalError = -128,
}

impl From<i32> for CardError {
    fn from(i: i32) -> CardError {
        match i {
            0 => CardError::Ready,
            -1 => CardError::Busy,
            -2 => CardError::WrongDevice,
            -3 => CardError::NoCard,
            -4 => CardError::NoFile,
            -5 => CardError::IoError,
            -6 => CardError::Broken,
            -7 => CardError::Exist,
            -8 => CardError::NoEnt,
            -9 => CardError::InsSpace,
            -10 => CardError::NoPerm,
            -11 => CardError::Limit,
            -12 => CardError::NameTooLong,
            -13 => CardError::Encoding,
            -14 => CardError::Canceled,
            -128 => CardError::FatalError,
            _ => unreachable!(),
        }
    }
}

#[repr(C)]
#[derive(Debug, Default)]
pub struct CardInfo {
    chan: i32,
    file_num: i32,
    offset: i32,
    length: i32,
    i_block: u16,
}

#[repr(C)]
#[derive(Debug, Default)]
pub struct CardStat {
    // Read-only (Set by CARDGetStatus)
    file_name: [u8; CARD_FILENAME_MAX],
    length: u32,
    game_name: [u8; 4],
    company: [u8; 2],

    // Read/Write (Set by CARDGetStatus/CardSetStatus)
    banner_format: u8,
    icon_addr: u32,
    icon_format: u16,
    icon_speed: u16,
    commend_addr: u32,

    // Read-only (Set by CARDGetStatus)
    offset_banner: u32,
    offset_banner_tlut: u32,
    offset_icon: [u32; CARD_ICON_MAX],
    offset_icon_tlut: u32,
    offset_data: u32,
}

extern "C" {
    #[link_name = "CARDOpen"]
    pub fn open(chan: i32, name: *const u8, card_info: *mut CardInfo) -> i32;
    #[link_name = "CARDRead"]
    pub fn read(card_info: *mut CardInfo, data: *mut u8, len: i32, offset: i32) -> i32;
    #[link_name = "CARDClose"]
    pub fn close(card_info: *mut CardInfo) -> i32;
    #[link_name = "CARDCreate"]
    pub fn create(chan: i32, name: *const u8, size: u32, card_info: *mut CardInfo) -> i32;
    #[link_name = "CARDProbeEx"]
    pub fn probe_ex(chan: i32, card_size: *mut i32, sector_size: *mut i32) -> i32;
    #[link_name = "CARDGetStatus"]
    pub fn get_status(chan: i32, file_no: i32, card_stat: *mut CardStat) -> i32;
    #[link_name = "CARDWrite"]
    pub fn write(card_info: *mut CardInfo, data: *const u8, length: i32, offset: i32) -> i32;
}

type Result<T> = result::Result<T, CardError>;

#[repr(align(32))]
pub struct SectorBuf {
    buf: [u8; SAVE_SIZE],
}

unsafe impl Array for SectorBuf {
    type Item = u8;
    type Index = u16;
    #[inline(always)]
    fn as_ptr(&self) -> *const u8 {
        self.buf.as_ptr()
    }
    #[inline(always)]
    fn as_mut_ptr(&mut self) -> *mut u8 {
        self.buf.as_mut_ptr()
    }
    #[inline(always)]
    fn capacity() -> usize {
        SAVE_SIZE
    }
}

impl Deref for SectorBuf {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        &self.buf
    }
}

impl DerefMut for SectorBuf {
    fn deref_mut(&mut self) -> &mut [u8] {
        &mut self.buf
    }
}

pub struct Card {
    card_info: CardInfo,
    sector_size: i32,
}

impl Card {
    /// # Panics
    ///
    /// File name length needs to be less than or equal to 32.
    pub fn open(file_name: &str) -> Result<Card> {
        let mut sector_size = i32::MAX;
        let mut card_info = CardInfo::default();
        let mut file_name_buffer = ArrayString::<[u8; CARD_FILENAME_MAX * 2]>::new();
        assert!(file_name.len() <= 32);
        let _ = write!(file_name_buffer, "{}\0", file_name);
        card_try!(probe_ex(0, ::core::ptr::null_mut(), &mut sector_size));
        card_try!(open(0, file_name_buffer.as_ptr(), &mut card_info));
        Ok(Card {
            card_info,
            sector_size,
        })
    }

    /// # Panics
    ///
    /// File name length needs to be less than or equal to 32.
    pub fn create(file_name: &str) -> Result<Card> {
        let mut sector_size = i32::MAX;
        let mut card_info = CardInfo::default();
        let mut file_name_buffer = ArrayString::<[u8; CARD_FILENAME_MAX * 2]>::new();
        assert!(file_name.len() <= 32);
        let _ = write!(file_name_buffer, "{}\0", file_name);
        card_try!(probe_ex(0, ::core::ptr::null_mut(), &mut sector_size));
        let mut result = unsafe {
            create(
                0,
                file_name_buffer.as_ptr(),
                sector_size as u32,
                &mut card_info,
            ).into()
        };

        if result == CardError::Exist {
            result = unsafe { open(0, file_name_buffer.as_ptr(), &mut card_info).into() };
        }

        if result == CardError::Ready {
            Ok(Card {
                card_info,
                sector_size,
            })
        } else {
            Err(result)
        }
    }

    /// # Panics
    ///
    /// The buffer needs to be aligned to 32 bytes.
    ///
    /// Buffer length needs to be a multiple of 512 bytes.
    pub fn read(&mut self, buffer: &mut [u8]) -> Result<()> {
        assert_eq!(buffer.as_ptr() as usize % 32, 0);
        assert_eq!(buffer.len() % CARD_READ_SIZE, 0);
        card_try!(read(
            &mut self.card_info,
            buffer.as_mut_ptr(),
            buffer.len() as i32,
            0
        ));
        Ok(())
    }

    #[cfg(feature = "serialize")]
    pub fn deserialize_read<T, F>(&mut self, f: F) -> Result<()>
    where
        F: FnOnce(T),
        for<'de> T: Deserialize<'de>,
    {
        let mut buf: SectorBuf = unsafe { ::core::mem::uninitialized() };
        self.read(&mut buf)?;
        let save: T = bincode::deserialize(&buf).map_err(|_| CardError::Encoding)?;
        f(save);
        Ok(())
    }

    /// # Panics
    ///
    /// The buffer needs to be aligned to 32 bytes.
    ///
    /// Buffer length needs to be a multiple of the card's sector size.
    pub fn write(&mut self, data: &[u8]) -> Result<()> {
        assert_eq!(data.as_ptr() as usize % 32, 0);
        assert_eq!(data.len() % self.sector_size as usize, 0);
        card_try!(write(
            &mut self.card_info,
            data.as_ptr(),
            data.len() as i32,
            0
        ));
        Ok(())
    }

    #[cfg(feature = "serialize")]
    pub fn serialize_write<T: Serialize>(&mut self, data: &T) -> Result<()> {
        let mut buf = ArrayVec::<SectorBuf>::new();
        {
            bincode::serialize_into(&mut buf, data).map_err(|_| CardError::Encoding)?;
        }
        unsafe {
            buf.set_len(SAVE_SIZE);
        }
        self.write(&buf)?;
        Ok(())
    }
}

impl Drop for Card {
    fn drop(&mut self) {
        unsafe {
            close(&mut self.card_info);
        }
    }
}
