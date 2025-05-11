# git

gitはバージョン(version)管理ツールです。linus(linuxを作った人)が開発しています。

projectにverを付けて、いつでも古いverに戻したり、あるいは新しい機能を別の場所(branch)で開発し、開発が完了したら統合(merge)するようにしたりします。そのほうがprojectが管理しやすいからです。

```sh
$ winget install git.git
```

## tui

[jesseduffield/lazygit](https://github.com/jesseduffield/lazygit)を使います。

```
$ winget install jesseduffield.lazygit
$ lazygit
```

`q`で終了です。`vim`の操作感で使用できます。

## gitの解説

gitでやることは基本的なことさえ理解していればokです。

ようはコードの修正履歴の管理です。

基本的にprojectフォルダのrootから操作を行います。

ここに`git init`で`.git/`を作成し、`.git/config`が設定ファイルになります。

> $project/.git/config

```sh
$ cd $project
$ git init
$ git remote add origin https://github.com/OWNER/REPOSITORY.git
---
$ cat .git/config
[remote "origin"]
	url = https://github.com/OWNER/REPOSITORY.git
	fetch = +refs/heads/*:refs/remotes/origin/*
```

`git remote add`も`.git/config`に書き込んでいるだけなので、直接ファイルを編集しても構いません。git-commandは基本的に`.git/`以下のファイルを変更しているに過ぎないからです。

### config

gitは変更履歴ですが、誰がどのような変更をしたかわかるようになっています。

最初にuserを設定しましょう。mailが必要です。これは、`.git/config`に書いてもいいですが、共通する設定は`~/.gitconfig`に書いておくと便利です。

> ~/.gitconfig

```sh
[user]
	name = syui
	email = syui@users.noreply.github.com
```

もしmailを公開したくない場合は、`$USER@users.noreply.github.com`にしておくとよいでしょう。

### commit

```sh
# 追加
$ echo # title > README.md
$ git add README.md

# コミット
$ git commit -m "first"

# コミットを確定
$ git push -u origin master
```

まずaddでファイルをgit管理に追加しています。

次に、その変更を名前をつけて保存します。これをcommitといいます。

最後にpushして、localに保存されている変更履歴がurlにupload(アップロード)されます。

ここまでが一連の流れです。履歴の確認は`git log`です。

```sh
$ git log
```

### branch

次に、修正を本体のある部分から分離(branch)して行い、最後に本体に取り込むまでの流れを説明します。

```sh
$ git branch -b new-version
---
# コード(ファイル)を修正する
$ vim README.md
----
#これを繰り返す
$ git add .
$ git commit -m test-1
$ git add .
$ git commit -m test-2
$ git add .
$ git commit -m test-3
---
# 修正をまとめる
$ git rebase @~3
r a45ba54 test-1
f c3d7514 test-2
f 21b8b59 test-3

update new-version (rでタイトルを変更)
---
$ git push -f origin new-version
---
```

そして、pull-requestを作成し、mergeで本体(origin/master)に取り込みます。

```sh
# branchはいつでも切り替えられる
$ git branch
$ git checkout master

# 削除も簡単。変更が取り込まれたら削除して構わない
$ git branch -D new-version
```

### push / pull

gitのすべては`.git/`に保存されています。`.git/config`で設定します。

push/pull先を変更してみます。

```diff
+[remote "origin"]
+	url = git@git.syui.ai:syui/test.git
+	fetch = +refs/heads/*:refs/remotes/origin/*
-[remote "origin"]
-	url = https://github.com/OWNER/REPOSITORY.git
-	fetch = +refs/heads/*:refs/remotes/origin/*
```

```sh
$ git pull
```

なお、urlのprotocolはhttpのほか`ssh@github.com`や`git@github.com`などがあります。ただし、urlはprotocolのruleに基づくので注意してください。

```diff
+ git@github.com:syui
- git@github.com/syui
```

ssh, gitは鍵認証でpasswordを省略できます。自身が管理するprojectは`git`にしましょう。

### conflict

gitを使っていて一番厄介な事は、おそらくconflict(衝突)でしょう。

例えば、AさんとBさん、Cさんの三人で開発していたとしましょう。

AさんとBさんは二人とも同じcommitからbranchを切って、作業、つまり、commitを進めていました。一つの丸(commit)があり、そこから別々に枝分かれ(branch)して、丸(commit)が進むイメージです。

ここで、Bさんのほうが早くにpull-reqを出し、新しいコードが本体にmerge、取り込まれました。

次にAさんがpull-reqを出します。しかし、mergeしようとすると、できません。conflictが発生したのです。

AさんもBさんも、同じ箇所に別々の機能を実装しようとしていて、Bさんの変更が先に取り込まれていたからです。

さて、この解消にはいくつか方法があります。Aさんがconflictを解消するコードに修正するか、mergeするCさんが解消するかです。

通常は、Aさんがbranchのcommitを最新のコードに対応したものに作り直します。あるいはpull-reqそのものを作り直すかです。通常は前者になります。branchのcommitを進め、conflictが発生しないように修正するのです。この場合、pull-reqを作り直す必要はありません。

修正は、例えば、他の場所に機能を移すか、Bさんの機能を合体させたものを作るかなどの方法があります。

