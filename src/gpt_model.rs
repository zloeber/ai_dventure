#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum GptModel {
    Gpt3_5Turbo,
    Gpt4o,
    Gpt4oMini,
    Gpt4Turbo,
    Gpt5,
    OpenAio4Mini,
    OpenAio3Pro
}

impl Into<String> for GptModel {
    fn into(self) -> String {
        match self {
            GptModel::Gpt3_5Turbo => "gpt-3.5-turbo".to_string(),
            GptModel::Gpt4o => "gpt-4o".to_string(),
            GptModel::Gpt4oMini => "gpt-4o-mini".to_string(),
            GptModel::Gpt4Turbo => "gpt-4-turbo".to_string(),
            GptModel::Gpt5 => "gpt-5".to_string(),
            GptModel::OpenAio4Mini => "openai-o4-mini".to_string(),
            GptModel::OpenAio3Pro => "openai-o3-pro".to_string()
        }
    }
}

impl Into<GptModel> for String {
    fn into(self) -> GptModel {
        match self.as_str() {
            "gpt-3.5-turbo" => GptModel::Gpt3_5Turbo,
            "gpt-4o" => GptModel::Gpt4o,
            "gpt-4o-mini" => GptModel::Gpt4oMini,
            "gpt-4-turbo" => GptModel::Gpt4Turbo,
            "gpt-5" => GptModel::Gpt5,
            "openai-o4-mini" => GptModel::OpenAio4Mini,
            "openai-o3-mini" => GptModel::OpenAio3Pro,
            _ => panic!("Unknown model: {}", self)
        }
    }
}