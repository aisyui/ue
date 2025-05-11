# ultra dynamic sky

[ultra dynamic sky](https://docs.google.com/document/d/1xAr0Hd3mY7Mp0g0waKLUvJaddUPaVxEeRoEEFXctCE0/)

blueprintなどでも`uds`と略されます。

1. `/Content/UltraDynamicSky/Blueprints/Ultra_Dynamic_Sky`, `Ultra_Dynamic_Weather`をlevelに置きます。
2. `Ultra_Dynamic_Weather(Self)`の詳細から`Random Weather Variation:Hourly`にします。これで天候がすぐに変わります。

## planet systemを作る

地上から宇宙にシームレスに移動できるmapを作ります。といっても様々な問題があり、完全には難しいです。一応、[WorldScape Plugin](https://www.fab.com/listings/0ef85bf8-a0be-4b74-87f8-b66bb44d6ae2)というものがあります。

1. `/Content/UltraDynamicSky/Blueprints/Ultra_Dynamic_Sky`を開きます。
2. そこに地球と月と太陽のBPを入れます。各自が用意してください。なお、地球は`SkyAtmosphere`の下に置いてください。
> 名前は 地球(BP_Earth), 月(BP_Moon), 太陽(BP_Sun) としておきます。
3. `BP_Earth`は詳細から`transform-location-z:-636000000`, `transform-scale:6360000`にします。
4. `Sky_Sphere_Mesh`の`transform-scale:50000`にします。
5. 関数の`Current Star Color(pure)`にて高度を取得できるため、各componentの表示と非表示(visibility)を切り替えます。

<iframe src="https://blueprintue.com/render/k3xgicx_/1" scrolling="no" allowfullscreen style="width:100%;height:400px"></iframe>

- `BP_Earth`: `transform-location-z:-636000000`, `transform-scale:6370000`
- `Sky_Sphere_Mesh`: `transform-scale:50000`

この方法に関しては、[worldscape](https://iolacorp-1.gitbook.io/worldscape-plugin)と[cesium](https://cesium.com/platform/cesium-for-unreal/)が有名です。worldscape + udsの実装は[こちら](/plan/07_wsp.html)です。

## issue: mapのcollisionがおかしくなる

これはearthを`SkyAtmosphere`の下に置くと発生します。親子関係を解除しましょう。

## issue: earthの下に黒丸(影)がある

earthのscaleを636から637に変更することで解消できます。

earthを下に見ると、黒い丸が映り込みます。これは、udsをupdateすると発生し始めました。

また、`Captured Scene Sky Lightの`Real Time Capture`をdisableにすると雲も影も無くなります。したがって、条件でon/offを切り変える方法で緩和できます。この方法は使用しなくても良いです。

> リアルタイムキャプチャが有効なスカイライトがシーンにあります。少なくともskyatmosphereコンポーネント、volumetriccloudコンポーネント、またはisksyとしてマテリアルタグが付いたメッシュが必要です。これらがない場合は黒になります。

## issue: 地平線を消す

これは`Fog Max Opacity:0.0`にします。

## issue: 透明な地上が目立つ

Fogを消すと透明な地上が目立つようになります。特に目立つのは夜ですね。

これは`SkyAtmosphere`の色合いで調整しましょう。

## 雪を積もらせる

1. landscapeにあるmaterialを編集します。
2. ultra dynamic weatherと検索して追加します。
3. base color, normalなどのpinがあればつなげます。

<iframe width="100%" height="415" src="https://www.youtube.com/embed/ENMuqbNSLTs?start=912&&rel=0&showinfo=0&controls=0" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen></iframe>

## 天候を変える

天候を変える技を実装しました。

ただし、天候を変更する直前に`BP_Volumetric Stom Sky`を表示しています。これは台風のような雲を表現します。

<iframe width="100%" height="415" src="https://www.youtube.com/embed/Bxdo49vMQEA?rel=0&showinfo=0&controls=0" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen></iframe>

<iframe src="https://blueprintue.com/render/0icb8i4d/2" scrolling="no" allowfullscreen style="width:100%;height:400px"></iframe>
