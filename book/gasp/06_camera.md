# カメラワークを設定する

スキル発動したときカメラをぐるっと回す演出です。
正面カメラを設置し、タイムラインで動かしたあとに通常(後方)カメラに切り替えることで実現しています。

<iframe width="100%" height="415" src="https://www.youtube.com/embed/BsLOlAr-wBY?si=jahPUVD8YMMfefvm&amp;start=29&end=34&amp;mute=1&rel=0&showinfo=0&controls=0" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen></iframe>


```sh
/Content/Blueprints/CBP_SandboxCharacter
```

<iframe src="https://blueprintue.com/render/exmpoyfu/1/" scrolling="no" allowfullscreen style="width:100%;height:400px"></iframe>

ボス戦のときだけこの演出を実行します。なお、`true`, `false`はどちらも最終的に`skill(スキル)`の実行につながるようにしてください。
