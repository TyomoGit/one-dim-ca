use std::fs;

use indicatif::ProgressBar;
use one_dim_ca::{generator::make_rule_ca, rule::Rule};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

fn main() {
    let _ = fs::create_dir("graph");

    let bar = ProgressBar::new(u8::MAX as u64);

    (0..=u8::MAX).into_par_iter().for_each(|rule| {
        for is_loop_edges in [true, false] {
            let loop_edges = if is_loop_edges { "_loop" } else { "" };
            make_rule_ca(
                Rule::new(rule),
                1000,
                &format!("graph/rule_{}{}.png", rule, loop_edges),
                is_loop_edges,
            )
            .unwrap();
        }
        bar.inc(1);
    });
}
