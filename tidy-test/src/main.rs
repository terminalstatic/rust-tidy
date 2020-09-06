#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use tidy::*;
use std::boxed::Box;
use std::error::Error;
use std::ffi::CStr;
use std::ffi::CString;
use std::fmt;
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

fn test_sub() -> Result<(), Box<dyn Error>> {
  let xml = "        
  <breakfast_menu>
    <food>
      <name>Belgian Waffles</name>
      <price>$5.95</price>
      <description>Two of our famous Belgian Waffles with plenty of real maple syrup with > 500 and < 800 calories</description>
      <calories>650</calories>
    </food>
    <food>
      <name>Strawberry Belgian Waffles</name>
      <price>$7.95</price>
      <description>Light Belgian waffles covered with strawberries and whipped cream</description>
      <calories>900</calories>
    </food>
    <food>
      <name>Berry-Berry Belgian Waffles</name>
      <price>$8.95</price>
      <description>Light Belgian waffles covered with an assortment of fresh berries and whipped cream</description>
      <calories>900</calories>
    </food>
    <food>
      <name>French Toast</name>
      <price>$4.50</price>
      <description>Thick slices made from our homemade sourdough bread</description>
      <calories>600</calories>
    </food>
    <food>
      <name>Homestyle Breakfast</name>
      <price>$6.95</price>
      <description>Two eggs, bacon or sausage, toast, and our ever-popular hash browns</description>
      <calories>950</calories>
    </food>
  </breakfast_menu>";
  let tidy = tidy::Tidy::new()?;
  println!("Tidy release date: {}", tidy.tidyReleaseDate());
  println!("Tidy library version: {}", tidy.tidyLibraryVersion());

  tidy.tidyOptSetBool(tidy::TidyOptionId::TidyXmlTags, true)?;
  tidy.tidyOptSetBool(tidy::TidyOptionId::TidyXmlDecl, true)?;

  let option: tidy::_TidyOption = unsafe { *tidy.tidyGetOption(tidy::TidyOptionId::TidyForceOutput) };
  let option_ptr = &option as tidy::TidyOption;
  println!("{:?}", tidy::Tidy::tidyOptGetName(option_ptr));
  println!("ID: {:?}", tidy::Tidy::tidyOptGetId(option_ptr));
  println!("Option: {:?}", option);
  tidy.tidySetCharEncoding("utf8")?;
  tidy.tidySetOutCharEncoding("utf8")?;
  tidy.tidyParseString(xml.as_bytes().to_vec())?;
  //tidy.tidyParseFile("./test.xml")?;

  println!("Tidy html version: {}", tidy.tidyDetectedHtmlVersion());

  tidy.tidyCleanAndRepair()?;
  match tidy.tidyRunDiagnostics() {
    Ok(v) => match v {
      tidy::TidySeverity::Error => {
        tidy.tidyOptSetBool(tidy::TidyOptionId::TidyForceOutput, true)?;
      }
      _ => (),
    },
    Err(e) => return Err(Box::new(e)),
  }

  println!("Tidy tdoc status: {}", tidy.tidyStatus());
  println!("Tidy xml?: {}", tidy.tidyDetectedGenericXml());
  println!("Tidy xhtml?: {}", tidy.tidyDetectedXhtml());

  println!("Tidy warning count: {}", tidy.tidyWarningCount());
  println!("Tidy error count: {}", tidy.tidyErrorCount());

  tidy.tidyErrorSummary();
  println!("\nDiagnostics:\n\n {}", tidy.errbuf_as_string());
  //tidy.tidySaveBuffer()?;
  tidy.tidySaveStdout()?;
  tidy.tidyOptSaveFile("./tidyOpts.cfg")?;
  //io::stdout().write_all(&tidy.output_as_vector().unwrap())?;

  Ok(())
}
pub fn main() -> Result<(), Box<dyn Error>> {
  let handle = thread::spawn(|| {
    for _i in 1..2 {
      test_sub();
      //thread::sleep(Duration::from_millis(10));
    }
  });

  handle.join().unwrap();

  Ok(())
}
