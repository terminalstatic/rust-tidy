#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::boxed::Box;
use std::error::Error;
use tidy::*;


//use std::thread;
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
  let tidy = Tidy::new()?;
  println!("Tidy release date: {}", tidy.release_date());
  println!("Tidy library version: {}", tidy.library_version());

  tidy.opt_set_bool(TidyOptionId::TidyXmlTags, true)?;
  tidy.opt_set_bool(TidyOptionId::TidyXmlDecl, true)?;

  let option: TidyOption = tidy.get_option(TidyOptionId::TidyForceOutput);
  println!("Option ID: {:?}", Tidy::opt_get_id(option));
  tidy.set_char_encoding("utf8")?;
  tidy.set_out_char_encoding("utf8")?;
  tidy.parse_string(xml.as_bytes().to_vec())?;
  //tidy.tidyParseFile("./test.xml")?;

  println!("Tidy html version: {}", tidy.detected_html_version());

  tidy.clean_and_repair()?;
  match tidy.run_diagnostics() {
    Ok(v) => match v {
      TidySeverity::Error => {
        tidy.opt_set_bool(TidyOptionId::TidyForceOutput, true)?;
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
  tidy.save_buffer()?;
  //tidy.save_stdout()?;
  
  //tidy.opt_save_file("./tidyOpts.cfg")?;
  print!("{}", String::from_utf8_lossy(&TidyUtil::output_as_vector(&tidy).unwrap()));

  Ok(())
}

pub fn main() -> Result<(), Box<dyn Error>> {
  test_sub()
  /*let handle = thread::spawn(|| {
    for _i in 1..2 {
      match  {
        Err(_e) => panic!(),
        _ => (),
      }
    }
  });

  handle.join().unwrap();

  Ok(())*/
}
