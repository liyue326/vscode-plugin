use wasm_bindgen::prelude::*;
use std::collections::HashSet;

#[wasm_bindgen]
pub struct ImportOptimizer {
    rules: OptimizationRules,
}

#[wasm_bindgen]
impl ImportOptimizer {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            rules: OptimizationRules::default(),
        }
    }

    #[wasm_bindgen]
    pub fn optimize(&self, code: &str) -> String {
        let imports = self.parse_imports(code);
        self.generate_optimized_code(code, &imports)
    }

    #[wasm_bindgen]
    pub fn set_rules(&mut self, sort_imports: bool, remove_duplicates: bool) {
        self.rules.sort_imports = sort_imports;
        self.rules.remove_duplicates = remove_duplicates;
    }
}

#[derive(Default)]
struct OptimizationRules {
    sort_imports: bool,
    remove_duplicates: bool,
}

#[derive(Debug)]
struct ImportInfo {
    line: String,
    source: String,
    line_number: usize,
}

impl ImportOptimizer {
    fn parse_imports(&self, code: &str) -> Vec<ImportInfo> {
        let mut imports = Vec::new();
        
        for (line_num, line) in code.lines().enumerate() {
            let trimmed = line.trim();
            
            if trimmed.starts_with("import") {
                if let Some(source) = self.extract_import_source(trimmed) {
                    imports.push(ImportInfo {
                        line: line.to_string(),
                        source,
                        line_number: line_num,
                    });
                }
            }
        }
        
        imports
    }

    fn extract_import_source(&self, line: &str) -> Option<String> {
        // 简单的导入源提取
        if let Some(from_pos) = line.find("from") {
            let source_part = &line[from_pos + 4..].trim(); // "from".len() = 4
            let source = source_part
                .trim_matches(|c: char| c == '\'' || c == '"' || c == ';' || c.is_whitespace())
                .to_string();
            
            if !source.is_empty() {
                return Some(source);
            }
        }
        None
    }

    fn generate_optimized_code(&self, original_code: &str, imports: &[ImportInfo]) -> String {
        if imports.is_empty() {
            return original_code.to_string();
        }

        let lines: Vec<&str> = original_code.lines().collect();
        let mut optimized_imports = Vec::new();
        let mut other_lines = Vec::new();

        // 处理导入
        let mut import_lines: Vec<&ImportInfo> = imports.iter().collect();
        
        // 应用规则
        if self.rules.remove_duplicates {
            let mut seen = HashSet::new();
            import_lines.retain(|imp| seen.insert(imp.line.trim()));
        }
        
        if self.rules.sort_imports {
            import_lines.sort_by(|a, b| a.source.cmp(&b.source));
        }

        for imp in import_lines {
            optimized_imports.push(imp.line.as_str());
        }

        // 分离非导入行
        let mut in_import_section = true;
        for (line_num, line) in lines.iter().enumerate() {
            let trimmed = line.trim();
            
            if in_import_section {
                if !trimmed.starts_with("import") && !trimmed.is_empty() {
                    in_import_section = false;
                    other_lines.push(*line);
                } else if !trimmed.starts_with("import") {
                    // 空行在导入区域
                }
            } else {
                other_lines.push(*line);
            }
        }

        // 重新组合代码
        let mut result = String::new();
        
        // 添加优化后的导入
        if !optimized_imports.is_empty() {
            result.push_str(&optimized_imports.join("\n"));
        }
        
        // 添加其他代码
        if !other_lines.is_empty() {
            if !result.is_empty() {
                result.push_str("\n\n");
            }
            result.push_str(&other_lines.join("\n"));
        }

        result
    }
}

// 测试函数
#[wasm_bindgen]
pub fn test_optimizer() -> String {
    let optimizer = ImportOptimizer::new();
    let test_code = r#"import React from 'react';
import { useState } from 'react';
import { Button } from './components';
import React from 'react';

const App = () => {
    return <div>Hello</div>;
};"#;

    optimizer.optimize(test_code)
}

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! Import Optimizer is ready.", name)
}