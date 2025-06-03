use std::collections::HashMap;

#[allow(dead_code)]
pub trait TreeNode<T> {
    fn id(&self) -> i64;
    fn parent_id(&self) -> i64;
    fn children(&mut self, list: Vec<T>);
    fn sort(&self) -> i32;
}

#[allow(dead_code)]
pub struct TreeUtil<T: TreeNode<T>> {
    list: Vec<T>,
    root_id: i64,
}

impl<T: TreeNode<T>> TreeUtil<T> {
    #[allow(dead_code)]
    pub fn build(list: Vec<T>, root_id: Option<i64>) -> TreeUtil<T> {
        Self {
            list,
            root_id: root_id.unwrap_or(0),
        }
    }

    #[allow(dead_code)]
    pub fn build_tree(self) -> Vec<T> {
        // 创建哈希映射以便快速查找节点
        let mut map: HashMap<i64, T> = HashMap::new();
        let mut root_nodes = Vec::new();
        // 先构建所有节点并将其添加到映射中
        for mut item in self.list.into_iter() {
            // 清空子节点列表（准备重新构建）
            item.children(Vec::new());
            map.insert(item.id(), item);
        }
        // 创建父节点 ID 与子节点列表的映射
        let mut parent_children: HashMap<i64, Vec<i64>> = HashMap::new();
        for id in map.keys() {
            let item = map.get(id).unwrap();
            parent_children
                .entry(item.parent_id())
                .or_insert_with(Vec::new)
                .push(*id);
        }
        // 处理根节点（父节点为 0 的节点）
        if let Some(root_ids) = parent_children.get(&self.root_id) {
            for &root_id in root_ids {
                let mut root = map.remove(&root_id).unwrap();
                // 构建根节点的子节点
                root.children(TreeUtil::<T>::build_children(
                    root_id,
                    &mut map,
                    &parent_children,
                ));
                root_nodes.push(root);
            }
        }
        // 按排序值对根节点排序
        root_nodes.sort_by_key(|r| r.sort());
        root_nodes
    }

    // 递归构建树形结构
    pub fn build_children(
        parent_id: i64,
        map: &mut HashMap<i64, T>,
        parent_children: &HashMap<i64, Vec<i64>>,
    ) -> Vec<T> {
        let mut children = Vec::new();
        if let Some(child_ids) = parent_children.get(&parent_id) {
            for &child_id in child_ids {
                let mut child = map.remove(&child_id).unwrap();
                // 递归添加子节点
                child.children(TreeUtil::<T>::build_children(
                    child_id,
                    map,
                    parent_children,
                ));
                children.push(child);
            }
        }
        // 按排序值排序
        children.sort_by_key(|c| c.sort());
        children
    }
}
