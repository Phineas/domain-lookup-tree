use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::rc::Weak;

/// RuleTree is a data structure which provides efficient domain name lookup matching with support
/// for wildcard entries.
///
/// Requirements for this implementation:
/// - Given a domain name, determine if it matches an entry in the tree
/// - There can be an ever-growing amount of tree entries
/// - Entries can be absolute matches, e.g.: www.google.com
/// - Entries may be wildcard entries, which is denoted in the entry by providing a leading dot,
///   e.g.: .twitter.com, .en.wikipedia.org, .giggl.app
/// - Wilcard entries can not be embedded
///
/// To achieve this, we implement a simple tree-style structure which has a root structure that
/// contains a vector of nodes. These nodes can then contain other node decendants, and also be
/// marked as "wildcard" which means theres a rule that matches that domain level and all of its
/// decendants.
///
/// If, when performing a lookup, the domain contains segments deeper than the wildcard
/// match, it can continue to traverse the tree until it exhausts its lookup options. At that
/// point, the deepest wildcard entry found would be returned, if no absolute match was found.
///
/// It's good to keep in mind that, when traversing the tree, domain names are sorted by top level
/// to infinite n-level, or in simpler terms, in reverse. This means that if "google.com" is looked
/// up in the tree, it would split by ".", reverse the vector, then first perform a root node
/// lookup for "com", and so on.
///
/// Walking down the tree - the story of a lookup:
/// Let's say have a RuleTree with an entry ".giggl.app" which means that the tree looks like this:
///
/// app
/// └── giggl [wildcard]
///
/// A rule lookup for "canary.giggl.app" is requested. First, "app" is matched, but it's not a
/// wildcard, so it's ignored. We now check the decendants of "app" for "giggl" - it matches, and
/// it's a wildcard match, so we store it within the context of the lookup. This lookup will now
/// 100% return a match, even if it isn't absolute. Anyway, we now check the decendants of "giggl"
/// for "canary", though it doesn't exist, and the traversal ends. Now, we didn't have an absolute
/// match, but we did have a wildcard match earlier on for ".giggl.app", so we successfully return
/// the result ".giggl.app" from the lookup function.
///
///

type NodeList = HashMap<String, Node>;

#[derive(Debug)]
pub struct DomainLookupTree {
    nodes: Box<NodeList>,
    minimum_level: usize,
}

#[derive(Debug)]
pub struct Node {
    wildcard: bool,
    nodes: Box<NodeList>,
    parent: Option<Weak<Node>>,
    data: String,
}

// The comments in the implementation are written in relation to the "story of a lookup", above

impl DomainLookupTree {
    pub fn new(minimum_level: usize) -> DomainLookupTree {
        DomainLookupTree {
            nodes: Box::new(NodeList::new()),
            minimum_level,
        }
    }

    // For inserting an item into the tree, we need to make sure that t
    pub fn insert(&mut self, domain: &str) {
        let is_wildcard = domain.starts_with(".");
        let segments = domain_to_rseg(domain);
        let segments_len = segments.len();

        let head: &mut Box<NodeList> = &mut self.nodes.borrow_mut();

        for (i, segment) in segments.iter().enumerate() {
            let node = head.get(segment.to_owned());

            match node {
                None => {
                    &head.insert(
                        segment.to_string(),
                        Node {
                            wildcard: i == segments_len && is_wildcard,
                            nodes: Box::new(NodeList::new()),
                            parent: None,
                            data: segment.to_string(),
                        },
                    );
                }
                Some(&n) => {
                    *head = n.nodes;
                }
            }
        }

        // deepest_match.insert(de)
        // match deepest_node {
        // 	(None, _) => {
        // 		// We've hit the root node
        // 		let (tree, seg) = domain_to_node_list(domain, 1);
        // 		self.nodes.insert(seg.to_string(), tree);
        // 	}
        // 	Some(&mut node, depth) => {
        // 		let (tree, seg) = domain_to_node_list(domain, 1);
        // 		node.nodes.insert(seg.to_string(), tree);
        // 		node.wildcard = is_wildcard;
        // 	}
        // }
    }

    pub fn lookup(&self, domain: &str) -> Option<String> {
        match self.traverse(domain) {
            None => None,
            Some(node) => return Some(node.data.to_owned()),
        }
    }

    pub fn traverse(&self, domain: &str) -> Option<&Node> {
        let segments = domain_to_rseg(domain);
        let mut matched: Option<&Node> = None;
        // We start the traversal at the root
        let mut head: &NodeList = &self.nodes;

        let mut depth: usize = 0;
        // We traverse the tree in level-reverse order
        'sl: for (i, segment) in segments.iter().enumerate() {
            depth = depth + 1;
            // Now we look up the children of the latest matched node
            // If this is the first iteration, then it's the root NodeList
            let node = &head.get(segment.to_owned());
            match node {
                None => {
                    // We have exhausted the traversal
                    return matched;
                }
                Some(child) => {
                    // We've matched a decendant, so we set the traversal root for the next iteration
                    //
                    // In our example, if this was the first iteration of 'sl, then child would be the
                    // node for the "app" segment, and it's children contain a hashmap including the
                    // "giggl" key
                    //
                    // If this was the second iteration of our example, then "child" would be the node for
                    // the "giggl" segment, and wildcard would be true.
                    head = &child.nodes;
                    if child.wildcard {
                        // Current node is wildcard, so we now 100% have a value to return
                        matched = Some(child);
                    }

                    if child.nodes.len() == 0 {
                        // We have exhausted the traversal. If the traversal depth is equal to the segment
                        // length, then we've found an absolute match!
                        if i == segments.len() {
                            matched = Some(child);
                        }
                        // Break the loop.
                        break 'sl;
                    }
                }
            }
        }
        return matched;
    }
}

fn domain_to_rseg(domain: &str) -> Vec<&str> {
    let mut segments: Vec<&str> = domain.split(".").collect::<Vec<&str>>();
    segments.reverse();

    segments
}

// fn build_string_from_node(node: Node) -> String {
// 	let mut str = "";
// 	if node.wildcard {
// 		str = ".";
// 	}

// 	let mut segments = Vec::new();
// 	loop {
// 		match node.parent {
// 			None => {
// 				// we've hit the root!
// 				break;
// 			}
// 			Some(parent) => {
// 				seg
// 			}
// 		}
// 	}

// 	str.to_string()
// }

// This function converts a domain into a nested tree structure for insertion into an existing
// DomainLookupTree. strip_level allows for the creation of nested trees by slicing out the
// portion of the domain that already exists in the tree structure of the caller
// fn domain_to_node_list(domain: &str, strip_level: usize) -> (Node, &str) {
// 	// Example: www.google.com
// 	// -> Vec<str> [www, google, com]
// 	let mut segments: Vec<&str> = domain.split(".").collect::<Vec<&str>>();
// 	// -> Vec<str> [com, google, www]
// 	segments.reverse();
// 	// Example if strip_level was set to 1:
// 	// -> [google, www]
// 	let sliced = &segments[strip_level..];
// 	// -> com
// 	let highest_level = &segments[sliced.len()..strip_level][0];

// 	let mut root = Node {
// 		wildcard: false,
// 		nodes: NodeList::new(),
// 	};
// 	let mut head = root.nodes;
// 	for segment in sliced {
// 		let n = Node {
// 			wildcard: false,
// 			nodes: NodeList::new(),
// 		};
// 		head.insert(segment.to_string(), n);
// 		head = n.nodes;
// 	}

// 	(root, highest_level)
// }
