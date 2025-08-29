# NoBrainer
    _   _  _  _          _  _  
|\ | | |_||_||_| | |\ | |_ |_|  
| \|_| |_||\ | | | | \| |_ |\  


## Error:
- [x] -h oder help tut mal sowas von garnicht, es wird immer trozdem das main Programm gestartet
- [x] -f macht nicht was es soll, es müsste bei -f die VAR um -1 sein, da sonst die aktuelle Woche verlangt wird

## Ideen für 1.1
- [ ] Bunt?
- [ ] alle bisherigen Themen in einer Liste?

## Ideen:
- [ ] wenn kein Thema genommen wird, erst schauen, ob es noch welche in einer Liste gibt?
    - [ ] dafür muss erstmal eine Liste angelegt werden, diese liegt schlecht, da es keinen Sinn macht eine Liste in den NoBrainer Ordner zu legen
- [x] Jahresurlaub, fza als Option einbauen
    - also praktisch einfach -J oder -F / --fza
- [ ] abchecken, ob Thema schon genommen wurde, mit 80% Sicherheit oder so
    - [ ] evtl mit grep? danach y/n ob thema genommen werden soll???
    - [ ] beachten, dass es Jahresurlaub und so öfters gibt
- [x] schauen, wie viele Berichte ausstehen
- [ ] nächstes datum mit timy aufrufen um zu sehen, was darin abgeht


## Geplante Funktionen:

## release:
1.04    feature:  
            - -w jahreswochen hinzufuegen mit überblick, welche gerade ist  
                - geradige in Farbe?  
            - -t timy anzeigen  
            - bei -F führt -f mit -t aus  
1.03    feature: -f soll timy ausführen für die versch Wochen geradige Woche ausführen, logo hinzugefügt  
1.02    bug: -f sagt nun 0 bei allen Wochen, exklusive der geradigen  
1.01    feature: option `-f/ --fehlende_berichte`  
1.0     first stable Version  


```mermaid
---
title: TITEL
---
flowchart LR

(Hallo)
```

