use std::collections::HashMap;
use std::rc::Weak;

/// DomainLookupTree is a data structure which provides efficient domain name lookup matching with
/// support for wildcard entries.
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
/// If, when performing a lookup, the search domain contains segments deeper than the wildcard
/// match, it can continue to traverse the tree until it exhausts its lookup options. At that
/// point, the deepest wildcard entry found would be returned, if no absolute match was found.
///
/// It's good to keep in mind that, when traversing the tree, domain names are sorted by top level
/// to infinite n-level, or in simpler terms, in reverse. This means that if "google.com" is looked
/// up in the tree, it would split by ".", reverse the vector, then first perform a root node
/// lookup for "com", and so on.
///
/// Walking down the tree - the story of a lookup:
/// Let's say have a DomainLookupTree with an entry ".giggl.app" which means that the tree looks
/// like this:
///
/// app
/// └── giggl [wildcard]
///
/// A domain lookup for "canary.giggl.app" is requested. First, "app" is matched, but it's not a
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
    nodes: NodeList,
    minimum_level: usize,
}

#[derive(Debug)]
pub struct Node {
    wildcard: bool,
    nodes: NodeList,
    #[allow(unused)]
    parent: Option<Weak<Self>>,
    data: String,
}

impl Node {
    fn new(wildcard: bool, data: &str) -> Self {
        Self {
            wildcard,
            nodes: Default::default(),
            parent: None,
            data: data.to_owned(),
        }
    }
}

impl DomainLookupTree {
    pub fn new(minimum_level: usize) -> DomainLookupTree {
        DomainLookupTree {
            nodes: Default::default(),
            minimum_level,
        }
    }

    // For inserting an item into the tree, we need to make sure that t
    pub fn insert(&mut self, domain: &str) {
        let is_wildcard = domain.starts_with(".");
        let segments = domain_to_rseg(domain);
        let n_segments = segments.len();

        let mut head = &mut self.nodes;
        // let mut fqdn = String::new();
        for (i, segment) in segments.iter().copied().enumerate() {
            let node = head
                .entry(segment.to_owned())
                .or_insert_with(|| Node::new(i == n_segments - 2 && is_wildcard, segment));

            if i == n_segments - 2 && is_wildcard {
                return;
            }

            head = &mut node.nodes;
        }
    }

    pub fn lookup(&self, domain: &str) -> Option<String> {
        match self.traverse(domain) {
            None => None,
            Some((fqdn, node)) => {
                return Some(format!("{}{}", if node.wildcard { "." } else { "" }, fqdn))
            }
        }
    }

    pub fn traverse(&self, domain: &str) -> Option<(String, &Node)> {
        let segments = domain_to_rseg(domain);
        let mut wildcard_match = None;
        // We start the traversal at the root
        let mut head: &NodeList = &self.nodes;

        let mut fqdn = String::new();

        // We traverse the tree in level-reverse order
        for (i, segment) in segments.iter().copied().enumerate() {
            // Now we look up the children of the latest matched node
            // If this is the first iteration, then it's the root NodeList
            if let Some(child) = head.get(segment) {
                fqdn = format!(
                    "{}{}{}",
                    segment.to_owned(),
                    if i == 0 { "" } else { "." },
                    fqdn
                );
                head = &child.nodes;
                // We have exhausted the traversal. If the traversal depth is equal to the segment
                // length, then we've found an absolute match!
                if i == segments.len() - 1 {
                    return Some((fqdn, child));
                } else if child.wildcard {
                    // Current node is wildcard, so we now 100% have a value to return
                    wildcard_match = Some(child);
                }
            } else {
                // We have exhausted the traversal.
                break;
            }
        }

        if let Some(m) = wildcard_match {
            Some((fqdn, m))
        } else {
            None
        }
    }
}

fn domain_to_rseg(domain: &str) -> Vec<&str> {
    domain.rsplit(".").collect::<Vec<&str>>()
}
