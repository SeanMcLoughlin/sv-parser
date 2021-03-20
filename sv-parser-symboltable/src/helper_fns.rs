use crate::node::Node;
use std::fmt::Display;

pub fn pprint_tree<T, U>(node: &T)
where
    T: Display + Node<U>,
{
    fn pprint_tree<T: Display + Node<U>>(node: &T, prefix: String, last: bool) {
        let prefix_current = if last { "`- " } else { "|- " };

        println!("{}{}{}", prefix, prefix_current, node);

        let prefix_child = if last { "   " } else { "|  " };
        let prefix = prefix + prefix_child;

        if !node.get_children().is_empty() {
            let last_child = node.get_children().len() - 1;

            for (i, child) in node.get_children().iter().enumerate() {
                pprint_tree(&child, prefix.to_string(), i == last_child);
            }
        }
    }

    pprint_tree(node, "".to_string(), true);
}
