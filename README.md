# rust 素振りのために 9cc 再入門

過去の c でやってみたやつ

- https://github.com/Ryomasao/9cc

制御構文手前まで done

## 環境

こちらを fork して利用している。

https://github.com/microsoft/vscode-remote-try-rust

## 教科書

- 9cc
  経典。
  https://www.sigbus.info/compilerbook

- rust + 9cc

Rust の書き方、設計がすごい勉強になる！こちらの劣化版をつくってく。

https://github.com/utam0k/r9cc

## gdb メモ

gdb のアセンブラを Intel 記法にする。
ホーム配下に `.gdbinit`を作成して設定しとく。

```
cd ~
echo set disassembly-flavor intel > .gdbinit
```

```
& gdb tmp

(gdb) start
# アセンブリ表示にする
(gdb) disas
# アセンブリをステップ実行
(gdb) ni
# レジスタの中身を表示
(gdb) i r

```
