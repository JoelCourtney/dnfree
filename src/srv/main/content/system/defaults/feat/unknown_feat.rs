crate::name!("Unknown Feat");

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct UnknownFeat;

#[content]
impl Feat for UnknownFeat {
    properties! {}
}

describe! {r#"
    # Unknown Feat

    Please choose a feat. This is a placeholder.
"#}