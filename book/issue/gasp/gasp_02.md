## cameraが急接近する

よじ登ったり、柵を超えたりするとき、camera(カメラ)が急接近することがあります。

これは`GameplayCamera`の`CameraRig_CollisionOffset`が原因です。`/Content/Blueprints/CBP_SandboxCharacter`の`SetupCamera`という関数で使われていますので、その部分だけ外しておきましょう。

ただし、床が透けて映ってしまうようになります。


