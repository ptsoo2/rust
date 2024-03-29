use anyhow::bail;
use std::env::args;

use ex_common::{common, log};

#[derive(Default)]
pub struct CommandLine {
    pub args_: Vec<String>,
    pub config_file_path_: String,
}

impl CommandLine {
    pub fn load(mut self) -> anyhow::Result<CommandLine> {
        self.args_ = args().collect();
        log!("{:?}", self.args_);

        self._parse_config_path()?;
        Ok(self)
    }

    fn _parse_config_path(&mut self) -> anyhow::Result<()> {
        if self._empty() {
            // 없는 경우 하드코딩
            self.config_file_path_ = common::get_current_path_str() + "/cfg/config.yaml";
            return Ok(());
        }

        let file_path = self.args_.get(1);
        match file_path {
            Some(path) => {
                self.config_file_path_ = path.clone();
            }
            None => {
                bail!("not exist file path");
            }
        }
        Ok(())
    }

    fn _empty(&self) -> bool {
        self.args_.len() == 1
    }
}
