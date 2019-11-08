use sgx_isa::{Report, Targetinfo};

fn main() {
    let targetinfo = Targetinfo::from(Report::for_self());
    println!("{:?}", targetinfo);
}
