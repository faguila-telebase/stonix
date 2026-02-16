#!/bin/bash
echo "🚀 Iniciando Benchmark de Stonix en i9..."
start_time=$(date +%s%3N)

for i in {1..100}
do
   # Generamos datos únicos
   DATA="BENCH_DATA_POINT_${i}_$(date +%s%N)"
   # Calculamos el Hash real
   HASH=$(echo -n "$DATA" | sha256sum | cut -d' ' -f1)
   # Enviamos al motor Stonix
   echo "$HASH:$DATA" | nc -q 0 localhost 6500 > /dev/null &
done

wait
end_time=$(date +%s%3N)
elapsed=$((end_time - start_time))

echo "-----------------------------------------------"
echo "✅ 100 Operaciones concurrentes enviadas"
echo "⏱️ Tiempo total de ejecución: $elapsed ms"
echo "💾 Estado del almacenamiento:"
du -h stonix_storage.bin
echo "-----------------------------------------------"
