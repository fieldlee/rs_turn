
use stun::attributes::*;
use stun::message::*;

use util::Error;

// DontFragmentAttr represents DONT-FRAGMENT attribute.
#[derive(Debug, Default, PartialEq)]
pub struct DontFragmentAttr;

impl Setter for DontFragmentAttr {
    // AddTo adds DONT-FRAGMENT attribute to message.
    fn add_to(&self, m: &mut Message) -> Result<(), Error> {
        m.add(ATTR_DONT_FRAGMENT, &[]);
        Ok(())
    }
}

impl Getter for DontFragmentAttr {
    // get_from returns true if DONT-FRAGMENT attribute is set.
    fn get_from(&mut self, m: &Message) -> Result<(), Error> {
        let _ = m.get(ATTR_DONT_FRAGMENT)?;
        Ok(())
    }
}
