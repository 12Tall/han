# han
通过线程调用一个js 回调函数

## Usage  
首先安装好`cargo`、`node`、`npm`环境，然后克隆编译即可：
```bash
git clone git@github.com:12Tall/han.git
cd han
npm i
npm run build
node test.js  # 测试
```

## 线程
Node 本地模块可以实现多线程，也能保证本地代码并行，但是，如果在线程中试图调用Js 函数，那么无论这个函数是否是线程安全的，都会阻塞进程。即，某一时刻，Js 进程中最多只能执行一个Js 的函数。估计C++ 中也是这样，不然的话早就有人做出来这种功能了。  
> 启用线程安全功能需要至少启用tokio 包  


