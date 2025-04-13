# 惑星間の移動

惑星間の移動表現を銀河系の光で演出します。

1. sketchfabのgalaxyをbrowserで表示する
2. ボタンが押されたとき移動先へ移動する

<iframe width="100%" height="415" src="https://www.youtube.com/embed/Ub0B91UtGQI?rel=0&showinfo=0&controls=0" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen></iframe>

## sketchfab api

apiを使ってzoom-in, zoom-outします。

```html
<!DOCTYPE HTML>
<html>
	<head>
		<meta charset="UTF-8">
		<meta name="viewport" content="width=device-width, initial-scale=1">
		<title>Sketchfab Viewer API example</title>
		<script type="text/javascript" src="https://static.sketchfab.com/api/sketchfab-viewer-1.12.1.js"></script>
	</head>
	<body>
		<iframe id="api-frame" class="w-full" width="100%" style="position:fixed; top:0; left:0; bottom:0; right:0; width:100%; height:100%;" allow="xr-spatial-tracking; autoplay; fullscreen; xr-spatial-tracking; accelerometer" xr-spatial-tracking web-share frameborder="no" scrolling="no" ></iframe>
		<script type="text/javascript">
				var iframe = document.getElementById( 'api-frame' );
				var uid = 'd6521362b37b48e3a82bce4911409303';
				var client = new Sketchfab( iframe );
				var api;
				client.init( uid, {
					success: function onSuccess( api ){
						api.start({ preload:1 });
						api.addEventListener( 'viewerready', function() {
							api.getCameraLookAt((err, camera) => {
								if (err) return console.error('カメラ取得失敗');
								const direction = [
									camera.position[0] - camera.target[0],
									camera.position[1] - camera.target[1],
									camera.position[2] - camera.target[2]
								];
								const zoomFactor = 0.01;
								const newPosition = [
									camera.target[0] + direction[0] * zoomFactor,
									camera.target[1] + direction[1] * zoomFactor,
									camera.target[2] + direction[2] * zoomFactor
								];
								api.setCameraLookAt(newPosition, camera.target, 10, (err) => {
									if (!err) console.log('ズームアウト成功');
								});
							});
						} );
					},
					error: function onError() {
						console.log( 'Viewer error' );
					},
					camera:1,
					autospin: 0,
					preload: 0,
					ui_controls: 0,
					ui_infos: 0,
					ui_inspector: 0,
					ui_stop: 0,
					ui_watermark: 0,
					ui_watermark_link: 0,
					ui_annotations: 0,
					ui_hint: 0,
					ui_ar:0,ui_help:0,
					ui_settings:0,
					ui_vr:0,
					ui_fullscreen:0,
					ui_loading: 0,
				} );
		</script>
	</body>
</html>
```

## 本来やりたかったこと

本来はキャラクターの動きと空間が連動するようにしたかったです。

これはniagaraで空間ごと表現する方法がありますが、スペック的に可能かはわかりません。

また、playerのcameraにniagaraをくっつけて表現する方法があります。これは低負担で実装できますが、実態と異なり、playerの移動と連動して光の中を飛んでいるように見せるだけです。

実際に光の中を飛べるようにするのが理想ですが、ueでは難しいかもしれません。

