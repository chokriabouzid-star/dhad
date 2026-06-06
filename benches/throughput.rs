//! Throughput benchmarks — Dhad Pipeline
//! Fixtures embedded in source (no external files required)

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use dhad::mode_b::build_frame;
use dhad::model::DhadAtom;
use dhad::modes::{process_mode_a, process_mode_b};
use dhad::registry::base;

// ─── Fixtures ─────────────────────────────────────────────────────

/// البسملة الكاملة مع التشكيل
const BISMILLAH: &[u8] = "بِسْمِ اللَّهِ الرَّحْمَٰنِ الرَّحِيمِ".as_bytes();

/// فاتحة الكتاب كاملة مع التشكيل
const AL_FATIHA: &[u8] = concat!(
    "بِسْمِ اللَّهِ الرَّحْمَٰنِ الرَّحِيمِ ",
    "الْحَمْدُ لِلَّهِ رَبِّ الْعَالَمِينَ ",
    "الرَّحْمَٰنِ الرَّحِيمِ ",
    "مَالِكِ يَوْمِ الدِّينِ ",
    "إِيَّاكَ نَعْبُدُ وَإِيَّاكَ نَسْتَعِينُ ",
    "اهْدِنَا الصِّرَاطَ الْمُسْتَقِيمَ ",
    "صِرَاطَ الَّذِينَ أَنْعَمْتَ عَلَيْهِمْ ",
    "غَيْرِ الْمَغْضُوبِ عَلَيْهِمْ وَلَا الضَّالِّينَ"
)
.as_bytes();

fn arabic_1kb() -> Vec<u8> {
    let mut v = Vec::with_capacity(1024);
    while v.len() < 1024 {
        v.extend_from_slice(BISMILLAH);
        v.push(b' ');
    }
    v.truncate(1024);
    v
}

fn arabic_64kb() -> Vec<u8> {
    let mut v = Vec::with_capacity(65536);
    while v.len() < 65536 {
        v.extend_from_slice(BISMILLAH);
        v.push(b' ');
    }
    v.truncate(65536);
    v
}

// ─── Mode A Benchmarks ─────────────────────────────────────────────

fn bench_mode_a(c: &mut Criterion) {
    let mut group = c.benchmark_group("mode_a");

    let alef = &[0xD8u8, 0xA7u8];
    group.throughput(Throughput::Bytes(alef.len() as u64));
    group.bench_function("alef_bare", |b| {
        b.iter(|| process_mode_a(criterion::black_box(alef)).unwrap())
    });

    group.throughput(Throughput::Bytes(BISMILLAH.len() as u64));
    group.bench_function("bismillah", |b| {
        b.iter(|| process_mode_a(criterion::black_box(BISMILLAH)).unwrap())
    });

    group.throughput(Throughput::Bytes(AL_FATIHA.len() as u64));
    group.bench_function("al_fatiha", |b| {
        b.iter(|| process_mode_a(criterion::black_box(AL_FATIHA)).unwrap())
    });

    let kb1 = arabic_1kb();
    group.throughput(Throughput::Bytes(kb1.len() as u64));
    group.bench_function("arabic_1kb", |b| {
        b.iter(|| process_mode_a(criterion::black_box(&kb1)).unwrap())
    });

    let kb64 = arabic_64kb();
    group.throughput(Throughput::Bytes(kb64.len() as u64));
    group.bench_function("arabic_64kb", |b| {
        b.iter(|| process_mode_a(criterion::black_box(&kb64)).unwrap())
    });

    group.finish();
}

// ─── Mode B Benchmarks ─────────────────────────────────────────────

fn bench_mode_b(c: &mut Criterion) {
    let mut group = c.benchmark_group("mode_b");

    let single_atom = DhadAtom {
        base: base::ALEF,
        marks: 0,
        flags: 0,
        prosody: 0x08,
        reserved: 0,
    };
    let frame_1 = build_frame(&[single_atom]);
    group.throughput(Throughput::Bytes(frame_1.len() as u64));
    group.bench_function("single_atom", |b| {
        b.iter(|| process_mode_b(criterion::black_box(&frame_1)).unwrap())
    });

    let atoms_100: Vec<DhadAtom> = (0..100u16)
        .map(|i| DhadAtom {
            base: base::ALEF + (i % 28),
            marks: 0,
            flags: 0,
            prosody: 0,
            reserved: 0,
        })
        .collect();
    let frame_100 = build_frame(&atoms_100);
    group.throughput(Throughput::Bytes(frame_100.len() as u64));
    group.bench_function("100_atoms", |b| {
        b.iter(|| process_mode_b(criterion::black_box(&frame_100)).unwrap())
    });

    group.finish();
}

// ─── Hash Benchmarks ───────────────────────────────────────────────

fn bench_hashing(c: &mut Criterion) {
    let mut group = c.benchmark_group("hashing");

    let atoms: Vec<DhadAtom> = (0..100u16)
        .map(|i| DhadAtom {
            base: 0x0001 + (i % 28),
            marks: 0,
            flags: 0,
            prosody: (i % 4) as u8,
            reserved: 0,
        })
        .collect();

    group.throughput(Throughput::Elements(atoms.len() as u64));
    group.bench_function("core_hash_100_atoms", |b| {
        b.iter(|| dhad::hash::core_hash(criterion::black_box(&atoms)))
    });
    group.bench_function("phonetic_hash_100_atoms", |b| {
        let ch = dhad::hash::core_hash(&atoms);
        b.iter(|| dhad::hash::phonetic_hash(criterion::black_box(&atoms), &ch))
    });

    group.finish();
}

// ─── Parametric: input sizes ────────────────────────────────────────

fn bench_scaling(c: &mut Criterion) {
    let mut group = c.benchmark_group("scaling");
    group.sample_size(30);

    for size in [64usize, 256, 1024, 4096, 16384] {
        let mut input = Vec::with_capacity(size);
        while input.len() < size {
            input.extend_from_slice(BISMILLAH);
            input.push(b' ');
        }
        input.truncate(size);

        group.throughput(Throughput::Bytes(size as u64));
        group.bench_with_input(BenchmarkId::new("mode_a", size), &input, |b, i| {
            b.iter(|| process_mode_a(criterion::black_box(i)))
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_mode_a,
    bench_mode_b,
    bench_hashing,
    bench_scaling
);
criterion_main!(benches);
