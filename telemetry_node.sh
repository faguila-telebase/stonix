#!/bin/bash
TARGET_IP="127.0.0.1"
TARGET_PORT=6500
NODE_ID="OP5P_NODE_01"

echo "📡 Transmitiendo telemetría a Stonix..."

while true; do
    # Métricas
    CPU_LOAD=$(top -bn1 | grep "Cpu(s)" | awk '{print $2}')
    TEMP=$(cat /sys/class/thermal/thermal_zone0/temp 2>/dev/null || echo "0")
    TEMP_C=$(echo "scale=1; $TEMP / 1000" | bc)
    RAM_FREE=$(free -m | awk '/Mem:/ { print $4 }')

    # Mensaje y Hash
    DATA="[$NODE_ID] CPU:$CPU_LOAD% | TEMP:${TEMP_C}C | RAM_FREE:${RAM_FREE}MB"
    HASH=$(echo -n "$DATA" | sha256sum | cut -d' ' -f1)

    # Envío
    echo -n "$HASH:$DATA" | nc -w 1 $TARGET_IP $TARGET_PORT
    
    sleep 5
done
