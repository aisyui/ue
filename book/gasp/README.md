# game animation sample

[game aimation sample](https://www.fab.com/ja/listings/880e319a-a59e-4ed2-b268-b32dac7fa016)は`epic games`が提供しているassetです。

[G]ame [A]nimation [S]ample [P]rojectでGASPと略すことがあります。

[G]ame [A]bility [S]ystemもGASなので紛らわしいですね。

まずはこれを使ってキャラクター(character)を動かしてみましょう。

## level(map)

1. `/Content/Levels/DefaultLevel`を開きます。
2. 再生ボタンを押します。
3. ゲームがプレイできます。

|key|en|ja|
|---|---|---|
|w,a,s,d|move|移動|
|space|jump|ジャンプ|
|space|sprint|ダッシュ|
|ctrl|walk|歩く|
|c|crouch|しゃがむ|
|マウススクロール|camera|カメラ|

ボタンに乗るとキャラを切り替えることができます。

レベル(level)はマップ(map)とも呼ばれます。プレイ(play)する場所を作ります。

特にボタンが重要です。tipsを確認しましょう。

`/Content/Levels/DefaultLevel`を右クリックして`参照ビューア`を選択してみます。どのファイルを参照しているかわかります。

## BP

characterは`/Content/Blueprints/CBP_SandboxCharacter`で動かしています。このファイルを開いて編集してみましょう。

1. `EventGraph`というノードが記述されている場所(画面中央)で右クリックし、`Debug Key 1`を追加します。
2. Releasedというピン(pin)から`Print String`を追加します。
3. ゲームを再生します。
4. キーボード(keyboard)の`[1]`を押すと、画面に`Hello`が表示されます。

これがbp(blueprint)でゲームを作る要領になります。

## ABP

characterの動きは`/Content/Blueprints/ABP_SandboxCharacter`で設定されています。

## input

まずkeyがどこで設定されているのかと言うと、`/Content/Input/IMC_Sandbox`で設定されています。

例えば、`/Content/Input/IA_Aim`をコピー(copy)して、新しく`IA_One`を作ってみましょう。それを`IMC_Sandbox`で読み込んで`[1]`に割り当てます。

次に、`IA_One`を先程の`CBP_SandboxCharacter`にドラッグ&ドロップ(D&D)します。

すると`EnhancedInputAction IA_One`という赤いノードが作成されたと思います。`Debug Key 1`を削除して代わりにつなぎます。色にも意味がありますが、そのうちわかってくると思います。

ゲームを再生してみると先ほどと同じkeyで動きます。


