# dragon ik plugin

[dragon ik plugin](https://www.fab.com/ja/listings/d3f8d256-d8d9-4d27-91c1-c61e55e984a6)を使って、`chinese dragon`を動かします。

1. [codehawk64/dragonik-exampleproject](https://github.com/codehawk64/dragonik-exampleproject)を自分のprojectの`$project/Content/`にcopyします。
```sh
$ git clone https://github.com/codehawk64/DragonIK-ExampleProject.git
$ cp -rf DragonIK-ExampleProject/Content $project/
```
2. pluginで`dragon ik`を有効にする。 
3. `/Content/Maps/ChineseDragonSolver_Map`を開く。
4. `/Content/CharacterExamples/Quadruped/ChineseDragonSolverExamples/ChineseDragon2/ChineseDragon2_Control`を開いて編集する。

<iframe src="https://blueprintue.com/render/i9d2qt45/1" scrolling="no" allowfullscreen style="width:100%;height:400px"></iframe>

> [ InputAxis -> EnhancedInput ] 修正されているかもしれません。

ue5.3では、"InputAxis Turn"は通常マウスの横方向の移動量を取得するために使用されます。これは、カメラやキャラクターの水平回転（ヨー）を制御するのに適しています。

しかし、ue5.5でInputAxisを使うと`不明な軸を...`というerrorが出るので、`EnhancedInput`を使います。ここではGASPの`/Content/Input/IMC_Sandbox`を使用します。

Enhanced Inputシステムを使用している場合、これは2D軸入力アクションとして設定できます。この場合、X軸の値がTurnに対応し、Y軸の値がLookUp（縦方向の視点変更）に対応します。

