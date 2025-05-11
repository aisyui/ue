# トーンを変える

これには様々なやり方が存在します。

私はcomponentの`VrmPoseableMesh`を追加して、そこに配色タイプの`custom`を当てることで調整しています。ベースは配色タイプの`unlit`を使用します。もし`unlit`で`.vrm`をimportしていない場合は再度importしてください。

1. アセットブラウザで`All/Plugins/`を見えるようにします(プラグインコンテンツを表示)。
2. `/All/Plugins/VRM4U/Util/Actor/PostShadow/BP_PoseCopyToon`と`/All/Content/Blueprints/Characters/${name}/SK_${name}`をmapに配置します。そして、`BP_PoseCopyToon`に`SK_${name}`を選択します。
3. `/All/Plugins/VRM4U/Util/Actor/PostShadow/MI_PostToon`が更新されているので`/All/Plugins/VRM4U/ImportData/DS_VRMCustom`を開いて、全部をMI_PostToonにする
4. `.vrm`を`custom`で読み込む。そして、`/All/Content/Blueprints/CBP_SandboxCharater`のcomponentで`Vrm Poseable Mesh`を追加して、`custom/SK_${name}`を選択します。
