# replicated interaction kit vol 3

その後、動作が気に入らなかったので[replicated interaction kit vol 3](https://www.fab.com/ja/listings/3ce13688-fd10-462f-b90d-964c85a090ad)というassetを購入しました。しかし、キャラの背丈が合わない場合の調整が難しく、あまりおすすめしません。

また、用意されているものすべてを使わないと設定できません。難易度は高めです。具体的には`BP_Chair`, `BP_InteractKitVol3`, `ABP_Manny`, `BP_ThirdPersonCharacter`を使ってGASPの`CBP_SandboxCharacter`に組み込みます。

```sh
/Content/InteractionKitVol3/Blueprint/Blueprint/BP_Chair --> map(level)
/Content/InteractionKitVol3/Blueprint/ActorComponent/BP_InteractKitVol3 --> CBP_SandboxCharacter[Component]
/Content/InteractionKitVol3/Demo/ThirdPerson/Blueprints/BP_ThirdPersonCharacter --> CBP_SandboxCharacter[key-E, key-Space]
/Content/InteractionKitVol3/Demo/Characters/Mannequins/Animations/ABP_Manny --> CBP_SandboxCharacter[Set Anim Instance Class]
```

もしanimを作りたい場合は、`/Content/InteractionKitVol3/Demo/Characters/Mannequins/Rigs/RTG_UE5toUE5`を利用してください。

まずcomponentを`CBP_SandboxCharacter`に入れて、ABPを呼び出す方法があります。GASPの`Pre CMC Tick`の処理に注意してください。`InteractionKitVol3`を使用する場合は停止しなければなりません。

<iframe src="https://blueprintue.com/render/9e2ls2nx/2" scrolling="no" allowfullscreen style="width:100%;height:400px"></iframe>

しかし、ABPを呼び出して椅子から降りるときにもとに戻す方法では、その後動作がおかしくなります。

## 椅子から降りるとなにかにぶつかる

椅子から降りるとなにかにぶつかるようになるのは、`/Content/InteractionKitVol3/Blueprint/ActorComponent/BP_InteractKitVol3`の`Set Collision Response to Channel`の処理が原因です。

この処理をすべて削除しましょう。

しかし、座っている間は動けるようになるので、`IA_Move`に処理を入れます。

<iframe src="https://blueprintue.com/render/9e2ls2nx/4" scrolling="no" allowfullscreen style="width:100%;height:400px"></iframe>

## 背の低いキャラを座らせる

椅子とアニメーションを改造してちょうどいいように座らせることができます。矢印の向きが座る方向になります。これを逆にしてください。

<iframe width="100%" height="415" src="https://www.youtube.com/embed/0YFpV-sUups?si=aTNoK5NTO17ab-Rz&amp;mute=1&rel=0&showinfo=0&controls=0" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen></iframe>

色々と問題が多いです。collisionを一時的に無効化する処理を削除しているため、`manny`は椅子と距離が離れています。これ以上近づけると、逆に椅子の上に浮き上がってしまうためです。

## Enable Auto Blend Outを使わない

ABPを利用せずGASPと統合する方法を紹介します。

そのままだと椅子に座ったあとにすぐ立ち上がってしまいます。これを修正するには`/Content/InteractionKitVol3/Animations/AnimationMontage/AS_F_Start`に以下の設定を行います。

> `/Content/InteractionKitVol3/Animations/AnimationMontage/AS_F_Start`

1. `Enable Auto Blend Out`をfalseにする
2. 終了時に通知で`End`という名前を作る
3. `/Content/InteractionKitVol3/Blueprint/ActorComponent/BP_InteractKitVol3`を修正する

`Enable Auto Blend Out`にすると終了を受け取れなくなります。なので通知から受け取って処理するように書き換えます。

<iframe src="https://blueprintue.com/render/9e2ls2nx/3" scrolling="no" allowfullscreen style="width:100%;height:400px"></iframe>

これで正常に椅子に座る、椅子から降りる、という動作ができるようになりました。

