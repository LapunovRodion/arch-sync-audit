# Команды CLI

Документ описывает ожидаемое поведение команд `arch-sync-audit`.

## `export`

```bash
arch-sync-audit export --output system.toml
```

Назначение: собрать профиль текущей системы и сохранить его в файл.

Текущее поведение MVP: команда собирает реальные данные текущей системы:

- pacman-пакеты;
- AUR-пакеты;
- текущий shell;
- известные config paths;
- enabled systemd system services;
- enabled systemd user services, если `systemctl --user` доступен.

`config_paths` собираются только из заранее известных путей. Программа проверяет факт существования пути, но не читает содержимое файлов и директорий.

Если `systemctl --user` недоступен, команда продолжает работу, печатает warning в stderr и сохраняет пустой список `user_services`.

## `check`

```bash
arch-sync-audit check system.toml
```

Назначение: прочитать ожидаемый профиль из файла, собрать текущий профиль системы и вывести различия.

Команда сравнивает профиль из файла с реальным текущим состоянием системы. Отчет показывает количество отсутствующих элементов и сами отсутствующие элементы по секциям:

- pacman packages;
- AUR packages;
- config paths;
- system services;
- user services;
- shell.

Пример фрагмента отчета:

```text
System audit report

Pacman packages:
  missing: 0

AUR packages:
  missing: 1
  - visual-studio-code-bin

Shell:
  expected: /bin/zsh
  current:  /bin/fish
```

## `plan`

```bash
arch-sync-audit plan system.toml
```

Назначение: показать команды или подсказки, которые пользователь может выполнить вручную.

Команда читает профиль, собирает текущее состояние системы, строит diff и превращает его в безопасный текстовый план.

Команда `plan` не должна:

- устанавливать пакеты;
- менять shell;
- включать systemd services;
- редактировать конфиги;
- выполнять команды автоматически.

Для AUR-пакетов на этапе MVP безопаснее печатать комментарии, а не использовать `paru` или `yay` автоматически.

Для systemd services команда тоже печатает комментарии, а не выполняет `systemctl enable`.
