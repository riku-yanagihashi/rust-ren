## github cli
### ローカルでリポジトリの作り方
```sh  
# Gitを初期化
git init
# GitHub CLIを使用して新しいリモートリポジトリを作成します
gh repo create my-new-repo --public
# リモートリポジトリのURLをリモートリポジトリとして追加します
git remote add origin https://github.com/YOUR_USERNAME/my-new-repo.github

# READMEファイルを作成します
echo "# My New Repo" >> README.md

# すべてのファイルをステージングします
git add .

# 最初のコミットを作成します
git commit -m "Initial commit"

# リモートリポジトリにプッシュします
git push -u origin main
```

