// use std::mem;

const COLUMN_USERNAME_SIZE: usize = 32;
const COLUMN_EMAIL_SIZE: usize = 256;

#[repr(C)]
pub struct Row {
  id: usize,
  username: [char; COLUMN_EMAIL_SIZE],
  email: [char; COLUMN_USERNAME_SIZE],
}

const ID_SIZE: usize = std::mem::size_of::<Row>();

pub struct Table {
  pub num_rows: usize
}