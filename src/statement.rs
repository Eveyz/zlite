pub enum MetaCommandResult {
  MetaCommandSuccess,
  MetaCommandUnrecognizedCommand
}

pub enum PrepareResult {
  PrepareSuccess,
  PrepareUnrecognizedStatement
}

pub enum StatementType {
  StatementInsert,
  StatementSelect
}

const COLUMN_USERNAME_SIZE: usize = 32;
const COLUMN_EMAIL_SIZE: usize = 256;

struct Row {
  id: usize,
  username: [char; COLUMN_EMAIL_SIZE],
  email: [char; COLUMN_USERNAME_SIZE],
}

pub struct Statement {
  pub _type: Option<StatementType>,
  pub rowToInsert: Option<Row>,
}

impl Statement {
  fn new() -> Statement {
    Statement {
      _type: None,
      rowToInsert: None,
    }
  }
}