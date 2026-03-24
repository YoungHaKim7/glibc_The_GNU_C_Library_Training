# C vs Rust pointer comparison

| Concept           | C               | Rust              |
| ----------------- | --------------- | ----------------- |
| pointer to struct | `String_View *` | `&mut StringView` |
| arrow             | `sv->count`     | `sv.count`        |
| address           | `&s`            | `&mut s`          |
| modify            | pointer         | mutable reference |
| pointer math      | allowed         | not allowed       |
| slicing           | manual          | built-in          |
