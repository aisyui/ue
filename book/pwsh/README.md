# powershell

`pwsh`こと`powershell`を使った開発の紹介します。`cmd`を使ってもいいですが、基本的にはterminal(ターミナル)でcommand(コマンド)を実行します。

まず、`Win`+`R`でpwshを起動します。

## winget

package managerの[winget](https://github.com/microsoft/winget-cli)を入れてください。

- https://github.com/microsoft/winget-cli/releases

commandは`winget install xxx`です。
`.preview`を外すと古いverがinstallされます。

```sh
$ winget install git.git
```

|title|url|command(id)|
|---|---|---|
|terminal|https://github.com/microsoft/terminal|microsoft.windowsterminal.preview|
|pwsh|https://github.com/powershell/powershell|microsoft.powershell.preview|
|openssh|https://github.com/powershell/win32-openssh|microsoft.openssh.preview|
|wsl|https://github.com/microsoft/wsl|microsoft.wsl|
|vscode|https://github.com/microsoft/vscode|microsoft.visualstudiocode|
|vim|https://github.com/vim/vim-win32-installer|vim.vim|
|git|https://github.com/git/git|git.git|
|lazygit|https://github.com/jesseduffield/lazygit|jesseduffield.lazygit|
|node|https://github.com/nodejs/node|openjs.nodejs|
|nvm|https://github.com/nvm-sh/nvm|coreybutler.nvmforwindows|
|python|https://github.com/python|python.python.3.12|
|conda|https://github.com/anaconda|anaconda.miniconda3|

## その他

|title|command(id)|
|---|---|
|cuda|nvidia.cuda|
|epicgames launcher|epicgames.epicgameslauncher|
|blender|blenderfoundation.blender|
|discord|discord.discord|
|unity|unity.unity|
|unity hub|unity.unityhub|
|godot|godot.godot|
|obs|obsproject.obsstudio|
|ollama|ollama.ollama|

https://github.com/microsoft/winget-pkgs/tree/master/manifests/e/EpicGames/EpicGamesLauncher

## update

```sh
$ winget source update
$ winget upgrade
---
$ winget upgrade --all
```

## choco, scoop

昔は`winget`よりも[choco](https://chocolatey.org/install)や[scoop](https://github.com/ScoopInstaller/Scoop)を使っていました。

## pnpm

```sh
$ npm -g i pnpm
```
