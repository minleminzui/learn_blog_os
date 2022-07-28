## 独立可执行程序
- 由于是要写一个可执行的os，那么对于该程序，我们需要将os相关的依赖禁用
- 由于禁用了标准库，那么需要定义自己的panic处理函数
- `语言项`相关，语言项是一些编译器需求的特殊函数或者类型。
    - `eh_personality 语言项`，对于被此语言项标记的函数，将用于实现**栈展开(stack unwinding)**，那么需要依赖os的库，所以同理，需要禁用
    - `start`语言项。一些runtime(比如垃圾回收，绿色线程)需要在main函数之前启动。Rust从一个叫做crt0的runtime库开始，其用于建立适合C语言程序的环境，其中包含了栈的创建与可执行程序参数的传入，之后这个runtime库会调用**Rust的运行时入口**，这个入口点便是**start语言项**，Rust的runtime极小，有一些爆栈检测和打印**堆栈轨迹**。之后，便是调用**main**函数。所以需要重写**整个crt0库以及其入口点**
- **链接问题**，linker会默认去链接C运行时环境，所以需要改变其**目标三元组**`rustup target add thumbv7em-none-eabihf`并且在`cargo build --target thumbv7em-none-eabihf`时添加`--target`交叉编译程序，交叉编译就是指比如在pc上编译程序为安卓机上的二进制文件，而在这里就是将程序编译为`x86-64`环境的目标