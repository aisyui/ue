# 惑星間の移動

惑星間の移動表現を銀河系の光で演出します。

1. sketchfabのgalaxyをbrowserで表示する
2. ボタンが押されたとき移動先へ移動する

```html
<div class="sketchfab-embed-wrapper"> <iframe title="Need some space?" width="100%" style="position:fixed; top:0; left:0; bottom:0; right:0; width:100%; height:100%;" frameborder="0" allowfullscreen mozallowfullscreen="true" webkitallowfullscreen="true" allow="autoplay; fullscreen; xr-spatial-tracking" xr-spatial-tracking execution-while-out-of-viewport execution-while-not-rendered web-share src="https://sketchfab.com/models/d6521362b37b48e3a82bce4911409303/embed?autostart=1&ui_animations=0&ui_infos=0&ui_stop=0&ui_inspector=0&ui_watermark_link=0&ui_watermark=0&ui_hint=0&ui_ar=0&ui_help=0&ui_settings=0&ui_vr=0&ui_fullscreen=0&ui_annotations=0&ui_theme=dark"> </iframe> </div>
```

<iframe width="100%" height="415" src="https://www.youtube.com/embed/Ub0B91UtGQI?rel=0&showinfo=0&controls=0" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen></iframe>

## 本来やりたかったこと

本来はキャラクターの動きと空間が連動するようにしたかったです。

これはniagaraで空間ごと表現する方法がありますが、スペック的に可能かはわかりません。

また、playerのcameraにniagaraをくっつけて表現する方法があります。これは低負担で実装できますが、実態と異なり、playerの移動と連動して光の中を飛んでいるように見せるだけです。

実際に光の中を飛べるようにするのが理想ですが、ueでは難しいかもしれません。

