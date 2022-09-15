# Roadmap

## 0.1.1

- crate 名称改为 cubefs-bond。
- bond 改为 mount，同时只有传入参数 body 类型为 JSON，JSON 字段
  - volName 和 masterAddr 为必须。
  - 重置 mountPoint、logDir 和和 volName 相关的固定目录
  - logLevel 如果缺失，重置为 info。
  - owner 如果缺失，重置为 cfs。
- 添加 umount，用于解除挂载

## 0.1.2

- 使用 child 进程的方式启动 mount 程序
- 添加 release 构建脚本

## 0.1.3

- http 框架切换为 salvo
- 使用 unix socket 做绑定

## 0.1.x

- 传入参数 exporter_port 可选，使用默认值“9504”，如果占用递增，直到可使用。
- 传入参数 prof_port 可选，使用默认值“17410”，如果占用递增，直到可使用。
- 传入参数 owner 可选，使用默认值“cfs”。
