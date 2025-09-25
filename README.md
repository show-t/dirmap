# Dirmap

`Dirmap` is a command-line tool to create directory structures from YAML file and export existing directory structures to YAML file.   
`Dirmap` は YAML ファイルからディレクトリ構造を作成したり、既存のディレクトリ構造を YAMLファイルにエクスポートしたりできるコマンドラインツールです。

## 📦 Features

- Automatically generate directories and files from a YAML file  
  YAML からディレクトリとファイルを自動生成  
- Export directory structures to a YAML file  
  ディレクトリ構造を YAML ファイルにエクスポート

## 📚 Requirements

- Rust 1.70+ (with Cargo)

## 🚀 Getting Started

### 1. Clone the repository and build

```bash
git clone https://github.com/show-t/dirmap.git
cd dirmap
cargo build --release
```

The compiled binary will be located at:  
ビルド後の実行ファイルは以下の場所に生成されます：

```bash
target/release/
```

Optionally, move the binary to a directory in your PATH:  
必要に応じて、バイナリをPATHの通ったディレクトリに移動してください：

### 2. Create `dirmap.yaml` in your project root.

```yaml
project:
  src:
    main.rs:
    lib.rs:
  Cargo.toml: |
    [package]
    name = "my_app"
    version = "0.1.0"
    edition = "2021"
```

- example output

```
project/
├── src/
│   ├── main.rs
│   └── lib.rs
└── Cargo.toml
```

## 📖 Usage

### 1. Create directories from YAML

```bash
dirmap create -o ./out_dir
```

- Create directories and files under `./out_dir` according to `dirmap.yaml`  
  `dirmap.yaml` の内容に従い、 `./out_dir` にディレクトリおよびファイルを作成

### 2. Export directory structure to YAML

```bash
dirmap export -t project_dir
```

- Export the directory structure inside `project_dir` to `dirmap.yaml`  
  `project_dir` 内のディレクトリ構造を、YAMLとして `dirmap.yaml` に書き出す

## 🛠️ Options

### `create`

| Option                         | Description                                                                 |
|--------------------------------|-----------------------------------------------------------------------------|
| `-i`, `--input STRUCTURE_FILE` | Specify the directory structure definition file (default: `dirmap.yaml`) |
| `-o`, `--output OUTPUT_DIR`    | Specify the directory structure to extract to (default: `./`)               |

### `export`

| Option                        | Description                                           |
|-------------------------------|-------------------------------------------------------|
| `-t`, `--target TARGET_DIR`   | Specify the directory to export (default: `./`)       |
| `-o`, `--output OUTPUT_FILE`  | Specify output file (default: `dirmap.yaml`)       |

## 📁 Project Structure

```
.
├── src/            # Rust source files
│   └── main.rs
├── target/         # Compiled Rust output
├── cargo.toml      # Cargo configuration
├── .gitignore   
├── LICENSE   
└── README.md       # Project documentation
```

## ℹ️ Infomation

### Log Output Directory

- Default log directories are set according to the OS.:  
  OSごとにデフォルトのログディレクトリを設定しています。:
  - macOS: `/usr/local/var/logs/dirmap`
  - Linux: `/var/logs/dirmap`
  - Windows: `%USERPROFILE%\logs\dirmap`
- Write permissions may be required. If necessary, adjust permissions or the directory.  
  書き込み権限が必要な場合があります。必要に応じて、権限設定あるいはディレクトリを変更してください。

## 📄 License

This project is licensed under the MIT License.  
本プロジェクトは MIT ライセンスのもとで公開されています。  

See the [LICENSE](./LICENSE) file for more details.  
詳細は [LICENSE](./LICENSE) ファイルをご覧ください。