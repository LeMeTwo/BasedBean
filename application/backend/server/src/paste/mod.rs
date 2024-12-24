mod key;
mod manager;

pub use key::KeyClient as KeyClient;
pub use manager::add_paste as add_paste;
pub use manager::delete_paste as delete_paste;
pub use manager::get_paste as get_paste;