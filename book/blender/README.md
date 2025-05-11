# blender

[blender](https://projects.blender.org/blender/blender)でモデルを編集して、ueで読み込みます。

- version: `4.5`

```sh
# https://developer.blender.org/docs/handbook/building_blender/windows/
$ git clone https://projects.blender.org/blender/blender.git
$ cd blender
$ make update 
$ make
```

## addon

|addon|body|
|---|---|
|https://github.com/saturday06/VRM-Addon-for-Blender|vrmを読み込む|

## その他のaddon

結局、vrmを読み込むもの以外は使いませんでした。気になったaddonを載せておきます。

|addon|body|
|---|---|
|https://github.com/smokejohn/SKkeeper|modifireをobjectに反映|
|https://github.com/12funkeys/rigid_bodys_gen|rigを付ける|
|https://github.com/shteeve3d/blender-wiggle-2|rigを付ける|
|https://pielotopica.booth.pm/items/4854979|衣装を着せる|

なお、pythonを実行するのは危険も伴いますので、blender addonの実行(インストール)は最低限にしてましょう。
