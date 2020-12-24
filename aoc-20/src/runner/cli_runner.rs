use super::argument_parser::CliOption;
pub struct CliRunner {
  options: Vec<CliOption>,
  args: String,
}

impl CliRunner {
  pub fn from_builder() -> CliBuilder {
    CliBuilder::new()
  }
}

pub struct CliBuilder {
  options: Vec<CliOption>,
  arg_line: String,
}

impl CliBuilder {
  fn new() -> Self {
    Self {
      options: Vec::new(),
      arg_line: String::new(),
    }
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
    CliRunner {
      options: self.options,
      args: self.arg_line,
    }
  }
}
