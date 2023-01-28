# Парсер расписания
Основные структуры находятся в крейте [maiq-shared](https://github.com/pashokitsme/maiq-parser/tree/master/maiq-shared)

# todo
Умельцы приветствуются!
- [ ] 1. Заменить крейты **scraper**, **table_extract** крейтом [**tl**](https://github.com/y21/tl)
- [ ] 2. Переписать устаревший [**table_extract**](https://github.com/mk12/table-extract) с использованием [**tl**](https://github.com/y21/tl)
- [ ] 3. (Возможно) Переделать модуль замены *По расписанию*, убрать захардкоженые имена файлов

# Сборка
Собирается как исполняемый бинарник (с [**main.rs**](https://github.com/pashokitsme/maiq-parser/tree/master/src/main.rs/)), так и как крейт

Требования: **rustc ^1.66.0** (ниже хз), **cargo** + **stable-msvc** (windows) или **stable** (linux) **toolchain**

```bash
> cargo build --release
> cd target/release/
> ./maiq-parser.exe
```

Или

```toml
[dependencies]
maiq-parser = { git = "https://github.com/pashokitsme/maiq-parser", version = "0.6.3" }
```

Или

```toml
[dependencies]
maiq-shared = { git = "https://github.com/pashokitsme/maiq-parser", version = "0.2.2" }
```