type Cache = std::collections::HashMap<isize, usize>;

fn main() {
    let ops: Vec<_> = include_str!("../../input/24.txt").lines().map(Op::parse).collect();

    println!("Part 1: {}", solve(&ops, |a, b| a > b));
    println!("Part 2: {}", solve(&ops, |a, b| a < b));
}

fn solve(ops: &[Op], cmp: fn(a: usize, b: usize) -> bool) -> usize {
    let mut cache: Cache = [(0, 0)].into_iter().collect();

    for chunk in ops.chunks(18) {
        let mut next = Cache::default();
        
        for ((cached, model), i) in itertools::iproduct!(cache, 1..=9) {
            let mut vars = [0, 0, 0, cached];

            for op in chunk {
                op.run(&mut vars, i as isize);
            }

            let model = model * 10 + i;
            next.entry(vars[3]).and_modify(|m| if cmp(model, *m) { *m = model }).or_insert(model);
        }

        cache = next;
    }

    cache.into_iter().filter_map(|(k, v)| (k == 0).then(|| v)).reduce(|a, b| if cmp(a, b) { a } else { b }).unwrap()
}

enum Op {
    Inp(usize),
    Add(usize, Data),
    Mul(usize, Data),
    Div(usize, Data),
    Mod(usize, Data),
    Eql(usize, Data),
}

impl Op {
    fn parse(s: &str) -> Self {
        let mut ts = s.split(' ');

        match (ts.next().unwrap(), (ts.next().unwrap().as_bytes()[0] - b'w') as usize, ts.next().map(Data::parse)) {
            ("inp", a, None) => Self::Inp(a),
            ("add", a, Some(b)) => Self::Add(a, b),
            ("mul", a, Some(b)) => Self::Mul(a, b),
            ("div", a, Some(b)) => Self::Div(a, b),
            ("mod", a, Some(b)) => Self::Mod(a, b),
            ("eql", a, Some(b)) => Self::Eql(a, b),
            _ => unreachable!()
        }
    }

    fn run(&self, vars: &mut [isize; 4], input: isize) {
        match self {
            Self::Inp(i) => vars[*i] = input,
            Self::Add(a, b) => vars[*a] += b.eval(vars),
            Self::Mul(a, b) => vars[*a] *= b.eval(vars),
            Self::Div(a, b) => vars[*a] /= b.eval(vars),
            Self::Mod(a, b) => vars[*a] %= b.eval(vars),
            Self::Eql(a, b) => vars[*a] = (vars[*a] == b.eval(vars)) as isize,
        }
    }
}

enum Data {
    Var(usize),
    Num(isize)
}

impl Data {
    fn eval(&self, vars: &[isize; 4]) -> isize {
        match self {
            Self::Var(address) => vars[*address],
            Self::Num(immediate) => *immediate,
        }
    }

    fn parse(s: &str) -> Self {
        match s.parse() {
            Ok(n) => Self::Num(n),
            Err(_) => Self::Var((s.as_bytes()[0] - b'w') as usize),
        }
    }
}
