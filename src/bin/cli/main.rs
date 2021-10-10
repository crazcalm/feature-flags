mod cli;

fn main() {
    let matches = cli::get_app().get_matches();

    println!("{:?}", matches);
}
