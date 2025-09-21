## 前提
执行单文件，需要手动引入依赖，而通过rustc无法直接引入依赖，安装rust-script
```bash
cargo install rust-script
```
如果你需要引入依赖，在你编写的代码前加入注释，比如
<img width="1171" height="878" alt="图片" src="https://github.com/user-attachments/assets/1279d775-93a9-404a-80cb-024e1539e338" />
这就相当于你使用cargo创建的一个project里的cargo.toml文件。区别是，这里是单文件
## 编译
执行上述文件，这会在target下生成可执行文件grrs
```bash
cargo build
```
## 执行
选择中文编程文件执行，下面演示了猜数字游戏
```bash
grrs [path]
```
比如
```bash
grrs 猜数字游戏
```

## 转换规则
<img width="1110" height="823" alt="图片" src="https://github.com/user-attachments/assets/e857f27c-ca2d-4d00-9f43-9548e2b5c6af" />

### 替换前
<img width="1112" height="919" alt="图片" src="https://github.com/user-attachments/assets/ec78dda6-e5e4-4ab9-826c-d4e4dbedbdba" />

### 替换后
<img width="1112" height="919" alt="图片" src="https://github.com/user-attachments/assets/63cc4279-619d-4e62-9f57-e0bf5f690f2d" />

## 演示
<img width="1127" height="549" alt="图片" src="https://github.com/user-attachments/assets/24883cd1-f330-492f-86cb-91500435b559" />

