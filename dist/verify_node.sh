#!/bin/bash
# TELARIX Node Verification Script

if sha256sum -c checksums.txt; then
  echo "✅ Integridad de Stonix confirmada. Sello de hardware listo."
else
  echo "❌ ERROR: El binario ha sido manipulado o está corrupto."
  exit 1
fi