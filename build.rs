use std::io::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    b24()?;
    Ok(())
}

fn b24() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=src/bin/24c.rs");
    
    let instructions: Vec<_> = include_str!("input/24.txt").lines().collect();
    let mut f = std::fs::File::create("src/monad.rs")?;

    for (i, section) in instructions.chunks(18).enumerate() {
        writeln!(f, "#[allow(unused_assignments)]")?;
        writeln!(f, "fn section{}(vars: &[isize; 4], input: isize) -> [isize; 4] {{", i + 1)?;
        writeln!(f, "    let [mut w, mut x, mut y, mut z] = vars;\n")?;

        for line in section {
            f.write_all(parse(line).as_bytes())?;
        }

        f.write_all(b"\n\t[w, x, y, z]\n")?;
        f.write_all(b"}\n\n")?;
    }

    writeln!(f, "pub const SECTIONS: [fn(&[isize; 4], isize) -> [isize; 4]; {}] = [", instructions.len() / 18)?;

    for i in 1..=instructions.len() / 18 {
        writeln!(f, "\tsection{},", i)?;
    }

    f.write_all(b"];\n")?;

    Ok(())
}

fn parse(s: &str) -> String {
    let mut ts = s.split(' ');

    match (ts.next().unwrap(), ts.next().unwrap(), ts.next()) {
        ("inp", a, None) => format!("\t{} = input;\n", a),
        ("eql", a, Some(b)) => format!("\t{0} = ({0} == {1}) as isize;\n", a, b),
        (op, a, Some(b)) => format!("\t{0} = {0} {2} {1};\n", a, b, match op {
            "add" => "+",
            "mul" => "*",
            "div" => "/",
            "mod" => "%",
            _ => unreachable!()
        }),
        _ => unreachable!()
    }
}
