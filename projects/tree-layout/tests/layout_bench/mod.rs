use super::*;
#[bench]
#[ignore]
fn bench_tidy_layout_chart(_bench: &mut Bencher) {
    let mut layout = LayoutConfig::new(10., 10.);

    let mut rng = StdRng::seed_from_u64(1001);
    let mut out = vec![];
    let (mut root, mut nodes) = generator::prepare_tree(&mut rng);
    for num in (1000..500_000).step_by(1000) {
        generator::insert_new_to_tree(&mut rng, 1000, &mut nodes);
        let start = Instant::now();
        layout.layout(&mut root);
        let time = Instant::now().duration_since(start);
        out.push((num, time.as_micros()));

        if num % 100_000 == 0 {
            println!("{}", num);
            assert_eq!(root.center.x, 0.0);
        }
    }

    for (num, time) in out {
        println!("{} {}", num, time);
    }
}

#[bench]
fn bench_tidy_layout(bench: &mut Bencher) {
    let mut rng = StdRng::seed_from_u64(1001);
    let mut tree = generator::gen_tree(&mut rng, 100_000);
    let mut layout = LayoutConfig::new(10., 10.);

    bench.iter(black_box(|| {
        layout.layout(&mut tree);
    }));
}

#[bench]
fn bench_tidy_layout_large(bench: &mut Bencher) {
    let mut rng = StdRng::seed_from_u64(1001);
    let mut tree = generator::gen_tree(&mut rng, 1_000_000);
    let mut layout = LayoutConfig::new(10., 10.);
    bench.iter(black_box(|| {
        layout.layout(&mut tree);
    }));
}
