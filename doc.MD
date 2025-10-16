# 变量
- 变量遮蔽是两个变量两个let 第一个变量不生效了  
- 变量可变性mut 是一个变量 改变值 只用一个let

# 闭包函数
- JS 函数天生就是闭包，外部变量自动捕获和 println!一样内部借用了；Rust 函数是裸函数，要外部值只能闭包或显式传参
```
    let print = || println!("{}", color); // 闭包自动捕获 &color，不用写&

    let _reborrow = &color;               // 你*手动*再借一次，必须写 &
    print();                              // 闭包继续用它的 &color，无冲突
```

# mod 类型
 - 函数
 - 结构体（里层的变量也要看是不是私有的，尽管外面是共有的）
 - self和super(当前【模块】不是函数，的上一层)
 什么时候必须写 self::
 - 当前模块和外部模块存在同名函数/类型时 不存在时候 写不写效果一样
| 修饰符                | 可见范围             | 典型用途      |
| ------------------ | ---------------- | --------- |
| **（不写）**           | 仅当前模块            | 私有实现细节    |
| `pub(super)`       | 仅父模块             | 子模块给父用    |
| `pub(in path::to)` | 指定路径的某个祖先模块      | 精细控制可见域   |
| `pub(crate)`       | **整个当前 crate 内** | 内部工具函数/类型 |
| `pub`              | **全局公开**         | 库的公共 API  |


# 泛型
    - 泛型列表 <T, U, …>
    写在函数名右边，给编译期用：生成多少份单态化代码。
    - 参数列表 (x: T, y: U, …)
    写在函数名右边再右边，给运行期用：传什么值、占多少栈











# 第一次 js开发
- 检查多余没用到的包
- 版本号每次要不一致
- package.json里面内容要全，extension，ignore要有
- package.json 中的contributes.commands 要在extension中注册
- 发布流程 marketplace创建账号，填写插件信息然后上传vsix格式文件
- rust的缓存是target目录 可以清除掉
- 每次修改后reload window
- 打包vsce package 
- 本地调试安装插件 code --install-extension vscode-process-monitor-1.0.2.vsix

# 第二次 rust开发 
 - extension 里面不写逻辑 充当胶水代码 加载wasm
 - 切到rust-part目录  wasm-pack build --target nodejs 
 - rust文件有修改 切到rust目录 执行 wasm-pack build --target nodejs， cp -r pkg ../extension/wasm，cd ../extension，vsce package,code --install...
 - --target nodejs 就是告诉 wasm-pack：‘请生成 CommonJS 风格的胶水文件，让我可以 require('./wasm/xxx')’，专供 Node 环境（含 VS Code 扩 展）使用  web就是给浏览器用




# 打印 HELP-TOGGLE TOOL
  - js Output 面板右上角下拉框 → “进程监控”（你起的名字）
  - rust的语言打印 console::log_1(&"Rust: Monitor 构造函数被调用".into()); 
  - **多个参数**：
  ```
   console::log_2(
       &"User:".into(),
       &username.into()
   );
# rust 与 js的交互 wasm-bindgen