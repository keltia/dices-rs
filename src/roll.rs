use crate::result::Res;

pub trait Roll {
    fn roll<'a>(&self, r: &'a mut Res) -> &'a mut Res;
}

