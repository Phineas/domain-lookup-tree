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

#[test]
fn remove_inserted_domain() {
	let mut tree = DomainLookupTree::new();
	tree.insert("example.com");
	assert!(tree.lookup("example.com").is_some());
	tree.remove("example.com");
	assert_eq!(tree.lookup("example.com"), None);
}

#[test]
fn remove_nonexistent_is_noop() {
	let mut tree = DomainLookupTree::new();
	tree.insert("example.com");
	tree.remove("nonexistent.com");
	assert!(tree.lookup("example.com").is_some());
}

#[test]
fn remove_does_not_affect_siblings() {
	let mut tree = DomainLookupTree::new();
	tree.insert("a.example.com");
	tree.insert("b.example.com");
	tree.remove("a.example.com");
	assert_eq!(tree.lookup("a.example.com"), None);
	assert!(tree.lookup("b.example.com").is_some());
}

#[test]
fn remove_wildcard_entry() {
	let mut tree = DomainLookupTree::new();
	tree.insert(".example.com");
	assert!(tree.lookup("sub.example.com").is_some());
	tree.remove(".example.com");
	assert_eq!(tree.lookup("sub.example.com"), None);
}

#[test]
fn wildcard_set_on_existing_node() {
	let mut tree = DomainLookupTree::new();
	tree.insert("foo.up.railway.app");
	tree.insert(".up.railway.app");
	assert_eq!(tree.lookup("bar.up.railway.app"), Some(".up.railway.app".to_string()));
}

#[test]
fn insert_transparent_skips_covered_domain() {
	let mut tree = DomainLookupTree::new();
	tree.insert(".up.railway.app");
	tree.insert_transparent("foo.up.railway.app");
	assert_eq!(tree.lookup("foo.up.railway.app"), Some(".up.railway.app".to_string()));
}

#[test]
fn insert_still_allows_specific_under_wildcard() {
	let mut tree = DomainLookupTree::new();
	tree.insert(".up.railway.app");
	tree.insert("foo.up.railway.app");
	assert_eq!(tree.lookup("foo.up.railway.app"), Some("foo.up.railway.app".to_string()));
}
