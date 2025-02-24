# もっと近づける

GASPにはマウスのDown/Upのキーでカメラ操作が設定されているので、それを改造します。今回は`Style:Close`でもっと近づけるようにします。

例えば、`GamePlay Camera`から`Get Initial Variable Table`を`Set Camera Rig Parameters`につなげます。そして、`Close_Strafe`を選択し、`/Content/Blueprints/Camera/CameraAsset_SandboxCharacter`にある`Close_Strafe`のOffsetを公開します。公開するにはnodeを伸ばして変数を作ればokです。

これで`/Content/Blueprints/CBP_SandboxCharacter`から値をいれることができます。ピンを分割して、x軸に`-70.0`を入れます。
