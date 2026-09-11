import QtQuick
import QtQuick.Controls

FocusScope {
    id: board
    property real availableHeight: 600
    property bool compact: availableHeight < 460
    property real gap: Math.max(14, width * .018)
    property real cardWidth: Math.min(128, (width - gap * 6 - 56) / 7, compact ? Math.max(70, (availableHeight - 68) / 3.7) : 128)
    property real cardHeight: cardWidth * 1.43
    property real insetX: (width - cardWidth * 7 - gap * 6) / 2
    property real topY: compact ? 16 : 36
    property real tableY: topY + cardHeight + (compact ? 36 : 62)
    signal revealFocus(real position, real extent)
    property int dragPile: -1
    property int dragRow: -1
    property bool dragging: false
    property real dragX: 0
    property real dragY: 0
    property int focusPile: 0
    property int focusRow: 0
    property bool keyboardUsed: false
    property real neededHeight: {
        let bottom = tableY + cardHeight + (compact ? 16 : 30)
        for (const c of game.cards) {
            if (c.pile >= 6) bottom = Math.max(bottom, tableY + c.offset * cardWidth + cardHeight + (compact ? 16 : 30))
        }
        return bottom
    }
    implicitHeight: neededHeight
    Accessible.role: Accessible.Pane
    Accessible.name: "Solitaire table. Arrow keys move focus. Enter selects and places. Space draws. F moves to a foundation."

    function column(pile) { return pile < 2 ? pile : pile < 6 ? pile + 1 : pile - 6 }
    function pileX(pile) { return insetX + column(pile) * (cardWidth + gap) }
    function pileY(pile) { return pile < 6 ? topY : tableY }
    function pileAt(px, py) {
        if (py < topY || px < insetX || px > width - insetX) return -1
        let col = Math.floor((px - insetX) / (cardWidth + gap))
        if (col < 0 || col > 6) return -1
        if (py >= tableY - 12) return col + 6
        if (py <= topY + cardHeight + 18) return col < 2 ? col : col > 2 ? col - 1 : -1
        return -1
    }
    function rowAt(pile) { return Math.max(0, game.counts[pile] - 1) }
    function resetDrag() { dragging = false; dragPile = -1; dragRow = -1; dragX = 0; dragY = 0 }
    function activate() { game.choose(focusPile, game.counts[focusPile] ? focusRow : -1) }
    function focusColumn(delta) {
        focusPile = (focusPile + delta + 13) % 13
        focusRow = rowAt(focusPile)
    }
    Keys.onPressed: event => {
        keyboardUsed = true
        if (event.key === Qt.Key_Left) focusColumn(-1)
        else if (event.key === Qt.Key_Right) focusColumn(1)
        else if (event.key === Qt.Key_Up) {
            if (focusPile >= 6 && focusRow > 0) focusRow--
            else { focusPile = Math.max(0, focusPile - 6); focusRow = rowAt(focusPile) }
        } else if (event.key === Qt.Key_Down) {
            if (focusPile < 6) { focusPile += 6; focusRow = rowAt(focusPile) }
            else focusRow = Math.min(game.counts[focusPile] - 1, focusRow + 1)
        } else if (event.key === Qt.Key_Return || event.key === Qt.Key_Enter) activate()
        else if (event.key === Qt.Key_Space) game.draw()
        else if (event.key === Qt.Key_F) game.toFoundation(focusPile, focusRow)
        else if (event.key === Qt.Key_Escape) { resetDrag(); game.clearSelection() }
        else { event.accepted = false; return }
        let focusY = pileY(focusPile)
        for (const c of game.cards) {
            if (c.pile === focusPile && c.row === focusRow && focusPile >= 6) focusY += c.offset * cardWidth
        }
        revealFocus(focusY, cardHeight)
        event.accepted = true
    }
    Connections {
        target: game
        function onBoardChanged() {
            board.focusRow = Math.min(board.focusRow, Math.max(0, game.counts[board.focusPile] - 1))
        }
    }

    Repeater {
        model: 13
        Item {
            required property int index
            x: board.pileX(index); y: board.pileY(index)
            width: board.cardWidth; height: board.cardHeight
            Rectangle {
                anchors.fill: parent
                radius: 6
                color: "transparent"
                border.width: game.status.hintTarget === index || (board.activeFocus && board.keyboardUsed && board.focusPile === index) ? 2 : 1
                border.color: game.status.hintTarget === index || (board.activeFocus && board.keyboardUsed && board.focusPile === index) ? theme.colors.accent : theme.colors.line
                Text {
                    anchors.centerIn: parent
                    text: index === 0 ? (game.counts[1] ? "↻" : "—") : index === 1 ? "" : index < 6 ? "A" : "K"
                    font.family: "DejaVu Serif"
                    font.pixelSize: board.cardWidth * .29
                    color: theme.colors.muted
                    opacity: .65
                }
                Rectangle {
                    visible: board.dragging ? game.canMove(board.dragPile, board.dragRow, index) : game.legalTargets.indexOf(index) >= 0
                    anchors.horizontalCenter: parent.horizontalCenter
                    anchors.top: parent.top; anchors.topMargin: 5
                    width: 22; height: 20; radius: 4
                    color: theme.colors.accent
                    Text { anchors.centerIn: parent; text: "↓"; color: theme.colors.onAccent; font.pixelSize: 15 }
                }
            }
            Text {
                anchors.top: parent.bottom; anchors.topMargin: 12
                width: parent.width
                text: index === 0 ? "STOCK · " + game.counts[0] : index === 1 ? "DRAW " + game.status.draw : index < 6 ? "FOUNDATION" : ""
                font.pixelSize: 9; font.letterSpacing: 1.4
                horizontalAlignment: Text.AlignHCenter
                color: theme.colors.muted
            }
            Accessible.role: Accessible.Button
            Accessible.name: index === 0 ? "Draw from stock" : index === 1 ? "Waste" : index < 6 ? "Foundation " + (index - 1) : "Column " + (index - 5)
            Accessible.onPressAction: game.choose(index, game.counts[index] - 1)
            MouseArea {
                anchors.fill: parent
                onClicked: {
                    board.forceActiveFocus()
                    board.keyboardUsed = false
                    board.focusPile = index
                    board.focusRow = board.rowAt(index)
                    game.choose(index, game.counts[index] - 1)
                }
                cursorShape: Qt.PointingHandCursor
            }
        }
    }

    Repeater {
        id: cards
        model: game.cards
        Card {
            id: tile
            required property var modelData
            property bool inDrag: board.dragging && board.dragPile === modelData.pile && modelData.row >= board.dragRow
            x: board.pileX(modelData.pile) + (modelData.pile === 1 ? modelData.fan * board.cardWidth * .19 : 0) + (inDrag ? board.dragX : 0)
            y: board.pileY(modelData.pile) + (modelData.pile >= 6 ? modelData.offset * board.cardWidth : 0) + (inDrag ? board.dragY : 0)
            z: (inDrag ? 1000 : modelData.pile * 30) + modelData.row
            width: board.cardWidth
            height: board.cardHeight
            visible: modelData.shown
            rank: modelData.rank
            value: modelData.value
            suit: modelData.suit
            red: modelData.red
            faceUp: modelData.faceUp
            selected: game.status.selectedPile === modelData.pile && modelData.row >= game.status.selectedRow
            focused: board.activeFocus && board.keyboardUsed && board.focusPile === modelData.pile && board.focusRow === modelData.row
            raised: inDrag
            destination: modelData.row === game.counts[modelData.pile] - 1 && (board.dragging ? game.canMove(board.dragPile, board.dragRow, modelData.pile) : game.legalTargets.indexOf(modelData.pile) >= 0)
            Accessible.role: Accessible.Button
            Accessible.name: modelData.name + (modelData.pile >= 6 ? ", column " + (modelData.pile - 5) : "")
            Accessible.ignored: !visible || !modelData.faceUp
            Accessible.onPressAction: game.choose(modelData.pile, modelData.row)
            MouseArea {
                id: pointer
                anchors.fill: parent
                acceptedButtons: Qt.LeftButton | Qt.RightButton
                property real originX: 0
                property real originY: 0
                preventStealing: board.dragging
                cursorShape: modelData.movable ? (pressed ? Qt.ClosedHandCursor : Qt.OpenHandCursor) : modelData.pile === 0 ? Qt.PointingHandCursor : Qt.ArrowCursor
                onPressed: mouse => {
                    board.forceActiveFocus()
                    board.keyboardUsed = false
                    const pt = mapToItem(board, mouse.x, mouse.y)
                    originX = pt.x; originY = pt.y
                    board.focusPile = modelData.pile; board.focusRow = modelData.row
                    if (mouse.button === Qt.LeftButton && modelData.movable && !game.status.completing) {
                        board.dragPile = modelData.pile; board.dragRow = modelData.row
                    }
                }
                onPositionChanged: mouse => {
                    if (!pressed || board.dragPile < 0) return
                    const pt = mapToItem(board, mouse.x, mouse.y)
                    const dx = pt.x - originX, dy = pt.y - originY
                    if (Math.abs(dx) + Math.abs(dy) > 7) board.dragging = true
                    if (board.dragging) { board.dragX = dx; board.dragY = dy }
                }
                onReleased: mouse => {
                    const pt = mapToItem(board, mouse.x, mouse.y)
                    const wasDrag = board.dragging
                    const source = modelData.pile, row = modelData.row
                    const target = board.pileAt(pt.x, pt.y)
                    board.resetDrag()
                    if (wasDrag) game.move(source, row, target)
                    else if (mouse.button === Qt.RightButton) game.toFoundation(source, row)
                    else game.choose(source, row)
                }
                onDoubleClicked: game.toFoundation(modelData.pile, modelData.row)
                onCanceled: board.resetDrag()
            }
        }
    }

    Text {
        x: board.insetX + 2 * (board.cardWidth + board.gap)
        y: board.topY + board.cardHeight * .42
        width: board.cardWidth
        text: "♠"
        font.family: "DejaVu Serif"
        font.pixelSize: 30
        color: theme.colors.line
        horizontalAlignment: Text.AlignHCenter
    }
}
