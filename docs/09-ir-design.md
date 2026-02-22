# IR 设计

## 设计原则

- 方案无关（不导入 `ckks` crate）
- DAG 表示（petgraph::DiGraph）
- 节点只携带标量元数据（level, scale, step）

## 节点类型

见 `crates/fhe-ir/src/node.rs`

## 优化 Pass

1. `LevelAssignPass` — 分配 FHE level
2. `RescaleInsertPass` — 自动插入 rescale 节点
3. `DeadCodePass` — 死代码消除

## TODO

- [ ] 实现 `LevelAssignPass`
- [ ] 实现 `RescaleInsertPass`
- [ ] 实现 `DeadCodePass`
- [ ] 实现 `Lowering::execute`
