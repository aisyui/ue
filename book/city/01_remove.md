# 惑星形式のmapを作る

game engineのmapは基本的に平面で作られています。どこまで行っても地平線が広がっているだけで、そこから抜け出すことはできません。月や太陽があっても背面の絵を動かしているだけです。

これを現実に合わせた形にします。地球があって、上に飛ぶと大気圏があり、大気圏を抜けると宇宙があり、月があり、太陽があるという形にすることを目指します。全てはつながっていて、そこに行くことができます。これを`planet system`と呼ぶことにします。

基本的に`/Map/Small_City_LVL`を使って構築していきます。

## 海の境界を消す

city sampleの海には境界があってcollisionが設定されています。邪魔になるので消します。

`GroundCollisionCube`,`DroneBlockingVolume`を探して削除します。

## ultra dynamic skyで天候と惑星を作る

[こちら](/plan/02_uds.html)

## ocean wavesで惑星の海を作る

[こちら](/plan/03_ocean.html)

## worldscape pluginを使う

[こちら](/plan/07_wsp.html)
