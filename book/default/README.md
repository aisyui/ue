# default

## 本書の目的 

ue5でゲームを作成するまでの過程をまとめます。

主に`example`形式で記述し、実行するとその通りの結果になることを目指します。

|name|latest|body|
|---|---|---|
|[unreal engine](https://dev.epicgames.com/documentation/ja-jp/unreal-engine/unreal-engine-5.5-release-notes)|5.5.3|ゲームエンジン|

## パソコンのスペック

開発するゲームによりますが、GPUはnvidiaの`RTX 4060Ti`を使っています。特に不満はありません。

作りたいゲームによってはGPUは不要です。オープンワールドを作るならGPUはあったほうがいいと思います。

多くの場合、ue5のバグのほうが問題で、GPUの性能をほとんど引き出せていないことのほうが問題です。スペックはあまり関係なかったりします。

ただし、ストレージの読み書き速度は重要です。ueやprojectを入れるストレージには注意してください。

## 使用するタグ

### youtube

`https://www.youtube.com/embed/${id}?start=0&end=10&mute=1&rel=0&showinfo=0&controls=0`

```html
<iframe width="100%" height="415" src="https://www.youtube.com/embed/?start=0&end=10&mute=1&rel=0&showinfo=0&controls=0" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen></iframe>
```

### blueprint

`https://blueprintue.com/render/${id}/${n}`

```html
<iframe src="https://blueprintue.com/render/" scrolling="no" allowfullscreen style="width:100%;height:400px"></iframe>
```
