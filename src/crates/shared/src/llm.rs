use crate::{configuration::CONFIG, serde_extensions::get_yaml_value};

pub fn get_prompt(path: &str) -> String {
    let err_msg = "The prompt config must be possible to load";
    let content = std::fs::read_to_string(&CONFIG.ai.prompts_path).expect(err_msg);
    get_yaml_value(&content, path).expect(err_msg)
}
