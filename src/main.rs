mod lib;
use lib::DomainLookupTree;

fn main() {
	let mut tree = DomainLookupTree::new(0);
	tree.insert("test.com");
	tree.insert("www.test.com");
	tree.insert("123.test.com");
	tree.insert(".google.com");
	tree.insert(".test.google.com");
	tree.insert("123.test.google.com");

	// let node = tree.traverse("test.com");

	println!("{:#?}", tree);
	println!("{:#?}", tree.lookup("googlae.com"));
}
