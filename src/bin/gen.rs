fn main() {
    let mut libfunc_root = "".to_string();
    let mut bench_root =
        "use criterion::{criterion_group, criterion_main, Criterion};use uuid::Uuid;".to_string();
    let mut criterion_group = |targets: &str| {
        format!(
            r#"
        criterion_group!(
            name = benches;
            config = Criterion::default();
            targets =
                {targets}

        );
        criterion_main!(benches);
    "#
        )
    };
    let mut targets = vec![];
    let func = |(libfunc, bench, [name1, name2]): &(String, String, [String; 2])| {
        libfunc_root.push_str(&libfunc);
        bench_root.push_str(&bench);
        targets.push(name1.clone());
        targets.push(name2.clone());
    };
    [build(10), build(100), build(500), build(1000)]
        .iter()
        .for_each(func);

    std::fs::write("./src/lib.rs", libfunc_root.as_bytes()).unwrap();
    bench_root.push_str(&criterion_group(&targets.join(",\n")));
    std::fs::write("./benches/benchmark.rs", bench_root.as_bytes()).unwrap();
}

fn build(r: i64) -> (String, String, [String; 2]) {
    let mut stack = vec!["_ => -1,".to_string()];
    let mut stack2 = vec!["_ => -1,".to_string()];
    for i in 0..r {
        let val = uuid::Uuid::new_v4();
        let s = format!("\"{val}\" => {i},");
        let s2 = format!("b\"{val}\" => {i},");
        stack.push(s);
        stack2.push(s2);
    }

    stack.reverse();
    stack2.reverse();
    let got = stack.join("\n");
    let got2 = stack2.join("\n");

    let name1 = format!("uuid_v4_cmp_string_{r}");
    let name2 = format!("uuid_v4_cmp_bytes_{r}");
    let bench = format!(
        r#"
        fn {name1}(c: &mut Criterion) {{
            c.bench_function("{name1}", |b| {{
                let uuid_v4 = Uuid::new_v4();
                let s = uuid_v4.to_string();
                b.iter(|| optimizing_serde_slash_rust_match_statement::{name1}(&s));
            }});
        }}

        fn {name2}(c: &mut Criterion) {{
            c.bench_function("{name2}", |b| {{
                let uuid_v4 = Uuid::new_v4();
                let s = uuid_v4.to_string();
                b.iter(|| optimizing_serde_slash_rust_match_statement::{name2}(&s));
            }});
        }}
    "#
    );
    let s = format!(
        r#"

        pub fn uuid_v4_cmp_string_{r}(s: &str) -> i64 {{
            match s {{{got}}}
        }}


        pub fn uuid_v4_cmp_bytes_{r}(s: &str) -> i64 {{
            match s.as_bytes() {{{got2}}}
        }}
    "#
    );
    return (s, bench, [name1, name2]);
}
