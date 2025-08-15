# powershell

`pwsh`こと`powershell`を使った開発の紹介します。`cmd`を使ってもいいですが、基本的にはterminal(ターミナル)でcommand(コマンド)を実行します。

まず、`Win`+`R`でpwshを起動します。

## winget

package managerの[winget](https://github.com/microsoft/winget-cli)を入れてください。

- https://github.com/microsoft/winget-cli/releases

commandは`winget install xxx`です。
`.preview`を外すと古いverがinstallされます。

|title|command(id)|url|
|---|---|---|
|terminal|microsoft.windowsterminal.preview|https://github.com/microsoft/terminal|
|pwsh|microsoft.powershell.preview|https://github.com/powershell/powershell|
|aishell|microsoft.aishell|https://github.com/powershell/aishell/|
|openssh|microsoft.openssh.preview|https://github.com/powershell/win32-openssh|
|wsl|microsoft.wsl|https://github.com/microsoft/wsl|
|vscode|microsoft.visualstudiocode|https://github.com/microsoft/vscode|
|vim|vim.vim|https://github.com/vim/vim-win32-installer|
|git|git.git|https://github.com/git/git|
|lazygit|jesseduffield.lazygit|https://github.com/jesseduffield/lazygit|
|node|openjs.nodejs|https://github.com/nodejs/node|
|nvm|coreybutler.nvmforwindows|https://github.com/nvm-sh/nvm|
|python|python.python.3.12|https://github.com/python|
|conda|anaconda.miniconda3|https://github.com/anaconda|

## その他

|title|command(id)|
|---|---|
|cuda|nvidia.cuda|
|epicgameslauncher|epicgames.epicgameslauncher|
|blender|blenderfoundation.blender|
|discord|discord.discord|
|unity|unity.unity|
|unity hub|unity.unityhub|
|godot|godot.godot|
|obs|obsproject.obsstudio|
|ollama|ollama.ollama|
|vlc|videolan.vlc|

```sh
$ winget install git.git

$ winget show --id=9NT1R1C2HH7J --source=msstore
公開元: OpenAI
発行元 URL: https://help.openai.com
```

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
