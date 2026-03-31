# apex
╔═══════════════════════════════════════════════════════════╗
║                    APEX — verze 1.1.0                     ║
║                      31. března 2026                      ║
╚═══════════════════════════════════════════════════════════╝

Třetí veřejná verze. Výrazně rozšířená — nové moduly,
8 grafických témat, anglická lokalizace, škála 1–10
a spousta quality-of-life vylepšení.

───────────────────────────────────────────────────────────
  INSTALACE
───────────────────────────────────────────────────────────

  Stáhni Apex_1.1.0_x64-setup.exe a spusť ho.
  Aplikace se sama nainstaluje včetně zástupce na ploše.
  Je možné, že aplikaci flagne antivirus - nech ho soubor oskenovat, je bezpečný.
  Žádné závislosti, žádná registrace.

  Data z předchozích verzí se automaticky převedou.

───────────────────────────────────────────────────────────
  CHANGELOG 1.1.0
───────────────────────────────────────────────────────────

  NOVÉ FUNKCE
  · Váha tracker — logování, trend graf, min/max/delta
  · Nálada & Energie v Životosprávě — zpětné zadávání
    se stejným UI jako ostatní zdravotní záznamy
  · Onboarding wizard — průvodce prvním spuštěním
    (jméno, moduly, první návyk)
  · Drag & drop řazení úkolů — přetahování pro vlastní
    pořadí s persistentním uložením
  · Undo toast — po smazání úkolu, cíle, návyku, poznámky
    nebo knihy máš 5 sekund na vrácení zpět
  · Streak statistiky — aktuální streak 🔥 + nejlepší
    streak 🏆 u každého návyku
  · Nedělní reminder — banner na dashboardu připomene
    týdenní review
  · Interaktivní chart tooltips — hover nad grafem ukáže
    datum a hodnotu
  · CSV export — export dat po modulech (nálada, spánek,
    návyky, váha, úkoly, focus)
  · Automatický backup — každých 7 dní záloha JSON
    (pouze v desktopové verzi)

  GRAFICKÁ ROZHRANÍ (7 TÉMAT)
  · Zeus — antické Řecko, zlaté akcenty, Cinzel font
  · Neon — cyberpunk, neonové barvy, JetBrains Mono
  · Zen — japonský minimalismus, tlumené tóny, Noto Serif
  · Terminal — retro CRT, zelená na černé, scanline efekt
  · Obsidian — ultra-premium dark, minimalistický
  · Rosé — feminní růžová, Quicksand font
  · Lavender — feminní fialová, jemné gradienty
  · Každé téma má vlastní dark i light variantu

  LOKALIZACE
  · Anglická (US) lokalizace — přepínání v nastavení
  · Kompletní překlad všech 280+ textových řetězců
  · Locale-aware formátování dat a grafů

  ŠKÁLA NÁLADY & ENERGIE
  · Rozšířena z 1–5 na 1–10 pro jemnější nuance
  · Stará data se automaticky přeškálují (×2)
  · 10 emoji pro náladu, 10 číselných tlačítek pro energii

  KLÁVESOVÉ ZKRATKY
  · Ctrl+N — nový úkol
  · Ctrl+S — uložit
  · Q — rychlý zápis myšlenky
  · 1–9 — přepínání sekcí
  · Esc — zavřít modal

  FADE ANIMACE
  · Plynulé přechody mezi sekcemi (fade + slide)
  · Modaly s scale-in efektem
  · Lifestyle taby s fade přechodem

  VÝKON
  · Asynchronní načítání theme-specifických fontů
  · backdrop-filter blur jen na otevřených modalech
  · Pauza animací na neviditelných sekcích
  · Eliminace všech transition: all pravidel

  BUGFIXES
  · 50+ opravených bugů napříč všemi moduly
  · XSS ochrana na všech uživatelských vstupech
  · Timezone-safe datové operace (localDateStr)
  · Timer zaznamenává skutečný čas, ne konfigurovaný
  · TickTick import správně mapuje priority
  · Save lock na všech formulářích proti dvojkliku
  · Null guards na DOM operacích
  · Správné řazení úkolů po importu

  CHANGELOG 1.0.1
  · Long-term goals nyní lze stanovit na daná období
  · Úkoly mají možnost nastavit štítek
  · Úkoly z TickTicku se importují společně se štítky
  · Zpětná kompatibilita uložených dat z 1.0.0
  · Bugfixes

───────────────────────────────────────────────────────────
  CO APEX UMÍ
───────────────────────────────────────────────────────────

  DASHBOARD
  · Denní check-in nálady (1–10) a energie (1–10)
  · Přehled dnešních úkolů, focus hodin, spánku a návyků
  · Streak badge u návyků přímo na dashboardu
  · Rychlý zápis myšlenek (klávesa Q)
  · Nedělní reminder na týdenní review
  · Vše se ukládá do historie

  DAILY TASKS
  · Tři úrovně priority (Vysoká / Střední / Nízká)
  · Drag & drop pro vlastní řazení
  · Vlastní štítky kompatibilní s TickTick importem
  · Import úkolů přímo z TickTick (CSV export)
  · Secondary a main tasks pro dnešní den
  · Editace, popis, opakování (denně, pracovní dny,
    vlastní dny)

  CÍLE
  · Dlouhodobé cíle rozdělené na týdenní, měsíční,
    kvartální a roční
  · Milestones s progress trackem
  · Nižší cíle vždy vedou k vyššímu — jasná hierarchie
    směrem k life goals
  · Týdenní review s reflexí

  FOCUS TIMER
  · Klasický pomodoro timer s work blocky a pauzami
  · Motivační citáty v češtině i angličtině
  · Automatický tracking — zpětně vidíš, kolik hodin
    jsi strávil/a focusem

  NÁVYKY
  · Sleduj dobré i špatné návyky
  · Aktuální streak + nejlepší streak
  · Koreluj je s daty ze životosprávy přes grafy

  ŽIVOTOSPRÁVA
  · Workouts, spánek, nálada & energie, Elonga,
    suplementy, váha, zdravotní kalendář
  · Vše vizualizováno v grafech — sleduj dlouhodobý
    vývoj svého stavu

  ANALYTIKA
  · Interaktivní grafy s hover tooltips
  · Nálada, energie, spánek, focus a návyky
  · Volitelné období (7 dní, 14 dní, měsíc, vlastní)
  · Uvidíš, jak životní styl ovlivňuje tvůj výkon
    (a jak ti drogy a chlast ničí život)

  POZNÁMKY & MYŠLENKY
  · Klasické poznámky pro delší zápisky
  · Rychlý zápis myšlenek — smaž nebo převeď do poznámek

  ČTENÍ
  · Databáze knih — chci přečíst / čtu / přečteno
  · Autor, hodnocení, poznámky

  DENÍK
  · Denní zápisník s datumem

  EXPORT & IMPORT
  · JSON záloha dat jedním kliknutím
  · CSV export po modulech (nálada, spánek, návyky,
    váha, úkoly, focus)
  · Automatický backup každých 7 dní
  · Snadný přenos dat při přechodu na nový počítač

  NASTAVENÍ
  · 8 grafických témat (výchozí + 7 unikátních)
  · Dark mode / light mode pro každé téma
  · Čeština / angličtina
  · Zapni/vypni jednotlivé moduly
  · Vlastní jméno v dashboardu

───────────────────────────────────────────────────────────
  POZNÁMKY
───────────────────────────────────────────────────────────

  · Data jsou uložena lokálně na tvém počítači
  · Před instalací nové verze doporučuji provést Export
  · Momentálně pouze Windows (x64), macOS brzy

───────────────────────────────────────────────────────────

  Baví vás to? Zpětná vazba vítána na IG @stepan.smatana

                                              — Apex v1.1.0
