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

  pub fn help(&self) -> String {
    let mut string_builder = String::new();

    for arg in self.args.iter() {
      string_builder.push_str(arg.option.help(SHORT_SWITCH, LONG_SWITCH).as_ref());
      string_builder.push('\n');
    }

    string_builder
  }

  fn new(args: String, options: Vec<CliOption>) -> CliRunner {
    CliRunner {
      args: Vec::new(),
      original_argument_line: args,
    }
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
    CliRunner::new(self.arg_line, self.options)
  }
}
