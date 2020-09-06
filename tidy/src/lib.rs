#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use libc::*;
use std::boxed::Box;
use std::error::Error;
use std::ffi::CStr;
use std::ffi::CString;
use std::fmt;
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

include!("bindings.rs");

#[derive(Debug, Clone)]
pub enum TidySeverity {
  Success,
  Warning,
  Error,
  Severe,
}

impl fmt::Display for TidySeverity {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self)
  }
}

#[derive(Debug, Clone)]
pub struct TidyError {
  severity: TidySeverity,
  message: String,
}

impl std::fmt::Display for TidyError {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "{}", self.message)
  }
}

impl Error for TidyError {}

pub struct Tidy {
  errbuf: *mut TidyBuffer,
  output: *mut TidyBuffer,
  tdoc: TidyDoc,
}

impl Tidy {
  pub fn new() -> Result<Tidy, TidyError> {
    // TODO find better approach
    let errbuf: TidyBuffer = unsafe { std::mem::zeroed() };
    let b_errbuf = Box::from(errbuf);
    let p_errbuf = Box::into_raw(b_errbuf);

    let output: TidyBuffer = unsafe { std::mem::zeroed() };
    let b_output = Box::from(output);
    let p_output = Box::into_raw(b_output);

    let tdoc = unsafe { tidyCreate() };

    unsafe {
      match tidySetErrorBuffer(tdoc, p_errbuf) {
        0 => Ok(Tidy {
          errbuf: p_errbuf,
          output: p_output,
          tdoc: tdoc,
        }),
        _ => Err(TidyError {
          severity: TidySeverity::Severe,
          message: String::from("Tidy set errorbuffer error"),
        }),
      }
    }
  }

  // Helper
  pub fn c_str_to_owned(in_str: ctmbstr) -> String {
    let c_str: &CStr = unsafe { CStr::from_ptr(in_str) };
    let str_slice: &str = c_str.to_str().unwrap();
    str_slice.to_owned()
  }

  pub fn bool_to_tidy_Bool(bool_in: bool) -> Bool {
    match bool_in {
      true => Bool_yes,
      _ => Bool_no,
    }
  }

  // Basic operations
  pub fn tidyReleaseDate(&self) -> String {
    unsafe { Self::c_str_to_owned(tidyReleaseDate()) }
  }

  pub fn tidyLibraryVersion(&self) -> String {
    unsafe { Self::c_str_to_owned(tidyLibraryVersion()) }
  }

  pub fn tidySetCharEncoding(&self, encoding: &str) -> Result<TidySeverity, TidyError> {
    unsafe {
      match tidySetCharEncoding(self.tdoc, CString::new(encoding).unwrap().as_ptr()) {
        0 => Ok(TidySeverity::Success),
        1 => Ok(TidySeverity::Warning),
        2 => Ok(TidySeverity::Error),
        _ => Err(TidyError {
          severity: TidySeverity::Severe,
          message: String::from("Tidy set char encoding error"),
        }),
      }
    }
  }

  pub fn tidySetOutCharEncoding(&self, encoding: &str) -> Result<TidySeverity, TidyError> {
    unsafe {
      match tidySetOutCharEncoding(self.tdoc, CString::new(encoding).unwrap().as_ptr()) {
        0 => Ok(TidySeverity::Success),
        1 => Ok(TidySeverity::Warning),
        2 => Ok(TidySeverity::Error),
        _ => Err(TidyError {
          severity: TidySeverity::Severe,
          message: String::from("Tidy set out char encoding error"),
        }),
      }
    }
  }

  pub fn tidySetInCharEncoding(&self, encoding: &str) -> Result<TidySeverity, TidyError> {
    unsafe {
      match tidySetInCharEncoding(self.tdoc, CString::new(encoding).unwrap().as_ptr()) {
        0 => Ok(TidySeverity::Success),
        1 => Ok(TidySeverity::Warning),
        2 => Ok(TidySeverity::Error),
        _ => Err(TidyError {
          severity: TidySeverity::Severe,
          message: String::from("Tidy set in char encoding error"),
        }),
      }
    }
  }

  pub fn tidyStatus(&self) -> TidySeverity {
    unsafe {
      match tidyStatus(self.tdoc) {
        0 => TidySeverity::Success,
        1 => TidySeverity::Warning,
        2 => TidySeverity::Error,
        _ => TidySeverity::Severe,
      }
    }
  }

  pub fn tidyDetectedHtmlVersion(&self) -> c_int {
    unsafe { tidyDetectedHtmlVersion(self.tdoc) }
  }

  pub fn tidyDetectedXhtml(&self) -> bool {
    unsafe {
      match tidyDetectedXhtml(self.tdoc) {
        Bool_yes => true,
        _ => false,
      }
    }
  }

  pub fn tidyDetectedGenericXml(&self) -> bool {
    unsafe {
      match tidyDetectedGenericXml(self.tdoc) {
        Bool_yes => true,
        _ => false,
      }
    }
  }

  pub fn tidyAccessWarningCount(&self) -> c_uint {
    unsafe { tidyAccessWarningCount(self.tdoc) }
  }

  pub fn tidyConfigErrorCount(&self) -> c_uint {
    unsafe { tidyConfigErrorCount(self.tdoc) }
  }

  pub fn tidyWarningCount(&self) -> c_uint {
    unsafe { tidyWarningCount(self.tdoc) }
  }

  pub fn tidyErrorCount(&self) -> c_uint {
    unsafe { tidyErrorCount(self.tdoc) }
  }

  pub fn tidyErrorSummary(&self) {
    unsafe { tidyErrorSummary(self.tdoc) }
  }

  pub fn tidyGeneralInfo(&self) {
    unsafe { tidyGeneralInfo(self.tdoc) }
  }

  // Diagnose and repair
  pub fn tidyRunDiagnostics(&self) -> Result<TidySeverity, TidyError> {
    unsafe {
      match tidyRunDiagnostics(self.tdoc) {
        0 => Ok(TidySeverity::Success),
        1 => Ok(TidySeverity::Warning),
        2 => Ok(TidySeverity::Error),
        _ => Err(TidyError {
          severity: TidySeverity::Severe,
          message: String::from("Tidy run diagnostics error"),
        }),
      }
    }
  }

  pub fn tidyCleanAndRepair(&self) -> Result<TidySeverity, TidyError> {
    unsafe {
      match tidyCleanAndRepair(self.tdoc) {
        0 => Ok(TidySeverity::Success),
        1 => Ok(TidySeverity::Warning),
        2 => Ok(TidySeverity::Error),
        _ => Err(TidyError {
          severity: TidySeverity::Severe,
          message: String::from("Tidy clean and repair error"),
        }),
      }
    }
  }

  pub fn tidyReportDoctype(&self) -> TidySeverity {
    unsafe {
      match tidyReportDoctype(self.tdoc) {
        0 => TidySeverity::Success,
        1 => TidySeverity::Warning,
        2 => TidySeverity::Error,
        _ => TidySeverity::Severe,
      }
    }
  }

  // Document Parse
  pub fn tidyParseString(&self, input: Vec<u8>) -> Result<TidySeverity, TidyError> {
    unsafe {
      let c_input = CString::from_vec_unchecked(input);
      let raw_input = c_input.into_raw();
      let rc = tidyParseString(self.tdoc, raw_input);
      let _c_input = CString::from_raw(raw_input);
      match rc {
        0 => Ok(TidySeverity::Success),
        1 => Ok(TidySeverity::Warning),
        2 => Ok(TidySeverity::Error),
        _ => Err(TidyError {
          severity: TidySeverity::Severe,
          message: String::from("Tidy parse string error"),
        }),
      }
    }
  }

  pub fn tidyParseFile(&self, filename: &str) -> Result<TidySeverity, TidyError> {
    unsafe {
      match tidyParseFile(self.tdoc, CString::new(filename).unwrap().as_ptr()) {
        0 => Ok(TidySeverity::Success),
        1 => Ok(TidySeverity::Warning),
        2 => Ok(TidySeverity::Error),
        _ => Err(TidyError {
          severity: TidySeverity::Severe,
          message: String::from("Tidy parse file error"),
        }),
      }
    }
  }

  pub fn tidyParseStdin(&self) -> Result<TidySeverity, TidyError> {
    unsafe {
      match tidyParseStdin(self.tdoc) {
        0 => Ok(TidySeverity::Success),
        1 => Ok(TidySeverity::Warning),
        2 => Ok(TidySeverity::Error),
        _ => Err(TidyError {
          severity: TidySeverity::Severe,
          message: String::from("Tidy parse stdin error"),
        }),
      }
    }
  }

  // Document save functions
  pub fn tidySaveBuffer(&self) -> Result<TidySeverity, TidyError> {
    unsafe {
      match tidySaveBuffer(self.tdoc, self.output) {
        0 => Ok(TidySeverity::Success),
        1 => Ok(TidySeverity::Warning),
        2 => Ok(TidySeverity::Error),
        _ => Err(TidyError {
          severity: TidySeverity::Severe,
          message: String::from("Tidy save buffer error"),
        }),
      }
    }
  }

  pub fn tidySaveFile(&self, filename: &str) -> Result<TidySeverity, TidyError> {
    unsafe {
      match tidySaveFile(self.tdoc, CString::new(filename).unwrap().as_ptr()) {
        0 => Ok(TidySeverity::Success),
        1 => Ok(TidySeverity::Warning),
        2 => Ok(TidySeverity::Error),
        _ => Err(TidyError {
          severity: TidySeverity::Severe,
          message: String::from("Tidy save file error"),
        }),
      }
    }
  }

  pub fn tidySaveStdout(&self) -> Result<TidySeverity, TidyError> {
    unsafe {
      match tidySaveStdout(self.tdoc) {
        0 => Ok(TidySeverity::Success),
        1 => Ok(TidySeverity::Warning),
        2 => Ok(TidySeverity::Error),
        _ => Err(TidyError {
          severity: TidySeverity::Severe,
          message: String::from("Tidy save stdout error"),
        }),
      }
    }
  }

  pub fn tidyOptSaveFile(&self, filename: &str) -> Result<TidySeverity, TidyError> {
    unsafe {
      match tidyOptSaveFile(self.tdoc, CString::new(filename).unwrap().as_ptr()) {
        0 => Ok(TidySeverity::Success),
        1 => Ok(TidySeverity::Warning),
        2 => Ok(TidySeverity::Error),
        _ => Err(TidyError {
          severity: TidySeverity::Severe,
          message: String::from("Tidy save option file error"),
        }),
      }
    }
  }

  // Configuration options
  pub fn tidyOptGetDefaultBool(opt: TidyOption) -> bool {
    unsafe {
      match tidyOptGetDefaultBool(opt) {
        Bool_yes => true,
        _ => false,
      }
    }
  }

  pub fn tidyOptGetDefaultInt(opt: TidyOption) -> c_ulong {
    unsafe { tidyOptGetDefaultInt(opt) }
  }

  pub fn tidyOptGetDefault(opt: TidyOption) -> String {
    unsafe { Self::c_str_to_owned(tidyOptGetDefault(opt)) }
  }

  pub fn tidyOptGetBool(&self, optid: TidyOptionId) -> bool {
    unsafe {
      match tidyOptGetBool(self.tdoc, optid) {
        Bool_yes => true,
        _ => false,
      }
    }
  }

  pub fn tidyOptSetBool(&self, optid: TidyOptionId, val: bool) -> Result<TidySeverity, TidyError> {
    unsafe {
      match tidyOptSetBool(self.tdoc, optid, Self::bool_to_tidy_Bool(val)) {
        Bool_yes => Ok(TidySeverity::Success),
        _ => Err(TidyError {
          severity: TidySeverity::Severe,
          message: String::from("Tidy  option set bool error"),
        }),
      }
    }
  }

  pub fn tidyOptGetInt(&self, optid: TidyOptionId) -> c_ulong {
    unsafe { tidyOptGetInt(self.tdoc, optid) }
  }

  pub fn tidyOptSetInt(
    &self,
    optid: TidyOptionId,
    val: c_ulong,
  ) -> Result<TidySeverity, TidyError> {
    unsafe {
      match tidyOptSetInt(self.tdoc, optid, val) {
        Bool_yes => Ok(TidySeverity::Success),
        _ => Err(TidyError {
          severity: TidySeverity::Severe,
          message: String::from("Tidy option set int error"),
        }),
      }
    }
  }

  pub fn tidyOptGetValue(&self, optid: TidyOptionId) -> String {
    unsafe { Self::c_str_to_owned(tidyOptGetValue(self.tdoc, optid)) }
  }

  pub fn tidyOptSetValue(&self, optid: TidyOptionId, val: &str) -> Result<TidySeverity, TidyError> {
    unsafe {
      match tidyOptSetValue(self.tdoc, optid, CString::new(val).unwrap().as_ptr()) {
        Bool_yes => Ok(TidySeverity::Success),
        _ => Err(TidyError {
          severity: TidySeverity::Severe,
          message: String::from("Tidy option set value error"),
        }),
      }
    }
  }

  pub fn tidyOptParseValue(&self, optnam: &str, val: &str) -> Result<TidySeverity, TidyError> {
    unsafe {
      match tidyOptParseValue(
        self.tdoc,
        CString::new(optnam).unwrap().as_ptr(),
        CString::new(val).unwrap().as_ptr(),
      ) {
        Bool_yes => Ok(TidySeverity::Success),
        _ => Err(TidyError {
          severity: TidySeverity::Severe,
          message: String::from("Tidy option parse value error"),
        }),
      }
    }
  }

  pub fn tidyGetOptionByName(&self, optnam: &str) -> TidyOption {
    unsafe { tidyGetOptionByName(self.tdoc, CString::new(optnam).unwrap().as_ptr()) }
  }

  pub fn tidyOptGetEncName(&self, optid: TidyOptionId) -> String {
    unsafe { Self::c_str_to_owned(tidyOptGetEncName(self.tdoc, optid)) }
  }

  pub fn tidyOptGetDoc(&self, opt: TidyOption) -> String {
    unsafe { Self::c_str_to_owned(tidyOptGetDoc(self.tdoc, opt)) }
  }

  pub fn tidyGetOption(&self, optid: TidyOptionId) -> TidyOption {
    unsafe { tidyGetOption(self.tdoc, optid) }
  }

  pub fn tidyOptGetType(opt: TidyOption) -> TidyOptionType {
    unsafe { tidyOptGetType(opt) }
  }

  pub fn tidyOptGetName(opt: TidyOption) -> String {
    unsafe { Self::c_str_to_owned(tidyOptGetName(opt)) }
  }

  pub fn tidyOptGetId(opt: TidyOption) -> TidyOptionId {
    unsafe { tidyOptGetId(opt) }
  }

  pub fn tidyOptGetCategory(opt: TidyOption) -> TidyConfigCategory {
    unsafe { tidyOptGetCategory(opt) }
  }

  pub fn tidyOptGetIdForName(optnam: &str) -> TidyOptionId {
    unsafe { tidyOptGetIdForName(CString::new(optnam).unwrap().as_ptr()) }
  }

  pub fn tidyOptSnapshot(&self) -> Result<TidySeverity, TidyError> {
    unsafe {
      match tidyOptSnapshot(self.tdoc) {
        Bool_yes => Ok(TidySeverity::Success),
        _ => Err(TidyError {
          severity: TidySeverity::Severe,
          message: String::from("Tidy option snapshot error"),
        }),
      }
    }
  }

  pub fn tidyOptResetToSnapshot(&self) -> Result<TidySeverity, TidyError> {
    unsafe {
      match tidyOptResetToSnapshot(self.tdoc) {
        Bool_yes => Ok(TidySeverity::Success),
        _ => Err(TidyError {
          severity: TidySeverity::Severe,
          message: String::from("Tidy reset to snapshot error"),
        }),
      }
    }
  }

  pub fn tidyOptDiffThanSnapshot(&self) -> bool {
    unsafe {
      match tidyOptDiffThanSnapshot(self.tdoc) {
        Bool_yes => true,
        _ => false,
      }
    }
  }

  pub fn tidyOptResetToDefault(&self, optid: TidyOptionId) -> Result<TidySeverity, TidyError> {
    unsafe {
      match tidyOptResetToDefault(self.tdoc, optid) {
        Bool_yes => Ok(TidySeverity::Success),
        _ => Err(TidyError {
          severity: TidySeverity::Severe,
          message: String::from("Tidy reset to default error"),
        }),
      }
    }
  }

  pub fn tidyOptDiffThanDefault(&self) -> bool {
    unsafe {
      match tidyOptDiffThanDefault(self.tdoc) {
        Bool_yes => true,
        _ => false,
      }
    }
  }

  pub fn tidyOptResetAll(&self) -> Result<TidySeverity, TidyError> {
    unsafe {
      match tidyOptResetAllToDefault(self.tdoc) {
        Bool_yes => Ok(TidySeverity::Success),
        _ => Err(TidyError {
          severity: TidySeverity::Severe,
          message: String::from("Tidy reset to default error"),
        }),
      }
    }
  }

  pub fn tidyOptCopyConfig(&self, tdoc_to: TidyDoc) -> Result<TidySeverity, TidyError> {
    unsafe {
      match tidyOptCopyConfig(tdoc_to, self.tdoc) {
        Bool_yes => Ok(TidySeverity::Success),
        _ => Err(TidyError {
          severity: TidySeverity::Severe,
          message: String::from("Tidy reset to default error"),
        }),
      }
    }
  }

  // Custom methods
  pub fn output_as_vector(&self) -> Option<Vec<u8>> {
    unsafe {
      if !(*self.errbuf).bp.is_null() {
        let c_str: &CStr = CStr::from_ptr((*self.output).bp as *const i8);
        return Some(c_str.to_bytes().to_vec());
      }
    }
    None
  }
  pub fn errbuf_as_string(&self) -> String {
    unsafe { Self::c_str_to_owned((*self.errbuf).bp as *const i8) }
  }
}

impl Drop for Tidy {
  fn drop(&mut self) {
    unsafe {
      //println! {"{:?}", *self.errbuf}
      if !(*self.errbuf).bp.is_null() {
        tidyBufFree(self.errbuf);
      }
      libc::free(self.errbuf as *mut libc::c_void);
      //println! {"{:?}", *self.errbuf}
      //println! {"{:?}", *self.output}
      if !(*self.output).bp.is_null() {
        tidyBufFree(self.output);
      }
      libc::free(self.output as *mut libc::c_void);
      //println! {"{:?}", *self.output}
      tidyRelease(self.tdoc);
      println!("All is tidy and free.")
    }
  }
}
