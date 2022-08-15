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
xtrace 默认命令：列出当前运行环境信息以及各项记录文件的最近10条记录
```bash
  -A	Auto clear mode, default: false.
  -M	Modify source file (require -d parameter),  default: false.
  -c int
    	Specified the record count to list/delete. default: 5 (print all)
  -s string
    	Conditions(pid/IP). multiple value can separated by commas.
  -f string
    	File path, multiple files can separated by commas, default: auto search utmp/wtmp/btmp files
```




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
