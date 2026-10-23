use super::*;

/// Keyed tree edits keep native node order, nesting, text, and expansion; user expansion and item
/// invocation reach the app with the node's text.
#[test]
fn tree_nodes_follow_declared_structure() -> Result<()> {
    process::run("tree", |ui| {
        nodes(ui, "+Fruit(Apple, Banana), -Vegetables")?;
        ui.invoke("insert-child")?;
        nodes(ui, "+Fruit(Apple, Cherry, Banana), -Vegetables")?;
        ui.invoke("remove-child")?;
        nodes(ui, "+Fruit(Apple, Cherry), -Vegetables")?;
        ui.invoke("reverse-children")?;
        nodes(ui, "+Fruit(Cherry, Apple), -Vegetables")?;
        ui.invoke("reverse-roots")?;
        nodes(ui, "-Vegetables, +Fruit(Cherry, Apple)")?;
        ui.invoke("rename")?;
        nodes(ui, "-Vegetables, +Fruits(Cherry, Apple)")?;

        // User expansion survives later updates; rich content replaces the node's text.
        expand(ui, "Vegetables")?;
        nodes(ui, "+Vegetables(Carrot content), +Fruits(Cherry, Apple)")?;
        ui.invoke("collapse")?;
        nodes(ui, "+Vegetables(Carrot content), -Fruits")?;
        ui.invoke("expand")?;
        nodes(ui, "+Vegetables(Carrot content), +Fruits(Cherry, Apple)")?;

        // Invocation reports the node's text, including for rich content.
        invoke(ui, "Apple")?;
        ui.name("invoked", "Invoked Apple; calls 1")?;
        invoke(ui, "Carrot content")?;
        ui.name("invoked", "Invoked Carrot; calls 2")?;

        // A removed subtree is rebuilt from its declaration.
        ui.invoke("remove-root")?;
        nodes(ui, "+Vegetables(Carrot content)")?;
        ui.invoke("restore-root")?;
        nodes(ui, "+Fruits(Cherry, Apple), +Vegetables(Carrot content)")
    })
}

const TREE_ITEM: i32 = 50024;
const LEVEL_PROPERTY: i32 = 30154;
const EXPANDED: ExpandCollapseState = 1;
const LEAF: ExpandCollapseState = 3;

/// Waits until the visible tree items match `expected`: names prefixed with `+` when expanded or
/// `-` when collapsed, children in parentheses after their parent.
fn nodes(ui: &Automation, expected: &str) -> Result<()> {
    // WinUI exposes TreeView items under its "ListControl" template part.
    let list = ui.element("ListControl")?;
    ui.wait(
        &format!("tree becomes {expected:?}"),
        || unsafe {
            let items = list.FindAll(TreeScope_Children, &ui.client.CreateTrueCondition()?)?;
            let mut tree = String::new();
            let mut depth = 1;
            for index in 0..items.Length()? {
                let item = items.GetElement(index)?;
                if item.CurrentControlType()? != TREE_ITEM {
                    continue;
                }
                let level = item.GetCurrentPropertyValue(LEVEL_PROPERTY)?;
                let level = level.Anonymous.Anonymous.Anonymous.lVal;
                if level > depth {
                    tree.push('(');
                } else {
                    tree.push_str(&")".repeat(usize::try_from(depth - level).unwrap()));
                    if !tree.is_empty() {
                        tree.push_str(", ");
                    }
                }
                depth = level;
                let state = item
                    .GetCurrentPatternAs::<IUIAutomationExpandCollapsePattern>(
                        EXPAND_COLLAPSE_PATTERN,
                    )?
                    .CurrentExpandCollapseState()?;
                tree.push_str(match state {
                    EXPANDED => "+",
                    LEAF => "",
                    _ => "-",
                });
                tree.push_str(&String::from_utf16_lossy(&item.CurrentName()?));
            }
            tree.push_str(&")".repeat(usize::try_from(depth - 1).unwrap()));
            Ok(tree)
        },
        |tree| tree == expected,
    )?;
    Ok(())
}

fn item(ui: &Automation, name: &str) -> Result<IUIAutomationElement> {
    let list = ui.element("ListControl")?;
    let item = ui.wait(
        &format!("find tree item {name:?}"),
        || unsafe {
            let items = list.FindAll(TreeScope_Children, &ui.client.CreateTrueCondition()?)?;
            for index in 0..items.Length()? {
                let item = items.GetElement(index)?;
                if item.CurrentControlType()? == TREE_ITEM && item.CurrentName()? == name {
                    return Ok(Some(item));
                }
            }
            Ok(None)
        },
        Option::is_some,
    )?;
    Ok(item.unwrap())
}

fn expand(ui: &Automation, name: &str) -> Result<()> {
    eprintln!("expand tree item {name:?}");
    unsafe {
        item(ui, name)?
            .GetCurrentPatternAs::<IUIAutomationExpandCollapsePattern>(EXPAND_COLLAPSE_PATTERN)?
            .Expand()
            .ok()?;
    }
    Ok(())
}

fn invoke(ui: &Automation, name: &str) -> Result<()> {
    eprintln!("invoke tree item {name:?}");
    unsafe {
        item(ui, name)?
            .GetCurrentPatternAs::<IUIAutomationInvokePattern>(INVOKE_PATTERN)?
            .Invoke()
            .ok()?;
    }
    Ok(())
}
