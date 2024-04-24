//! One-dimensional Cellular Automaton

pub mod cell;
#[cfg(feature = "graph")]
pub mod generator;
#[cfg(feature = "graph")]
pub mod graph;
pub mod rule;
pub mod world;
