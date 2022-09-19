## 直接在 Pod 中挂载

cubefs 中的 volume，如果直接在 pod 中挂载，需要较高的权限去执行 `modprobe fuse`。因此，安全风险较高，如果 Pod 内的其它容器被攻破，可能导致宿主机也被攻破……

```bash
modprobe fuse && \
cd /cfs/client && \
sed -i s/volume_name/hm_pre_biz_r/g config.json && \
sed -i s/master_address/10.201.3.28:8868,10.201.3.29:8868,10.201.3.30:8868/g config.json && \
/cfs/client/cfs-client -f -c /cfs/client/config.json
```

## 使用本 cubefs-bond 挂载

### 缺点

- 占用宿主机端口，且需要人式维护
  - exporterPort
  - profPort
-

### 接口

- param
  - volName
    - mountPoint
    - logDir
  - master_address
  - exporterPort 不能和其它卷相同
  - profPort
  - owner
- body:

样例：

```bash
curl --unix-socket /cfs/bond/salvo.sock  -d '{ "masterAddr": "10.201.3.28:8868,10.201.3.29:8868,10.201.3.30:8868", "volName": "hm_pre_hopper", "owner": "cfs", "logLevel": "info", "exporterPort": 19320, "profPort": "17320" }' http://locahost/mount
```
