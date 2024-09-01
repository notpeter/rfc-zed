mod ietf;

fn main() {
    let rfcs = ietf::IetfRfc::from_file("data/rfcs-json-only.json");
    for rfc in rfcs {
        println!("RFC {}: {} ({})", rfc.id, rfc.title, rfc.date);
    }
}
