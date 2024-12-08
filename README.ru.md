![alt text](./McDev_thin_900x70.png "McDev_thin_900x70.png")

[![Latest version](https://img.shields.io/crates/v/passgen-cmd.svg)](https://crates.io/crates/passgen-cmd)
[![Download](https://img.shields.io/crates/d/passgen-cmd.svg)](https://crates.io/crates/passgen-cmd)
[![docs.rs](https://docs.rs/passgen-cmd/badge.svg)](https://docs.rs/passgen-cmd/)
[![MIT](https://img.shields.io/badge/license-MIT-blue.svg)](https://choosealicense.com/licenses/mit/)
[![Build Status](https://github.com/mammothcoding/passgen-cmd/actions/workflows/rust.yml/badge.svg?event=push)](https://github.com/mammothcoding/passgen-cmd/actions/workflows/rust.yml)
[![made-with-rust](https://img.shields.io/badge/Made%20with-Rust-1f425f.svg)](https://www.rust-lang.org/)

Readme на разных языках:
[EN](https://github.com/mammothcoding/passgen-cmd/blob/master/README.md)
[RU](https://github.com/mammothcoding/passgen-cmd/blob/master/README.ru.md)

# 💻 Passgen-cmd

Корссплатформенная утилита для генерации криптографически защищенных паролей/токенов и других наборов и последовательностей.

Используются [CSPRNGs](https://rust-random.github.io/book/guide-rngs.html#cryptographically-secure-pseudo-random-number-generators-csprngs) Isaac64Rng и Hc128Rng.

## Устновка
```bash
cargo install passgen-cmd
```

## Использование

#### Вывести помощь
```bash
passgen-cmd -h
```
### Примеры
#### Создать стойкий токен, включающий все летералы, цифры и специальные символы длиной 30 символов:
```bash
passgen-cmd 30
```
#### Создать стойкий и удобный пароль с длиной по умолчанию 8 символов:
```bash
passgen-cmd -S
```
#### Сгенерировать криптостойкую случайную строку из вашего набора символов длиной 12:
```bash
passgen-cmd 12 -c bla@.321
```
#### Сгенерировать токен на подобии токенов Telegram (1-ая часть: 10 цифр, 2-ая: 30 символов из набора всех летералов и цифр ) [unix-like]:
```bash
echo $(passgen-cmd 10 -n)":"$(passgen-cmd 35 -lLn)
```
#### Сгенерировать UUID (паттерн: 8-4-4-4-4-12 из шестнадцатеричного набора) [unix-like]:
```bash
echo $(passgen-cmd 8 -c 0123456789abcdef)"-"$(passgen-cmd 4 -c 0123456789abcdef)"-"$(passgen-cmd 4 -c 0123456789abcdef)"-"$(passgen-cmd 4 -c 0123456789abcdef)"-"$(passgen-cmd 12 -c 0123456789abcdef)
```

## Загрузки
[Собранные релизы](https://github.com/mammothcoding/passgen-cmd/releases/)

## Лицензия
[MIT](https://choosealicense.com/licenses/mit/)