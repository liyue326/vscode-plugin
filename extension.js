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
            const wasmPath = path.join(__dirname, 'vscode-extension/pkg', 'import_optimizer.js');
            console.log('WASM 路径:', wasmPath);
            
            if (!fs.existsSync(wasmPath)) {
                throw new Error(`WASM 文件不存在: ${wasmPath}`);
            }
            
            this.wasmModule = require(wasmPath);
            console.log('✅ WASM 优化器加载成功');
            
            // 测试 WASM 是否工作
            if (this.wasmModule.greet) {
                console.log('Greet 测试:', this.wasmModule.greet('VSCode'));
            }
            
        } catch (error) {
            console.error('❌ WASM 加载失败:', error);
            throw new Error('无法加载 WASM 优化器引擎');
        }
    }

    async optimizeCode(code) {
        console.log('开始优化代码，长度:', code.length);
        console.log('代码内容:', JSON.stringify(code));
        
        if (!this.wasmModule || !this.wasmModule.ImportOptimizer) {
            throw new Error('WASM 优化器未加载');
        }

        try {
            console.log('使用 Rust WASM 优化器...');
            const optimizer = new this.wasmModule.ImportOptimizer();
            
            console.log('调用 optimize 方法...');
            optimizer.set_rules(true, true, true);
            const result = optimizer.optimize(code);
            console.log('Rust WASM 优化完成');
            return result;
            
        } catch (error) {
            console.error('Rust WASM 优化失败:', error);
            throw new Error(`优化失败: ${error.message}`);
        }
    }
}

// 全局优化器实例
let optimizer;

async function activate(context) {
    console.log('🚀 Import Optimizer 扩展激活');
    
    try {
        optimizer = new ImportOptimizer();
        console.log('✅ 优化器初始化完成');
        
    } catch (error) {
        console.error('❌ 优化器初始化失败:', error);
        vscode.window.showErrorMessage(`导入优化器初始化失败: ${error.message}`);
        return;
    }

    // 注册优化命令
    const optimizeCommand = vscode.commands.registerCommand('import-optimizer.optimize', async () => {
        await optimizeImports();
    });

    // 硬编码合并测试命令
    const hardcodedTestCommand = vscode.commands.registerCommand('import-optimizer.hardcodedTest', async () => {
        if (optimizer.wasmModule && optimizer.wasmModule.test_hardcoded_merge) {
            try {
                const result = optimizer.wasmModule.test_hardcoded_merge();
                
                const outputChannel = vscode.window.createOutputChannel('Import Optimizer Hardcoded Test');
                outputChannel.show();
                outputChannel.appendLine(result);
                
                vscode.window.showInformationMessage('硬编码合并测试完成！查看输出面板');
            } catch (error) {
                vscode.window.showErrorMessage(`测试失败: ${error.message}`);
            }
        } else {
            vscode.window.showWarningMessage('硬编码测试功能不可用');
        }
    });

    // 基本功能测试命令
    const basicTestCommand = vscode.commands.registerCommand('import-optimizer.basicTest', async () => {
        if (optimizer.wasmModule && optimizer.wasmModule.greet) {
            try {
                const result = optimizer.wasmModule.greet('VSCode Extension');
                
                const outputChannel = vscode.window.createOutputChannel('Import Optimizer Basic Test');
                outputChannel.show();
                outputChannel.appendLine('基本功能测试:');
                outputChannel.appendLine(result);
                outputChannel.appendLine('\nWASM 模块导出方法:');
                outputChannel.appendLine(Object.keys(optimizer.wasmModule).join(', '));
                
                vscode.window.showInformationMessage('基本功能测试完成！查看输出面板');
            } catch (error) {
                vscode.window.showErrorMessage(`基本测试失败: ${error.message}`);
            }
        } else {
            vscode.window.showWarningMessage('基本测试功能不可用');
        }
    });

    

    context.subscriptions.push(
        optimizeCommand,
        hardcodedTestCommand,
        basicTestCommand,
        
    );
    
    vscode.window.showInformationMessage('Import Optimizer 已激活！');
}

async function optimizeImports() {
    const editor = vscode.window.activeTextEditor;
    if (!editor) {
        vscode.window.showWarningMessage('没有活动的文本编辑器');
        return;
    }
    
    await optimizeImportsInDocument(editor.document);
}

async function optimizeImportsInDocument(document) {
    try {
        const code =  document.getText();
        
        await vscode.window.withProgress({
            location: vscode.ProgressLocation.Notification,
            title: "优化导入语句...",
            cancellable: false
        }, async (progress) => {
            progress.report({ increment: 0 });

            console.log('开始优化文档...');
            const optimizedCode = await optimizer.optimizeCode(code);
            console.log(optimizedCode);
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
                    
                    // 显示优化统计
                    const originalLines = code.split('\n').filter(l => l.trim().startsWith('import')).length;
                    const optimizedLines = optimizedCode.split('\n').filter(l => l.trim().startsWith('import')).length;
                    console.log(`优化统计: ${originalLines} → ${optimizedLines} 个导入`);
                }
            } else {
                vscode.window.showInformationMessage('✅ 导入语句已经是最优状态');
            }
        });
        
    } catch (error) {
        console.error('优化过程错误:', error);
        vscode.window.showErrorMessage(`优化失败: ${error.message}`);
    }
}

function isSupportedLanguage(languageId) {
    return ['javascript', 'typescript', 'javascriptreact', 'typescriptreact'].includes(languageId);
}

function deactivate() {
    console.log('Import Optimizer 扩展已停用');
}

module.exports = {
    activate,
    deactivate
};