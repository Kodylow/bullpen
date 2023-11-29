use std::fmt::{Display, Error, Formatter, Result};

use serde::{Deserialize, Serialize};

// https://api.perplexity.ai
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum PplxChatModel {
    #[serde(rename = "mistral-7b-instruct")]
    Mistral7bInstruct,
    #[serde(rename = "codellama-34b-instruct")]
    Codellama34bInstruct,
    #[serde(rename = "llama-2-13b-chat")]
    Llama213bChat,
    #[serde(rename = "llama-2-70b-chat")]
    Llama270bChat,
    // #[serde(rename = "openhermes-2-mistral-7b")]
    // Openhermes2Mistral7b,
    // #[serde(rename = "openhermes-2.5-mistral-7b")]
    // Openhermes25Mistral7b,
    #[serde(rename = "pplx-7b-chat")]
    Pplx7bChat,
    #[serde(rename = "pplx-70b-chat")]
    Pplx70bChat,
    #[serde(rename = "pplx-7b-online")]
    Pplx7bOnline,
    #[serde(rename = "pplx-70b-online")]
    Pplx70bOnline,
}

impl Display for PplxChatModel {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", serde_json::to_string(self).map_err(|_| Error)?)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum PplxCompletionModel {
    #[serde(rename = "replit-code-v1.5-3b")]
    ReplitCodeV15_3b,
}

impl Display for PplxCompletionModel {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", serde_json::to_string(self).map_err(|_| Error)?)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ModelfarmChatModel {
    #[serde(rename = "chat-bison")]
    ChatBison,
}

impl Display for ModelfarmChatModel {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", serde_json::to_string(self).map_err(|_| Error)?)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ModelfarmCompletionModel {
    #[serde(rename = "text-bison")]
    TextBison,
}

impl Display for ModelfarmCompletionModel {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", serde_json::to_string(self).map_err(|_| Error)?)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ModelfarmEmbeddingModel {
    #[serde(rename = "textembedding-gecko")]
    TextEmbeddingGecko,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum OllamaModel {
    #[serde(rename = "mistral")]
    Mistral,
    #[serde(rename = "llama2")]
    Llama2,
    #[serde(rename = "codellama")]
    Codellama,
    #[serde(rename = "llama2-uncensored")]
    Llama2Uncensored,
    #[serde(rename = "orca-mini")]
    OrcaMini,
    #[serde(rename = "vicuna")]
    Vicuna,
    #[serde(rename = "wizard-vicuna-uncensored")]
    WizardVicunaUncensored,
    #[serde(rename = "phind-codellama")]
    PhindCodellama,
    #[serde(rename = "mistral-openorca")]
    MistralOpenorca,
    #[serde(rename = "nous-hermes")]
    NousHermes,
    #[serde(rename = "wizardcoder")]
    Wizardcoder,
    #[serde(rename = "zephyr")]
    Zephyr,
    #[serde(rename = "wizard-math")]
    WizardMath,
    #[serde(rename = "llama2-chinese")]
    Llama2Chinese,
    #[serde(rename = "stable-beluga")]
    StableBeluga,
    #[serde(rename = "falcon")]
    Falcon,
    #[serde(rename = "codeup")]
    Codeup,
    #[serde(rename = "everythinglm")]
    Everythinglm,
    #[serde(rename = "medllama2")]
    Medllama2,
    #[serde(rename = "wizardlm-uncensored")]
    WizardlmUncensored,
    #[serde(rename = "wizard-vicuna")]
    WizardVicuna,
    #[serde(rename = "open-orca-platypus2")]
    OpenOrcaPlatypus2,
    #[serde(rename = "starcoder")]
    Starcoder,
    #[serde(rename = "dolphin2.2-mistral")]
    Dolphin22Mistral,
    #[serde(rename = "orca2")]
    Orca2,
    #[serde(rename = "yarn-mistral")]
    YarnMistral,
    #[serde(rename = "deepseek-coder")]
    DeepseekCoder,
    #[serde(rename = "openhermes2.5-mistral")]
    Openhermes25Mistral,
    #[serde(rename = "samantha-mistral")]
    SamanthaMistral,
    #[serde(rename = "openchat")]
    Openchat,
    #[serde(rename = "openhermes2-mistral")]
    Openhermes2Mistral,
    #[serde(rename = "sqlcoder")]
    Sqlcoder,
    #[serde(rename = "yi")]
    Yi,
    #[serde(rename = "wizardlm")]
    Wizardlm,
    #[serde(rename = "dolphin2.1-mistral")]
    Dolphin21Mistral,
    #[serde(rename = "mistrallite")]
    Mistrallite,
    #[serde(rename = "codebooga")]
    Codebooga,
    #[serde(rename = "yarn-llama2")]
    YarnLlama2,
    #[serde(rename = "neural-chat")]
    NeuralChat,
    #[serde(rename = "goliath")]
    Goliath,
    #[serde(rename = "nexusraven")]
    Nexusraven,
    #[serde(rename = "xwinlm")]
    Xwinlm,
    #[serde(rename = "alfred")]
    Alfred,
}
