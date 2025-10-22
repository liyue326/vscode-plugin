# Import Optimizer - VSCode 扩展

一个使用 Rust WASM 高性能引擎优化 JavaScript/TypeScript 导入语句的 VSCode 扩展。

## 功能特性

✨ **智能导入优化**
- 合并相同来源的导入语句
- 自动去除重复导入
- 按字母顺序排序导入
- 支持默认导入、命名导入和命名空间导入

🚀 **高性能引擎**
- 基于 Rust WASM 构建，优化速度快
- 原生性能，处理大型文件无压力

🛠 **广泛支持**
- 支持 JavaScript 和 TypeScript
- 兼容 Vuex、React、Angular 等主流框架
- 处理各种导入语法


## 使用方法

###：命令面板
1. 打开 JavaScript 或 TypeScript 文件
2. 按 `Ctrl+Shift+P` (Windows/Linux) 或 `Cmd+Shift+P` (Mac)
3. 输入 "Import: 开始优化"
4. 按 Enter 执行优化



## 优化示例

### 优化前
```javascript
import { mapState } from "vuex";
import { mapActions } from "vuex";
import request from "@/api/request.js";
import Enum from "@/data/Enum";
import utils from "@/utils/utils";
import config from "@/data/config.json";
```

### 优化后
```javascript
import { mapActions, mapState } from 'vuex';
import request from '@/api/request.js';
import Enum from '@/data/Enum';
import utils from '@/utils/utils';
import config from '@/data/config.json';
```

## 支持的特性

### 导入类型支持
- ✅ 默认导入：`import React from 'react'`
- ✅ 命名导入：`import { useState, useEffect } from 'react'`
- ✅ 命名空间导入：`import * as React from 'react'`
- ✅ 混合导入：`import React, { Component } from 'react'`

### 框架支持
- ✅ Vue & Vuex
- ✅ React
- ✅ Angular
- ✅ 所有使用 ES6 模块的库

## 配置选项



### 配置说明
- `autoOptimizeOnSave`: 保存时自动优化（默认：false）
- `sortImports`: 按字母顺序排序导入（默认：true）
- `removeDuplicates`: 移除重复导入（默认：true）
- `mergeImports`: 合并相同来源的导入（默认：true）



## 技术架构

- **前端**: VSCode Extension API
- **引擎**: Rust + WebAssembly (WASM)
- **构建工具**: wasm-pack, webpack
