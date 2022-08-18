# xtrace
A tool to clean up traces of operations on linux.

## 核心功能
- utmp类文件的读取、修改
- auth.log, secure文件的修改
- lastlog, messages等文件的修改


## 开发步骤
- 命令行参数读取
- utmp文件的解析
- utmp文件的修改
- 其他log文件的处理

## 设计细节
### 命令行参数

```bash
xtrace 0.1.0

USAGE:
    xtrace [OPTIONS]

OPTIONS:
    -c <number>
            Specify the record count to aim in the target file [default: 5]

    -D
            DELETE the matched records in the target file(s)

    -h, --help
            Print help information

    -s <Pid | Hostname | UnionCode>
            search the condition to filter the records

    -t <file>
            Specify the target file [default: /run/utmp /var/log/wtmp /var/log/btmp]

    -V, --version
            Print version information
```

### 常用命令
```bash
./xtrace -t /var/run/utmp -c 3 -D
./xtrace -s 127.0.0.1 -D
```

## bug
- 记录显示的顺序倒序了。。。   【已处理】
- IP Addr的解析存在异常，不确定是否为大小端问题所致。。。。  通过nom库取IP数据时，刻意使用 `u32::from_be_bytes`来暂时纠正该问题。。。。

## TODO
当前只能对Linux系统的utmp/wtmp/btmp记录进行罗列及删除操作。。。

后续看情况。。。。

## Change Log
- Version 0.1.2 (2022.08.18)
    
    提示文件没有权限。。。

## 参考
### Linux日志文件
> https://www.eurovps.com/blog/important-linux-log-files-you-must-be-monitoring/
- /var/log/messages
- /var/log/auth.log
- /var/log/secure
- /var/log/boot.log
- /var/log/dmesg
- /var/log/kern.log
- /var/log/faillog
- /var/log/cron
- /var/log/yum.log
- /var/log/maillog or /var/log/mail.log
- /var/log/httpd/
- /var/log/mysqld.log or /var/log/mysql.log
