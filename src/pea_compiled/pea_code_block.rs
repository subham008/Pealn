#[derive(Clone, Debug, PartialEq)]
pub struct PeaCodeBlock {
    pub code: String,
}

impl PeaCodeBlock {
    pub fn from(raw_modifier: String) -> Option<Self> {
        //here we will check is the raw_modifier start and end with '{}' then only it willne consodered as code block
        let trimmed = raw_modifier.trim();
        if trimmed.starts_with('{') && trimmed.ends_with('}') {
            Some(PeaCodeBlock {
                code: trimmed[1..trimmed.len() - 1].to_string(),
            })
        } else {
            None
        }
    }
}
