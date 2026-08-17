# Windows で Rust 環境をセットアップする手順

## 1. 前提条件

- Windows 10 / 11 (64bit)
- インターネット接続

## 2. Visual C++ Build Tools のインストール

Rust のコンパイルには MSVC リンカーが必要です。

1. [Visual Studio Build Tools](https://visualstudio.microsoft.com/ja/visual-cpp-build-tools/) をダウンロードして実行
2. インストーラーで **「C++ によるデスクトップ開発」** にチェックを入れてインストール

> **代替手段:** Visual Studio 2019/2022 がすでにインストールされている場合は、この手順は不要です。

## 3. rustup のインストール

1. [https://rustup.rs/](https://rustup.rs/) にアクセス
2. `rustup-init.exe` をダウンロードして実行
3. プロンプトが表示されたら `1` (Proceed with standard installation) を選択して Enter

インストールが完了すると以下のツールが使用可能になります。

| ツール | 説明 |
|--------|------|
| `rustup` | Rustツールチェーン管理 |
| `rustc` | Rustコンパイラ |
| `cargo` | ビルドツール & パッケージマネージャ |

## 4. 環境変数の反映

インストール後、新しいターミナル（PowerShell / コマンドプロンプト）を開いて PATH を反映させます。

または、現在のセッションに即時反映する場合:

```powershell
$env:Path = [System.Environment]::GetEnvironmentVariable("Path", "Machine") + ";" + [System.Environment]::GetEnvironmentVariable("Path", "User")
```

## 5. インストール確認

```powershell
rustc --version
cargo --version
rustup --version
```

出力例:

```
rustc 1.80.0 (051478957 2024-07-21)
cargo 1.80.0 (376290515 2024-07-16)
rustup 1.27.1 (54dd3d00f 2024-04-24)
```

## 6. ツールチェーンの更新

```powershell
rustup update
```

## 7. VS Code 拡張機能（推奨）

| 拡張機能 | ID |
|----------|----|
| rust-analyzer | `rust-lang.rust-analyzer` |
| CodeLLDB (デバッガ) | `vadimcn.vscode-lldb` |

インストールコマンド:

```powershell
code --install-extension rust-lang.rust-analyzer
code --install-extension vadimcn.vscode-lldb
```

## 8. 動作確認

```powershell
cargo new hello_world
cd hello_world
cargo run
```

`Hello, world!` が表示されれば環境構築完了です。

## 参考リンク

- [The Rust Programming Language (日本語)](https://doc.rust-jp.rs/book-ja/)
- [Rust 公式サイト](https://www.rust-lang.org/ja)
- [crates.io](https://crates.io/)
