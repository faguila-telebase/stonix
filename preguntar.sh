#!/bin/bash
# Protocolo TELARIX: Puente de Memoria Soberana
TEMA=$1
PREGUNTA=$2

QUERY_JSON="{\"id\": \"${TEMA}_historia_real\"}"

# Consultamos a Stonix
SALIDA_STONIX=$(./target/release/stonix --db stonix.db --query "$QUERY_JSON")

# Extraemos el campo "realidad" usando sed
CONTEXTO=$(echo "$SALIDA_STONIX" | sed -n 's/.*"realidad": "\([^"]*\)".*/\1/p')

if [ -z "$CONTEXTO" ]; then
    echo "⚠️ No hay registros previos en Stonix para: $TEMA"
    ollama run llama3 "$PREGUNTA"
else
    echo "🧠 Memoria recuperada de Stonix [O(1)]..."
    PROMPT_FINAL="INSTRUCCIÓN: Eres el archivista de TELARIX. Los siguientes datos de Stonix son verdades absolutas: $CONTEXTO. Pregunta: $PREGUNTA"
    ollama run llama3 "$PROMPT_FINAL"
fi
