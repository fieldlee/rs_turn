
use stun::attributes::*;
use stun::message::*;

use util::Error;

// RFC 5766 Section 14.4
#[derive(Default, Debug, PartialEq)]
pub struct Data(pub Vec<u8>);

impl Setter for Data {
    // AddTo adds DATA to message.
    fn add_to(&self, m: &mut Message) -> Result<(), Error> {
        m.add(ATTR_DATA, &self.0);
        Ok(())
    }
}

impl Getter for Data {
    // GetFrom decodes DATA from message.
    fn get_from(&mut self, m: &Message) -> Result<(), Error> {
        self.0 = m.get(ATTR_DATA)?;
        Ok(())
    }
}
