# openssh

serverにaccess(アクセス)したり、または自身をserverにしたりします。

```sh
$ winget install microsoft.openssh.preview
```

## 他のpcからwindowsに接続する

windowsをssh serverにする方法です。

`sshd_config`は`c:/programdata/ssh/sshd_config`にあります。

> c:/programdata/ssh/sshd_config

```sh
PasswordAuthentication no
PermitEmptyPasswords yes
AuthorizedKeysFile      .ssh/authorized_keys
#Match Group administrators
#       AuthorizedKeysFile __PROGRAMDATA__/ssh/administrators_authorized_keys
```

```sh
# server
$ net start sshd
```

client側で`ssh-keygen`を実行して作成した`.pub`を`~/.ssh/authorized_keys`に追記します。これで鍵認証が通ります。

```sh
# client
$ ssh-keygen -f ~/.ssh/test
$ cat ~/.ssh/test.pub
ssh-rsa AAAAXXXX
---
# server
$ echo "ssh-rsa AAAAXXXX" >> ~/.ssh/authorized_keys
$ cat ~/.ssh/authorized_keys
ssh-rsa AAAAXXXX
```

この処理は`ssh-copy-id`などが使用できる場合は短縮できます。windowsのopensshが対応しているかはわかりませんが。

```sh
# client
$ ssh-copy-id -i ~/.ssh/test $USER@192.168.1.23
```

では他のpcからwindowsにアクセスしてみましょう。

```sh
# client
$ ssh -i ~/.ssh/test $USER@192.168.1.23
```

なお、`~/.ssh/config`に書いておくと省略できます。これはclient側です。

> ~/.ssh/config

```sh
Host windows
    User syui
    Hostname 192.168.1.23
    IdentityFile ~/.ssh/test
    Port 22
```

```sh
# client
$ ssh windows
```

file(dir) copyも容易です。

```sh
# server
$ echo 12345 > ~/file.txt

# client
$ scp -r windows:file.txt .
$ cat file.txt
12345
```

## default-shell

デフォルト(default)のshellを`pwsh`に変えます。

default-shellを変更した場合の注意ですが、更新した際にerrが出る場合があります。

> Permission denied (publickey,keyboard-interactive).

default-shellのpathが違うとpassword/publickey認証のどちらも通りませんので注意してください。

```sh
$ New-ItemProperty -Path "HKLM:\SOFTWARE\OpenSSH" -Name DefaultShell -Value "C:\Program Files\PowerShell\7\pwsh.exe" -PropertyType String -Force
```

例えば、pwsh-previewを使っている場合はこうなります。使用しているpwshのpathを確認してください。

```diff
+ C:\Program Files\PowerShell\7-preview\pwsh.exe
- C:\Program Files\PowerShell\7\pwsh.exe
```

