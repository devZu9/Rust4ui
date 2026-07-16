use crate::theme::Theme;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BorderType {
    Solid,
    Dash,
    Dot,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BorderPosition {
    Inside,
    Center,
    Outside,
}

#[derive(Debug, Clone, Copy)]
pub struct BorderStyle {
    pub width: f32,
    pub color: egui::Color32,
    pub border_type: BorderType,
    pub gap: f32,
    pub seg_len: f32,
    pub round_cap: bool,
    pub position: BorderPosition,
}

impl BorderStyle {
    pub fn is_visible(&self) -> bool {
        self.width > 0.0 && self.color.a() > 0
    }

    pub fn gap_len(&self) -> f32 {
        self.gap
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ShadowZOrder {
    Under,
    Over,
}

#[derive(Debug, Clone, Copy)]
pub struct Shadow {
    pub color: egui::Color32,
    pub offset: egui::Vec2,
    pub z_order: ShadowZOrder,
}

impl Shadow {
    pub fn is_visible(&self) -> bool {
        self.color.a() > 0
    }

    pub fn transparent() -> Self {
        Shadow { color: egui::Color32::TRANSPARENT, offset: egui::Vec2::ZERO, z_order: ShadowZOrder::Under }
    }

    pub fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Shadow {
            color: egui::Color32::from_rgba_unmultiplied(r, g, b, a),
            offset: egui::vec2(2.0, 2.0),
            z_order: ShadowZOrder::Under,
        }
    }
}

fn parse_shadow_impl(val: &serde_json::Value, default_xy: f32) -> Option<Shadow> {
    match val {
        serde_json::Value::Number(n) => {
            let opacity = n.as_f64()? as f32;
            let a = (opacity.clamp(0.0, 1.0) * 255.0) as u8;
            Some(Shadow { color: egui::Color32::from_rgba_unmultiplied(0, 0, 0, a), offset: egui::Vec2::ZERO, z_order: ShadowZOrder::Under })
        }
        serde_json::Value::Array(arr) if arr.len() >= 1 => {
            let opacity = arr[0].as_f64()? as f32;

            let z_order = if arr.len() >= 2 {
                match arr[1].as_str()? {
                    "over" => ShadowZOrder::Over,
                    _ => ShadowZOrder::Under,
                }
            } else {
                ShadowZOrder::Under
            };

            let base_color = if arr.len() >= 3 {
                arr[2].as_str().and_then(crate::theme::parse_hex_color).unwrap_or(egui::Color32::BLACK)
            } else {
                egui::Color32::BLACK
            };
            let a = (opacity.clamp(0.0, 1.0) * 255.0) as u8;
            let color = egui::Color32::from_rgba_unmultiplied(base_color.r(), base_color.g(), base_color.b(), a);

            let x = if arr.len() >= 4 { arr[3].as_f64()? as f32 } else { default_xy };
            let y = if arr.len() >= 5 { arr[4].as_f64()? as f32 } else { default_xy };
            Some(Shadow { color, offset: egui::vec2(x, y), z_order })
        }
        _ => None,
    }
}

/// Парсит тень фона/рамки. Default offset (2,2).
/// Форматы:
/// - число: `0.4` → чёрная тень, offset (2,2), z_order Under
/// - массив: `[opacity]` / `[opacity, "under"|"over"]` / `[opacity, "under"|"over", "#color", x, y]`
pub fn parse_shadow(val: &serde_json::Value) -> Option<Shadow> {
    parse_shadow_impl(val, 2.0)
}

/// Парсит тень контента/иконки/текста. Default offset (1,1).
pub fn parse_content_shadow(val: &serde_json::Value) -> Option<Shadow> {
    parse_shadow_impl(val, 1.0)
}

/// Рисует тень от фона (rect_filled).
pub fn draw_shadow_bg(ui: &egui::Ui, rect: egui::Rect, rounding: egui::CornerRadius, shadow: &Shadow) {
    if !shadow.is_visible() { return; }
    let r = rect.translate(shadow.offset);
    ui.painter().rect_filled(r, rounding, shadow.color);
}

/// Рисует тень от иконки/текста (galley).
pub fn draw_shadow_content(ui: &egui::Ui, pos: egui::Pos2, galley: std::sync::Arc<egui::Galley>, shadow: &Shadow) {
    if !shadow.is_visible() { return; }
    ui.painter().galley_with_override_text_color(pos + shadow.offset, galley, shadow.color);
}

/// Рисует тень от границы — повторяет dash/dot/gap/seg_len, только другим цветом.
pub fn draw_shadow_border(ui: &mut egui::Ui, rect: egui::Rect, rounding: egui::CornerRadius, border: &BorderStyle, shadow: &Shadow) {
    if !shadow.is_visible() || border.width <= 0.0 { return; }
    let shadow_border = BorderStyle {
        color: shadow.color,
        width: border.width,
        border_type: border.border_type,
        gap: border.gap,
        seg_len: border.seg_len,
        round_cap: border.round_cap,
        position: border.position,
    };
    draw_border(ui, rect.translate(shadow.offset), rounding, &shadow_border);
}

/// Парсит border из узла и темы.
/// Приоритет: 1. явные ключи на узле  2. border-массив на узле
///            3. явные ключи в теме    4. border-массив в теме   5. дефолт (0)
pub fn get_border(node: &serde_json::Value, theme: &Theme, widget: &str) -> BorderStyle {
    let w = node.get("border_width")
        .and_then(|v| v.as_f64())
        .or_else(|| shorthand_width(node))
        .or_else(|| theme_widget_f64(theme, widget, "border_width"))
        .or_else(|| theme_shorthand_width(theme, widget))
        .unwrap_or(0.0) as f32;

    let c = node.get("border_color")
        .and_then(crate::theme::parse_color_value)
        .or_else(|| shorthand_color(node))
        .or_else(|| theme_widget_color(theme, widget, "border_color"))
        .or_else(|| theme_shorthand_color(theme, widget))
        .unwrap_or(egui::Color32::from_rgb(0x44, 0x44, 0x55));

    let t = node_str(node, "border_type")
        .or_else(|| theme.widget.get(widget)
            .and_then(|w| w.get("border_type"))
            .and_then(|v| v.as_str()))
        .or_else(|| shorthand_type(node))
        .or_else(|| theme_shorthand_type(theme, widget))
        .and_then(parse_border_type)
        .unwrap_or(BorderType::Solid);

    let is_dash = matches!(t, BorderType::Dash);
    let default_gap = if is_dash { w * 3.0 } else { w * 2.0 };
    let default_seg = if is_dash { w * 4.0 } else { w };

    let g = node.get("border_gap")
        .and_then(|v| v.as_f64())
        .or_else(|| shorthand_gap(node))
        .or_else(|| theme_widget_f64(theme, widget, "border_gap"))
        .or_else(|| theme_shorthand_gap(theme, widget))
        .unwrap_or(default_gap as f64) as f32;

    let s = node.get("border_seg_len")
        .and_then(|v| v.as_f64())
        .or_else(|| shorthand_seg_len(node))
        .or_else(|| theme_widget_f64(theme, widget, "border_seg_len"))
        .or_else(|| theme_shorthand_seg_len(theme, widget))
        .unwrap_or(default_seg as f64) as f32;

    let rc = node.get("border_seg_cap")
        .and_then(|v| v.as_bool())
        .or_else(|| theme.widget.get(widget)
            .and_then(|w| w.get("border_seg_cap"))
            .and_then(|v| v.as_bool()))
        .unwrap_or(true);

    let bp = node.get("border_position")
        .and_then(|v| v.as_str())
        .or_else(|| theme.widget.get(widget)
            .and_then(|w| w.get("border_position"))
            .and_then(|v| v.as_str()))
        .and_then(parse_border_position)
        .unwrap_or(BorderPosition::Inside);

    BorderStyle { width: w, color: c, border_type: t, gap: g, seg_len: s, round_cap: rc, position: bp }
}

/// Применяет state-specific border (border_hover, border_click) поверх базового.
/// suffix = "hover" или "click". Читает ключ border_{suffix} как шортхенд-массив.
pub fn apply_state_border(node: &serde_json::Value, theme: &Theme, widget: &str,
                          suffix: &str, base: &BorderStyle) -> BorderStyle {
    let key = format!("border_{}", suffix);
    let val = node.get(&key)
        .or_else(|| theme.widget.get(widget).and_then(|w| w.get(&key)));
    let arr: &[serde_json::Value] = match val {
        Some(serde_json::Value::Array(a)) => a,
        Some(serde_json::Value::Number(_)) => {
            let mut r = *base;
            if let Some(n) = val.and_then(|v| v.as_f64()) { r.width = n as f32; }
            return r;
        }
        _ => return *base,
    };

    let mut r = *base;
    if arr.len() >= 1 { if let Some(n) = arr[0].as_f64() { r.width = n as f32; } }
    if arr.len() >= 2 { if let Some(c) = crate::theme::parse_color_value(&arr[1]) { r.color = c; } }
    let has_op = arr.len() >= 3 && arr[2].as_f64().is_some();
    let off = if has_op { 1 } else { 0 };
    if has_op {
        if let Some(o) = arr[2].as_f64() {
            let a = (r.color.a() as f32 * o.clamp(0.0, 1.0) as f32) as u8;
            r.color = egui::Color32::from_rgba_unmultiplied(r.color.r(), r.color.g(), r.color.b(), a);
        }
    }
    if arr.len() >= (3 + off) { if let Some(s) = arr[2 + off].as_str() { if let Some(t) = parse_border_type(s) { r.border_type = t; } } }
    if arr.len() >= (4 + off) { if let Some(n) = arr[3 + off].as_f64() { r.gap = n as f32; } }
    if arr.len() >= (5 + off) { if let Some(n) = arr[4 + off].as_f64() { r.seg_len = n as f32; } }
    r
}

/// Единый entry point для state-зависимого бордера.
/// Выбирает border_hover / border_click / border по resp.
pub fn get_state_border(node: &serde_json::Value, theme: &Theme, widget: &str,
                        resp: &egui::Response, enabled: bool) -> BorderStyle {
    let base = get_border(node, theme, widget);
    if !enabled { return base; }
    if resp.is_pointer_button_down_on() {
        apply_state_border(node, theme, widget, "click", &base)
    } else if resp.has_focus() {
        apply_state_border(node, theme, widget, "focus", &base)
    } else if resp.hovered() {
        apply_state_border(node, theme, widget, "hover", &base)
    } else {
        base
    }
}

// -- shorthand border: [width] / [width, "#color"] / [width, "#color", "type"] --

fn shorthand_width(node: &serde_json::Value) -> Option<f64> {
    match node.get("border")? {
        serde_json::Value::Number(n) => n.as_f64(),
        serde_json::Value::Array(arr) => arr.first().and_then(|v| v.as_f64()),
        _ => None,
    }
}

fn shorthand_color(node: &serde_json::Value) -> Option<egui::Color32> {
    let arr = node.get("border")?.as_array()?;
    if arr.len() < 2 { return None; }
    let mut c = crate::theme::parse_color_value(&arr[1])?;
    if arr.len() >= 3 {
        if let Some(o) = arr[2].as_f64() {
            let a = (c.a() as f32 * o.clamp(0.0, 1.0) as f32) as u8;
            c = egui::Color32::from_rgba_unmultiplied(c.r(), c.g(), c.b(), a);
        }
    }
    Some(c)
}

fn shorthand_type(node: &serde_json::Value) -> Option<&str> {
    let arr = node.get("border")?.as_array()?;
    let off = if arr.len() >= 3 && arr[2].as_f64().is_some() { 1 } else { 0 };
    if arr.len() < (3 + off) { return None; }
    arr[2 + off].as_str()
}

// -- helpers для чтения из theme.widget --

fn theme_widget_f64(theme: &Theme, widget: &str, key: &str) -> Option<f64> {
    theme.widget.get(widget)?.get(key)?.as_f64()
}

fn theme_widget_color(theme: &Theme, widget: &str, key: &str) -> Option<egui::Color32> {
    crate::theme::parse_color_value(theme.widget.get(widget)?.get(key)?)
}

fn theme_shorthand_width(theme: &Theme, widget: &str) -> Option<f64> {
    let val = theme.widget.get(widget)?.get("border")?;
    match val {
        serde_json::Value::Number(n) => n.as_f64(),
        serde_json::Value::Array(arr) => arr.first().and_then(|v| v.as_f64()),
        _ => None,
    }
}

fn theme_shorthand_color(theme: &Theme, widget: &str) -> Option<egui::Color32> {
    let arr = theme.widget.get(widget)?.get("border")?.as_array()?;
    if arr.len() < 2 { return None; }
    let mut c = crate::theme::parse_color_value(&arr[1])?;
    if arr.len() >= 3 {
        if let Some(o) = arr[2].as_f64() {
            let a = (c.a() as f32 * o.clamp(0.0, 1.0) as f32) as u8;
            c = egui::Color32::from_rgba_unmultiplied(c.r(), c.g(), c.b(), a);
        }
    }
    Some(c)
}

fn shorthand_gap(node: &serde_json::Value) -> Option<f64> {
    let arr = node.get("border")?.as_array()?;
    let off = if arr.len() >= 3 && arr[2].as_f64().is_some() { 1 } else { 0 };
    if arr.len() < (4 + off) { return None; }
    arr[3 + off].as_f64()
}

fn theme_shorthand_gap(theme: &Theme, widget: &str) -> Option<f64> {
    let arr = theme.widget.get(widget)?.get("border")?.as_array()?;
    let off = if arr.len() >= 3 && arr[2].as_f64().is_some() { 1 } else { 0 };
    if arr.len() < (4 + off) { return None; }
    arr[3 + off].as_f64()
}

fn shorthand_seg_len(node: &serde_json::Value) -> Option<f64> {
    let arr = node.get("border")?.as_array()?;
    let off = if arr.len() >= 3 && arr[2].as_f64().is_some() { 1 } else { 0 };
    if arr.len() < (5 + off) { return None; }
    arr[4 + off].as_f64()
}

fn theme_shorthand_seg_len(theme: &Theme, widget: &str) -> Option<f64> {
    let arr = theme.widget.get(widget)?.get("border")?.as_array()?;
    let off = if arr.len() >= 3 && arr[2].as_f64().is_some() { 1 } else { 0 };
    if arr.len() < (5 + off) { return None; }
    arr[4 + off].as_f64()
}

fn theme_shorthand_type<'a>(theme: &'a Theme, widget: &str) -> Option<&'a str> {
    let arr = theme.widget.get(widget)?.get("border")?.as_array()?;
    let off = if arr.len() >= 3 && arr[2].as_f64().is_some() { 1 } else { 0 };
    if arr.len() < (3 + off) { return None; }
    arr[2 + off].as_str()
}

fn node_str<'a>(node: &'a serde_json::Value, key: &str) -> Option<&'a str> {
    node.get(key)?.as_str()
}

fn parse_border_position(s: &str) -> Option<BorderPosition> {
    match s {
        "inside" => Some(BorderPosition::Inside),
        "center" => Some(BorderPosition::Center),
        "outside" => Some(BorderPosition::Outside),
        _ => None,
    }
}

fn parse_border_type(s: &str) -> Option<BorderType> {
    match s {
        "solid" => Some(BorderType::Solid),
        "dash" | "dashed" => Some(BorderType::Dash),
        "dot" | "dotted" => Some(BorderType::Dot),
        _ => None,
    }
}

/// Рисует border вокруг rect с заданным скруглением.
pub fn draw_border(
    ui: &mut egui::Ui,
    rect: egui::Rect,
    rounding: egui::CornerRadius,
    border: &BorderStyle,
) {
    if !border.is_visible() { return; }

    match border.border_type {
        BorderType::Solid => {
            let kind = match border.position {
                BorderPosition::Inside => egui::StrokeKind::Inside,
                BorderPosition::Center => egui::StrokeKind::Middle,
                BorderPosition::Outside => egui::StrokeKind::Outside,
            };
            ui.painter().rect_stroke(
                rect, rounding,
                egui::Stroke::new(border.width, border.color),
                kind,
            );
        }
        BorderType::Dash | BorderType::Dot => {
            let inset = match border.position {
                BorderPosition::Inside => border.width * 0.5,
                BorderPosition::Center => 0.0,
                BorderPosition::Outside => -border.width * 0.5,
            };
            let r = rect.shrink(inset);
            draw_pattern(ui, r, rounding, border, border.border_type == BorderType::Dash);
        }
    }
}

fn draw_pattern(
    ui: &mut egui::Ui,
    rect: egui::Rect,
    rounding: egui::CornerRadius,
    border: &BorderStyle,
    is_dash: bool,
) {
    let arc_steps = 24;
    let pts = rounded_rect_perimeter(rect, rounding, arc_steps);
    if pts.len() < 2 { return; }

    let seg_len = border.seg_len;
    let step = seg_len + border.gap;
    if step <= 0.0 { return; }

    let mut dists: Vec<f32> = Vec::with_capacity(pts.len());
    dists.push(0.0);
    for w in pts.windows(2) {
        dists.push(dists.last().copied().unwrap_or(0.0) + w[0].distance(w[1]));
    }
    if pts.len() >= 2 {
        dists.push(dists.last().copied().unwrap_or(0.0) + pts.last().unwrap().distance(pts[0]));
    }
    let total = *dists.last().unwrap_or(&0.0);
    if total <= 0.0 { return; }

    let stroke = egui::Stroke::new(border.width, border.color);

    // Подогнанный шаг — все gap'ы и стык одинаковы
    let n = (total / step).floor().max(1.0) as usize;
    let adjusted_step = total / n as f32;

    for i in 0..n {
        let start = i as f32 * adjusted_step;
        let end = (start + seg_len).min(total);

        if is_dash {
            let dash_pts = points_along(&pts, &dists, start, end);
            if dash_pts.len() >= 2 {
                for w in dash_pts.windows(2) {
                    ui.painter().line_segment([w[0], w[1]], stroke);
                }
            }
            if border.round_cap {
                let r = border.width * 0.5;
                if r > 0.0 && dash_pts.len() >= 2 {
                    ui.painter().circle_filled(dash_pts[0], r, border.color);
                    ui.painter().circle_filled(dash_pts[dash_pts.len() - 1], r, border.color);
                }
            }
        } else {
            let p = point_at_dist(&pts, &dists, start + seg_len * 0.5);
            ui.painter().circle_filled(p, border.width.max(1.5) * 0.5, border.color);
        }
    }
}

/// Возвращает все точки из pts чья дистанция ∈ (d_start, d_end),
/// плюс интерполированные точки на границах.
fn points_along(pts: &[egui::Pos2], dists: &[f32], d_start: f32, d_end: f32) -> Vec<egui::Pos2> {
    let mut result = Vec::new();
    if d_end <= d_start || pts.len() < 2 { return result; }

    let start_i = find_index(dists, d_start);
    let end_i = find_index(dists, d_end);

    result.push(point_at_dist(pts, dists, d_start));

    for idx in (start_i + 1)..=end_i {
        if idx > 0 && idx < pts.len() {
            result.push(pts[idx]);
        }
    }

    if dists.len() > 1 {
        let last_end = point_at_dist(pts, dists, d_end);
        let last = result.last().copied().unwrap_or(last_end);
        let dist = last.distance(last_end);
        if dist > 0.5 {
            result.push(last_end);
        }
    }

    result
}

fn find_index(dists: &[f32], d: f32) -> usize {
    if d <= dists[0] { return 0; }
    let last = dists.len() - 1;
    if d >= dists[last] { return last; }
    let i = dists.binary_search_by(|&v| v.partial_cmp(&d).unwrap()).unwrap_or_else(|i| i.saturating_sub(1));
    i.min(last)
}

fn rounded_rect_perimeter(rect: egui::Rect, rounding: egui::CornerRadius, n: usize) -> Vec<egui::Pos2> {
    let r = |cr: u8| (cr as f32).min(rect.width() * 0.5).min(rect.height() * 0.5);
    let (tl, tr, br, bl) = (r(rounding.nw), r(rounding.ne), r(rounding.se), r(rounding.sw));

    let mut pts = Vec::new();

    // Top edge (left → right)
    edge_pts(&mut pts, tl, tr, rect.left() + tl, rect.top(), rect.right() - tr, rect.top(), n);
    // Top-right arc
    arc_pts(&mut pts, rect.right() - tr, rect.top() + tr, tr, -90.0, 0.0, n);
    // Right edge (top → bottom)
    edge_pts(&mut pts, tr, br, rect.right(), rect.top() + tr, rect.right(), rect.bottom() - br, n);
    // Bottom-right arc
    arc_pts(&mut pts, rect.right() - br, rect.bottom() - br, br, 0.0, 90.0, n);
    // Bottom edge (right → left)
    edge_pts(&mut pts, br, bl, rect.right() - br, rect.bottom(), rect.left() + bl, rect.bottom(), n);
    // Bottom-left arc
    arc_pts(&mut pts, rect.left() + bl, rect.bottom() - bl, bl, 90.0, 180.0, n);
    // Left edge (bottom → top)
    edge_pts(&mut pts, bl, tl, rect.left(), rect.bottom() - bl, rect.left(), rect.top() + tl, n);
    // Top-left arc
    arc_pts(&mut pts, rect.left() + tl, rect.top() + tl, tl, 180.0, 270.0, n);

    pts
}

fn edge_pts(pts: &mut Vec<egui::Pos2>, _r1: f32, _r2: f32, x1: f32, y1: f32, x2: f32, y2: f32, _n: usize) {
    let len = egui::vec2(x2 - x1, y2 - y1).length();
    if len <= 0.0 { return; }
    // Always add start point; end point will be added by next segment
    pts.push(egui::pos2(x1, y1));
}

fn arc_pts(pts: &mut Vec<egui::Pos2>, cx: f32, cy: f32, r: f32, start_deg: f32, end_deg: f32, n: usize) {
    if r <= 0.0 { return; }
    let steps = n.max(2);
    let start = start_deg.to_radians();
    let end = end_deg.to_radians();
    for i in 1..=steps {
        let a = start + (end - start) * (i as f32 / steps as f32);
        pts.push(egui::pos2(cx + r * a.cos(), cy + r * a.sin()));
    }
}

fn point_at_dist(pts: &[egui::Pos2], dists: &[f32], d: f32) -> egui::Pos2 {
    if d <= 0.0 { return pts[0]; }
    let last = dists.len() - 1;
    if d >= dists[last] { return pts[pts.len() - 1]; }
    let i = match dists.binary_search_by(|&v| v.partial_cmp(&d).unwrap()) {
        Ok(i) => i.min(last - 1),
        Err(i) => {
            if i == 0 { return pts[0]; }
            (i - 1).min(last - 1)
        }
    };
    let edge_start = dists[i];
    let edge_len = dists[i + 1] - edge_start;
    if edge_len <= 0.001 { return pts[i]; }
    let t = ((d - edge_start) / edge_len).clamp(0.0, 1.0);
    egui::pos2(pts[i].x + (pts[i + 1].x - pts[i].x) * t, pts[i].y + (pts[i + 1].y - pts[i].y) * t)
}

/// Удобная обёртка: читает border из узла/темы и рисует.
pub fn widget_border(
    ui: &mut egui::Ui,
    rect: egui::Rect,
    node: &serde_json::Value,
    theme: &Theme,
    widget: &str,
    rounding: egui::CornerRadius,
    resp: Option<&egui::Response>,
    enabled: bool,
) {
    let border = if let Some(r) = resp {
        get_state_border(node, theme, widget, r, enabled)
    } else {
        get_border(node, theme, widget)
    };
    draw_border(ui, rect, rounding, &border);
}
