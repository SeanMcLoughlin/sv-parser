mod helper_fns;
mod node;
use helper_fns::pprint_tree;
use node::Node;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
struct SymbolTableEntry {
    row: usize,
    col: usize,
}

type SymbolTable = HashMap<String, SymbolTableEntry>;

#[derive(Debug, Default)]
struct SymbolTableNode {
    pub table: SymbolTable,
    pub parent: Option<Box<SymbolTableNode>>,
    pub children: Vec<Box<SymbolTableNode>>,
}

impl SymbolTableNode {
    pub fn new(parent: Option<Box<SymbolTableNode>>, table: SymbolTable) -> Self {
        SymbolTableNode {
            table,
            parent,
            children: Vec::new(),
        }
    }

    fn print(&self) -> fmt::Result {

        let result: String = 
        print(self, "".to_string(), true);
    }

    fn print_helper(node: Box<SymbolTableNode>, prefix: string, last: bool) {
        let prefix_current = if last { "`- " } else { "|- " };

        println!("{}{}{}", prefix, prefix_current, node);

        let prefix_child = if last { "   " } else { "|  " };
        let prefix = prefix + prefix_child;

        if !node.get_children().is_empty() {
            let last_child = node.get_children().len() - 1;

            for (i, child) in node.get_children().iter().enumerate() {
                print(&child, prefix.to_string(), i == last_child);
            }
        }
    }
}

impl fmt::Display for SymbolTableNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

    }
}

impl Node<Box<SymbolTableNode>> for SymbolTableNode {
    fn get_children(&self) -> Vec<Box<SymbolTableNode>> {
        return self.children;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn new_symbol_table() {
        let mut root_table = SymbolTable::new();
        root_table.insert("my_function".to_string(), SymbolTableEntry::default());
        let root = SymbolTableNode::new(None, root_table.clone());
        assert_eq!(root.table["my_function"], SymbolTableEntry::default());
    }

    #[test]
    fn display_symbol_table() {}
}
