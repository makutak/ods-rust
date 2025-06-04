# ODS Rust

Open Data Structures (ODS) の Rust 実装プロジェクトです。

## 概要

このプロジェクトは、[Open Data Structures](https://opendatastructures.org/)で紹介されているデータ構造を Rust で実装することを目的としています。

## 特徴

- 基本的なデータ構造の Rust 実装
- テスト駆動開発（TDD）によるロバストな実装
- パフォーマンスを考慮した設計

## プロジェクト構造

```
.
├── src/        # ソースコード
└── tests/      # テストコード
```

## 開発環境

- Rust 2021 Edition
- Cargo (Rust のパッケージマネージャー)

## インストール

```bash
git clone https://github.com/yourusername/ods-rust.git
cd ods-rust
cargo build
```

## テスト

```bash
cargo test
```

## 今後の実装予定

- [ ] ArrayDeque
- [ ] BinarySearchTree
- [ ] RedBlackTree
- [ ] BTree
- [ ] SkipList
- [ ] HashTable

## ライセンス

このプロジェクトは MIT ライセンスの下で公開されています。詳細は[LICENSE](LICENSE)ファイルを参照してください。

## 貢献

1. このリポジトリをフォークする
2. 機能追加用のブランチを作成する
3. 変更をコミットする
4. ブランチにプッシュする
5. Pull Request を作成する
