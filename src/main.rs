use ipquery::*;

fn main() {
    let provider = Provider::default();
    println!("{:?}", provider.query_ipv4());
}
