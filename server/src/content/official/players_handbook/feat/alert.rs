#[macros::feat]
pub struct Alert;

impl Modify for Alert {
    fn modify(&self, c: &mut Character) {
        c.initiative += 5;
    }
}

impl Featured for Alert {
    features!([
        Feature {
            text: Alert::description_without_title(),
        }
    ]);
}

describe! { r#"
    # Alert

    Always on the lookout for danger, you gain the following benefits:

    - You gain a +5 bonus to initiative.
    - You can't be surprised while you are conscious.
    - Other creatures don't gain advantage on attack rolls against you as a result of being unseen by you.
"# }