# arch-sync-audit

Учебная Rust CLI-утилита для сравнения двух Arch Linux систем: эталонной настроенной машины и новой машины, которую нужно привести к похожему состоянию.

Главная цель проекта - не только получить полезный инструмент, но и поэтапно изучить Rust на практической задаче.

## Цель

Сделать инструмент, который:

- экспортирует профиль текущей Arch-системы в файл;
- проверяет другую систему по этому профилю;
- показывает, каких пакетов, сервисов и конфигов не хватает;
- генерирует безопасный план команд, но не применяет его автоматически.

Инструмент не должен заменять `chezmoi`, `stow`, `ansible` или `nix`. Его задача - аудит системы и обучение Rust.

## Ограничения MVP

- Не применять изменения автоматически.
- Не копировать секреты.
- Не читать приватные ключи из `~/.ssh`.
- Не считать, что у пользователя установлен `paru` или `yay`.
- Не смешивать аудит системы и управление dotfiles.
- В MVP конфиги проверяются только по наличию путей, без чтения содержимого.

## Основные команды

```bash
arch-sync-audit export --output system.toml
arch-sync-audit check system.toml
arch-sync-audit plan system.toml
```

### `export`

Собирает профиль текущей системы и сохраняет его в `system.toml`.

В полном MVP профиль может включать:

- явно установленные pacman-пакеты;
- AUR-пакеты;
- текущий shell из `$SHELL`;
- список известных конфиг-путей из `$HOME`;
- включенные systemd user services;
- включенные systemd system services.

Важно: на этапе MVP конфиги не копируются и не читаются. Сохраняются только пути, которые существуют на эталонной системе.

### `check`

Сравнивает текущую систему с профилем из `system.toml` и выводит различия.

Пример вывода:

```text
Missing pacman packages:
- neovim
- ripgrep
- kitty

Missing AUR packages:
- visual-studio-code-bin

Missing config paths:
- ~/.config/hypr
- ~/.zshrc

Different shell:
expected: /bin/zsh
current:  /bin/bash
```

### `plan`

Печатает команды или подсказки, которые пользователь может выполнить вручную.

Пример:

```bash
sudo pacman -S neovim ripgrep kitty
# Install AUR packages manually:
# visual-studio-code-bin
chsh -s /bin/zsh
```

Команда `plan` только печатает действия. Она не должна ничего устанавливать, удалять или менять.

## Структура проекта

Предполагаемая структура после создания Rust-проекта:

```text
arch-sync-audit/
  Cargo.toml
  PLAN.md
  AGENTS.md
  docs/
    architecture.md
    commands.md
    profile-format.md
    development-notes.md
  src/
    main.rs
    cli.rs
    profile.rs
    collect.rs
    compare.rs
    plan.rs
  tests/
    fixtures/
```

## Rust-зависимости

Начальный набор:

```toml
[dependencies]
anyhow = "1"
clap = { version = "4", features = ["derive"] }
serde = { version = "1", features = ["derive"] }
toml = "0.8"
```

Позже можно добавить:

```toml
tabled = "0.16"
ratatui = "0.29"
crossterm = "0.28"
```

Дополнительные зависимости не добавляются заранее. Каждая новая зависимость должна решать конкретную проблему.

## Учебный roadmap

### Этап 0. Подготовка проекта

Результат:

```bash
cargo init --bin
cargo run
```

Что должно получиться:

```text
Hello, world!
```

Rust-темы:

- структура Cargo-проекта;
- `Cargo.toml`;
- `src/main.rs`;
- команда `cargo run`;
- команда `cargo check`.

Что писать руками:

- создать проект;
- запустить первые cargo-команды;
- посмотреть, где находится точка входа `fn main()`.

Проверка:

```bash
cargo check
cargo run
```

### Этап 1. Минимальный CLI

Результат:

```bash
arch-sync-audit export --output system.toml
arch-sync-audit check system.toml
arch-sync-audit plan system.toml
```

На этом этапе команды могут только печатать заглушки:

```text
export command
check command
plan command
```

Rust-темы:

- enum для подкоманд;
- derive-макросы;
- `clap`;
- разбор аргументов;
- `match`.

Что писать руками:

- добавить зависимость `clap`;
- создать структуру CLI;
- создать enum команд;
- написать `match` по командам.

Проверка:

```bash
cargo check
cargo run -- export --output system.toml
cargo run -- check system.toml
cargo run -- plan system.toml
```

### Этап 2. Модель профиля системы

Результат: в коде появляется структура профиля системы.

```rust
struct SystemProfile {
    pacman_packages: Vec<String>,
    aur_packages: Vec<String>,
    shell: Option<String>,
    config_paths: Vec<String>,
    system_services: Vec<String>,
    user_services: Vec<String>,
}
```

Rust-темы:

- `struct`;
- `Vec<T>`;
- `Option<T>`;
- владение строками через `String`;
- `#[derive(...)]`;
- отличие `String` от `&str`.

Что писать руками:

- создать `profile.rs`;
- перенести туда `SystemProfile`;
- подключить модуль из `main.rs`.

Проверка:

```bash
cargo check
```

### Этап 3. Сериализация в TOML на фиктивных данных

Результат: команда `export` создает `system.toml` с тестовыми данными.

Пример:

```toml
pacman_packages = ["neovim", "ripgrep"]
aur_packages = ["visual-studio-code-bin"]
shell = "/bin/zsh"
config_paths = ["~/.zshrc"]
system_services = []
user_services = []
```

Rust-темы:

- `serde`;
- `toml`;
- запись файла через `std::fs`;
- `Result`;
- оператор `?`;
- базовая обработка ошибок через `anyhow`.

Что писать руками:

- добавить зависимости `serde`, `toml`, `anyhow`;
- добавить `Serialize` и `Deserialize`;
- создать фиктивный `SystemProfile`;
- сохранить его в файл.

Проверка:

```bash
cargo run -- export --output system.toml
cargo check
```

### Этап 4. Чтение TOML

Результат: команда `check` читает `system.toml` и печатает загруженный профиль в отладочном виде.

Rust-темы:

- чтение файла;
- `Deserialize`;
- `Debug`;
- `Result`;
- передача пути в функцию;
- `PathBuf`.

Что писать руками:

- написать функцию чтения профиля;
- подключить ее к команде `check`;
- временно печатать структуру через `println!("{profile:#?}")`.

Проверка:

```bash
cargo run -- check system.toml
cargo check
```

### Этап 5. Чистая логика сравнения

Результат: появляется функция, которая сравнивает ожидаемый профиль и текущий профиль.

На этом этапе текущий профиль тоже фиктивный, не из системы.

Возможная модель результата:

```rust
struct ProfileDiff {
    missing_pacman_packages: Vec<String>,
    missing_aur_packages: Vec<String>,
    shell_diff: Option<ShellDiff>,
}
```

Rust-темы:

- функции;
- ссылки `&T`;
- заимствование;
- `HashSet`;
- разница между `Vec` и `HashSet`;
- возврат собственной структуры результата.

Что писать руками:

- создать `compare.rs`;
- написать функцию сравнения;
- не печатать внутри функции;
- возвращать данные, а не готовый текст.

Проверка:

```bash
cargo check
```

### Этап 6. Тесты для сравнения

Результат: появляются тесты для чистой логики сравнения.

Проверить нужно:

- пакет есть в профиле, но отсутствует в текущей системе;
- AUR-пакет отсутствует;
- shell отличается;
- если все совпадает, diff пустой.

Rust-темы:

- `#[test]`;
- `assert_eq!`;
- unit tests;
- подготовка тестовых данных;
- почему тестировать чистые функции проще.

Проверка:

```bash
cargo test
```

### Этап 7. Сбор текущего shell

Результат: `export` берет shell из переменной окружения `$SHELL`.

Rust-темы:

- `std::env::var`;
- `Option`;
- преобразование `Result` в `Option`;
- отсутствие переменной окружения.

Что писать руками:

- создать `collect.rs`;
- написать функцию получения shell;
- подключить ее к `export`.

Проверка:

```bash
cargo run -- export --output system.toml
```

### Этап 8. Запуск внешних команд

Результат: появляется функция, которая запускает команду и возвращает строки вывода.

Например:

```bash
pacman -Qqe
```

Rust-темы:

- `std::process::Command`;
- stdout/stderr;
- exit status;
- UTF-8 через `String::from_utf8`;
- обработка ошибок;
- почему нельзя использовать `unwrap`.

Функция должна возвращать `Result<Vec<String>>` и понятные ошибки.

Проверка:

```bash
cargo check
```

### Этап 9. Экспорт pacman и AUR пакетов

Результат: `export` реально собирает пакеты.

Команды:

```bash
pacman -Qqe
pacman -Qqm
```

Важная логика:

```text
pacman_packages = explicit_packages - aur_packages
aur_packages = foreign_packages
```

Rust-темы:

- `HashSet`;
- разность множеств;
- сортировка `Vec`;
- стабильный вывод;
- работа с системными командами.

Проверка:

```bash
cargo run -- export --output system.toml
cargo run -- check system.toml
cargo test
```

### Этап 10. Реальный `check` по пакетам и shell

Результат: `check` сравнивает профиль из файла с текущей системой и печатает отчет.

Пример:

```text
Missing pacman packages:
- neovim
- ripgrep

Missing AUR packages:
- visual-studio-code-bin

Different shell:
expected: /bin/zsh
current:  /bin/bash
```

Rust-темы:

- разделение логики и вывода;
- форматирование строк;
- работа с пустыми списками;
- функции, которые печатают результат.

Проверка:

```bash
cargo run -- check system.toml
cargo test
```

### Этап 11. Генерация безопасного plan

Результат: `plan` печатает команды, но ничего не выполняет.

Пример:

```bash
sudo pacman -S neovim ripgrep
# Install AUR packages manually:
# visual-studio-code-bin
chsh -s /bin/zsh
```

Rust-темы:

- генерация текста из структуры diff;
- условный вывод;
- безопасность CLI-инструментов;
- почему команда не должна выполнять изменения автоматически.

Рекомендация для AUR в MVP: печатать комментарии, а не использовать автоматически `paru` или `yay`.

Проверка:

```bash
cargo run -- plan system.toml
```

### Этап 12. Проверка конфигов по путям

Результат: `export` сохраняет только список существующих важных путей, а `check` показывает отсутствующие пути.

Пример:

```toml
config_paths = [
  "~/.zshrc",
  "~/.gitconfig",
  "~/.config/nvim",
]
```

Rust-темы:

- `PathBuf`;
- домашняя директория;
- проверка существования файла;
- отличие пути для хранения от реального пути;
- безопасность при работе с пользовательскими файлами.

Важно:

- не копировать содержимое конфигов;
- не читать приватные ключи;
- не обходить директории рекурсивно;
- только проверять наличие известных путей.

### Этап 13. Systemd services

Результат: профиль начинает содержать включенные systemd-сервисы.

Команды-кандидаты:

```bash
systemctl list-unit-files --state=enabled --type=service
systemctl --user list-unit-files --state=enabled --type=service
```

Rust-темы:

- парсинг табличного вывода;
- частичные ошибки;
- когда ошибка должна ломать программу, а когда только давать предупреждение;
- проектирование устойчивого CLI.

Важно: `systemctl --user` может не работать в некоторых окружениях, поэтому поведение нужно будет отдельно обсудить.

### Этап 14. Улучшение вывода

Результат: отчет становится аккуратнее и понятнее.

Пример:

```text
System audit report

Pacman packages:
  missing: 3

AUR packages:
  missing: 1

Shell:
  expected: /bin/zsh
  current:  /bin/bash
```

Rust-темы:

- форматирование CLI-вывода;
- функции отображения;
- отделение данных от представления.

### Этап 15. Документация

Результат: проектная документация поддерживается параллельно с разработкой.

Документировать нужно:

- зачем нужен проект;
- что делает каждая команда;
- формат `system.toml`;
- почему `plan` ничего не применяет;
- как устроены модули;
- какие Rust-темы уже изучены;
- важные архитектурные решения.

Документацию ведет агент по просьбе пользователя и после значимых этапов работы.

## MVP-0

Первый учебный результат:

- CLI с командами `export`, `check`, `plan`;
- `SystemProfile`;
- запись `system.toml` на фиктивных данных;
- чтение `system.toml`;
- сравнение двух фиктивных профилей;
- тесты для `compare.rs`.

Почему начинать с этого:

- можно изучить базу Rust без сложностей Linux-команд;
- быстрее появляется работающее ядро;
- чистую логику проще тестировать;
- реальные `pacman` и `$SHELL` подключаются позже к уже готовой модели.

## MVP-1

Первый полезный системный результат:

- `export` собирает shell;
- `export` собирает pacman/AUR пакеты;
- `check` сравнивает текущую систему с профилем;
- `plan` печатает безопасные команды.

## MVP-2

Расширенный аудит:

- `config_paths`;
- systemd services;
- улучшенный отчет;
- документация по архитектуре и формату профиля.

## Первый полезный результат

Минимальная версия считается практически полезной, когда можно выполнить:

```bash
arch-sync-audit export --output system.toml
arch-sync-audit check system.toml
```

И получить понятный отчет хотя бы по pacman/AUR-пакетам и shell.
