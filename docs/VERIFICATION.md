# Verification and remaining acceptance

## Local evidence

- Python 3.12; PySide6 6.11.2; chess 1.11.2.
- 33 tests passed, including real Stockfish opening/hint play, cancellation, a silent-engine timeout, promotion selection, keyboard/mouse/drag interaction, history input protection, PGN validation, draw handling, resume and corrupt-save preservation.
- Stockfish compiled from the upstream source in this environment; generic x86-64 build. This executable is test infrastructure and is not included in the app.
- Native window rendered and visually inspected at 1060×780 and 740×560 using Qt's offscreen platform.
- Lint and format checks applied. CI also validates the desktop entry and builds Python wheel/source and Arch package artifacts.

## Still requires a real desktop

This environment is not Tom's Dell and does not provide an Omarchy/Hyprland session. Before a stable release:

- Install the native packages; confirm launcher icon and own-window behaviour.
- Play full games as both colours, including a promotion and a completed game.
- Switch actual Omarchy themes while playing; inspect dark and light themes.
- Test Wayland, monitor scaling, resizing, keyboard focus, file dialogs and resume after reboot.
- Check screen-reader access; custom board accessibility is incomplete.
- Check acceptable battery/CPU use and perceived difficulty on the Dell.
- Confirm the chosen Stockfish distribution/install route is simple enough.

No stable release, AUR submission or marketplace listing has been published by this work.
