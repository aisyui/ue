# 惑星形式のmapを作る

game engineのmapは基本的に平面で作られています。どこまで行っても地平線が広がっているだけで、そこから抜け出すことはできません。月や太陽があっても背面の絵を動かしているだけです。

これを現実に合わせた形にします。地球があって、上に飛ぶと大気圏があり、大気圏を抜けると宇宙があり、月があり、太陽があるという形にすることを目指します。全てはつながっていて、そこに行くことができます。これを`planet system`と呼ぶことにします。

基本的に`/Map/Small_City_LVL`を使って構築していきます。

## 海の境界を消す

city sampleの海には境界があってcollisionが設定されています。邪魔になるので消します。

`GroundCollisionCube`,`DroneBlockingVolume`を探して削除します。

## ultra dynamic skyで天候と惑星を作る

[ultra dynamic sky](https://docs.google.com/document/d/1xAr0Hd3mY7Mp0g0waKLUvJaddUPaVxEeRoEEFXctCE0/)

1. `/Content/UltraDynamicSky/Blueprints/Ultra_Dynamic_Sky`を開きます。
2. そこに地球と月と太陽のBPを入れます。各自が用意してください。なお、地球は`SkyAtmosphere`の下に置いてください。
> 名前は 地球(BP_Earth), 月(BP_Moon), 太陽(BP_Sun) としておきます。
3. `BP_Earth`は詳細から`transform-location-z:-636000000`, `transform-scale:6360000`にします。
4. `Sky_Sphere_Mesh`の`transform-scale:50000`にします。
5. 関数の`Current Star Color(pure)`にて高度を取得できるため、各componentの表示と非表示(visibility)を切り替えます。

<iframe src="https://blueprintue.com/render/k3xgicx_/1" scrolling="no" allowfullscreen style="width:100%;height:400px"></iframe>

- `BP_Earth`: `transform-location-z:-636000000`, `transform-scale:6360000`
- `Sky_Sphere_Mesh`: `transform-scale:50000`

## ocean wavesで惑星の海を作る

[こちら](/plan/03_ocean.html)
