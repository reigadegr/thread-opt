use super::common::CmdType;
use hashbrown::HashMap;
// use hashbrown::HashMap;
#[derive(Default)]
struct TrieNode {
    children: HashMap<u8, TrieNode>,
    cmd_type: Option<CmdType>,
}

pub struct Trie {
    root: TrieNode,
}

impl Trie {
    pub fn new() -> Self {
        Self {
            root: TrieNode::default(),
        }
    }

    pub fn insert(&mut self, word: &[u8], cmd_type: CmdType) {
        let mut node = &mut self.root;
        for &ch in word {
            node = node.children.entry(ch).or_insert_with(TrieNode::default);
        }
        node.cmd_type = Some(cmd_type);
    }

    pub fn search(&self, word: &[u8]) -> Option<CmdType> {
        let mut node = &self.root;
        for &ch in word {
            match node.children.get(&ch) {
                Some(next) => node = next,
                None => return None,
            }
        }
        node.cmd_type.clone()
    }
}
