use one_dim_ca::{generator::make_rule_ca, rule::Rule};

fn main() {
    let rule = 30;


    make_rule_ca(Rule::new(rule), 1000, &format!("rule_{}.png", rule)).unwrap();
}
