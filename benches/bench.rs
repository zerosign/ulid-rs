use bencher::{benchmark_group, benchmark_main, Bencher};
use chrono::Utc;
use ulid::{Generator, Ulid, ULID_LEN};
use rand::rngs::ThreadRng;

fn bench_new(b: &mut Bencher) {
    b.iter(|| Ulid::new());
}

fn bench_from_generate_with_datetime_source(b: &mut Bencher) {
	 let time = Utc::now();
	 let mut rng = ThreadRng::default();
	 let mut gen = Generator::new();

	 b.iter(|| {
		  let result = gen.generate_from_datetime_with_source(time, &mut rng);
		  assert!(result.is_ok());
	 });
}

fn bench_generator_generate(b: &mut Bencher) {
    let mut gen = Generator::new();
    b.iter(|| gen.generate().unwrap());
}

fn bench_from_time(b: &mut Bencher) {
    let time = Utc::now();
    b.iter(|| Ulid::from_datetime(time));
}

fn bench_to_str(b: &mut Bencher) {
    let ulid = Ulid::new();
    b.iter(|| {
        let mut buffer = [0; ULID_LEN];
        ulid.to_str(&mut buffer).unwrap();
    });
}

fn bench_to_string(b: &mut Bencher) {
    let ulid = Ulid::new();
    b.iter(|| ulid.to_string());
}

fn bench_from_string(b: &mut Bencher) {
    let s = Ulid::new().to_string();
    b.iter(|| Ulid::from_string(&s).unwrap());
}

benchmark_group!(
    ulid_perf,
    bench_new,
    bench_generator_generate,
    bench_from_time,
	 bench_from_generate_with_datetime_source,
    bench_to_str,
    bench_to_string,
    bench_from_string
);

benchmark_main!(ulid_perf);
