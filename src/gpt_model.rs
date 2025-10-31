#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GptModel {
    model_name: String,
}

impl GptModel {
    pub fn new(model_name: String) -> Self {
        GptModel { model_name }
    }
    
    pub fn as_str(&self) -> &str {
        &self.model_name
    }
}

impl AsRef<str> for GptModel {
    fn as_ref(&self) -> &str {
        &self.model_name
    }
}

impl Into<String> for GptModel {
    fn into(self) -> String {
        self.model_name
    }
}

impl Into<GptModel> for String {
    fn into(self) -> GptModel {
        GptModel::new(self)
    }
}