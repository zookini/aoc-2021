type Cache = dashmap::DashMap<isize, [usize; 2], std::hash::BuildHasherDefault<rustc_hash::FxHasher>>;

use rayon::prelude::*;

fn main() {
    let mut cache: Cache = [(0, [0, 0])].into_iter().collect();

    for section in aoc_2021::monad::SECTIONS {
        let next = Cache::default();
        
        cache.into_par_iter().for_each(|(cached, model)| {
            for i in 1..=9 {
                let vars = section(&[0, 0, 0, cached], i as isize);
                let model = [model[0] * 10 + i, model[1] * 10 + i];
                next.entry(vars[3]).and_modify(|m| *m = [m[0].min(model[0]), m[1].max(model[1])]).or_insert(model);
            }
        });

        cache = next;
    }

    let [p1, p2] = cache
        .into_iter()
        .filter_map(|(z, model)| (z == 0).then(|| model))
        .fold([usize::MAX, 0], |m1, m2| [m1[0].min(m2[0]), m1[1].max(m2[1])]);

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}
