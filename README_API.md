# Storix Engine API (Stonix v1.3.0)

## Error Codes / Códigos de Error

| Code | ES | EN | DE | RU | ZH |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **0** | Éxito | Success | Erfolgreich | Успех | 成功 |
| **1** | Error General / Cifrado | General / Encryption Error | Allgemeiner Fehler | Общая ошибка | 通用/加密错误 |
| **2** | Espacio Insuficiente (<100MB) | Low Disk Space (<100MB) | Wenig Speicherplatz | Мало места на диске | 磁盘空间不足 |

---

## Functions / Funciones

### `stonix_put(key, value)`
- **ES**: Guarda un par clave-valor cifrado con sello de hardware.
- **EN**: Saves an encrypted key-value pair with hardware seal.
- **DE**: Speichert ein verschlüsseltes Schlüssel-Wert-Paar mit Hardware-Siegel.
- **RU**: Сохраняет зашифрованную пару ключ-значение с аппаратной печатью.
- **ZH**: 保存带有硬件密封 e 加密键值对。

### `stonix_get_stats()`
- **ES**: Devuelve JSON con el tamaño de la DB y espacio libre en disco.
- **EN**: Returns JSON with DB size and free disk space.
- **DE**: Gibt JSON mit DB-Größe und freiem Speicherplatz zurück.
- **RU**: Возвращает JSON с размером БД и свободным местом.
- **ZH**: 返回包含数据库大小和磁盘剩余空间的 JSON。
