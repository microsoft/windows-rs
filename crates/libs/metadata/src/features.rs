use super::*;

pub fn collect_trees<'a>(excluded_namespaces: &[&str], tree: &'a TypeTree) -> Vec<&'a TypeTree> {
    let mut trees = Vec::new();
    collect_trees_internal(excluded_namespaces, tree, &mut trees);
    trees
}

fn collect_trees_internal<'a>(excluded_namespaces: &[&str], tree: &'a TypeTree, trees: &mut Vec<&'a TypeTree>) {
    if excluded_namespaces.iter().any(|&x| x == tree.namespace) {
        return;
    }

    trees.push(tree);
    for tree in tree.namespaces.values() {
        collect_trees_internal(excluded_namespaces, tree, trees);
    }
}

pub fn features(trees: &Vec<&TypeTree>, root_namespace: &str) -> Vec<String> {
    let child_namespace_position = root_namespace.len() + 1;

    // Skip the root Windows tree while writing features
    // TODO: don't include parent features automatically
    trees.iter().skip(1).map(|&tree| tree.namespace[child_namespace_position..].replace('.', "_")).collect()
}
