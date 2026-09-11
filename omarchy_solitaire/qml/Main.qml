import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import QtQuick.Dialogs

ApplicationWindow {
    id: window
    objectName: "mainWindow"
    width: 1120; height: 800
    minimumWidth: 800; minimumHeight: 600
    visible: true
    title: "Omarchy Solitaire"
    color: theme.colors.table
    font.family: "DejaVu Sans"
    palette.window: theme.colors.table
    palette.windowText: theme.colors.text
    palette.base: theme.colors.surface
    palette.text: theme.colors.text
    palette.button: theme.colors.surface
    palette.buttonText: theme.colors.text
    palette.highlight: theme.colors.accent
    palette.highlightedText: theme.colors.onAccent
    property bool celebrating: false
    onActiveChanged: game.setActive(active)
    onClosing: game.save()

    Shortcut { sequence: "Ctrl+N"; onActivated: newDeal.open() }
    Shortcut { sequence: "Ctrl+Z"; enabled: !newDeal.visible; onActivated: game.undo() }
    Shortcut { sequence: "H"; enabled: !settings.visible && !newDeal.visible; onActivated: game.hint() }
    Shortcut { sequence: "F1"; onActivated: help.open() }
    Shortcut { sequence: "Ctrl+,"; onActivated: settings.open() }
    Shortcut { sequence: "Ctrl+Q"; onActivated: window.close() }

    header: Item {
        height: 136
        Rectangle { anchors.fill: parent; color: theme.colors.surface }
        ColumnLayout {
            anchors.fill: parent
            anchors.leftMargin: 28; anchors.rightMargin: 28
            anchors.topMargin: 18; anchors.bottomMargin: 14
            spacing: 16
            RowLayout {
                spacing: 12
                Image { source: "../assets/omarchy-logo.svg"; Layout.preferredWidth: 25; Layout.preferredHeight: 25 }
                Text { text: "SOLITAIRE"; color: theme.colors.text; font.pixelSize: 16; font.weight: Font.DemiBold; font.letterSpacing: 3 }
                Rectangle { Layout.preferredWidth: 1; Layout.preferredHeight: 18; color: theme.colors.line }
                Text { text: "A little time well spent."; color: theme.colors.muted; font.pixelSize: 12; visible: window.width > 900 }
                Item { Layout.fillWidth: true }
                Text { text: theme.name; color: theme.colors.muted; font.pixelSize: 11; elide: Text.ElideRight; Layout.maximumWidth: 160 }
                Rectangle { width: 7; height: 7; radius: 4; color: theme.colors.accent }
            }
            RowLayout {
                spacing: 8
                TableButton { objectName: "newButton"; text: "New game"; primary: true; onClicked: newDeal.open() }
                TableButton { objectName: "undoButton"; text: "↶  Undo"; enabled: game.status.canUndo; onClicked: game.undo(); ToolTip.visible: hovered; ToolTip.text: "Undo the last move · Ctrl+Z" }
                TableButton { objectName: "hintButton"; text: "Hint"; enabled: !game.status.won && !game.status.completing; onClicked: game.hint(); ToolTip.visible: hovered; ToolTip.text: "Show a legal suggestion · H" }
                TableButton { text: game.status.completing ? "Stop" : "Finish"; visible: game.status.canComplete || game.status.completing; onClicked: game.status.completing ? game.stopCompletion() : game.complete() }
                Item { Layout.fillWidth: true }
                Text { text: "KLONDIKE · DRAW " + game.status.draw; color: theme.colors.muted; font.pixelSize: 10; font.letterSpacing: 1 }
                TableButton { text: "Deck & settings"; onClicked: settings.open() }
                TableButton { text: "?"; implicitWidth: 38; Accessible.name: "How to play"; onClicked: help.open() }
            }
        }
        Rectangle { anchors.bottom: parent.bottom; width: parent.width; height: 1; color: theme.colors.line }
    }

    Flickable {
        id: scroller
        anchors.fill: parent
        contentWidth: width
        contentHeight: Math.max(height, table.neededHeight)
        clip: true
        boundsBehavior: Flickable.StopAtBounds
        ScrollBar.vertical: ScrollBar { policy: scroller.contentHeight > scroller.height ? ScrollBar.AsNeeded : ScrollBar.AlwaysOff }
        Board {
            id: table
            objectName: "board"
            width: scroller.width
            height: Math.max(scroller.height, neededHeight)
            availableHeight: scroller.height
            focus: true
            onRevealFocus: (position, extent) => {
                if (position < scroller.contentY) scroller.contentY = position
                else if (position + extent > scroller.contentY + scroller.height) scroller.contentY = Math.min(scroller.contentHeight - scroller.height, position + extent - scroller.height + 8)
            }
        }
    }

    footer: Rectangle {
        height: 83
        color: theme.colors.surface
        Rectangle { width: parent.width; height: 1; color: theme.colors.line }
        ColumnLayout {
            anchors.fill: parent; anchors.margins: 12
            anchors.leftMargin: 28; anchors.rightMargin: 28
            spacing: 7
            RowLayout {
                spacing: 20
                Text { text: game.status.time; font.pixelSize: 18; font.family: "DejaVu Sans Mono"; color: theme.colors.text }
                Text { text: game.status.moves + " moves"; font.pixelSize: 12; color: theme.colors.muted }
                Text { text: game.status.score + " points"; font.pixelSize: 12; color: theme.colors.muted }
                Item { Layout.fillWidth: true }
                // Functional signature: the small home tally shows progress without an intrusive meter.
                Row {
                    spacing: 3
                    Repeater {
                        model: 13
                        Rectangle { required property int index; width: 5; height: 13; radius: 1; color: game.status.foundationCount >= (index + 1) * 4 ? theme.colors.accent : theme.colors.line }
                    }
                }
                Text { text: game.status.foundationCount + " / 52 home"; font.pixelSize: 11; color: theme.colors.muted }
                TableButton { text: "Statistics"; implicitHeight: 28; onClicked: statistics.open() }
            }
            Text {
                objectName: "statusMessage"
                Layout.fillWidth: true
                text: game.message
                color: theme.colors.muted
                font.pixelSize: 11
                elide: Text.ElideRight
                Accessible.role: Accessible.StaticText
                Accessible.name: text
                ToolTip.visible: statusHover.hovered
                ToolTip.text: text
                HoverHandler { id: statusHover }
            }
        }
    }

    Dialog {
        id: newDeal
        title: "A fresh table"
        parent: Overlay.overlay
        anchors.centerIn: parent
        width: 390
        modal: true
        standardButtons: Dialog.Cancel
        onOpened: game.setPaused(true)
        onClosed: { game.setPaused(false); table.forceActiveFocus() }
        ColumnLayout {
            width: parent.width; spacing: 16
            Label { text: "Start a new deal, or have another go at this one.\nYour current table will be replaced."; wrapMode: Text.WordWrap; Layout.fillWidth: true }
            TableButton { text: "New game · Draw one"; primary: true; Layout.fillWidth: true; onClicked: { game.newGame(1, false); newDeal.close() } }
            TableButton { text: "New game · Draw three"; Layout.fillWidth: true; onClicked: { game.newGame(3, false); newDeal.close() } }
            TableButton { text: "Restart this deal"; Layout.fillWidth: true; onClicked: { game.newGame(game.status.draw, true); newDeal.close() } }
            Label { text: "Deal " + game.status.seed; font.pixelSize: 10; color: theme.colors.muted }
        }
    }

    Dialog {
        id: settings
        title: "Your deck, your table"
        parent: Overlay.overlay
        anchors.centerIn: parent
        width: 550
        height: Math.min(690, window.height - 32)
        modal: true
        standardButtons: Dialog.Close
        onOpened: game.setPaused(true)
        onClosed: { game.setPaused(false); table.forceActiveFocus() }
        contentItem: ScrollView {
            clip: true
            ColumnLayout {
            width: settings.availableWidth; spacing: 16
            Label { text: "The Omarchy edition"; font.pixelSize: 18; font.weight: Font.Medium }
            Label { text: "Official mark. Colours from your desktop."; color: theme.colors.muted }
            RowLayout {
                spacing: 14
                Repeater {
                    model: ["Woven", "Diamond", "Minimal"]
                    ColumnLayout {
                        required property int index
                        required property string modelData
                        spacing: 6
                        Card {
                            width: 90; height: 129
                            faceUp: false
                            pattern: index
                            artwork: ""
                            selected: game.preferences.pattern === index
                            Accessible.role: Accessible.Button
                            Accessible.name: modelData + " card back"
                            Accessible.onPressAction: game.preference("pattern", index)
                            MouseArea { anchors.fill: parent; cursorShape: Qt.PointingHandCursor; onClicked: game.preference("pattern", index) }
                        }
                        TableButton { text: modelData; implicitWidth: 90; onClicked: game.preference("pattern", index) }
                    }
                }
                Item { Layout.fillWidth: true }
                ColumnLayout {
                    Card { width: 90; height: 129; faceUp: false; pattern: 3; selected: game.preferences.pattern === 3; artwork: game.customArt }
                    TableButton { text: "Custom"; implicitWidth: 90; enabled: game.customArt !== ""; onClicked: game.preference("pattern", 3) }
                }
            }
            CheckBox { text: "Follow the active Omarchy theme"; checked: game.preferences.followTheme; onClicked: game.preference("followTheme", checked) }
            CheckBox { text: "Holographic finish · shimmers on hover"; checked: game.preferences.finish === "holographic"; onClicked: game.preference("finish", checked ? "holographic" : "matte") }
            CheckBox { text: "Reduce motion"; checked: game.preferences.reducedMotion; onClicked: game.preference("reducedMotion", checked) }
            TableButton { text: "Import a special card back…"; onClicked: artPicker.open() }
            Label { text: "SVG or PNG · up to 2 MB · 5:7 artwork recommended\nCustom artwork replaces the whole back. Include the decal in your design."; wrapMode: Text.WordWrap; Layout.fillWidth: true; color: theme.colors.muted; font.pixelSize: 11 }
            Label { visible: theme.artwork !== ""; text: "This Omarchy theme supplies its own card artwork."; color: theme.colors.accent }
            Label { visible: theme.artworkError !== ""; text: theme.artworkError; wrapMode: Text.WordWrap; Layout.fillWidth: true }
            Label { text: game.message; wrapMode: Text.WordWrap; Layout.fillWidth: true; font.pixelSize: 11; color: theme.colors.muted }
            }
        }
    }
    FileDialog {
        id: artPicker
        title: "Choose a card back"
        nameFilters: ["Card artwork (*.svg *.png)"]
        fileMode: FileDialog.OpenFile
        onAccepted: game.importBack(selectedFile)
    }

    Dialog {
        id: help
        title: "Make yourself at home"
        parent: Overlay.overlay
        anchors.centerIn: parent
        width: 520
        height: Math.min(590, window.height - 32)
        modal: true
        standardButtons: Dialog.Close
        onOpened: game.setPaused(true)
        onClosed: { game.setPaused(false); table.forceActiveFocus() }
        contentItem: ScrollView {
            clip: true
            ColumnLayout {
            width: help.availableWidth; spacing: 15
            Label { text: "Get all 52 cards onto the four foundations, building each suit from ace to king."; wrapMode: Text.WordWrap; Layout.fillWidth: true; font.pixelSize: 15 }
            Label { text: "On the table, build down in alternating colours. Move a single card or a whole ordered sequence. Only a king can fill an empty column. Draw from the stock when you need another card. Recycling is unlimited."; wrapMode: Text.WordWrap; Layout.fillWidth: true }
            Label { text: "Click a card, then its destination, or drag it there.\nDouble-click or right-click to move a card to a foundation."; wrapMode: Text.WordWrap; Layout.fillWidth: true }
            Label { text: "← →  Move between piles       ↑ ↓  Choose a card\nEnter  Select / place                  Space  Draw\nF  Send to foundation                 Esc  Clear selection\nH  Hint     Ctrl+Z  Undo     Ctrl+N  New game\nCtrl+,  Settings     F1  Help     Ctrl+Q  Quit"; font.pixelSize: 12; Layout.fillWidth: true }
            Label { text: "Scoring: +10 to a foundation, +5 from waste to table, +5 for revealing a card; −15 back from a foundation, −20 for recycling. Undo restores the previous score. The timer pauses while the window is inactive or a dialog is open."; wrapMode: Text.WordWrap; Layout.fillWidth: true; color: theme.colors.muted; font.pixelSize: 11 }
            Label { text: "Hints suggest legal moves; random deals are not guaranteed winnable. Finish appears when the stock and waste are empty and every table card is face up."; wrapMode: Text.WordWrap; Layout.fillWidth: true; color: theme.colors.muted; font.pixelSize: 11 }
            Label { text: "Omarchy Solitaire 0.1.0 · Community application\nOmarchy artwork belongs to its respective owners."; font.pixelSize: 10; color: theme.colors.muted }
            }
        }
    }

    Dialog {
        id: statistics
        title: "Time at the table"
        parent: Overlay.overlay
        anchors.centerIn: parent
        width: 420
        modal: true
        standardButtons: Dialog.Close
        onOpened: game.setPaused(true)
        onClosed: { game.setPaused(false); table.forceActiveFocus() }
        ColumnLayout {
            width: parent.width; spacing: 18
            Repeater {
                model: ["1", "3"]
                ColumnLayout {
                    required property string modelData
                    property var record: game.status.statistics[modelData]
                    Label { text: "Draw " + modelData; font.pixelSize: 17 }
                    Label { text: record.won + " won / " + record.played + " started  ·  " + (record.played ? Math.round(record.won * 100 / record.played) : 0) + "%" }
                    Label { text: record.won ? "Best time: " + Math.floor(record.best / 60) + "m " + record.best % 60 + "s" : "Your first win is waiting."; color: theme.colors.muted }
                }
            }
            Label { text: "A deal counts when you make your first move.\nRestarting or undoing doesn't count the same deal twice."; font.pixelSize: 11; color: theme.colors.muted }
        }
    }

    Item {
        id: winLayer
        anchors.fill: parent
        visible: game.status.won
        clip: true
        Repeater {
            model: window.celebrating && !game.preferences.reducedMotion ? 28 : 0
            Card {
                required property int index
                property real progress: 0
                width: 64; height: 92
                rank: ["A", "K", "Q", "J"][index % 4]
                value: [1,13,12,11][index % 4]
                suit: ["♠","♥","♣","♦"][index % 4]
                red: index % 2 === 1
                x: winLayer.width * .55 + index % 4 * 52 - progress * (winLayer.width * .9 + index * 9)
                y: 25 + Math.abs(Math.sin(progress * Math.PI * 2.3)) * (winLayer.height - 130) + progress * 50
                rotation: progress * (index % 2 ? 28 : -28)
                SequentialAnimation on progress {
                    running: window.celebrating
                    PauseAnimation { duration: index * 65 }
                    NumberAnimation { from: 0; to: 1.2; duration: 2300 }
                }
            }
        }
        Rectangle {
            anchors.horizontalCenter: parent.horizontalCenter
            anchors.bottom: parent.bottom; anchors.bottomMargin: 24
            width: 380; height: 156; radius: 8
            color: theme.colors.surface; border.color: theme.colors.accent
            ColumnLayout {
                anchors.centerIn: parent; spacing: 12
                Label { text: "All home."; font.family: "DejaVu Serif"; font.pixelSize: 30; Layout.alignment: Qt.AlignHCenter }
                Label { text: game.status.time + "  ·  " + game.status.moves + " moves  ·  " + game.status.score + " points"; color: theme.colors.muted; Layout.alignment: Qt.AlignHCenter }
                TableButton { text: "Another game?"; primary: true; Layout.alignment: Qt.AlignHCenter; onClicked: newDeal.open() }
            }
        }
    }
    Connections {
        target: game
        function onVictory() { if (!game.preferences.reducedMotion) { window.celebrating = true; celebrationEnd.restart() } }
    }
    Timer { id: celebrationEnd; interval: 4300; onTriggered: window.celebrating = false }
}
