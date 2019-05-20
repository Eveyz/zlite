use crate::table::Row;

pub enum MetaCommandResult {
  MetaCommandSuccess,
  MetaCommandUnrecognizedCommand
}

pub enum PrepareResult {
  PrepareSuccess,
  PrepareUnrecognizedStatement,
  PrepareSyntaxError
}

pub enum StatementType {
  StatementInsert,
  StatementSelect
}

pub struct Statement {
  pub statement_type: Option<StatementType>,
  row_to_insert: Option<Row>,
}

impl Statement {
  pub fn new() -> Statement {
    Statement {
      statement_type: None,
      row_to_insert: None,
    }
  }
}