# モーションキャプチャで動かす

カメラ(camera)からcharacterを動かす技術をモーションキャプチャ(motion capture)といいます。様々なprotocol(プロトコル)がありますが、`vrm4u`では[vmc](https://qiita.com/mintan/items/72d63cce4e6197b151b7)というprotocolを使います。

`epic games`は[livelink](https://dev.epicgames.com/documentation/ja-jp/unreal-engine/live-link-in-unreal-engine)というものを作っています。

必要なものは多く、基本的には`web-camera`、`client-app`, `ue-plugin`が必要です。私は以下を使っています。

- camera: ノートパソコンに付属しているwebカメラ
- client: [webcam motion capture](https://webcammotioncapture.info/)
- plugin: [vrm4u](https://github.com/ruyo/VRM4U)

clientはそれぞれのosにあったものを選びます。無料でも可能ですが、性能的には有料アプリがおすすめです。いくつか紹介しておきます。

この辺の情報は最初はわからないと思いますが、基本的には以下の流れで情報を処理します。

```sh
[camera] --> [client] --> [plugin]
```

## client

|name|body|free|
|---|---|---|
|[vmc](https://github.com/sh-akira/VirtualMotionCapture)|vmcの開発元が出しているclient。protocolと同じ名前がつけられている|🟩|
|[vseeface](https://www.vseeface.icu/)|高性能なclient|🟩|
|[xr animator](https://booth.pm/ja/items/4513654)|お手軽に表示できるclient|🟩|
|[waidayo](https://booth.pm/ja/items/1779185)|iosにも対応しているclient|🟩|
|[vrm posing desktop](https://store.steampowered.com/app/1895630/VRM_Posing_Desktop/)|steamから出ているclient|🟥|
|[webcam motion capture](https://webcammotioncapture.info/)|安定したclient。使ってみた中では動きが一番良かった|🟥|

## build packageを使うときportに注意

clientから送信されるportをpluginで受信します。build packageの`.exe`で確認するときは、editor(エディタ)を落としておきましょう。

## 停止したときだけvmcを有効にする

<iframe width="100%" height="415" src="https://www.youtube.com/embed/BsLOlAr-wBY?si=jahPUVD8YMMfefvm&start=195&end=204&mute=1&rel=0&showinfo=0&controls=0" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen></iframe>

キャラクターがidle状態、つまり、停止しているときだけVMCのモーションキャプチャを反映させます。
これはVMCモードの変数を用意し、ABPで条件を書いて実現しています。キャラクターを動かしたときは一時的に無効にします。

関数を作り、bool型でグローバル変数で作ります。それをVMCモードの`enable/disable`とします。カメラは正面と通常を切り替えています。

```sh
/Content/Blueprints/CBP_SandboxCharacter
```

<iframe src="https://blueprintue.com/render/za634zjp/2" scrolling="no" allowfullscreen style="width:100%;height:400px"></iframe>

移動するときは一時的に無効にします。移動が完了すると有効にします。GASの移動は終了後に少し滑るので`delay`を入れています。

<iframe src="https://blueprintue.com/render/za634zjp/3" scrolling="no" allowfullscreen style="width:100%;height:400px"></iframe>

```sh
/Content/Chracters/$model/ABP_Pose_$model
```

`Blend Poses by bool`を使います。`[Mesh Space RefPose] --> [VrmVMC] --> [Blend Poses by bool(true)]`

<iframe src="https://blueprintue.com/render/za634zjp/4" scrolling="no" allowfullscreen style="width:100%;height:400px"></iframe>

