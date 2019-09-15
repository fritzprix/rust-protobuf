//! This module contains functions references for reflection in generated code.

#![doc(hidden)]

pub use crate::reflect::accessor::map::make_map_accessor;
pub use crate::reflect::accessor::repeated::make_repeated_field_accessor;
pub use crate::reflect::accessor::repeated::make_vec_accessor;
pub use crate::reflect::accessor::singular::make_option_accessor;
pub use crate::reflect::accessor::singular::make_simple_field_accessor;
pub use crate::reflect::accessor::singular::make_singular_message_accessor;
pub use crate::reflect::accessor::singular::make_singular_ptr_field_accessor;
pub use crate::reflect::accessor::singular::make_singular_field_accessor;
pub use crate::reflect::accessor::singular::oneof::make_oneof_copy_has_get_set_accessors;
pub use crate::reflect::accessor::singular::oneof::make_oneof_deref_has_get_set_accessor;
pub use crate::reflect::accessor::singular::oneof::make_oneof_message_has_get_mut_set_accessor;
