# domain-lookup-tree

## Overview

DomainLookupTree is a data structure which provides efficient domain name lookup matching with support for wildcard entries.

Requirements for this implementation:

- Given a domain name, determine if it matches an entry in the tree
- There can be an ever-growing amount of tree entries
- Entries can be absolute matches, e.g.: www.google.com
- Entries may be wildcard entries, which is denoted in the entry by providing a leading dot, e.g.: .twitter.com, .en.wikipedia.org, .hop.io
- Wildcard entries can not be embedded

## Usage

First, add `domain-lookup-tree` to your Cargo.toml:

```toml
[dependencies]
domain-lookup-tree = "0.1"
```

Now you can import and use the `domain_lookup_tree::DomainLookupTree` type:

```rs
extern crate domain_lookup_tree;
use domain_lookup_tree::DomainLookupTree;

let mut tree = DomainLookupTree::new();

// Insert some domains

tree.insert(".google.com"); // prefix with a dot to denote a wildcard entry
tree.insert("api.twitter.com");
tree.insert("phineas.io");

// Perform lookups

tree.lookup("www.google.com");
// => Some(".google.com")

tree.lookup("twitter.com");
// => None

tree.lookup("api.twitter.com");
// => Some("api.twitter.com")
```

## Implementation

To achieve this, we implement a simple tree-style structure which has a root structure that contains a HashMap of nodes. These nodes can then contain other node decendants, and also be marked as "wildcard" which means theres a rule that matches that domain level and all of its decendants.

If, when performing a lookup, the domain contains segments deeper than the wildcard match, it can continue to traverse the tree until it exhausts its lookup options. At that point, the deepest wildcard entry found would be returned, if no absolute match was found.

It's good to keep in mind that, when traversing the tree, domain names are sorted by top level to infinite n-level, or in simpler terms, in reverse. This means that if "google.com" is looked up in the tree, it would split by ".", reverse the vector, then first perform a root node lookup for "com", and so on.

### Walking down the tree - the story of a lookup

Let's say have a DomainLookupTree with an entry ".hop.io", which means that the tree looks like this:

io  
└── hop [wildcard]

A rule lookup for "api.hop.io" is requested. First, "io" is matched, but it's not a wildcard, so it's ignored. We now check the decendants of "io" for "hop" - it matches, and it's a wildcard match, so we store it within the context of the lookup. This lookup will now 100% return a match, even if it isn't absolute. We now check the decendants of "hop" for "api", though it doesn't exist, and the traversal ends. Now, we didn't have an absolute match, but we did have a wildcard match earlier on for ".hop.io", so we successfully return the result ".hop.io" from the lookup function.
