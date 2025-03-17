# pixel streaming

`pixel streaming`は`.exe`をサーバー(server)に置いてwebからプレイするためのものです。

[pixel streaming 2](https://github.com/EpicGamesExt/PixelStreamingInfrastructure/)が最新です。[ガイド](https://github.com/EpicGamesExt/PixelStreamingInfrastructure/blob/master/Docs/pixel-streaming-2-migration-guide.md)に従い構築しましょう。

1. projectのpluginで`pixel streaming`をdisableにし、`pixel streaming 2`をenableにする。
2. これをpackage buildして、app.exeを作ります。引数は以下のようにしてください。

なお、sshなどで作業している場合はfirewallの許可が出ませんので、local-desktopで作業してください。または手動でruleを更新してください。

```sh
./$project/Windows/app.exe -AudioMixer -RenderOffScreen -PixelStreamingSignallingURL="ws://127.0.0.1:8888"
```

次にserverの設定です。

```sh
# git clone https://github.com/EpicGamesExt/PixelStreamingInfrastructure.git
$ git clone --branch UE5.5 https://github.com/EpicGamesExt/PixelStreamingInfrastructure.git
$ cd PixelStreamingInfrastructure/SignallingWebServer/platform_scripts/cmd
$ ./setup.bat
$ ./start.bat
```

`PixelStreamingInfrastructure/SignallingWebServer/config.json`の書き換えと起動。特に注意すべきは`http_root`です。Publicからwwwに変更されています。path(パス)にも注意してください。

```sh
$ cd PixelStreamingInfrastructure/SignallingWebServer
$ vim config.json
$ npm start -- --public_ip localhost
```
