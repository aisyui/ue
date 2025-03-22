# ttyd

windows terminalをueのweb browserから操作しようと思ったけど、失敗した話。

`ttyd`というものを使います。これはwebからterminal操作するためのものです。

```sh
# https://github.com/tsl0922/ttyd
$ winget install tsl0922.ttyd

# windowsで動かすためには互換性をwin8(プロパティ->互換性)にしないといけない
# -w ~/ttyd など作業dirを指定しないといけない
$ Get-Command ttyd -ErrorAction SilentlyContinue | Select-Object -ExpandProperty Source

http://localhost:7681
```

なぜかbrowserからキー入力を受け付けないのか動かなかった。

## やりたかったこと

やりたいこととしては、ゲーム内にノートパソコンを置いて、キャラクターが操作しているように見せたい。

ueのweb browserについては[こちら](/city/05_browser.html)を参照してください。

なお、ゲーム内から操作できなくても、windowsで操作しているものをwebで共有する方法があればよい。

