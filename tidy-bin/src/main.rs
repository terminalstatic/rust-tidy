#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::boxed::Box;
use std::error::Error;
use std::thread;
use tidy::TidyUtil;
//use std::io::{self, Write};
//use std::time::Duration;

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
  println!("Tidy release date: {}", tidy.release_date());
  println!("Tidy library version: {}", tidy.library_version());

  tidy.opt_set_bool(tidy::TidyOptionId::TidyXmlTags, true)?;
  tidy.opt_set_bool(tidy::TidyOptionId::TidyXmlDecl, true)?;

  let option: tidy::_TidyOption = unsafe { *tidy.get_option(tidy::TidyOptionId::TidyForceOutput) };
  let option_ptr = &option as tidy::TidyOption;
  println!("{:?}", tidy::Tidy::opt_get_name(option_ptr));
  println!("ID: {:?}", tidy::Tidy::opt_get_id(option_ptr));
  println!("Option: {:?}", option);
  tidy.set_char_encoding("utf8")?;
  tidy.set_out_char_encoding("utf8")?;
  tidy.parse_string(xml.as_bytes().to_vec())?;
  //tidy.tidyParseFile("./test.xml")?;

  println!("Tidy html version: {}", tidy.detected_html_version());

  tidy.clean_and_repair()?;
  match tidy.run_diagnostics() {
    Ok(v) => match v {
      tidy::TidySeverity::Error => {
        tidy.opt_set_bool(tidy::TidyOptionId::TidyForceOutput, true)?;
      }
      _ => (),
    },
    Err(e) => return Err(Box::new(e)),
  }

  println!("Tidy tdoc status: {}", tidy.status());
  println!("Tidy xml?: {}", tidy.detected_generic_xml());
  println!("Tidy xhtml?: {}", tidy.detected_xhtml());

  println!("Tidy warning count: {}", tidy.warning_count());
  println!("Tidy error count: {}", tidy.error_count());

  println!("\nDiagnostics:\n\n {}", TidyUtil::errbuf_as_string(&tidy));
  //tidy.tidySaveBuffer()?;
  tidy.save_stdout()?;
  tidy.opt_save_file("./tidyOpts.cfg")?;
  //io::stdout().write_all(&tidy.output_as_vector().unwrap())?;

  Ok(())
}

pub fn main() -> Result<(), Box<dyn Error>> {
  let handle = thread::spawn(|| {
    for _i in 1..2 {
      match test_sub() {
        Err(_e) => panic!(),
        _ => (),
      }
      //thread::sleep(Duration::from_millis(10));
    }
  });

  handle.join().unwrap();

  Ok(())
}
