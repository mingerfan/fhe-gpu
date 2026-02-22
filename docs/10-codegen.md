# 代码生成路径规划

## 当前：IR → Rust Evaluator

`fhe-compiler::lowering` 将 IR 节点翻译为 `CkksEvaluator` 调用。

## 未来：IR → C++/CUDA

TODO: 规划 CUDA kernel 生成路径

## TODO

- [ ] 完成 `Lowering::execute`
- [ ] 规划 GPU 后端架构
