# Third-party components

Omarchy Chess is GPL-3.0-or-later. Full license: LICENSE.

- **python-chess / chess 1.11.2**, Niklas Fiekas and contributors: GPL-3.0-or-later. Used for rules, PGN, SVG pieces and UCI communication. https://github.com/niklasf/python-chess
- **Chess piece artwork** rendered by `chess.svg`: adapted by python-chess from the Wikimedia chess pieces by **Cburnett**, used under the GPL option stated in python-chess's SVG module. Artwork is supplied by that dependency; it is not copied into this repository. https://github.com/niklasf/python-chess/blob/master/chess/svg.py
- **Stockfish**, the Stockfish developers: GPL-3.0-or-later. External executable, not included in this application's wheel or Arch package. https://github.com/official-stockfish/Stockfish
- **Qt for Python / PySide6**, The Qt Company and contributors: LGPLv3/GPLv3/commercial licensing. Used as a separately installed dependency under compatible open-source terms. https://doc.qt.io/qtforpython-6/licenses.html

The small app icon in packaging/ is original project artwork, GPL-3.0-or-later. The app reads user-installed Omarchy theme colours but does not redistribute Omarchy branding or theme files.

If distributing a combined installer, retain the corresponding component licenses and provide the required corresponding source. This repository's package recipes keep dependencies separately managed.
