use std::io::Reader;

pub struct Body<'a> {
  reader: &'a mut Reader
}

impl<'a> Body<'a> {
  pub fn new<'b>(reader: &'b mut Reader) -> Body<'b> {
    Body { reader: reader }
  }
}
