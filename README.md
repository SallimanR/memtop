# `memtop` — утилита для мониторинга процессов в Linux

В отличие от аналогов, показывающих только RSS (общая физическая память), memtop также выводит PSS (пропорциональное распределение — разделяемая память делится поровну между процессами), давая реалистичную картину потребления памяти.

Материалы для изучения:
[статья о метриках памяти в Linux](https://blog.blackwell-systems.com/posts/memory-taxonomy/)

## Режим дерева процессов
<p align="center">
  <img src="./assets/process-tree.png" alt="Дерево процессов" width="1000">
</p>

![white](https://img.shields.io/badge/White-gray) — пользовательские процессы  
 └ ![blue](https://img.shields.io/badge/Blue-blue) — потоки процессов  
![green](https://img.shields.io/badge/Green-green) — потоки ядра Linux

## Архитектура
Многопоточная архитектура с разделением ответственности:

- **Поток сбора данных** — читает `/proc/<pid>/smaps_rollup`, `/proc/<pid>/comm`, `/proc/<pid>/status` и другую статистику, парсит и отправляет через канал
- **Главный поток** — получает данные через `mpsc::channel`, обрабатывает клавиши пользователя (навигация, выход),
затем обновляет состояние (только при наличии изменений для экономии ресурсов) и рендерит интерфейс через Ratatui

Синхронизация между потоками — через Multiple Producer, Single Consumer канал `mpsc::channel::<UpdateInfo>()`.
Каждый поток работает независимо, без блокировок, что обеспечивает отзывчивый интерфейс даже при интенсивном чтении `/proc`.

Профилирование производительности через Tracy и монитор потребления ресурсов.

## Разработка

Запуск без оптимизаций:
```sh
cargo run
```
Релизная сборка:
```sh
cargo build --release
```

Профилирование через Tracy:
1. Сборка с флагом профилирования:
```sh
cargo build --profile dev
```
2. Запуск:
```sh
./target/release-profile/memtop
```

3. Собрать Tracy profiler версии 13.1
4. Запустить Tracy → подключиться к запущенному memtop
<img src="./assets/profiling-tracy.png" alt="Профилирование через Tracy" width="1000">
Просмотр статистики использования:
<img src="./assets/profiling-tracy-statistics.png" width="800">


# ENGLISH:

# `memtop` is utility to monitor Linux and system processes

Unlike other tools that only show RSS (total physical memory), memtop also displays PSS (proportional — shared memory divided equally among processes), giving a realistic view of memory consumption.

Learning:
[cool blog post about memory metrics in Linux](https://blog.blackwell-systems.com/posts/memory-taxonomy/)

## Process tree mode
<p align="center">
  <img src="./assets/process-tree.png" alt="Process tree" width="1000">
</p>

![white](https://img.shields.io/badge/White-gray) ones are user space processes  
 └ ![blue](https://img.shields.io/badge/Blue-blue) ones are process' threads  
![green](https://img.shields.io/badge/Green-green) ones are kernel threads

## Architecture
Multi-threaded architecture with separation of concerns:

- **Data collection thread** — reads `/proc/<pid>/smaps_rollup`, `/proc/<pid>/comm`, `/proc/<pid>/status` and other statistics, parses them and sends through a channel
- **Main thread** — receives data via `mpsc::channel`, handles user input (navigation, exit), then updates state (only on changes to save resources) and renders the interface via Ratatui

Synchronization between threads — via a Multiple Producer, Single Consumer channel `mpsc::channel::<UpdateInfo>()`.
Each thread runs independently, without locks, ensuring a responsive interface even under intensive `/proc` reads.

Performance profiling via Tracy and resource usage monitor.

## Developing

run without optimizations:
```sh
cargo run
```

build in release mode:
```sh
cargo build --release
```

profiling with Tracy tool
1. build with profile feature flag:
```sh
cargo build --profile dev
```
2. run
```sh
./target/release-profile/memtop
```

3. build tracy profiler with specific version (13.1)
4. run tracy -> connect to running `memtop`

<img src="./assets/profiling-tracy.png" alt="Profiling with tracy" width="1000">

View statistics of usage
<img src="./assets/profiling-tracy-statistics.png" width="800">

