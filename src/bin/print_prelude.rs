use baselang::Prelude;

fn main() {
    let lines = Prelude::new().print();
    for line in lines {
        println!("{line}\n");
    }
}
