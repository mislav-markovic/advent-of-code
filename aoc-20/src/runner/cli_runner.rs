use super::argument_parser::{CliArgument, CliOption};

const SHORT_SWITCH: &str = "-";
const LONG_SWITCH: &str = "--";

pub struct CliRunner {
  args: Vec<CliArgument>,
  original_argument_line: String,
}

impl CliRunner {
  pub fn from_builder() -> CliBuilder {
    CliBuilder::new()
  }

  pub fn help(&self) -> String {}

  fn new(args: String, options: Vec<CliOption>) -> CliRunner {}
}

pub struct CliBuilder {
  options: Vec<CliOption>,
  arg_line: String,
}

impl CliBuilder {
  fn new() -> Self {
    Self { options: Vec::new(),
           arg_line: String::new() }
  }

  pub fn register_option(mut self, option: CliOption) -> CliBuilder {
    self.options.push(option);
    self
  }

  pub fn with_arguments(mut self, args: &str) -> CliBuilder {
    self.arg_line = args.to_string();
    self
  }

  pub fn make_runner(self) -> CliRunner {
    CliRunner::new(self.arg_line, self.options)
  }
}
