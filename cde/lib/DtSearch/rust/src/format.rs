#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
use libc::{c_char, c_int, c_long, c_void, time_t, FILE};

pub type DtSrINT32 = i32;
pub type DtSrUINT32 = u32;
pub type DtSrINT16 = i16;
pub type DtSrKeytype = c_void; // Placeholder for now
pub type DtSrObjdate = i32;
pub type DtSrResult = c_void; // Placeholder
pub type DtSrHitword = c_void; // Placeholder
pub type DB_ADDR = i32;

pub const OR_DBREC: c_int = 10000;
pub const OR_DBMISCREC: c_int = 10001;
pub const OR_OBJREC: c_int = 10002;
// ... add others as needed

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DBREC {
    pub or_dbflags: DtSrUINT32,
    pub or_dbuflags: DtSrUINT32,
    pub or_reccount: DtSrINT32,
    pub or_maxdba: DtSrINT32,
    pub or_availd99: DtSrINT32,
    pub or_unavaild99: DtSrINT32,
    pub or_hufid: DtSrINT32,
    pub or_dbsecmask: DtSrINT32,
    pub or_version: [c_char; 8],
    pub or_dbfill: [c_char; 50],
    pub or_dbotype: DtSrINT16,
    pub or_compflags: DtSrINT16,
    pub or_dbaccess: DtSrINT16,
    pub or_minwordsz: DtSrINT16,
    pub or_maxwordsz: DtSrINT16,
    pub or_recslots: DtSrINT16,
    pub or_fzkeysz: DtSrINT16,
    pub or_abstrsz: DtSrINT16,
    pub or_language: DtSrINT16,
}

impl DBREC {
    pub fn swab_to_host(&mut self) {
        self.or_dbflags = u32::from_be(self.or_dbflags);
        self.or_dbuflags = u32::from_be(self.or_dbuflags);
        self.or_reccount = i32::from_be(self.or_reccount);
        self.or_maxdba = i32::from_be(self.or_maxdba);
        self.or_availd99 = i32::from_be(self.or_availd99);
        self.or_unavaild99 = i32::from_be(self.or_unavaild99);
        self.or_hufid = i32::from_be(self.or_hufid);
        self.or_dbsecmask = i32::from_be(self.or_dbsecmask);
        // char arrays don't need update
        self.or_dbotype = i16::from_be(self.or_dbotype);
        self.or_compflags = i16::from_be(self.or_compflags);
        self.or_dbaccess = i16::from_be(self.or_dbaccess);
        self.or_minwordsz = i16::from_be(self.or_minwordsz);
        self.or_maxwordsz = i16::from_be(self.or_maxwordsz);
        self.or_recslots = i16::from_be(self.or_recslots);
        self.or_fzkeysz = i16::from_be(self.or_fzkeysz);
        self.or_abstrsz = i16::from_be(self.or_abstrsz);
        self.or_language = i16::from_be(self.or_language);
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PARG {
    pub dblk: *mut c_void,
    pub ftext: *mut FILE,
    pub string: *mut c_void,
    pub etxdelim: *mut c_void,
    pub offsetp: *mut c_long,
    pub flags: c_long,
    pub extra: *mut c_void,
}

#[repr(C)]
pub struct WORDTREE {
    pub rlink: *mut WORDTREE,
    pub llink: *mut WORDTREE,
    pub len: c_int,
    pub word: *mut c_void,
}

#[repr(C)]
pub struct DBLK {
    pub link: *mut DBLK,
    pub name: [c_char; 11],
    pub is_selected: c_char,
    pub label: *mut c_char,
    pub flags: c_long,
    pub path: *mut c_char,
    pub syofile: *mut FILE,
    pub syifile: *mut FILE,
    pub ranges: *mut c_void,
    pub iifile: *mut FILE,
    pub iimtime: time_t,
    pub vistano: c_int,
    pub ktcount: c_int,
    pub keytypes: *mut DtSrKeytype,
    pub dbrec: DBREC,
    pub zblk: *mut c_void,

    // Huffman
    pub hufid: time_t,
    pub hufroot: c_int,
    pub huftree: *mut c_int,

    // Language
    pub fname_stp: *mut c_char,
    pub fname_inc: *mut c_char,
    pub fname_sfx: *mut c_char,
    pub charmap: *mut c_int,
    pub stoplist: *mut WORDTREE,
    pub inclist: *mut WORDTREE,
    pub lstrupr: Option<unsafe extern "C" fn(*mut c_char, *mut DBLK) -> *mut c_char>,
    pub parser: Option<unsafe extern "C" fn(*mut PARG) -> *mut c_char>,
    pub parse_extra: *mut c_void,
    pub stemmer: Option<unsafe extern "C" fn(*mut c_char, *mut DBLK) -> *mut c_char>,
    pub stem_flags: c_long,
    pub stem_extra: *mut c_void,
    pub lang_extra: *mut c_void,
    pub lang_flags: c_long,

    // User Params
    pub maxhits: c_int,
    pub resuser: *mut c_void,
}

pub const DtSrMAX_STEMCOUNT: usize = 20; // Assume 20 for now, check Search.h later
pub const DtSrMAXWIDTH_HWORD: usize = 128; // Assume 128

#[repr(C)]
#[derive(Copy, Clone)]
pub struct or_objrec {
    pub or_objflags: DtSrUINT32,
    pub or_objuflags: DtSrUINT32,
    pub or_objsize: DtSrINT32,
    pub or_objdate: DtSrINT32,
    pub or_objsecmask: DtSrINT32,
    pub or_objkey: [c_char; 32],
    pub or_objfill: [c_char; 34],
    pub or_objaccess: DtSrINT16,
    pub or_objtype: DtSrINT16,
    pub or_objcost: DtSrINT16,
    pub or_objhdroffset: DtSrINT16,
    pub or_objeureka: DtSrINT16,
}

impl or_objrec {
    pub fn swab_to_host(&mut self) {
        self.or_objflags = u32::from_be(self.or_objflags);
        self.or_objuflags = u32::from_be(self.or_objuflags);
        self.or_objsize = i32::from_be(self.or_objsize);
        self.or_objdate = i32::from_be(self.or_objdate);
        self.or_objsecmask = i32::from_be(self.or_objsecmask);
        self.or_objaccess = i16::from_be(self.or_objaccess);
        self.or_objtype = i16::from_be(self.or_objtype);
        self.or_objcost = i16::from_be(self.or_objcost);
        self.or_objhdroffset = i16::from_be(self.or_objhdroffset);
        self.or_objeureka = i16::from_be(self.or_objeureka);
    }
}

pub const OR_HWORDREC: c_int = 10007;

// Field constants
pub const OR_HWORDKEY: c_long = 7000;
pub const OR_HWOFFSET: c_long = 7001;
pub const OR_HWFREE: c_long = 7002;
pub const OR_HWADDRS: c_long = 7003;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct or_hwordrec {
    pub or_hwordkey: [c_char; 134],
    pub or_hwoffset: DtSrINT32,
    pub or_hwfree: DtSrINT32,
    pub or_hwaddrs: DtSrINT32,
}

impl or_hwordrec {
    pub fn swab_to_host(&mut self) {
        self.or_hwoffset = i32::from_be(self.or_hwoffset);
        self.or_hwfree = i32::from_be(self.or_hwfree);
        self.or_hwaddrs = i32::from_be(self.or_hwaddrs);
    }
}

#[repr(C)]
pub struct LLIST {
    pub link: *mut LLIST,
    pub data: *mut c_void,
}

#[repr(C)]
pub struct USRBLK {
    pub userid: [c_char; 10],
    pub search_type: c_int,
    pub flags: c_long,
    pub debug: c_long,
    pub request: c_int,
    pub retncode: c_int,
    pub query: *mut c_char,
    pub objdate1: DtSrObjdate,
    pub objdate2: DtSrObjdate,
    pub dba: DB_ADDR,
    pub dbatab: *mut DB_ADDR,
    pub dbacount: c_int,
    pub workproc: Option<unsafe extern "C" fn()>,
    pub dblist: *mut DBLK,
    pub dblk: *mut DBLK,
    pub dittolist: *mut DtSrResult,
    pub dittocount: c_long,
    pub stemcount: c_int,
    pub stems: [[c_char; DtSrMAXWIDTH_HWORD]; DtSrMAX_STEMCOUNT],
    pub objrec: or_objrec,
    pub abstrbuf: *mut c_char,
    pub abstrbufsz: c_int,
    pub cleartext: *mut c_char,
    pub clearlen: c_long,
    pub notes: *mut LLIST,
    pub hitwords: *mut DtSrHitword,
    pub hitwcount: c_long,
}

extern "C" {
    pub static mut usrblk: USRBLK;
}
