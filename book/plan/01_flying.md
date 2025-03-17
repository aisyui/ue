# superhero flight animations

[superhero flight animations](https://www.fab.com/ja/listings/41185c19-5191-4153-8293-8cc9901efa95)

GASと連携するには飛行モードを有効にするときだけABPを切り替えます。いくつかの調整が必要になります。

<iframe src="https://blueprintue.com/render/thxeju-z" scrolling="no" allowfullscreen style="width:100%;height:400px"></iframe>

## スピードとカメラ設定

|component|name|body|
|---|---|---|
|Character Movement|Max Fly Speed|基本スピード。これを上げてスピードを調整する|
|SFA|Sprint Fly Speed|基本スピード。現時点では機能しないみたい|
|SFA|Sprint Acceleration|徐々にスピードを上げていく値。大きいほどすぐに上昇する|
|SFA|Flight Breaking Deceleration|小さいと飛行後のダウンタイムが発生|
|SprintArm|Target Arm Length|最大でどれだけカメラを離すか|
|SprintArm|Camera Lag Speed|ターゲットに到達する速度。大きいほど遅くなる|
|SprintArm|Camera Lag Max Distance|遅れを取る最大距離|

## アニメーションの変更

これはBPで変更できるものもあれば、ABPで変更しなければいけないものもあります。

例えば、idleのanimを変更します。

1. `/Content/SuperheroFlight/Characters/Mannequins/Animations/ABP_Player_UE5`を開いて、AnimGraphのHoverFlightIdleを見ます。
2. そこにSequence Playerがありますが、`A_Flight_Idle_A`を変更します。
3. 私はMagicalAnimSet(有料)を使いました。`/Content/MagicalAnimSet/Animations/locomotion/Inplace/run_f_loop`を右クリックして、アニメリターゲットで`Content/SuperheroFlight/Characters/Mannequins/Meshes/SKM_Quinn`をターゲットに参照します。superheroで動くanimの`run_f_loop`が作成されます。
4. `HoverFlightIdle`のところで`A_Flight_Idle_A -> run_f_loop`と入れ替えます。

この要領でanimを変更していきます。
