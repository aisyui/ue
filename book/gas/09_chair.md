# 椅子に座る

椅子に座る動作を設定します。大変ですが以下のcomponentを使うと簡単にできます。調整は難しいですね。私の場合はgame animation sampleを使っていますが、最新版では勝手に動きが制御されるためanim montageをそのまま再生できません。したがって、別にabpを作成し、それを呼び出します。

- https://www.youtube.com/watch?v=VzyvpFvon0g
- https://blueprintue.com/blueprint/wg_vyr4o/

その後、動作が気に入らなかったため[replicated interaction kit vol 3](https://www.fab.com/ja/listings/3ce13688-fd10-462f-b90d-964c85a090ad)というassetを購入しましたが、結果はほとんど変わりませんでした。

これは用意されているものをすべて使わないと設定できません。具体的にはBP_Chair, BP_InteractKitVol3, ABP_Manny, BP_ThirdPersonCharacterです。

まずcomponentを`CBP_SandboxCharacter`に入れて、ABPを呼び出し、keyを設定します。なお、anim montageはcomponentの方にも別のものを設定できますので、キャラによって背丈などが合わない場合には個別に設定します。

<iframe src="https://blueprintue.com/render/9e2ls2nx/1" scrolling="no" allowfullscreen style="width:100%;height:400px"></iframe>

用意されているanim montageは後ろ向きになっているため、animを180度回してから録画して新たに作ります。この際、高さなども調整してください。anim montageはloopさせるため、立ち上がり(front_end)が反対になってしまいます。これはBP_Chairにある矢印方向を180度回せばokでした。
