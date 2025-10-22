
use wasm_bindgen::prelude::*;
use std::collections::HashMap;

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
    pub fn set_rules(&mut self, sort_imports: bool, remove_duplicates: bool, merge_imports: bool) {
        self.rules.sort_imports = sort_imports;
        self.rules.remove_duplicates = remove_duplicates;
        self.rules.merge_imports = merge_imports;
    }
}

#[derive(Default)]
struct OptimizationRules {
    sort_imports: bool,
    remove_duplicates: bool,
    merge_imports: bool,
}

#[derive(Debug, Clone)]
struct ImportInfo {
    original_line: String,
    source: String,
    default_import: Option<String>,
    named_imports: Vec<NamedImport>,
    namespace_import: Option<String>,
    line_number: usize,
}

#[derive(Debug, Clone)]
struct NamedImport {
    name: String,
    alias: Option<String>,
}

#[derive(Debug, Clone)]
struct MergedImport {
    source: String,
    default_import: Option<String>,
    named_imports: Vec<NamedImport>,
    namespace_import: Option<String>,
}

impl ImportOptimizer {
    fn parse_imports(&self, code: &str) -> Vec<ImportInfo> {
        let mut imports = Vec::new();
        
        for (line_num, line) in code.lines().enumerate() {
            let trimmed = line.trim();
            
            if trimmed.starts_with("import") {
                if let Some(import_info) = self.parse_import_line(trimmed, line_num) {
                    imports.push(import_info);
                }
            }
        }
        
        imports
    }

    fn parse_import_line(&self, line: &str, line_num: usize) -> Option<ImportInfo> {
        // 必须包含 "from" 才是有效的导入语句
        if !line.contains("from") {
            return None;
        }
    
        // 使用 splitn 只分割一次
        let parts: Vec<&str> = line.splitn(2, "from").collect();
        if parts.len() != 2 {
            return None;
        }
    
        let import_part = parts[0]["import".len()..].trim();
        let source_part = parts[1].trim();
        
        println!("[RUST] 来源部分原始: '{}'", source_part);
        
        // 修复：更精确地提取来源字符串
        let source = self.extract_source_string(source_part);
        
        println!("[RUST] 提取后的来源: '{}'", source);
        
        if source.is_empty() {
            return None;
        }
    
        let (default_import, named_imports, namespace_import) = self.parse_import_clause(import_part);
    
        Some(ImportInfo {
            original_line: line.to_string(),
            source,
            default_import,
            named_imports,
            namespace_import,
            line_number: line_num,
        })
    }
    
    fn extract_source_string(&self, source_part: &str) -> String {
        // 查找第一个引号的位置
        let quote_start = source_part.find(|c| c == '\'' || c == '"');
        let quote_end = source_part.rfind(|c| c == '\'' || c == '"');
        
        if let (Some(start), Some(end)) = (quote_start, quote_end) {
            if start < end {
                // 提取引号内的内容
                let extracted = &source_part[start+1..end];
                println!("[RUST] 从引号中提取来源: '{}'", extracted);
                return extracted.to_string();
            }
        }
        
        // 如果没有找到匹配的引号，使用原来的逻辑但更安全
        let trimmed = source_part
            .trim_matches(|c: char| c == '\'' || c == '"' || c == ';' || c.is_whitespace());
        
        // 确保不会包含额外的分号
        if let Some(semi_pos) = trimmed.find(';') {
            return trimmed[..semi_pos].trim().to_string();
        }
        
        trimmed.to_string()
    }

    fn parse_import_clause(&self, import_part: &str) -> (Option<String>, Vec<NamedImport>, Option<String>) {
        let part = import_part.trim();
        
        println!("[DEBUG] 解析导入子句: '{}'", part);
        
        if part.is_empty() {
            return (None, Vec::new(), None);
        }
        
        // 首先检查命名空间导入
        if part.starts_with('*') {
            if let Some(as_pos) = part.find("as") {
                let namespace = part[as_pos + 2..].trim().to_string();
                return (None, Vec::new(), Some(namespace));
            }
        }
        
        // 然后检查纯命名导入 - 这是最重要的修复
        if part.starts_with('{') && part.ends_with('}') {
            println!("[DEBUG] 检测到纯命名导入: '{}'", part);
            let inner = part[1..part.len()-1].trim();
            
            let named_imports = if inner.is_empty() {
                Vec::new()
            } else {
                self.parse_named_imports(inner)
            };
            
            println!("[DEBUG] 解析出的命名导入数量: {}", named_imports.len());
            return (None, named_imports, None);
        }
        
        // 检查混合导入（默认导入 + 命名导入）
        if part.contains('{') && part.contains('}') {
            println!("[DEBUG] 检测到混合导入: '{}'", part);
            if let Some(brace_start) = part.find('{') {
                if let Some(brace_end) = part.rfind('}') {
                    let default_part = part[..brace_start].trim().trim_end_matches(',');
                    let named_part = &part[brace_start..=brace_end];
                    
                    println!("[DEBUG] 默认部分: '{}', 命名部分: '{}'", default_part, named_part);
                    
                    let default_import = if !default_part.is_empty() {
                        Some(default_part.to_string())
                    } else {
                        None
                    };
                    
                    let named_imports = if named_part.starts_with('{') && named_part.ends_with('}') {
                        let inner = named_part[1..named_part.len()-1].trim();
                        self.parse_named_imports(inner)
                    } else {
                        Vec::new()
                    };
                    
                    return (default_import, named_imports, None);
                }
            }
        }
        
        // 最后检查纯默认导入 - 只有在不包含 { } * 的情况下
        if !part.contains('{') && !part.contains('}') && !part.contains('*') {
            println!("[DEBUG] 检测到纯默认导入: '{}'", part);
            return (Some(part.to_string()), Vec::new(), None);
        }
        
        println!("[DEBUG] 无法识别的导入格式: '{}'", part);
        (None, Vec::new(), None)
    }
    

    fn parse_named_imports(&self, inner: &str) -> Vec<NamedImport> {
        let mut named_imports = Vec::new();
        
        if inner.trim().is_empty() {
            return named_imports;
        }
        
        for item in inner.split(',') {
            let item = item.trim();
            if item.is_empty() {
                continue;
            }
            
            if let Some(as_pos) = item.find("as") {
                let name = item[..as_pos].trim().to_string();
                let alias = item[as_pos + 2..].trim().to_string();
                if !name.is_empty() {
                    named_imports.push(NamedImport {
                        name,
                        alias: Some(alias),
                    });
                }
            } else {
                named_imports.push(NamedImport {
                    name: item.to_string(),
                    alias: None,
                });
            }
        }
        
        named_imports
    }

    fn generate_optimized_code(&self, original_code: &str, imports: &[ImportInfo]) -> String {
        if imports.is_empty() {
            return original_code.to_string();
        }

        let lines: Vec<&str> = original_code.lines().collect();
        let mut other_lines = Vec::new();

        // 分离非导入行
        let mut in_import_section = true;
        for (line_num, line) in lines.iter().enumerate() {
            let is_import_line = imports.iter().any(|imp| imp.line_number == line_num);
            
            if in_import_section {
                if !is_import_line && !line.trim().is_empty() {
                    in_import_section = false;
                    other_lines.push(*line);
                }
            } else {
                other_lines.push(*line);
            }
        }

        // 合并相同来源的导入
        let merged_imports = if self.rules.merge_imports {
            println!("[RUST] 所有导入: {:#?}", imports);  // 详细的格式化输出
            self.merge_imports(imports)
        } else {
            println!("开始sort，共有 {} 个导入", imports.len());
            // 不合并，只去重和排序
            let mut unique_imports = Vec::new();
            let mut seen_lines = std::collections::HashSet::new();
            
            for imp in imports {
                if self.rules.remove_duplicates {
                    if seen_lines.insert(imp.original_line.trim()) {
                        unique_imports.push(imp.clone());
                    }
                } else {
                    unique_imports.push(imp.clone());
                }
            }
            
            if self.rules.sort_imports {
                unique_imports.sort_by(|a, b| a.source.cmp(&b.source));
            }
            
            unique_imports.into_iter().map(|imp| MergedImport {
                source: imp.source,
                default_import: imp.default_import,
                named_imports: imp.named_imports,
                namespace_import: imp.namespace_import,
            }).collect()
        };

        // 生成优化后的导入代码
        let optimized_imports = self.format_imports(&merged_imports);

        // 重新组合代码
        let mut result = String::new();
        
        if !optimized_imports.is_empty() {
            result.push_str(&optimized_imports);
        }
        
        if !other_lines.is_empty() {
            if !result.is_empty() {
                result.push_str("\n\n");
            }
            result.push_str(&other_lines.join("\n"));
        }

        result
    }

    fn merge_imports(&self, imports: &[ImportInfo]) -> Vec<MergedImport> {
        println!("[RUST] === merge_imports 开始 ===");
        println!("[RUST] 输入 imports 数量: {}", imports.len());
        
        // 打印所有输入的导入信息
        for (i, imp) in imports.iter().enumerate() {
            println!("[RUST] 导入 {}:", i);
            println!("[RUST]   来源: '{}'", imp.source);
            println!("[RUST]   默认导入: {:?}", imp.default_import);
            println!("[RUST]   命名空间: {:?}", imp.namespace_import);
            println!("[RUST]   命名导入数量: {}", imp.named_imports.len());
            for named in &imp.named_imports {
                println!("[RUST]     - {} (别名: {:?})", named.name, named.alias);
            }
            println!("[RUST]   原始行: '{}'", imp.original_line);
        }
        
        let mut import_map: HashMap<String, MergedImport> = HashMap::new();
        
        for imp in imports {
            println!("[RUST] 处理导入: 来源='{}'", imp.source);
            
            let entry = import_map.entry(imp.source.clone()).or_insert(MergedImport {
                source: imp.source.clone(),
                default_import: None,
                named_imports: Vec::new(),
                namespace_import: None,
            });
            
            println!("[RUST]   当前合并状态 - 默认导入: {:?}, 命名导入数量: {}", 
                     entry.default_import, entry.named_imports.len());
            
            // 合并默认导入
            if let Some(ref default) = imp.default_import {
                if entry.default_import.is_none() {
                    entry.default_import = Some(default.clone());
                    println!("[RUST]   添加默认导入: {}", default);
                }
            }
            
            // 合并命名空间导入
            if let Some(ref namespace) = imp.namespace_import {
                if entry.namespace_import.is_none() {
                    entry.namespace_import = Some(namespace.clone());
                    println!("[RUST]   添加命名空间导入: {}", namespace);
                }
            }
            
            // 合并命名导入
            for named in &imp.named_imports {
                let exists = entry.named_imports.iter().any(|existing| {
                    existing.name == named.name
                });
                
                if !exists {
                    entry.named_imports.push(named.clone());
                    println!("[RUST]   添加命名导入: {} (别名: {:?})", named.name, named.alias);
                } else {
                    println!("[RUST]   跳过重复命名导入: {}", named.name);
                }
            }
            
            println!("[RUST]   合并后状态 - 默认导入: {:?}, 命名导入数量: {}", 
                     entry.default_import, entry.named_imports.len());
        }
        
        println!("[RUST] 合并完成，共有 {} 个不同的来源", import_map.len());
        
        let mut merged: Vec<MergedImport> = import_map.into_values().collect();
        
        // 打印合并结果
        for (i, imp) in merged.iter().enumerate() {
            println!("[RUST] 合并结果 {}:", i);
            println!("[RUST]   来源: '{}'", imp.source);
            println!("[RUST]   默认导入: {:?}", imp.default_import);
            println!("[RUST]   命名空间: {:?}", imp.namespace_import);
            println!("[RUST]   命名导入数量: {}", imp.named_imports.len());
            for named in &imp.named_imports {
                println!("[RUST]     - {} (别名: {:?})", named.name, named.alias);
            }
        }
        
        if self.rules.sort_imports {
            println!("[RUST] 开始排序导入");
            merged.sort_by(|a, b| a.source.cmp(&b.source));
            
            // 对每个导入的命名导入也排序
            for imp in &mut merged {
                imp.named_imports.sort_by(|a, b| a.name.cmp(&b.name));
            }
            
            println!("[RUST] 排序完成");
        }
        
        println!("[RUST] === merge_imports 结束 ===");
        merged
    }

    fn format_imports(&self, imports: &[MergedImport]) -> String {
        let mut result = Vec::new();
        
        for imp in imports {
            let import_clause = self.format_import_clause(imp);
            if !import_clause.is_empty() {
                result.push(format!("import {} from '{}';", import_clause, imp.source));
            }
        }
        
        result.join("\n")
    }

    fn format_import_clause(&self, imp: &MergedImport) -> String {
        let mut parts = Vec::new();
        
        // 默认导入
        if let Some(ref default) = imp.default_import {
            parts.push(default.clone());
        }
        
        // 命名空间导入
        if let Some(ref namespace) = imp.namespace_import {
            parts.push(format!("* as {}", namespace));
        }
        
        // 命名导入
        if !imp.named_imports.is_empty() {
            let named_parts: Vec<String> = imp.named_imports.iter().map(|named| {
                if let Some(ref alias) = named.alias {
                    format!("{} as {}", named.name, alias)
                } else {
                    named.name.clone()
                }
            }).collect();
            
            let named_str = if named_parts.len() == 1 {
                named_parts[0].clone()
            } else {
                format!("{{ {} }}", named_parts.join(", "))
            };
            
            parts.push(named_str);
        }
        
        parts.join(", ")
    }
}
#[wasm_bindgen]
pub fn run_test_case() -> String {
    let mut optimizer = ImportOptimizer::new();
    optimizer.set_rules(true, true, true);
    
    let test_input = r#"import { mapState } from "vuex";
    import { mapActions } from "vuex";
    import request from "@/api/request.js";
    import Enum from "@/data/Enum";
    import utils from "@/utils/utils";
    import config from "@/data/config.json";"#;
    
    let result = optimizer.optimize(test_input);
    
    format!("测试输入:\n{}\n\n优化结果:\n{}", test_input, result)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_import_merging() {
        let mut optimizer = ImportOptimizer::new();
        optimizer.set_rules(true, true, true);
        
        let test_input = r#"import { mapState } from "vuex";
        import { mapActions } from "vuex";
        import request from "@/api/request.js";
        import Enum from "@/data/Enum";
        import utils from "@/utils/utils";
        import config from "@/data/config.json";"#;
        
        let result = optimizer.optimize(test_input);
        
        println!("=== 基础导入合并测试 ===");
        println!("输入:\n{}", test_input);
        println!("\n输出:\n{}", result);
        
        // 验证合并
        assert!(result.contains("mapActions, mapState"), "应该合并 vuex 的命名导入");
        
        // // 检查不同默认导入是否保留
        assert!(result.contains("utils from"), "应该保留 utils");
        // assert!(result.contains("utils2 from"), "应该保留 utils2");
    }
}