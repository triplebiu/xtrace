# utmp

utmp文件没有幻数(Magic Number)。

```bash
# 查看utmp中的记录块
$ xxdoffsetnum=$((384*1));xxd -s $xxdoffsetnum -g 16 -l 384 utmp
```


## Reference
  - [hidemyass](https://github.com/evilpan/hidemyass)
  - [utmp(5)-Linux man page](https://linux.die.net/man/5/utmp)


