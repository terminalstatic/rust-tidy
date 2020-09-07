#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use libc::*;
use std::boxed::Box;
use std::error::Error;
use std::ffi::CStr;
use std::ffi::CString;
use std::fmt;

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

pub struct TidyUtil;
impl TidyUtil {
  pub fn c_str_to_owned(in_str: ctmbstr) -> String {
    let c_str: &CStr = unsafe { CStr::from_ptr(in_str) };
    let str_slice: &str = c_str.to_str().unwrap();
    str_slice.to_owned()
  }

  pub fn bool_to_tidy_bool(bool_in: bool) -> Bool {
    match bool_in {
      true => Bool_yes,
      _ => Bool_no,
    }
  }
  pub fn tidy_bool_to_bool(tidy_bool_in: Bool) -> bool {
    match tidy_bool_in {
      Bool_yes => true,
      _ => false,
    }
  }

  pub fn output_as_vector(tidy: &Tidy) -> Option<Vec<u8>> {
    unsafe {
      if !(*tidy.output).bp.is_null() {
        let c_str: &CStr = CStr::from_ptr((*tidy.output).bp as *const i8);
        return Some(c_str.to_bytes().to_vec());
      }
    }
    None
  }
  pub fn errbuf_as_string(tidy: &Tidy) -> String {
    unsafe { TidyUtil::c_str_to_owned((*tidy.errbuf).bp as *const i8) }
  }
}
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

  // Basic operations
  /// Indicates the number of TidyAccess messages that were generated.
  ///
  /// # Returns
  /// Returns the number of TidyAccess messages that were generated.
  pub fn access_warning_count(&self) -> c_uint {
    unsafe { tidyAccessWarningCount(self.tdoc) }
  }

  /// Indicates the number of configuration error messages that were generated.
  ///
  /// # Returns
  /// Returns the number of configuration error messages that were generated.
  pub fn config_error_count(&self) -> c_uint {
    unsafe { tidyConfigErrorCount(self.tdoc) }
  }

  /// Indicates whether or not the input document was XML.
  ///
  /// If TidyXml tags is true, or there was an XML declaration in the input document, then this function will return yes.
  ///
  /// # Returns
  /// Returns true if the input document was XML.
  pub fn detected_generic_xml(&self) -> bool {
    unsafe { TidyUtil::tidy_bool_to_bool(tidyDetectedGenericXml(self.tdoc)) }
  }

  /// Gets the version of HTML that was output, as an integer, times 100.
  ///
  /// For example, HTML5 will return 500; HTML4.0.1 will return 401.
  ///
  /// # Returns
  /// Returns the HTML version number (x100).
  pub fn detected_html_version(&self) -> c_int {
    unsafe { tidyDetectedHtmlVersion(self.tdoc) }
  }

  /// Indicates whether the output document is or isn't XHTML.
  ///
  /// # Returns
  /// Returns true if the document is an XHTML type.
  pub fn detected_xhtml(&self) -> bool {
    unsafe {
      match tidyDetectedXhtml(self.tdoc) {
        Bool_yes => true,
        _ => false,
      }
    }
  }

  /// Indicates the number of TidyError messages that were generated.
  ///
  /// For any value greater than 0, output is suppressed unless TidyForceOutput is set.
  ///
  /// # Returns
  /// Returns the number of TidyError messages that were generated.
  pub fn error_count(&self) -> c_uint {
    unsafe { tidyErrorCount(self.tdoc) }
  }

  /// Get the version number for the current library.
  /// Returns
  /// The string representing the version number.
  pub fn library_version(&self) -> String {
    unsafe { TidyUtil::c_str_to_owned(tidyLibraryVersion()) }
  }

  /// Load an ASCII Tidy configuration file and set the configuration per its contents.
  ///
  /// # Returns
  /// Returns TidySeverity::Success upon success, or a TidyError if there was an error.
  ///
  /// # Parameters
  /// **config_file**	The complete path to the file to load.
  pub fn load_config(&self, config_file: &str) -> Result<TidySeverity, TidyError> {
    unsafe {
      match tidyLoadConfig(self.tdoc, CString::new(config_file).unwrap().as_ptr()) {
        0 => Ok(TidySeverity::Success),
        _ => Err(TidyError {
          severity: TidySeverity::Severe,
          message: String::from("Tidy load config error"),
        }),
      }
    }
  }

  /// Get the release date for the current library.
  ///
  /// # Returns
  /// The string representing the release date.
  pub fn release_date(&self) -> String {
    unsafe { TidyUtil::c_str_to_owned(tidyReleaseDate()) }
  }

  /// Set the input/output character encoding for parsing markup.
  ///
  /// Valid values include ascii, latin1, raw, utf8, iso2022, mac, win1252, utf16le, utf16be, utf16, big5, and shiftjis. These values are not case sensitive.
  ///
  /// # Note
  /// This is the same as using set_in_char_encoding() and set_out_char_encoding() to set the same value.
  /// # Returns
  /// Returns TidySeverity::Success upon success, or a TidyError if there was an error.
  /// # Parameters
  /// **encnam**	The encoding name as described above.
  pub fn set_char_encoding(&self, encnam: &str) -> Result<TidySeverity, TidyError> {
    unsafe {
      match tidySetCharEncoding(self.tdoc, CString::new(encnam).unwrap().as_ptr()) {
        0 => Ok(TidySeverity::Success),
        _ => Err(TidyError {
          severity: TidySeverity::Severe,
          message: String::from("Tidy set char encoding error"),
        }),
      }
    }
  }

  /// Set the input encoding for parsing markup.
  ///
  /// Valid values include ascii, latin1, raw, utf8, iso2022, mac, win1252, utf16le, utf16be, utf16, big5, and shiftjis. These values are not case sensitive.
  ///
  /// # Returns
  /// Returns TidySeverity::Success upon success, or a TidyError if there was an error.
  /// # Parameters  
  /// **encnam**	The encoding name as described above.
  pub fn set_in_char_encoding(&self, encnam: &str) -> Result<TidySeverity, TidyError> {
    unsafe {
      match tidySetInCharEncoding(self.tdoc, CString::new(encnam).unwrap().as_ptr()) {
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

  /// Set the output encoding for writing markup.
  ///
  /// Valid values include ascii, latin1, raw, utf8, iso2022, mac, win1252, utf16le, utf16be, utf16, big5, and shiftjis. These values are not case sensitive.
  ///
  /// # Returns
  /// Returns TidySeverity::Success upon success, or a TidyError if there was an error.
  /// # Parameters
  /// **encnam**	The encoding name as described above.
  pub fn set_out_char_encoding(&self, encnam: &str) -> Result<TidySeverity, TidyError> {
    unsafe {
      match tidySetOutCharEncoding(self.tdoc, CString::new(encnam).unwrap().as_ptr()) {
        0 => Ok(TidySeverity::Success),
        _ => Err(TidyError {
          severity: TidySeverity::Severe,
          message: String::from("Tidy set out char encoding error"),
        }),
      }
    }
  }

  /// Get status of current document.
  ///
  /// # Returns
  /// Returns TidySeverity::Error indicating that errors were present in the document, TidySeverity::Warning indicating warnings, and TidySeverity::Success in the case of everything being okay.
  pub fn status(&self) -> TidySeverity {
    unsafe {
      match tidyStatus(self.tdoc) {
        0 => TidySeverity::Success,
        1 => TidySeverity::Warning,
        2 => TidySeverity::Error,
        _ => TidySeverity::Severe,
      }
    }
  }

  /// Indicates the number of TidyWarning messages that were generated.
  ///
  /// # Returns
  /// Returns the number of warning messages that were generated.
  pub fn warning_count(&self) -> c_uint {
    unsafe { tidyWarningCount(self.tdoc) }
  }

  // Configuration options
  pub fn opt_get_default_bool(opt: TidyOption) -> bool {
    unsafe {
      match tidyOptGetDefaultBool(opt) {
        Bool_yes => true,
        _ => false,
      }
    }
  }

  pub fn opt_get_default_int(opt: TidyOption) -> c_ulong {
    unsafe { tidyOptGetDefaultInt(opt) }
  }

  pub fn opt_get_default(opt: TidyOption) -> String {
    unsafe { TidyUtil::c_str_to_owned(tidyOptGetDefault(opt)) }
  }

  pub fn tidyOptGetBool(&self, optid: TidyOptionId) -> bool {
    unsafe {
      match tidyOptGetBool(self.tdoc, optid) {
        Bool_yes => true,
        _ => false,
      }
    }
  }

  pub fn opt_set_bool(&self, optid: TidyOptionId, val: bool) -> Result<TidySeverity, TidyError> {
    unsafe {
      match tidyOptSetBool(self.tdoc, optid, TidyUtil::bool_to_tidy_bool(val)) {
        Bool_yes => Ok(TidySeverity::Success),
        _ => Err(TidyError {
          severity: TidySeverity::Severe,
          message: String::from("Tidy  option set bool error"),
        }),
      }
    }
  }

  pub fn opt_get_int(&self, optid: TidyOptionId) -> c_ulong {
    unsafe { tidyOptGetInt(self.tdoc, optid) }
  }

  pub fn opt_set_int(&self, optid: TidyOptionId, val: c_ulong) -> Result<TidySeverity, TidyError> {
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

  pub fn opt_get_value(&self, optid: TidyOptionId) -> String {
    unsafe { TidyUtil::c_str_to_owned(tidyOptGetValue(self.tdoc, optid)) }
  }

  pub fn opt_set_value(&self, optid: TidyOptionId, val: &str) -> Result<TidySeverity, TidyError> {
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

  pub fn opt_parse_value(&self, optnam: &str, val: &str) -> Result<TidySeverity, TidyError> {
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

  pub fn get_option_by_name(&self, optnam: &str) -> TidyOption {
    unsafe { tidyGetOptionByName(self.tdoc, CString::new(optnam).unwrap().as_ptr()) }
  }

  pub fn opt_get_enc_name(&self, optid: TidyOptionId) -> String {
    unsafe { TidyUtil::c_str_to_owned(tidyOptGetEncName(self.tdoc, optid)) }
  }

  pub fn opt_get_doc(&self, opt: TidyOption) -> String {
    unsafe { TidyUtil::c_str_to_owned(tidyOptGetDoc(self.tdoc, opt)) }
  }

  pub fn get_option(&self, optid: TidyOptionId) -> TidyOption {
    unsafe { tidyGetOption(self.tdoc, optid) }
  }

  pub fn opt_get_type(opt: TidyOption) -> TidyOptionType {
    unsafe { tidyOptGetType(opt) }
  }

  pub fn opt_get_name(opt: TidyOption) -> String {
    unsafe { TidyUtil::c_str_to_owned(tidyOptGetName(opt)) }
  }

  pub fn opt_get_id(opt: TidyOption) -> TidyOptionId {
    unsafe { tidyOptGetId(opt) }
  }

  pub fn opt_get_category(opt: TidyOption) -> TidyConfigCategory {
    unsafe { tidyOptGetCategory(opt) }
  }

  pub fn opt_get_id_for_name(optnam: &str) -> TidyOptionId {
    unsafe { tidyOptGetIdForName(CString::new(optnam).unwrap().as_ptr()) }
  }

  pub fn opt_snapshot(&self) -> Result<TidySeverity, TidyError> {
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

  pub fn opt_reset_to_snapshot(&self) -> Result<TidySeverity, TidyError> {
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

  pub fn opt_diff_than_snapshot(&self) -> bool {
    unsafe {
      match tidyOptDiffThanSnapshot(self.tdoc) {
        Bool_yes => true,
        _ => false,
      }
    }
  }

  pub fn opt_reset_to_default(&self, optid: TidyOptionId) -> Result<TidySeverity, TidyError> {
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

  pub fn opt_diff_than_default(&self) -> bool {
    unsafe {
      match tidyOptDiffThanDefault(self.tdoc) {
        Bool_yes => true,
        _ => false,
      }
    }
  }

  pub fn opt_reset_all(&self) -> Result<TidySeverity, TidyError> {
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

  pub fn opt_copy_config(&self, tdoc_to: TidyDoc) -> Result<TidySeverity, TidyError> {
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

  // Diagnose and repair
  pub fn run_diagnostics(&self) -> Result<TidySeverity, TidyError> {
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

  pub fn clean_and_repair(&self) -> Result<TidySeverity, TidyError> {
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

  pub fn report_doctype(&self) -> TidySeverity {
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
  pub fn parse_string(&self, input: Vec<u8>) -> Result<TidySeverity, TidyError> {
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

  pub fn parse_file(&self, filename: &str) -> Result<TidySeverity, TidyError> {
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

  pub fn parse_stdin(&self) -> Result<TidySeverity, TidyError> {
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
  pub fn save_buffer(&self) -> Result<TidySeverity, TidyError> {
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

  pub fn save_file(&self, filename: &str) -> Result<TidySeverity, TidyError> {
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

  pub fn save_stdout(&self) -> Result<TidySeverity, TidyError> {
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

  pub fn opt_save_file(&self, filename: &str) -> Result<TidySeverity, TidyError> {
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
    }
  }
}
