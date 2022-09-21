# CubeFS-Bond

## 背景

在 K8S 中 使用 cubefs 时,使用 csi 容易出现 csi-driver 进程重启时,导致 pv 失效, 这是 K8S 本身的问题导致.

因此, 选择使用使用 cfs-client 挂载的文件使用 cubefs.

使用 cfs-client 挂载也有两种方式:

- 在 Pod 中挂载
- 在宿主机中挂载

### 直接在 Pod 中挂载

cubefs 中的 volume，如果直接在 pod 中挂载，需要较高的权限去执行 `modprobe fuse`。
因此，安全风险较高，如果 Pod 内的其它容器被攻破，可能导致宿主机也被攻破……

```bash
modprobe fuse && \
cd /cfs/client && \
sed -i s/volume_name/hm_pre_biz_r/g config.json && \
sed -i s/master_address/10.201.3.28:8868,10.201.3.29:8868,10.201.3.30:8868/g config.json && \
/cfs/client/cfs-client -f -c /cfs/client/config.json
```

### 在宿主机中挂载

1. 在宿主机中安装代理程序
2. 当 Pod 创建时,通过 initContainers 访问宿主机代理程序
3. 宿主机代理程序进行 cfs-client 挂载

cubefs-bond 是此种方式的宿主机代理实现.

#### 缺点

- 多了一个代理程序
- 占用宿主机端口，且需要人工维护
  - exporterPort
  - profPort
-

## cubefs-bond 原理

1. 准备
   - 创建配置文件，位于：`/cfs/bond/{volume_name}/config.json`
     - 重置配置目录 logDir 为：`/cfs/bond/{volume_name}/log`
     - 重置挂载目录 mountPoint 为：`/cfs/mount/{volume_name}`
     - logLevel 如果缺失，重置为 info。
     - owner 如果缺失，重置为 cfs。
     -
   - 创建启动文件，位于：`/cfs/bond/{volume_name}/start.sh`
   - 创建日志目录，位于：`/cfs/bond/{volume_name}/log`
2. 执行挂载 `sh /cfs/bond/{volume_name}/start.sh`
   - 挂载目录位于 `/cfs/mount/{volume_name}`
3. 休眠 1.5 秒, cfs-client 程序如果启动失败,通常会在 1.5 秒内完成.
4. 如果 `/cfs/client/cfs-client -f -c /cfs/bond/{volume_name}/config.json` 进程存在, 说明挂载成功.

## 安装 cubefs-bond

需要先安装 cubefs-client

确保宿主机中 cfs-client 客户端位于: /cfs/client/cfs-client

### 发布版安装

下载 cubefs-bond.tar.gz

// TODO

```bash
tar -zxvf cubefs-bond.tar.gz -C /opt
```

### 源码安装

```bash
git clone https://github.com/taoistwar/cubefs-bond.git
cd cubefs-bond
./build
tar -zxcv dist/release.tar.gz -C  /opt/
```

## 启停

### 启动

```bash
/opt/cubefs-bond/bin/stop.sh
```

### 停止

```bash
/opt/cubefs-bond/bin/stop.sh
```

### 查看

```bash
/opt/cubefs-bond/bin/pids.sh
```

## 使用

### 挂载卷

```bash
curl -XPOST -d '{ "masterAddr": "10.201.3.28:8868,10.201.3.29:8868,10.201.3.30:8868", "volName": "test", "owner": "cfs", "logLevel": "info", "exporterPort": "19320", "profPort": "17320" }' http://localhost:18101/api/bond
```

### 查看卷

```bash
curl http://localhost:18101/api/bond/test
```

### 卸载卷

```bash
curl -XDELETE http://localhost:18101/api/bond/test
```

## 使用场景

### ElasticSearch 集群

### Redis 集群

### Doris 集群

### ClickHouse 集群
