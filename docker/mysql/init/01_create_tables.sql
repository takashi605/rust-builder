-- テーブルの作成
CREATE TABLE users (
    id INT AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL UNIQUE
);

-- サンプルデータの挿入
INSERT INTO users (name, email) VALUES ('山田 太郎', 'taro.yamada@example.com');
INSERT INTO users (name, email) VALUES ('佐藤 花子', 'hanako.sato@example.com');
INSERT INTO users (name, email) VALUES ('鈴木 一郎', 'ichiro.suzuki@example.com');
INSERT INTO users (name, email) VALUES ('田中 直子', 'naoko.tanaka@example.com');
INSERT INTO users (name, email) VALUES ('高橋 誠', 'makoto.takahashi@example.com');
