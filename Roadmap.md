# Roadmap

## TODO

- maybe
  - 传入参数 exporter_port 可选，使用默认值“9504”，如果占用递增，直到可使用。
  - 传入参数 prof_port 可选，使用默认值“17410”，如果占用递增，直到可使用。
- REST
  - 只有成功使用 200 状态码，出现错误使用 500 状态码
- 声明式
  - 在 delete bond 之前，一直监听 cfs-client 进程是否存在，如果挂了自动启动。
  - 不再是无状态的了，需要存储了？
- 错误处理
  - 充满了 golang 式的 if err 式处理
  - 使用 try {} catch 式处理？ panic!应该是进程崩溃的错误，但非嵌入设备可以捕获……
- OpenTelemetry 集成？
- Metrics for Prometheus Exporter
- 可配置 bond、cfs-client、mount 目录

## 0.1.4

- 切换回 rocket，使用 127.0.0.1 进行交互
- 使用 /bond 的 post method 进行挂载、get method 进行查看、delete method 进行卸载
- 传入参数 owner 可选，使用默认值“cfs”。

## 0.1.3

- http 框架切换为 salvo
- 使用 unix socket 做绑定

## 0.1.2

- 使用 child 进程的方式启动 mount 程序
- 添加 release 构建脚本

## 0.1.1

- crate 名称改为 cubefs-bond。
- bond 改为 mount，同时只有传入参数 body 类型为 JSON，JSON 字段
  - volName 和 masterAddr 为必须。
  - 重置 mountPoint、logDir 和和 volName 相关的固定目录
  - logLevel 如果缺失，重置为 info。
  - owner 如果缺失，重置为 cfs。
- 添加 umount，用于解除挂载
