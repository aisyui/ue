# ocean waves

[ocean waves](https://www.unrealengine.com/marketplace/ja/product/ocean-waves)

必要なものを`/Content/OceanWaves/Levels/EarthSizedOceanPlanet`からcopyして持ってきましょう。

- `BP_EarthSizedSphericalMesh`
- `BP_EarthSizedOcean`
- `WaterVolume`

`BP_EarthSizedSphericalMesh`の`transform-location-z:-63600000`にします。`Sphere Radius:63600000`にします。`SphereEdge Length:16000000`になるはずです。

次に海上の影問題を修正するため`Material Overrides`, `Material Outer/Inner`をすべて変更します。私は`/Vefects/Water/VFX/WaterMaterials`を使用しました。

次に`BP_EarthSizedOcean`の`Volume Maaterials`で`WaterVolume`をセットします。`Above/Underwater`を`/Vefects/Water/VFX/UnderWater`に変更します。`height:0`にします。これは海に入って出たときに海中を適用する高さを設定します。

## city sampleで使うと空間にロードされていないアクタを参照

> WaterVolume: 空間にロードされていないアクタを参照しています。

これは`BP_EarthSizedOcean`, `BP_EarthSizedSphericalMesh`の詳細から`is spatially loaded`のチェックを外します。
