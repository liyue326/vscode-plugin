const vscode = require('vscode');
const path = require('path');
const fs = require('fs');

class ImportOptimizer {
    constructor() {
        this.wasmModule = null;
        this.loadWasm();
    }

    async loadWasm() {
        try {
            const wasmPath = path.join(__dirname, 'pkg', 'import_optimizer.js');
            if (fs.existsSync(wasmPath)) {
                this.wasmModule = require(wasmPath);
                console.log('✅ WASM 优化器加载成功');
                
                // 测试 WASM
                if (this.wasmModule.greet) {
                    console.log(this.wasmModule.greet('VSCode'));
                }
            }
        } catch (error) {
            console.log('⚠️ WASM 加载失败，使用 JavaScript 备用方案');
        }
    }

    async optimizeCode(code) {
        // 优先使用 WASM
        if (this.wasmModule && this.wasmModule.ImportOptimizer) {
            try {
                const optimizer = new this.wasmModule.ImportOptimizer();
                // 设置优化规则：排序 + 去重
                optimizer.set_rules(true, true);
                return optimizer.optimize(code);
            } catch (error) {
                console.log('WASM 优化失败:', error);
            }
        }
        
        // 备用 JavaScript 实现
        return this.javaScriptOptimize(code);
    }

    javaScriptOptimize(code) {
        const lines = code.split('\n');
        const importLines = [];
        const otherLines = [];
        
        let inImportSection = true;
        
        for (const line of lines) {
            const trimmed = line.trim();
            
            if (inImportSection && (trimmed.startsWith('import') || trimmed === '')) {
                if (trimmed.startsWith('import')) {
                    importLines.push(line);
                }
            } else {
                inImportSection = false;
                otherLines.push(line);
            }
        }
        
        // 去重和排序
        const uniqueImports = [...new Set(importLines)].sort();
        
        let result = uniqueImports.join('\n');
        if (result && otherLines.length > 0) {
            result += '\n\n';
        }
        result += otherLines.join('\n');
        
        return result;
    }
}

// 全局优化器实例
let optimizer;

function activate(context) {
    console.log('🚀 Import Optimizer 扩展激活');
    
    optimizer = new ImportOptimizer();

    // 注册优化命令
    const optimizeCommand = vscode.commands.registerCommand('import-optimizer.optimize', async () => {
        const editor = vscode.window.activeTextEditor;
        if (!editor) {
            vscode.window.showWarningMessage('请先打开一个文件');
            return;
        }

        const document = editor.document;
        const code = document.getText();
        
        try {
            await vscode.window.withProgress({
                location: vscode.ProgressLocation.Notification,
                title: "优化导入语句...",
                cancellable: false
            }, async (progress) => {
                progress.report({ increment: 0 });
                
                const optimizedCode = await optimizer.optimizeCode(code);
                
                progress.report({ increment: 100 });

                if (optimizedCode !== code) {
                    const edit = new vscode.WorkspaceEdit();
                    const fullRange = new vscode.Range(
                        document.positionAt(0),
                        document.positionAt(code.length)
                    );
                    
                    edit.replace(document.uri, fullRange, optimizedCode);
                    const success = await vscode.workspace.applyEdit(edit);
                    
                    if (success) {
                        await document.save();
                        vscode.window.showInformationMessage('✅ 导入优化完成！');
                    }
                } else {
                    vscode.window.showInformationMessage('✅ 导入语句已经是最优状态');
                }
            });
            
        } catch (error) {
            console.error('优化错误:', error);
            vscode.window.showErrorMessage(`优化失败: ${error.message}`);
        }
    });

    // 测试命令
    const testCommand = vscode.commands.registerCommand('import-optimizer.test', async () => {
        if (optimizer.wasmModule && optimizer.wasmModule.test_optimizer) {
            const result = optimizer.wasmModule.test_optimizer();
            vscode.window.showInformationMessage('测试完成！查看控制台输出');
            console.log('测试结果:', result);
        } else {
            vscode.window.showWarningMessage('WASM 测试功能不可用');
        }
    });

    context.subscriptions.push(optimizeCommand, testCommand);
    
    vscode.window.showInformationMessage('Import Optimizer 已激活！');
}

function deactivate() {
    console.log('Import Optimizer 扩展已停用');
}

module.exports = {
    activate,
    deactivate
};