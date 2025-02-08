# キャラクターの見た目を変える

キャラクターの見た目を変えるには`.vrm`を使うと便利です。これは`pixiv`が作っている規格です。

ueで読み込むには`vrm4u`というpluginを使用します。

<iframe width="100%" height="415" src="https://www.youtube.com/embed/0Ig_-JSRV0M?si=Kz_jCbYTHr_OzPpP&start=0&end=23&mute=1&rel=0&showinfo=0&controls=0" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen></iframe>

## vrm4u

1. [vrm4u](https://github.com/ruyo/VRM4U/releases)をダウンロード(download)して、`$project/Plugins`に入れる。
> $project/Plugins/VRM4U/VRM4U.uplugin
2. editorのファイルエクスプローラーでモデルファイルの`.vrm`をD&Dします。色々と聞かれますが適当にokや選択します。配色タイプが聞かれます。例えば、`$project/Content/Characters`に`model`フォルダを作り`model.vrm`をimport(インポート)した場合で解説します。
> $project/Content/Characters/$model

モデル(model)は[こちら](https://hub.vroid.com/users/36144806)からdownloadすることができます。私は、`vrm 1.0`を使用ていますが、`vrm 0.0`のほうが安定しています。

重要なファイルは`$project/Content/Characters/$model/SK_$model`, `ABP_Post_$model`, `RTG_UEFN_$model`になります。

https://vrm.dev/vrm1/

## 指の角度を調整する

これは`RTG_UEFN_$model`で調整します。各指にある線を選択して値を変更します。

- `回転アルファ:0.5`

## 前髪の角度を調整する

これは`/Content/Character/$model/VM_${model}_VrmMeta`で調整します。具体的には以下のような値にすればいいでしょう。

```json
[
  {
    "bone Name": "J_Sec_Hair1_03",
    "Hit Radius": 0
  },
  {
    "bone Name": "J_Sec_Hair2_03",
    "Hit Radius": 0.01
  },
  {
    "bone Name": "J_Sec_Hair3_03",
    "Hit Radius": 0.01
  }
],
[
  {
    "bone Name": "J_Sec_Hair1_04",
    "Hit Radius": 0
  },
  {
    "bone Name": "J_Sec_Hair2_04",
    "Hit Radius": 0.01
  },
  {
    "bone Name": "J_Sec_Hair3_04",
    "Hit Radius": 0.01
  }
],
[
  {
    "bone Name": "J_Sec_Hair1_05",
    "Hit Radius": 0
  },
  {
    "bone Name": "J_Sec_Hair2_05",
    "Hit Radius": 0.01
  },
  {
    "bone Name": "J_Sec_Hair3_05",
    "Hit Radius": 0.01
  }
]
```

飛行時に髪が爆散する問題は以下です。

```json
{
  "bone Name": "J_Sec_Hair2_03",
  "Hit Radius": 0.0
},
{
  "bone Name": "J_Sec_Hair1_09",
  "Hit Radius": 0.01
},
{
  "bone Name": "J_Sec_Hair1_10",
  "Hit Radius": 0.01
}
```

## アウトラインを追加する

アウトライン(outline)

1. `/Content/Blueprints/RetargetedCharacters/CBP_SandboxCharacter_$model`を作成します。前と同じ要領で新しいキャラクターを追加し、GASPで使えるようにします。
2. componentで`BP_VrmOutlineComponent`を追加します。

## 見た目をきれいにする

これには様々なやり方が存在します。調べた限りではちゃんと機能するやり方がわからないように感じます。

私はcomponentの`VrmPoseableMesh`を追加して、そこに配色タイプの`custom`を当てることで調整しています。ベースは配色タイプの`unlit`を使用します。もし`unlit`で`.vrm`をimportしていない場合は再度importしてください。

1. アセットブラウザで`All/Plugins/`を見えるようにします(プラグインコンテンツを表示)。
2. `/All/Plugins/VRM4U/Util/Actor/PostShadow/BP_PoseCopyToon`と`/All/Content/Blueprints/Characters/${name}/SK_${name}`をmapに配置します。そして、`BP_PoseCopyToon`に`SK_${name}`を選択します。
3. `/All/Plugins/VRM4U/Util/Actor/PostShadow/MI_PostToon`が更新されているので`/All/Plugins/VRM4U/ImportData/DS_VRMCustom`を開いて、全部をMI_PostToonにする
4. `.vrm`を`custom`で読み込む。そして、`/All/Content/Blueprints/CBP_SandboxCharater`のcomponentで`Vrm Poseable Mesh`を追加して、`custom/SK_${name}`を選択します。
