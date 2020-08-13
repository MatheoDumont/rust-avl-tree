use rand::Rng;

mod arbre;

fn main() {
    // println!("Hello, world!");
    let mut root = arbre::Avl::new();
    let mut rng = rand::thread_rng();

    for _ in 0..10 {
        root.insert(rng.gen_range(0, 100));
    }

    root.display();
}
