# nobrainer

```mermaid
---
title: Anzahl fehlende Berichte
---
A(start)
A --> B[/ standart_weg /]
A --> C[/ max_number = 0/]
A --> D[ for file in $standar_weg ]
D --> E{Datei endet auf .Rmd}
E --> |Nein| D
E --> |Ja| F [Suche Nummer heraus]
