extern crate domain_lookup_tree;

use domain_lookup_tree::DomainLookupTree;

#[test]
fn matches_wildcard_upper_level() {
	let mut tree = DomainLookupTree::new();

	tree.insert(".test.com");

	assert_eq!(tree.lookup("123.test.com"), Some(".test.com".to_string()))
}

#[test]
fn matches_wildcard_direct() {
	let mut tree = DomainLookupTree::new();
	tree.insert(".test.com");
	assert_eq!(tree.lookup("test.com"), Some(".test.com".to_string()))
}

#[test]
fn does_not_match_noninserted() {
	let mut tree = DomainLookupTree::new();
	tree.insert(".test.com");
	assert_eq!(tree.lookup("google.com"), None)
}

#[test]
fn matches_direct() {
	let mut tree = DomainLookupTree::new();
	tree.insert("test.com");
	assert_eq!(tree.lookup("test.com"), Some("test.com".to_string()))
}

#[test]
fn matches_wildcard_n_upper_level() {
	let mut tree = DomainLookupTree::new();
	tree.insert(".test.com");

	assert_eq!(
		tree.lookup("a.b.c.123.test.com"),
		Some(".test.com".to_string())
	)
}

#[test]
fn matches_multiple_inserts_under_common_gtld() {
	let mut tree = DomainLookupTree::new();
	tree.insert(".test.com");
	tree.insert("google.com");
	tree.insert("abc.com");
	tree.insert("phineas.io");

	assert_eq!(tree.lookup("google.com"), Some("google.com".to_string()));
	assert_eq!(tree.lookup("phineas.io"), Some("phineas.io".to_string()));
	assert_eq!(tree.lookup("test.com"), Some(".test.com".to_string()))
}
