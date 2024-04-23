use crate::{cell::Cell, graph::make_graph, rule::Rule, world::World};

pub fn make_rule_ca(
    rule: Rule,
    width: usize,
    out_file_name: &str,
    loop_edges: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut world = World::initial_central_live_cell(width, rule.clone(), loop_edges);
    let mut history: Vec<Vec<Cell>> = Vec::new();

    history.push(world.cells().into());
    while !world.is_glowed() {
        history.push(world.forward().into());
    }

    make_graph(out_file_name, &history, rule)?;

    Ok(())
}
