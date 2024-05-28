# TINY AGENT

一个小型的agent，可以作为socket服务端，也可以作为socket客户端
无论是客户端还是服务端都将收到的消息作为子进程命令行执行

## 使用方法
```
Usage: tiny-agent [OPTIONS] --server-ip <SERVER_IP>

Options:
      --server
          当前agent作为socket服务端，默认作为服务端
      --client
          当前agent作为socket客户端
      --server-port <SERVER_PORT>
          作为服务端是socket监听端口，作为客户端是服务端的连接端口，默认8080 [default: 8080]
      --server-ip <SERVER_IP>
          服务端IP地址
      --heartbeat
          是否带socket心跳，默认不带
      --heartbeat-interval <HEARTBEAT_INTERVAL>
          socket心跳间隔，默认10000毫秒 [default: 10000]
      --heartbeat-timeout <HEARTBEAT_TIMEOUT>
          socket心跳超时时长，默认30000毫秒 [default: 30000]
      --offline-retry
          作为客户端时socket是否带掉线重试机制，默认不带
      --offline-retry-interval <OFFLINE_RETRY_INTERVAL>
          作为客户端时socket掉线重试机制间隔，默认30000毫秒 [default: 30000]
```

## socket服务调试

可以使用如下命令快速创建一个socket服务:
```
tiny-agent --server-ip 127.0.0.1
```

接着使用nc或telnet进行调试
```
nc localhost 8080
ls
pwd
...
```