# Формат профиля системы

Профиль системы хранится в файле `system.toml`.

## Назначение

Файл описывает ожидаемое состояние эталонной Arch Linux системы. На другой машине программа читает этот файл и сравнивает его с текущим состоянием.

## Структура

```toml
pacman_packages = ["neovim", "ripgrep"]
aur_packages = ["visual-studio-code-bin"]
shell = "/bin/zsh"
config_paths = ["~/.zshrc"]
system_services = ["bluetooth.service"]
user_services = ["pipewire.service"]
```

## Поля

`pacman_packages`:

- явно установленные пакеты из официальных репозиториев;
- получаются из `pacman -Qqe` после исключения AUR-пакетов.

`aur_packages`:

- foreign packages из `pacman -Qqm`;
- не должны дублироваться в `pacman_packages`.

`shell`:

- текущий shell пользователя;
- берется из переменной окружения `$SHELL`;
- может отсутствовать, поэтому в Rust-модели это `Option<String>`.

`config_paths`:

- список важных путей, которые существуют на эталонной системе;
- в MVP хранится только факт наличия пути;
- содержимое файлов и директорий не сохраняется.
- текущий список кандидатов: `~/.zshrc`, `~/.gitconfig`, `~/.config/nvim`.

`system_services`:

- enabled systemd system services;
- собираются через `systemctl list-unit-files --state=enabled --type=service`;
- в профиль записываются имена unit-файлов, например `bluetooth.service`.

`user_services`:

- enabled systemd user services;
- собираются через `systemctl --user list-unit-files --state=enabled --type=service`;
- если пользовательский systemd недоступен, программа печатает warning и использует пустой список.

## Безопасность

Профиль не должен содержать:

- приватные ключи;
- токены;
- пароли;
- содержимое конфигов с секретами;
- дампы домашних директорий.

Профиль должен оставаться простым и безопасным: списки пакетов, shell, пути и имена enabled services без содержимого пользовательских файлов.
