use crate::{cell::Cell, graph::make_graph, rule::Rule, world::World};

pub fn make_rule_ca(rule: Rule, width: usize, out_file_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut world = World::initial_central_live_cell(width, rule.clone());
    let mut history: Vec<Vec<Cell>> = Vec::new();

    while !world.is_glowed() {
        history.push(world.forward().into());
    }

    make_graph(out_file_name, &history, rule)?;

    Ok(())
}
