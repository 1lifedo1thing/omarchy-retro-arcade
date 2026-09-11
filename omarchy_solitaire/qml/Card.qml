import QtQuick

Item {
    id: card
    property string rank: "A"
    property int value: 1
    property string suit: "♠"
    property bool red: false
    property bool faceUp: true
    property bool selected: false
    property bool focused: false
    property bool raised: false
    property bool destination: false
    property int pattern: game.preferences.pattern
    property string artwork: pattern === 3 ? game.customArt : theme.artwork
    property real phase: .55
    HoverHandler { id: cardHover }
    width: 112
    height: width * 1.43

    Rectangle {
        anchors.fill: parent
        anchors.topMargin: card.raised ? 6 : 2
        anchors.bottomMargin: card.raised ? -6 : -2
        anchors.leftMargin: card.raised ? 3 : 0
        anchors.rightMargin: card.raised ? -3 : 0
        radius: 7
        color: "#38000000"
    }
    Rectangle {
        id: body
        anchors.fill: parent
        radius: 6
        color: card.faceUp ? theme.colors.face : theme.colors.back
        border.width: card.selected || card.focused ? 3 : 1
        border.color: card.selected || card.focused ? theme.colors.accent : card.faceUp ? "#d7d5cd" : theme.colors.backPattern
    }

    Item {
        anchors.fill: parent
        visible: card.faceUp
        Text {
            x: card.width * .085; y: card.width * .04
            text: card.rank + "\n" + card.suit
            font.family: "DejaVu Serif"
            font.pixelSize: card.width * .19
            font.weight: Font.DemiBold
            lineHeight: .9
            horizontalAlignment: Text.AlignHCenter
            color: card.red ? theme.colors.red : theme.colors.ink
        }
        Text {
            anchors.right: parent.right; anchors.bottom: parent.bottom
            anchors.rightMargin: card.width * .085; anchors.bottomMargin: card.width * .04
            text: card.rank + "\n" + card.suit
            rotation: 180
            font.family: "DejaVu Serif"
            font.pixelSize: card.width * .19
            font.weight: Font.DemiBold
            lineHeight: .9
            horizontalAlignment: Text.AlignHCenter
            color: card.red ? theme.colors.red : theme.colors.ink
        }
        Item {
            anchors.centerIn: parent
            width: parent.width * .52
            height: parent.height * .63
            visible: card.value <= 10
            Repeater {
                model: card.value <= 10 ? card.value : 0
                Text {
                    required property int index
                    property var positions: {
                        const layouts = {
                            1: [[.5,.5]], 2: [[.5,.1],[.5,.9]], 3: [[.5,.1],[.5,.5],[.5,.9]],
                            4: [[.25,.1],[.75,.1],[.25,.9],[.75,.9]],
                            5: [[.25,.1],[.75,.1],[.5,.5],[.25,.9],[.75,.9]],
                            6: [[.25,.1],[.75,.1],[.25,.5],[.75,.5],[.25,.9],[.75,.9]],
                            7: [[.25,.1],[.75,.1],[.5,.3],[.25,.5],[.75,.5],[.25,.9],[.75,.9]],
                            8: [[.25,.1],[.75,.1],[.5,.3],[.25,.5],[.75,.5],[.5,.7],[.25,.9],[.75,.9]],
                            9: [[.25,.05],[.75,.05],[.25,.35],[.75,.35],[.5,.5],[.25,.65],[.75,.65],[.25,.95],[.75,.95]],
                            10: [[.25,.05],[.75,.05],[.5,.2],[.25,.35],[.75,.35],[.25,.65],[.75,.65],[.5,.8],[.25,.95],[.75,.95]]
                        }
                        return layouts[card.value] || [[.5,.5]]
                    }
                    x: positions[index][0] * parent.width - width / 2
                    y: positions[index][1] * parent.height - height / 2
                    text: card.suit
                    font.family: "DejaVu Serif"
                    font.pixelSize: card.width * (card.value === 1 ? .44 : card.value >= 9 ? .18 : .22)
                    color: card.red ? theme.colors.red : theme.colors.ink
                    rotation: positions[index][1] > .5 ? 180 : 0
                }
            }
        }
        Rectangle {
            visible: card.value > 10
            anchors.centerIn: parent
            width: parent.width * .49; height: parent.height * .57
            color: "transparent"
            border.width: 1
            border.color: card.red ? "#cc9c9e" : "#aab4ad"
            Rectangle {
                anchors.centerIn: parent
                width: parent.width * .69; height: width
                rotation: 45
                color: card.red ? "#f3e2dd" : "#e5e9e0"
            }
            Text {
                anchors.centerIn: parent
                text: card.rank + "\n" + card.suit
                font.family: "DejaVu Serif"
                font.pixelSize: card.width * .31
                font.bold: true
                horizontalAlignment: Text.AlignHCenter
                color: card.red ? theme.colors.red : theme.colors.ink
            }
        }
    }

    Item {
        anchors.fill: parent
        anchors.margins: 6
        visible: !card.faceUp
        clip: true
        Rectangle { anchors.fill: parent; color: "transparent"; border.color: theme.colors.backPattern; radius: 2 }
        Canvas {
            id: weave
            anchors.fill: parent
            visible: card.artwork === "" || artImage.status === Image.Error
            onWidthChanged: requestPaint()
            onHeightChanged: requestPaint()
            Connections { target: theme; function onChanged() { weave.requestPaint() } }
            Connections { target: card; function onPatternChanged() { weave.requestPaint() } }
            onPaint: {
                const c = getContext("2d")
                c.reset()
                c.strokeStyle = theme.colors.backPattern
                c.lineWidth = .6
                const step = card.pattern === 1 ? 15 : 9
                if (card.pattern !== 2) {
                    for (let i = -height; i < width + height; i += step) {
                        c.beginPath(); c.moveTo(i, 0); c.lineTo(i + height, height); c.stroke()
                        c.beginPath(); c.moveTo(i, 0); c.lineTo(i - height, height); c.stroke()
                    }
                }
            }
        }
        Image {
            id: artImage
            anchors.fill: parent
            anchors.margins: 2
            source: card.artwork
            visible: source.toString() !== "" && status === Image.Ready
            fillMode: Image.PreserveAspectFit
            cache: false
            asynchronous: true
        }
        Rectangle {
            visible: card.artwork === "" || artImage.status === Image.Error
            anchors.centerIn: parent
            width: card.width * .63; height: width
            radius: 3
            color: theme.colors.back
            border.color: theme.colors.backPattern
            Rectangle {
                anchors.fill: parent; anchors.margins: 4
                color: "#182622"; radius: 2
                Image {
                    anchors.fill: parent; anchors.margins: parent.width * .15
                    source: "../assets/omarchy-logo.svg"
                    fillMode: Image.PreserveAspectFit
                    sourceSize: Qt.size(180,180)
                }
            }
        }
    }
    Item {
        anchors.fill: parent; anchors.margins: 6
        clip: true
        visible: !card.faceUp && game.preferences.finish === "holographic"
        Rectangle {
            width: parent.width * .75; height: parent.height * 1.8
            x: -width + card.phase * (parent.width + width)
            y: -parent.height * .4
            rotation: 25
            opacity: .16
            gradient: Gradient {
                orientation: Gradient.Horizontal
                GradientStop { position: 0; color: "transparent" }
                GradientStop { position: .25; color: "#81d5fa" }
                GradientStop { position: .5; color: "#eccba3" }
                GradientStop { position: .75; color: "#dda5ec" }
                GradientStop { position: 1; color: "transparent" }
            }
        }
    }
    SequentialAnimation on phase {
        running: !card.faceUp && cardHover.hovered && game.preferences.finish === "holographic" && !game.preferences.reducedMotion
        loops: Animation.Infinite
        NumberAnimation { from: 0; to: 1; duration: 1700; easing.type: Easing.InOutSine }
        PauseAnimation { duration: 350 }
        onStopped: card.phase = .55
    }
    Rectangle {
        visible: card.selected || card.focused
        anchors.horizontalCenter: parent.horizontalCenter
        anchors.top: parent.top; anchors.topMargin: -3
        width: 18; height: 6; radius: 3
        color: theme.colors.accent
    }
    Rectangle {
        visible: card.destination
        anchors.horizontalCenter: parent.horizontalCenter
        anchors.top: parent.top; anchors.topMargin: 5
        width: 22; height: 20; radius: 4
        color: theme.colors.accent
        Text { anchors.centerIn: parent; text: "↓"; color: theme.colors.onAccent; font.pixelSize: 15; font.bold: true }
    }
}
