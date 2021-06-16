use aya::{programs::KProbe, Bpf};
use std::convert::TryInto;

fn main() -> anyhow::Result<()> {
    let mut bpf = Bpf::load_file("kprob/target/release/kprob")?;

    let program: &mut KProbe = bpf.program_mut("tcp_connect")?.try_into()?;
    program.load()?;

    program.attach("tcp_connect", 0, None)?;

    Ok(())
}
