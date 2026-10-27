use std::collections::BTreeMap;
use std::path::Path;

#[derive(Default)]
enum TreeNode {
    #[default]
    File,
    Dir(BTreeMap<String, TreeNode>),
}

impl TreeNode {
    fn insert(&mut self, parts: &[&str]) {
        if parts.is_empty() {
            return;
        }
        
        if !matches!(self, TreeNode::Dir(_)) {
            *self = TreeNode::Dir(BTreeMap::new());
        }
        
        if let TreeNode::Dir(children) = self {
            if parts.len() == 1 {
                children.entry(parts[0].to_string()).or_default();
            } else {
                children
                    .entry(parts[0].to_string())
                    .or_insert_with(|| TreeNode::Dir(BTreeMap::new()))
                    .insert(&parts[1..]);
            }
        }
    }
}

/// Generates a correct ASCII file tree
pub fn generate_tree(_root: &Path, files: &[String]) -> String {
    let mut tree = String::new();
    tree.push_str("# 📁 Project Structure\n");
    tree.push_str("#\n");

    let mut root_node = TreeNode::Dir(BTreeMap::new());

    for file in files {
        let parts: Vec<&str> = file.split('/').collect();
        root_node.insert(&parts);
    }

    fn render_tree(node: &TreeNode, prefix: &str, output: &mut String) {
        if let TreeNode::Dir(children) = node {
            let entries: Vec<(&String, &TreeNode)> = children.iter().collect();
            
            for (i, (name, child)) in entries.iter().enumerate() {
                let is_last = i == entries.len() - 1;
                let connector = if is_last { "└── " } else { "├── " };
                
                output.push_str(&format!("# {}{}{}\n", prefix, connector, name));
                
                let new_prefix = if is_last {
                    format!("{}    ", prefix)
                } else {
                    format!("{}│   ", prefix)
                };
                
                render_tree(child, &new_prefix, output);
            }
        }
    }

    if let TreeNode::Dir(children) = &root_node {
        let entries: Vec<(&String, &TreeNode)> = children.iter().collect();
        for (i, (name, child)) in entries.iter().enumerate() {
            let is_last = i == entries.len() - 1;
            let connector = if is_last { "└── " } else { "├── " };
            
            tree.push_str(&format!("# {}{}\n", connector, name));
            
            let new_prefix = if is_last { "    " } else { "│   " };
            render_tree(child, &new_prefix, &mut tree);
        }
    }

    tree.push_str("#\n\n");
    tree
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_tree_structure() {
        let files = vec![
            "Cargo.toml".to_string(),
            "src/main.rs".to_string(),
            "src/utils/helper.rs".to_string(),
        ];
        let tree = generate_tree(Path::new("."), &files);
        
        assert!(tree.contains("Cargo.toml"));
        assert!(tree.contains("src"));
        assert!(tree.contains("main.rs"));
        assert!(tree.contains("helper.rs"));
        
        // Check tree structure characters
        assert!(tree.contains("├──"));
        assert!(tree.contains("└──"));
        
        // Ensure helper.rs is nested deeper than main.rs
        let main_pos = tree.find("main.rs").unwrap();
        let helper_pos = tree.find("helper.rs").unwrap();
        assert!(helper_pos > main_pos);
    }
}
