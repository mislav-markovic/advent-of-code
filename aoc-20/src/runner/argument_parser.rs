pub struct CliOption {
  short_name: String,
  long_name: String,
  mandatory: bool,
  is_flag: bool,
  description: String,
}

impl CliOption {
  pub fn new(
    short_name: &str,
    long_name: &str,
    mandatory: bool,
    is_flag: bool,
    desc: &str,
  ) -> Self {
    Self {
      short_name: short_name.to_string(),
      long_name: long_name.to_string(),
      mandatory: mandatory,
      is_flag: is_flag,
      description: desc.to_string(),
    }
  }

  pub fn help(&self, short_switch: &str, long_switch: &str) -> String {
    format!(
      "{s_switch}{s_name}, {l_switch}{l_name} | {desc}",
      s_switch = short_switch,
      s_name = self.short_name,
      l_switch = long_switch,
      l_name = self.long_name,
      desc = self.description
    )
  }
}

pub struct CliArgument {
  pub option: CliOption,
  pub value: Option<String>,
}

impl CliArgument {
  pub fn new(option: CliOption, value: &str) -> Self {
    Self {
      option: option,
      value: if value.is_empty() {
        None
      } else {
        Some(value.to_string())
      },
    }
  }
}
