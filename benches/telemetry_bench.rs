use std::collections::HashMap;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use std::hint::black_box;

// 請確認 src/lib.rs 有 pub mod lex; pub mod par;
use cpl::lex::lexer::Lexer; // 根據你的實際名稱/路徑微調
use cpl::par::parser::SyntaxTree;

fn bench_parser_expressions(c: &mut Criterion) {
    let inputs = [
        ("input1", "a = 5"),
        ("input2", "-1 + 1"),
        ("input3", "-(a + 5) -5"),
    ];

    let mut group = c.benchmark_group("SyntaxTree_Parse");
    let mut var_table:HashMap<String, f64> = HashMap::new();

    for (name, input) in inputs {
        group.bench_with_input(BenchmarkId::new("eval", name), input, |b, input_str| {
            b.iter(|| {
                // 1. Lexing
                let mut lexer = Lexer::new(black_box(input_str));
                lexer.scan_tokens();

                // 2. Parsing
                let mut tree = SyntaxTree::new(lexer.tokens);
                tree.parser_expression(0.0);

                //3. Eval
                let start_index = tree.nodes.len() - 1;
                tree.eval(start_index, &mut var_table);

                black_box(tree)
            });
        });
    }

    group.finish();
}

criterion_group!(benches, bench_parser_expressions);
criterion_main!(benches);