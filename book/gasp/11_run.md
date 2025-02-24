# アニメーションを変更する

今回はダッシュ(sprint)で一定のスピード(speed)に達すると`kawaii_run`と名付けたアラレちゃん走りに切り替わるようにします。

<iframe width="100%" height="415" src="https://www.youtube.com/embed/HaykQT5vBNY?mute=1&rel=0&showinfo=0&controls=0" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen></iframe>

1. ABP_SandboxCharacterのAnimGraphを開きます。`cached pose`を作って処理を分割します。
2. cached poseを呼び出して`Layered blend per bone`のbaseにつなぎます。重要なのはLayered blendの詳細で`spine_03`のボーンを入れること。
3. blendにアニメシーケンスをつなぎます。私は`unity -> .vrma -> vrm4u`で作成したものを使いました。
4. blend weightsの値をconfigから持ってきます。これで`CBP_SandboxCharacter`から制御できます。ダッシュが一定速度に達すれば1、それ以外は0にします。

![](/img/0015.png)

<iframe src="https://blueprintue.com/render/9jgpq1oj/1" scrolling="no" allowfullscreen style="width:100%;height:400px"></iframe>

## アニメシーケンスを作る

- https://booth.pm/ja/items/2845548

これをunityを使って[変換](/unity/01_fbx.html)して使いました。

## リターゲットによる調整

`kawaii_run`は走っているときに両足を中央に持っていくのが正しいですが、このままでは足が開いてしまいます。

これを調整するにはリターゲットのposeを変更します。使用している`RTG_UEFN_$model`を開いて、`Running Retarget`から`Editing Retarget Pose`にします。両足を回転さればokです。一方はプラス、もう一方はマイナスの値です。

- left: 0.0, -2.0, 20.0
- right:  0.0, 2.0, -20.0

## GASPでアニメーションを変更するには

`CHT_AnimationsForStateMachine`でそれぞれの項目のanimを変更すればいいのだろうかと思いましたが、走るanimに関しては、複雑すぎる上に全方位のものを用意する必要があります。

したがって、`ABP_SandboxCharacter`のblendを活用しながら適用しました。
