mod lib;
use lib::DomainLookupTree;

fn main() {
	let mut tree = DomainLookupTree::new(0);
	tree.insert("test.com");

	// let node = tree.traverse("test.com");

	println!("{:?}", tree);
}
