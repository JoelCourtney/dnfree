#[macros::subrace("Halfling")]
pub struct Stout;

impl Content for Stout {
    fn modify(&self, c: &mut Character) {
        c.constitution += 1;
    }

    fn write_features(&self) -> Vec<FeatureSerial> {
        vec![
            FeatureSerial {
                text: indoc! {r"
                    # Ability Score Increase

                    Your Constitution score increases by 1.
                "},
                ..def!()
            },
            FeatureSerial {
                text: indoc! {r"
                    # Stout Resilience

                    You have advantage on saving throws against poison, and you have resistance against poison damage.
                "},
                ..def!()
            }
        ]
    }
}

describe! {r#"
    # Stout

    As a stout halfling, you’re hardier than average and have some resistance to poison. Some say that stouts have dwarven blood. In the Forgotten Realms, these halflings are called stronghearts, and they’re most common in the south.

    ## Ability Score Increase

    Your Constitution score increases by 1.

    ## Stout Resilience

    You have advantage on saving throws against poison, and you have resistance against poison damage.
"#}