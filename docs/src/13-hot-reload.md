# Hot-reload

## Как это работает

Приложение запускает файловый watcher (крейт `notify`) в отдельном потоке. Watcher следит за папкой `demo/` и её подпапками.

```
start_file_watcher(demo_path, changed_flag)
    │
    └─ notify::RecommendedWatcher
         └─ on file change → set atomic flag
                │
    main loop:  │
    update() ───┘ check flag
         │
         ├─ reload ui.json → перепарсить, разрешить $ref
         ├─ reload theme.json → обновить тему
         └─ проверить контрасты
```

## Какие файлы отслеживаются

Watcher рекурсивно следит за всеми файлами в `demo/`:

- `demo/ui.json` — корневой UI-файл
- `demo/theme.json` — тема
- Все файлы, подключенные через `$ref` (например, `demo/tabs/main.json`)
- Любые `.json` файлы в подпапках `demo/`

## Что перезагружается

При изменении любого файла в `demo/`:

1. **UI дерево** — `ui.json` парсится заново, все `$ref` резолвятся
2. **Тема** — `theme.json` перечитывается, обновляется `RenderCtx.theme`
3. **Контрасты** — проверка WCAG контрастов новой темы

Состояние (`StateRegistry`) **не сбрасывается** при hot-reload — binding-значения сохраняются.

## Код watcher'а

```rust
fn start_file_watcher(demo_path: PathBuf, changed: Arc<AtomicBool>) {
    std::thread::spawn(move || {
        let (tx, rx) = std::sync::mpsc::channel();
        let mut watcher = RecommendedWatcher::new(tx, Config::default()).unwrap();
        watcher.watch(&demo_path, RecursiveMode::Recursive).unwrap();

        for event in rx {
            if let Ok(ev) = event {
                if matches!(ev.kind, EventKind::Modify(_) | EventKind::Create(_)) {
                    changed.store(true, Ordering::Relaxed);
                }
            }
        }
    });
}
```

## Код перезагрузки

```rust
fn reload_ui_tree(&mut self) {
    let content = std::fs::read_to_string(&ui_path)?;
    let root: Value = serde_json::from_str(&strip_json_comments(&content))?;
    let mut resolver = RefResolver::new();
    self.tree = resolver.resolve(&root, &self.base.join("demo"))?;
}
```

## Цикл проверки

```rust
fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    if self.files_changed.swap(false, Ordering::Relaxed) {
        log::info!("Обнаружено изменение файлов, hot-reload...");
        self.reload_ui_tree();
        self.ctx.theme = load_theme(&self.base);
    }
    // ... рендеринг ...
    ctx.request_repaint_after(Duration::from_millis(100));
}
```

## Замечания

- Watcher использует `notify` крейт с `Config::default()` (best-effort)
- На Windows использует `ReadDirectoryChanges` API
- На Linux — `inotify`
- Изменения, произошедшие быстрее чем за ~3мс (между проверками флага), не теряются благодаря атомарному флагу
- Если `demo/theme.json` не существует, используется встроенная дефолтная тема (без ошибки)

## Ограничения

- Watcher не следит за `locales/` — для перезагрузки локалей нужен ручной рестарт (планируется)
- При ошибке парсинга `ui.json` старое дерево сохраняется
- При ошибке парсинга `theme.json` старая тема сохраняется
