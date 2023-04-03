use crate::fluent::LangResource;
use lang_id::LangID;
use std::ops::{Deref, DerefMut};

/// This implementation allows mutable access to the underlying data of the LangResource struct.
impl<'a> DerefMut for LangResource<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.id
    }
}

/// This implementation allows read-only access to the underlying data of the LangResource struct.
impl<'a> Deref for LangResource<'a> {
    type Target = LangID;

    fn deref(&self) -> &Self::Target {
        &self.id
    }
}
