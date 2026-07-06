# AUTOPILOT — Автономная сборка Rust4ui

Ты выполняешь проект от начала до конца по чек-листу. Один запрос разрешений в начале — дальше без подтверждений.

## Возобновление после обрыва сессии

Если ты попал сюда после того, как сессия оборвалась:

1. Прочитай `autopilot_state.json`
2. Если файла нет — запроси разрешения (Фаза 0), затем начни с Фазы 1
3. Если файл есть:
   - Найди первый шаг со status `"pending"` или `"in_progress"`
   - Если `"in_progress"` — проверь, что реально сделано (cargo test по этому шагу)
   - Продолжи с этого шага, не спрашивая разрешений (они уже были даны)
4. Если все шаги `"done"` → сообщи: «Проект собран. Все тесты пройдены.»

**Промпт для возобновления (вставить в новую сессию):**

```
Прочитай AUTOPILOT.md. Найди autopilot_state.json и продолжи автономную сборку с того шага, на котором остановились.
```

## Как это работает

**Файл прогресса:** `autopilot_state.json`.

После каждого завершённого шага ты обновляешь state, чтобы при обрыве сессии прогресс не потерялся.

```json
{
  "current": "2.1",
  "steps": {
    "1.1": { "status": "done" },
    "1.2": { "status": "done", "test": true },
    "2.1": { "status": "in_progress" },
    "2.2": { "status": "pending" }
  }
}
```

**Цикл каждого шага:**

```
1. Реализуй
2. Напиши тесты (навык rust-testing)
3. cargo test -- имя_теста
   ├── красные? → исправляй → goto 3
   └── зелёные? → пометь step как done в state → следующий пункт
4. cargo fmt (после каждых 3 шагов)
```

**При новом запуске (сессия началась заново):**

- Прочитать `autopilot_state.json`
- Если нет → Фаза 0 (запрос разрешений)
- Найти первый step со status `"pending"` или `"in_progress"`
- Если все `"done"` → доложить: «Проект собран. `cargo build` ✓, `cargo test` ✓»
- Если уже какие-то шаги пройдены (есть Cargo.toml, src/lib.rs, но state.json нет) → определить текущее состояние и продолжить

---

## Фаза 0 — Запрос разрешений (один раз, до старта)

До начала работы выдай пользователю ОДНО сообщение:

> Для автономной сборки Rust4ui мне понадобятся разрешения:
>
> - Создание и редактирование файлов (.rs, .json, .toml)
> - Выполнение cargo build, cargo test, cargo clippy, cargo fmt
> - Чтение токена из `%USERPROFILE%\.github_token`
> - Выполнение git add, git commit, git push
> - Загрузка внешних ресурсов (шрифты)
>
> Подтверждаешь?

После «да» — создать `autopilot_state.json`, начать Фазу 1. До слова «Готово» — ни одного запроса подтверждения.

---

## Фаза 1 — Cargo-проект

- [ ] **1.1** `Cargo.toml` с зависимостями: egui 0.32, eframe 0.32, serde + serde_json, dirs
  - Тест: `cargo build`

---

## Фаза 2 — Ядро

- [ ] **2.1** `UiNode` — базовый тип, парсинг JSON (`serde_json::from_str`), доступ к атрибутам
  - Тест: `test_parse_simple_node`

- [ ] **2.2** Система `$ref` — рекурсивный резолвер с кэшем, обнаружение циклов
  - Тест: `test_ref_simple`, `test_ref_nested`, `test_ref_cycle`

- [ ] **2.3** Theme — загрузка theme.json, merge дефолтов, приоритет: ui.json > theme > egui-дефолт
  - Тест: `test_theme_priority`, `test_theme_missing`

- [ ] **2.4** StateRegistry — HashMap binding, типы: String, f64, i64, usize, bool, Vec<String>
  - Тест: `test_state_bind_read_write`, `test_state_wrong_type`

- [ ] **2.5** ActionRegistry — вызов по имени, ctx.target(), ctx.app::<T>()
  - Тест: `test_action_invoke`, `test_action_target`

- [ ] **2.6** LocaleRegistry — загрузка locale JSON, резолв {{key}}, {expr}-интерполяция из state
  - Тест: `test_locale_resolve`, `test_locale_fallback_en`

- [ ] **2.7** CLDR plural-правила — ru, en, de, fr, pl, uk, be, ja, zh, ko
  - Тест: `test_plural_ru_all_forms`, `test_plural_en`

- [ ] **2.8** Pre-flight валидатор — проверка type, binding, action, items, {{key}}, типов атрибутов
  - Тест: `test_validate_valid_json`, `test_validate_missing_binding`

---

## Фаза 3 — Рендерер + v0.1 виджеты

- [ ] **3.1** Renderer — диспатч по `type`, рекурсивный обход JSON, вызов egui
  - Тест: smoke на Column с одним Label

- [ ] **3.2** Label — рендеринг, size, color, bold, italic, monospace, wrap
  - Тест: smoke + unit на атрибуты

- [ ] **3.3** Button — рендеринг, fill, rounding, min_width, action, tooltip
  - Тест: smoke + unit на clicked и change

- [ ] **3.4** TextField — singleline, password, binding → String
  - Тест: smoke + unit

- [ ] **3.5** Checkbox — binding → bool
  - Тест: smoke + unit

- [ ] **3.6** Separator — разделитель, space
  - Тест: smoke

- [ ] **3.7** Column — ui.vertical(), gap, padding, align
  - Тест: smoke + unit

- [ ] **3.8** Row — ui.horizontal(), gap, padding, wrap, align
  - Тест: smoke + unit

---

## Фаза 4 — Виджеты v0.2

- [ ] **4.1** RadioGroup — binding → usize, options, direction
  - Тест: smoke + unit

- [ ] **4.2** Slider — binding → f64, min, max, step
  - Тест: smoke + unit

- [ ] **4.3** ComboBox — binding → usize, items → Vec<String>
  - Тест: smoke + unit

- [ ] **4.4** Tabs / Tab — переключение вкладок, enabled
  - Тест: smoke + unit

- [ ] **4.5** Panel — fill, rounding, stroke_width, stroke_color, padding
  - Тест: smoke + unit

- [ ] **4.6** ScrollArea — axis, max_height, max_width
  - Тест: smoke + unit

- [ ] **4.7** TextField mode=number — min, max, step, decimals, hover-stepper, scroll
  - Тест: smoke + unit

- [ ] **4.8** Window — базовый + modal, anchor, id-persist, title_bar, show_close
  - Тест: smoke + unit

---

## Фаза 5 — Системные виджеты

- [ ] **5.1** Spinner — size, color, text
  - Тест: smoke + unit

- [ ] **5.2** Shortcut — глобальная горячая клавиша + атрибут shortcut на Button
  - Тест: smoke + unit

- [ ] **5.3** ColorPicker — binding → String hex, alpha
  - Тест: smoke

- [ ] **5.4** FileDrop — accept, multi, highlight_color, action
  - Тест: smoke

- [ ] **5.5** Indicator — color, size, pulse
  - Тест: smoke

- [ ] **5.6** StatusBar — height, fill, children с anchor
  - Тест: smoke

- [ ] **5.7** IconBar + IconButton + Caption — vertical/horizontal, anchor: start/center/end/fill
  - Тест: smoke

- [ ] **5.8** Toast / Notifications — зона + ctx.notify() (4 уровня)
  - Тест: smoke

- [ ] **5.9** Context Menu — context_menu на любом виджете, MenuItem, SubMenu, shortcut, icon
  - Тест: smoke + unit

- [ ] **5.10** MenuBar + Menu — горизонтальная панель меню
  - Тест: smoke

---

## Фаза 6 — Интеграция + демо

- [ ] **6.1** Загрузка demo/ui.json (резолв $ref) — 5 вкладок + 3 окна рендерятся без ошибок
  - Тест: `cargo test --test integration`

- [ ] **6.2** Smoke-тест — каждый виджет изолированно, без паники
  - Тест: `cargo test --test smoke`

- [ ] **6.3** Persistence-тест — save → reload → restore
  - Тест: `cargo test --test persistence`

- [ ] **6.4** Encoding-тест — все файлы UTF-8 без BOM
  - Тест: `cargo test --test encoding`

---

## Фаза 7 — Публикация и финализация

- [ ] **7.1** `cargo test` — все тесты зелёные
- [ ] **7.2** `cargo build` — без ошибок
- [ ] **7.3** `cargo clippy` — без warnings (или допустить warnings в пределах разумного)
- [ ] **7.4** `cargo fmt` — форматирование
- [ ] **7.5** `git push` — запушено на GitHub
- [ ] **7.6** Доложить пользователю: «Готово. Проект собран, тесты пройдены, запушен.»
