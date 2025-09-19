# アニメーションを作成する

control rigからも作成できます。しかし、購入したものを使うのが一番です。

[magicalanimset](https://www.fab.com/ja/listings/a63386b8-7cad-42cd-8b81-a9de147e1f08)

無料でやるなら`.fbx`, `.vrma`などから作成する方法があります。

- https://www.mixamo.com/
- https://booth.pm/ja/items/5512385
- https://github.com/BandaiNamcoResearchInc/Bandai-Namco-Research-Motiondataset

## control rigから作る

再生して録画ボタンを押すと作れます。

## tatoolsを使う

[tatools](https://www.fab.com/ja/listings/a5d3b60d-b886-4564-bf6d-15d46a8d27fe)を使います。

https://github.com/threepeatgames/ThreepeatAnimTools

使い方は簡単ですが、動画が分かりづらいので、ポイントだけ解説します。pluginの起動、既存のアニメーションの修正、保存です。

1. pluginの起動は、`/Engine/Plugins/ThreepeatAnimTools/Picker/ThreepeatAnimTools_CR_Picker`を起動します。アウトライナーにでもウィンドウを追加しましょう。
2. 修正したいアニメーション(アニメシーケンス)を開いて、`シーケンサで編集 -> コントロールリグにベイク -> CR_UEFNMannyTatoolsRig`します。
3. これでlevel(map)上でレベルシーケンスを開けます。
4. ここからが修正ですが、まず、例えば、腕を選択して向きを変えたとしましょう。これだけでは保存されません。もとに戻ってしまいます。ここで、(1)シーケンサの下にあるアニメーションを削除し、(2)選択している部位のすべてのコンマを削除します。再生してみると編集したとおりになります。
5. 保存は、シーケンサのメニューバーにある保存ボタン(現在のシーケンスとサブシーケンスを保存)を押します。もとのアニメーションを開くと反映されています。

![](/img/0016.png)

## unityから持ってくる

[こちら](/unity/01_fbx.html)を参考にしてください。

