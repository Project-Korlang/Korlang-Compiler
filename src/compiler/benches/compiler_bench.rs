use criterion::{black_box, criterion_group, criterion_main, Criterion};
use korlang_compiler::lexer::Lexer;
use korlang_compiler::parser::Parser;

fn bench_compiler(c: &mut Criterion) {
    let src = r#"
        fun fib(n: Int) -> Int {
            if n < 2 { return n; }
            return fib(n - 1) + fib(n - 2);
        }

        struct Point {
            x: Float,
            y: Float
        }

        fun main() {
            let p = Point { x: 10.0, y: 20.0 };
            let res = fib(10);
            print("result: ${res}");
        }
    "#;

    c.bench_function("lexer", |b| {
        b.iter(|| {
            let lexer = Lexer::new(black_box(src));
            let _ = lexer.tokenize();
        })
    });

    let tokens = Lexer::new(src).tokenize().unwrap();
    c.bench_function("parser", |b| {
        b.iter(|| {
            let parser = Parser::new(black_box(tokens.clone()));
            let _ = parser.parse_program();
        })
    });
}

criterion_group!(benches, bench_compiler);
criterion_main!(benches);
